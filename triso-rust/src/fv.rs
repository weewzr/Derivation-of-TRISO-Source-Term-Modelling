use crate::config::Config;
use anyhow::{bail, Result};
use std::f64::consts::PI;

#[derive(Clone, Debug)]
pub struct Cell {
    pub left: f64,
    pub right: f64,
    pub centroid: f64,
    pub volume: f64,
    pub material: usize,
    pub diffusivity: f64,
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub cells: Vec<Cell>,
    pub conductance: Vec<f64>,
    pub outer_conductance: f64,
    pub outer_radius: f64,
}

pub type TimeSeries = Vec<(f64, StepDiagnostics)>;

#[derive(Clone, Debug)]
pub struct StepDiagnostics {
    pub inventory: f64,
    pub cumulative_release: f64,
    pub release_fraction: f64,
    pub release_rate: f64,
    pub balance_residual: f64,
    pub min_concentration: f64,
}

pub fn shell_volume(a: f64, b: f64) -> f64 {
    4.0 * PI / 3.0 * (b.powi(3) - a.powi(3))
}

pub fn volume_centroid(a: f64, b: f64) -> f64 {
    if a == 0.0 {
        0.75 * b
    } else {
        0.75 * (b.powi(4) - a.powi(4)) / (b.powi(3) - a.powi(3))
    }
}

pub fn build_mesh(cfg: &Config) -> Result<Mesh> {
    cfg.validate()?;
    let mut cells = Vec::new();
    let mut inner = 0.0;
    for (mat, region) in cfg.regions.iter().enumerate() {
        let width = (region.outer_radius_m - inner) / region.cells as f64;
        for k in 0..region.cells {
            let left = inner + k as f64 * width;
            let right = if k + 1 == region.cells {
                region.outer_radius_m
            } else {
                inner + (k + 1) as f64 * width
            };
            cells.push(Cell {
                left,
                right,
                centroid: volume_centroid(left, right),
                volume: shell_volume(left, right),
                material: mat,
                diffusivity: region.diffusivity_m2_s,
            });
        }
        inner = region.outer_radius_m;
    }
    if cells.len() < 2 {
        bail!("mesh requires at least two cells");
    }

    let mut conductance = Vec::with_capacity(cells.len() - 1);
    for pair in cells.windows(2) {
        let left = &pair[0];
        let right = &pair[1];
        let face = left.right;
        let area = 4.0 * PI * face * face;
        let dl = face - left.centroid;
        let dr = right.centroid - face;
        conductance.push(area / (dl / left.diffusivity + dr / right.diffusivity));
    }
    let last = cells.last().expect("nonempty mesh");
    let outer_radius = cfg.regions.last().expect("five regions").outer_radius_m;
    let outer_area = 4.0 * PI * outer_radius * outer_radius;
    let outer_conductance =
        outer_area * last.diffusivity / (outer_radius - last.centroid);

    Ok(Mesh {
        cells,
        conductance,
        outer_conductance,
        outer_radius,
    })
}

pub fn initial_concentration(cfg: &Config, mesh: &Mesh) -> Vec<f64> {
    mesh.cells
        .iter()
        .map(|c| {
            if c.material == 0 {
                cfg.initial.kernel_concentration_mol_m3
            } else {
                0.0
            }
        })
        .collect()
}

pub fn inventory(mesh: &Mesh, c: &[f64]) -> f64 {
    mesh.cells
        .iter()
        .zip(c)
        .map(|(cell, &x)| cell.volume * x)
        .sum()
}

fn solve_tridiagonal(a: &[f64], b: &[f64], c: &[f64], rhs: &[f64]) -> Result<Vec<f64>> {
    let n = b.len();
    if a.len() != n || c.len() != n || rhs.len() != n || n == 0 {
        bail!("invalid tridiagonal dimensions");
    }
    let mut cp = vec![0.0; n];
    let mut dp = vec![0.0; n];
    if b[0].abs() < 1e-300 {
        bail!("singular tridiagonal pivot");
    }
    cp[0] = c[0] / b[0];
    dp[0] = rhs[0] / b[0];
    for i in 1..n {
        let den = b[i] - a[i] * cp[i - 1];
        if den.abs() < 1e-300 {
            bail!("singular tridiagonal pivot");
        }
        cp[i] = if i + 1 == n { 0.0 } else { c[i] / den };
        dp[i] = (rhs[i] - a[i] * dp[i - 1]) / den;
    }
    let mut x = vec![0.0; n];
    x[n - 1] = dp[n - 1];
    for i in (0..n - 1).rev() {
        x[i] = dp[i] - cp[i] * x[i + 1];
    }
    Ok(x)
}

pub fn backward_euler_step(mesh: &Mesh, old: &[f64], dt: f64) -> Result<Vec<f64>> {
    let n = mesh.cells.len();
    if old.len() != n || dt <= 0.0 {
        bail!("invalid state or timestep");
    }
    let mut a = vec![0.0; n];
    let mut b = vec![0.0; n];
    let mut c = vec![0.0; n];
    let mut rhs = vec![0.0; n];

    for i in 0..n {
        let gw = if i == 0 { 0.0 } else { mesh.conductance[i - 1] };
        let ge = if i + 1 == n {
            mesh.outer_conductance
        } else {
            mesh.conductance[i]
        };
        a[i] = if i == 0 { 0.0 } else { -gw };
        b[i] = mesh.cells[i].volume / dt + gw + ge;
        c[i] = if i + 1 == n { 0.0 } else { -ge };
        rhs[i] = mesh.cells[i].volume * old[i] / dt;
    }
    solve_tridiagonal(&a, &b, &c, &rhs)
}

pub fn run(
    cfg: &Config,
    mesh: &Mesh,
) -> Result<(Vec<(f64, StepDiagnostics)>, Vec<f64>)> {
    let mut c = initial_concentration(cfg, mesh);
    let initial = inventory(mesh, &c);
    if initial <= 0.0 {
        bail!("initial inventory must be positive");
    }
    let steps = (cfg.time.end_time_s / cfg.time.dt_s).ceil() as usize;
    let dt = cfg.time.end_time_s / steps as f64;
    let mut released = 0.0;
    let mut rows = Vec::new();

    rows.push((
        0.0,
        StepDiagnostics {
            inventory: initial,
            cumulative_release: 0.0,
            release_fraction: 0.0,
            release_rate: mesh.outer_conductance * c[c.len() - 1],
            balance_residual: 0.0,
            min_concentration: c.iter().copied().fold(f64::INFINITY, f64::min),
        },
    ));

    for step in 1..=steps {
        let next = backward_euler_step(mesh, &c, dt)?;
        let release_rate = mesh.outer_conductance * next[next.len() - 1];
        released += dt * release_rate;
        c = next;
        let inv = inventory(mesh, &c);
        let residual = inv + released - initial;
        if step % cfg.time.output_every_steps == 0 || step == steps {
            rows.push((
                step as f64 * dt,
                StepDiagnostics {
                    inventory: inv,
                    cumulative_release: released,
                    release_fraction: released / initial,
                    release_rate,
                    balance_residual: residual,
                    min_concentration: c.iter().copied().fold(f64::INFINITY, f64::min),
                },
            ));
        }
    }
    Ok((rows, c))
}

pub fn explicit_dt_bound(mesh: &Mesh) -> f64 {
    mesh.cells
        .iter()
        .enumerate()
        .map(|(i, cell)| {
            let gw = if i == 0 { 0.0 } else { mesh.conductance[i - 1] };
            let ge = if i + 1 == mesh.cells.len() {
                mesh.outer_conductance
            } else {
                mesh.conductance[i]
            };
            cell.volume / (gw + ge)
        })
        .fold(f64::INFINITY, f64::min)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, InitialConfig, OutputConfig, RegionConfig, TimeConfig};

    fn cfg(cells: usize) -> Config {
        Config {
            case_name: "test".into(),
            species: "synthetic".into(),
            temperature_k: 1000.0,
            regions: (0..5)
                .map(|i| RegionConfig {
                    name: format!("r{i}"),
                    outer_radius_m: (i + 1) as f64 * 1e-4,
                    diffusivity_m2_s: 1e-10,
                    cells,
                })
                .collect(),
            initial: InitialConfig {
                kernel_concentration_mol_m3: 1.0,
            },
            time: TimeConfig {
                dt_s: 0.1,
                end_time_s: 1.0,
                output_every_steps: 1,
            },
            output: OutputConfig {
                directory: "target/test".into(),
            },
        }
    }

    #[test]
    fn interfaces_are_faces() {
        let c = cfg(4);
        let m = build_mesh(&c).unwrap();
        for r in c.regions.iter().take(4) {
            assert!(m.cells.iter().any(|x| (x.right - r.outer_radius_m).abs() < 1e-15));
        }
    }

    #[test]
    fn internal_conductances_are_positive() {
        let m = build_mesh(&cfg(3)).unwrap();
        assert!(m.conductance.iter().all(|&g| g > 0.0));
        assert!(m.outer_conductance > 0.0);
    }

    #[test]
    fn backward_euler_is_positive_and_conservative() {
        let c = cfg(3);
        let m = build_mesh(&c).unwrap();
        let (rows, _) = run(&c, &m).unwrap();
        assert!(rows.iter().all(|(_, d)| d.min_concentration >= -1e-13));
        let initial = rows[0].1.inventory;
        let max_res = rows
            .iter()
            .map(|(_, d)| d.balance_residual.abs() / initial)
            .fold(0.0, f64::max);
        assert!(max_res < 1e-10);
    }

    #[test]
    fn explicit_bound_is_finite() {
        assert!(explicit_dt_bound(&build_mesh(&cfg(2)).unwrap()).is_finite());
    }
}
