use anyhow::Result;
use std::f64::consts::PI;
use std::time::Instant;
use triso_rust::{
    config::{Config, InitialConfig, OutputConfig, RegionConfig, TimeConfig},
    fv,
};

fn make_cfg(cells_per_region: usize, dt: f64, end: f64, ds: [f64; 5]) -> Config {
    let radii = [2.0e-4, 3.0e-4, 3.5e-4, 4.0e-4, 4.5e-4];
    Config {
        case_name: "synthetic-five-layer".into(),
        species: "synthetic".into(),
        temperature_k: 1000.0,
        regions: (0..5)
            .map(|i| RegionConfig {
                name: ["Kernel", "Buffer", "IPyC", "SiC", "OPyC"][i].into(),
                outer_radius_m: radii[i],
                diffusivity_m2_s: ds[i],
                cells: cells_per_region,
            })
            .collect(),
        initial: InitialConfig {
            kernel_concentration_mol_m3: 1.0,
        },
        time: TimeConfig {
            dt_s: dt,
            end_time_s: end,
            output_every_steps: usize::MAX,
        },
        output: OutputConfig {
            directory: "target/verify".into(),
        },
    }
}

fn sphere_release(t: f64, r: f64, d: f64) -> f64 {
    let mut s = 0.0;
    for n in 1..10000 {
        let term = (-((n * n) as f64) * PI * PI * d * t / (r * r)).exp() / (n * n) as f64;
        s += term;
        if term.abs() < 1e-15 {
            break;
        }
    }
    1.0 - 6.0 / (PI * PI) * s
}

fn homogeneous() -> Result<()> {
    let cfg = make_cfg(40, 0.5, 1.0e3, [1e-10; 5]);
    let mesh = fv::build_mesh(&cfg)?;
    let mut c = vec![1.0; mesh.cells.len()];
    let m0 = fv::inventory(&mesh, &c);
    let mut rel = 0.0;
    let steps = (cfg.time.end_time_s / cfg.time.dt_s) as usize;
    for _ in 0..steps {
        c = fv::backward_euler_step(&mesh, &c, cfg.time.dt_s)?;
        rel += cfg.time.dt_s * mesh.outer_conductance * c[c.len() - 1];
    }
    let f = rel / m0;
    let exact = sphere_release(cfg.time.end_time_s, mesh.outer_radius, 1e-10);
    println!(
        "HOMOGENEOUS fv={f:.10} exact={exact:.10} abs={:.3e}",
        (f - exact).abs()
    );
    assert!((f - exact).abs() < 0.01);
    Ok(())
}

fn two_layer_control() -> Result<()> {
    // Five numerical/physical labels, but regions 1-4 share D2; this exactly reduces
    // to a two-diffusivity problem with the only D jump at r=50 um.
    let radii = [50e-6, 60e-6, 70e-6, 85e-6, 100e-6];
    let mut cfg = make_cfg(20, 0.002, 32.0, [1e-10, 1e-9, 1e-9, 1e-9, 1e-9]);
    for (i, r) in radii.iter().enumerate() {
        cfg.regions[i].outer_radius_m = *r;
    }
    let mesh = fv::build_mesh(&cfg)?;
    let (rows, _) = fv::run(&cfg, &mesh)?;
    let f = rows.last().unwrap().1.release_fraction;
    println!(
        "TWO_LAYER final_t=32 fv={f:.10} reference=0.99998700 abs={:.3e}",
        (f - 0.999987).abs()
    );
    assert!((f - 0.999987).abs() < 0.01);
    Ok(())
}

fn five_layer_and_convergence() -> Result<()> {
    let ds = [1.0e-12, 4.0e-10, 8.0e-13, 2.0e-14, 7.0e-13];
    let end = 2.0e5;
    let start = Instant::now();
    let mut spatial = Vec::new();
    for n in [20usize, 40, 80, 160, 320] {
        let cfg = make_cfg(n, 2.0e2, end, ds);
        let mesh = fv::build_mesh(&cfg)?;
        let (rows, _) = fv::run(&cfg, &mesh)?;
        let last = &rows.last().unwrap().1;
        println!(
            "SPATIAL cells={} F={:.12e} balance_rel={:.3e} min_c={:.3e}",
            mesh.cells.len(),
            last.release_fraction,
            last.balance_residual.abs() / rows[0].1.inventory,
            last.min_concentration
        );
        assert!(last.min_concentration >= -1e-12);
        assert!(last.balance_residual.abs() / rows[0].1.inventory < 1e-9);
        spatial.push(last.release_fraction);
    }
    for w in spatial.windows(2) {
        println!("SPATIAL_DIFF {:.12e}", (w[1] - w[0]).abs());
    }
    let mut temporal = Vec::new();
    for dt in [8.0e2, 4.0e2, 2.0e2, 1.0e2] {
        let cfg = make_cfg(160, dt, end, ds);
        let mesh = fv::build_mesh(&cfg)?;
        let (rows, _) = fv::run(&cfg, &mesh)?;
        let last = &rows.last().unwrap().1;
        println!(
            "TEMPORAL dt={dt:.3e} F={:.12e} balance_rel={:.3e}",
            last.release_fraction,
            last.balance_residual.abs() / rows[0].1.inventory
        );
        temporal.push(last.release_fraction);
    }
    for w in temporal.windows(2) {
        println!("TEMPORAL_DIFF {:.12e}", (w[1] - w[0]).abs());
    }
    println!("VERIFY_RUNTIME_S {:.6}", start.elapsed().as_secs_f64());
    Ok(())
}

fn main() -> Result<()> {
    homogeneous()?;
    two_layer_control()?;
    five_layer_and_convergence()?;
    println!("FV_VERIFICATION PASS");
    Ok(())
}
