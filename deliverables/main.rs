//! Code A, main file.
//!
//! Reads one input file, then runs
//! - code B (`discrete_finite_volume_tridiagonal.rs`): the discrete finite-volume approach with a tridiagonal solve, and
//! - code C (`analytical_sturm_liouville.rs`): the analytical Sturm-Liouville eigenfunction series,
//!
//! on the same inputs. It writes the CSV results, the comparison of B with C and the figures.
//!
//! This file also holds the input-file reader (`config`), the comparison and refinement
//! study (`compare`) and the figure drawing (`plots`).

// Some helper functions are only used by the extra checks in `verification/code_checks`.
#![allow(dead_code)]

use self::{config::Config, discrete_finite_volume_tridiagonal as fv};
use anyhow::Result;
use std::{env, fs, path::Path};

// Code B and code C are separate files in this folder.
pub mod analytical_sturm_liouville;
pub mod discrete_finite_volume_tridiagonal;

pub mod config {
    use anyhow::{bail, Context, Result};
    use serde::Deserialize;
    use std::{fs, path::Path};

    pub const PLACEHOLDER: &str = "INSERT_VALUE_HERE";

    #[derive(Clone, Debug, Deserialize)]
    pub struct RegionConfig {
        pub name: String,
        pub outer_radius_m: f64,
        pub diffusivity_m2_s: f64,
        pub cells: usize,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct InitialConfig {
        pub kernel_concentration_mol_m3: f64,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct TimeConfig {
        pub dt_s: f64,
        pub end_time_s: f64,
        pub output_every_steps: usize,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct OutputConfig {
        pub directory: String,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Config {
        pub case_name: String,
        pub species: String,
        pub temperature_k: f64,
        pub regions: Vec<RegionConfig>,
        pub initial: InitialConfig,
        pub time: TimeConfig,
        pub output: OutputConfig,
    }

    /// Report every uncommented `INSERT_VALUE_HERE` with the instruction comment above it,
    /// so a half-filled template fails with a readable list instead of a TOML type error.
    fn unfilled_placeholders(raw: &str) -> Vec<String> {
        let mut last_instruction = String::new();
        let mut found = Vec::new();
        for (i, line) in raw.lines().enumerate() {
            let trimmed = line.trim_start();
            if trimmed.starts_with('#') {
                if trimmed.starts_with("# INSERT") && trimmed.ends_with("BELOW") {
                    last_instruction = trimmed.trim_start_matches('#').trim().to_string();
                }
            } else if trimmed.contains(PLACEHOLDER) {
                found.push(format!("line {}: {}", i + 1, last_instruction));
            }
        }
        found
    }

    impl Config {
        pub fn load(path: impl AsRef<Path>) -> Result<Self> {
            let raw = fs::read_to_string(path.as_ref())
                .with_context(|| format!("reading {}", path.as_ref().display()))?;
            let missing = unfilled_placeholders(&raw);
            if !missing.is_empty() {
                bail!(
                    "{} literature value(s) in {} are still {PLACEHOLDER}:\n  {}",
                    missing.len(),
                    path.as_ref().display(),
                    missing.join("\n  ")
                );
            }
            let cfg: Self = toml::from_str(&raw).context("parsing TOML")?;
            cfg.validate()?;
            Ok(cfg)
        }

        pub fn validate(&self) -> Result<()> {
            if self.regions.len() != 5 {
                bail!("exactly five physical regions are required");
            }
            if self.species.trim().is_empty() {
                bail!("species must not be empty");
            }
            if !(self.temperature_k.is_finite() && self.temperature_k > 0.0) {
                bail!("temperature_k must be a positive finite number (kelvin)");
            }
            let mut last = 0.0;
            for r in &self.regions {
                if !(r.outer_radius_m.is_finite() && r.diffusivity_m2_s.is_finite())
                    || r.outer_radius_m <= last
                    || r.diffusivity_m2_s <= 0.0
                    || r.cells == 0
                {
                    bail!("invalid region {}", r.name);
                }
                last = r.outer_radius_m;
            }
            let t = &self.time;
            if !(t.dt_s.is_finite() && t.end_time_s.is_finite())
                || t.dt_s <= 0.0
                || t.end_time_s <= 0.0
                || t.output_every_steps == 0
            {
                bail!("invalid time controls");
            }
            let c0 = self.initial.kernel_concentration_mol_m3;
            if !c0.is_finite() || c0 < 0.0 {
                bail!("initial concentration must be nonnegative and finite");
            }
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn placeholder_scan_reports_instruction_and_line() {
            let raw = "a = 1\n# INSERT LITERATURE-VERIFIED KERNEL DIFFUSIVITY (m^2/s) BELOW\nd = \"INSERT_VALUE_HERE\"\n# END X\n# d2 = \"INSERT_VALUE_HERE\"\n";
            let found = unfilled_placeholders(raw);
            assert_eq!(found.len(), 1);
            assert!(found[0].contains("line 3"));
            assert!(found[0].contains("KERNEL DIFFUSIVITY"));
        }
    }
}

pub mod compare {
    //! Cross-check of the FV solver against the exact analytical release series for the
    //! SAME inputs (same radii, diffusivities, initial kernel inventory, absorbing outer
    //! surface), plus a mesh/time refinement study measured against the exact answer.

    use super::{
        analytical_sturm_liouville::{self as analytical, Layer, ModalRelease},
        config::Config,
        discrete_finite_volume_tridiagonal as fv,
    };
    use anyhow::{bail, Result};

    /// Upper bound on eigenmodes kept for the exact series.
    pub const MAX_MODES: usize = 20_000;

    pub fn layers_from_config(cfg: &Config) -> Vec<Layer> {
        cfg.regions
            .iter()
            .map(|r| Layer {
                outer_radius: r.outer_radius_m,
                diffusivity: r.diffusivity_m2_s,
            })
            .collect()
    }

    #[derive(Clone, Debug)]
    pub struct CompareRow {
        pub time_s: f64,
        pub fv: f64,
        /// `None` when the time is earlier than the exact series can resolve.
        pub exact: Option<f64>,
    }

    #[derive(Clone, Debug)]
    pub struct ReleaseComparison {
        pub rows: Vec<CompareRow>,
        pub modes: usize,
        pub lambda_covered: f64,
        pub min_valid_time_s: f64,
        pub max_abs_error: f64,
        pub final_abs_error: f64,
        /// See `ModalRelease::min_flux_ratio`.
        pub min_flux_ratio: f64,
        /// See `ModalRelease::accuracy_floor`.
        pub accuracy_floor: f64,
    }

    /// Compare FV release fractions with the exact series at every positive output time.
    pub fn compare_release(cfg: &Config, fv_rows: &fv::TimeSeries) -> Result<ReleaseComparison> {
        let layers = layers_from_config(cfg);
        let times: Vec<f64> = fv_rows
            .iter()
            .map(|(t, _)| *t)
            .filter(|t| *t > 0.0)
            .collect();
        if times.is_empty() {
            bail!("no positive output times to compare");
        }
        let t_min = times.iter().copied().fold(f64::INFINITY, f64::min);
        let series = ModalRelease::new(&layers, 45.0 / t_min, MAX_MODES)?;
        let valid = series.min_valid_time();
        let mut rows = Vec::new();
        let (mut max_err, mut final_err) = (0.0f64, f64::NAN);
        for (t, d) in fv_rows.iter().filter(|(t, _)| *t > 0.0) {
            let exact = (*t >= valid).then(|| series.release_fraction(*t));
            if let Some(e) = exact {
                let err = (d.release_fraction - e).abs();
                max_err = max_err.max(err);
                final_err = err;
            }
            rows.push(CompareRow {
                time_s: *t,
                fv: d.release_fraction,
                exact,
            });
        }
        if rows.iter().all(|r| r.exact.is_none()) {
            bail!(
            "exact series (capped at {MAX_MODES} modes) resolves only t >= {valid:.3e} s, later than every output time"
        );
        }
        Ok(ReleaseComparison {
            rows,
            modes: series.eigenvalues.len(),
            lambda_covered: series.lambda_covered,
            min_valid_time_s: valid,
            max_abs_error: max_err,
            final_abs_error: final_err,
            min_flux_ratio: series.min_flux_ratio,
            accuracy_floor: series.accuracy_floor(),
        })
    }

    #[derive(Clone, Debug)]
    pub struct ConvergencePoint {
        /// Total FV cells (spatial study) or timestep in seconds (temporal study).
        pub cells: usize,
        pub dt_s: f64,
        pub fv_release: f64,
        pub abs_error: f64,
    }

    #[derive(Clone, Debug)]
    pub struct ConvergenceStudy {
        pub exact_final: f64,
        /// Estimated absolute accuracy of the exact series (see `ModalRelease::accuracy_floor`).
        pub accuracy_floor: f64,
        pub spatial: Vec<ConvergencePoint>,
        pub temporal: Vec<ConvergencePoint>,
    }

    impl ConvergenceStudy {
        /// Observed order between successive refinements (each refinement halves h or dt).
        pub fn orders(points: &[ConvergencePoint]) -> Vec<f64> {
            points
                .windows(2)
                .map(|w| (w[0].abs_error / w[1].abs_error).log2())
                .collect()
        }

        /// True when every error is well above the accuracy floor of the exact series, so
        /// observed orders mean something.
        pub fn resolved(&self, points: &[ConvergencePoint]) -> bool {
            points
                .iter()
                .all(|p| p.abs_error > 3.0 * self.accuracy_floor)
        }
    }

    fn scaled_cells(cfg: &Config, factor: f64) -> Config {
        let mut c = cfg.clone();
        for r in &mut c.regions {
            r.cells = ((r.cells as f64 * factor).round() as usize).max(1);
        }
        c
    }

    fn with_dt(cfg: &Config, dt: f64) -> Config {
        let mut c = cfg.clone();
        c.time.dt_s = dt;
        c.time.output_every_steps = usize::MAX;
        c
    }

    fn final_release(cfg: &Config) -> Result<f64> {
        let mesh = fv::build_mesh(cfg)?;
        let (rows, _) = fv::run(cfg, &mesh)?;
        Ok(rows.last().expect("initial row").1.release_fraction)
    }

    /// Release fraction with the Backward-Euler time error removed by Richardson
    /// extrapolation, (8 F(dt/4) - 6 F(dt/2) + F(dt)) / 3. The error expansion of Backward
    /// Euler is a1 dt + a2 dt^2 + ..., so this cancels both leading terms.
    /// Used to isolate the spatial error in refinement studies.
    pub fn time_extrapolated_release(cfg: &Config) -> Result<f64> {
        let f = |m: f64| final_release(&with_dt(cfg, cfg.time.dt_s * m));
        Ok((8.0 * f(0.25)? - 6.0 * f(0.5)? + f(1.0)?) / 3.0)
    }

    /// Rough work estimate (cell-steps) for one FV run.
    pub fn run_cost(cfg: &Config) -> f64 {
        let cells: usize = cfg.regions.iter().map(|r| r.cells).sum();
        cfg.time.end_time_s / cfg.time.dt_s * cells as f64
    }

    /// Estimated cell-steps for the whole refinement study.
    pub fn study_cost(cfg: &Config) -> f64 {
        let spatial: f64 = [0.5, 1.0, 2.0, 4.0, 8.0]
            .iter()
            .map(|f| run_cost(&scaled_cells(cfg, *f)) * 1.75)
            .sum();
        let temporal: f64 = [4.0, 2.0, 1.0, 0.5, 0.25]
            .iter()
            .map(|m| run_cost(&with_dt(cfg, cfg.time.dt_s * m)))
            .sum();
        spatial + temporal
    }

    /// Spatial study: cells per region scaled by 0.5, 1, 2, 4, 8. To keep the Backward-Euler
    /// time error from masking the spatial error, each FV value is the time-extrapolated
    /// value from `time_extrapolated_release`.
    /// Temporal study: dt multiplied by 4, 2, 1, 1/2, 1/4 on the configured mesh.
    /// Errors are measured against the exact series at the end time.
    pub fn convergence_study(cfg: &Config) -> Result<ConvergenceStudy> {
        let layers = layers_from_config(cfg);
        let t_end = cfg.time.end_time_s;
        let series = ModalRelease::new(&layers, 45.0 / t_end, MAX_MODES)?;
        if t_end < series.min_valid_time() {
            bail!("exact series does not resolve the end time");
        }
        let exact_final = series.release_fraction(t_end);

        let mut spatial = Vec::new();
        for f in [0.5, 1.0, 2.0, 4.0, 8.0] {
            let c = scaled_cells(cfg, f);
            let extrapolated = time_extrapolated_release(&c)?;
            spatial.push(ConvergencePoint {
                cells: c.regions.iter().map(|r| r.cells).sum(),
                dt_s: cfg.time.dt_s,
                fv_release: extrapolated,
                abs_error: (extrapolated - exact_final).abs(),
            });
        }

        let mut temporal = Vec::new();
        for m in [4.0, 2.0, 1.0, 0.5, 0.25] {
            let c = with_dt(cfg, cfg.time.dt_s * m);
            let f = final_release(&c)?;
            temporal.push(ConvergencePoint {
                cells: c.regions.iter().map(|r| r.cells).sum(),
                dt_s: cfg.time.dt_s * m,
                fv_release: f,
                abs_error: (f - exact_final).abs(),
            });
        }
        Ok(ConvergenceStudy {
            exact_final,
            accuracy_floor: series.accuracy_floor(),
            spatial,
            temporal,
        })
    }

    /// Exact release fraction at one time for a configuration (helper for tests and tools).
    pub fn exact_release_at(cfg: &Config, t: f64) -> Result<f64> {
        let v = analytical::exact_release(&layers_from_config(cfg), &[t], MAX_MODES)?;
        v[0].ok_or_else(|| anyhow::anyhow!("exact series does not resolve t = {t:e} s"))
    }
}

pub mod plots {
    //! SVG figures written with `plotters` (no system fonts or image libraries needed).

    use super::compare::{ConvergencePoint, ConvergenceStudy, ReleaseComparison};
    use anyhow::{anyhow, Result};
    use plotters::prelude::*;
    use std::path::Path;

    fn pe<E: std::fmt::Display>(e: E) -> anyhow::Error {
        anyhow!("plot error: {e}")
    }

    fn sci(v: &f64) -> String {
        format!("{v:.0e}")
    }

    fn positive_range(values: impl Iterator<Item = f64>) -> (f64, f64) {
        let (mut lo, mut hi) = (f64::INFINITY, 0.0f64);
        for v in values.filter(|v| *v > 0.0 && v.is_finite()) {
            lo = lo.min(v);
            hi = hi.max(v);
        }
        if !lo.is_finite() || hi <= lo {
            let base = if lo.is_finite() { lo } else { 1.0 };
            return (base * 0.5, base * 2.0);
        }
        (lo / 1.5, hi * 1.5)
    }

    /// Release fraction F(t): FV against the exact series, with the signed error below.
    pub fn release_comparison(path: &Path, case: &str, cmp: &ReleaseComparison) -> Result<()> {
        let root = SVGBackend::new(path, (900, 760)).into_drawing_area();
        root.fill(&WHITE).map_err(pe)?;
        let (upper, lower) = root.split_vertically(470);

        let (t_lo, t_hi) = positive_range(cmp.rows.iter().map(|r| r.time_s));
        let mut chart = ChartBuilder::on(&upper)
            .caption(
                format!("Release fraction, FV vs exact series ({case})"),
                ("sans-serif", 20),
            )
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d((t_lo..t_hi).log_scale(), 0.0f64..1.05f64)
            .map_err(pe)?;
        chart
            .configure_mesh()
            .x_desc("time (s)")
            .y_desc("release fraction F")
            .x_labels(6)
            .x_label_formatter(&sci)
            .light_line_style(TRANSPARENT)
            .draw()
            .map_err(pe)?;
        chart
            .draw_series(LineSeries::new(
                cmp.rows.iter().map(|r| (r.time_s, r.fv)),
                BLUE.stroke_width(2),
            ))
            .map_err(pe)?
            .label("FV, Backward Euler")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE.stroke_width(2)));
        chart
            .draw_series(cmp.rows.iter().filter_map(|r| {
                r.exact
                    .map(|e| Circle::new((r.time_s, e), 3, RED.stroke_width(1)))
            }))
            .map_err(pe)?
            .label("exact eigenfunction series")
            .legend(|(x, y)| Circle::new((x + 10, y), 3, RED.stroke_width(1)));
        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperLeft)
            .background_style(WHITE.mix(0.85))
            .border_style(BLACK)
            .draw()
            .map_err(pe)?;

        let errs: Vec<(f64, f64)> = cmp
            .rows
            .iter()
            .filter_map(|r| r.exact.map(|e| (r.time_s, r.fv - e)))
            .collect();
        let bound = errs
            .iter()
            .map(|(_, e)| e.abs())
            .fold(0.0f64, f64::max)
            .max(1e-12)
            * 1.2;
        let mut err_chart = ChartBuilder::on(&lower)
            .caption("Signed error FV - exact", ("sans-serif", 16))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(70)
            .build_cartesian_2d((t_lo..t_hi).log_scale(), -bound..bound)
            .map_err(pe)?;
        err_chart
            .configure_mesh()
            .x_desc("time (s)")
            .y_desc("F_FV - F_exact")
            .x_labels(6)
            .x_label_formatter(&sci)
            .y_label_formatter(&|v| format!("{v:.1e}"))
            .light_line_style(TRANSPARENT)
            .draw()
            .map_err(pe)?;
        err_chart
            .draw_series(LineSeries::new(errs, RED.stroke_width(2)))
            .map_err(pe)?;
        root.present().map_err(pe)?;
        Ok(())
    }

    fn loglog_panel<DB: DrawingBackend>(
        area: &DrawingArea<DB, plotters::coord::Shift>,
        title: &str,
        x_desc: &str,
        points: &[(f64, f64)],
        slope: f64,
        floor: f64,
    ) -> Result<()> {
        let (x_lo, x_hi) = positive_range(points.iter().map(|p| p.0));
        let ys: Vec<f64> = points.iter().map(|p| p.1.max(1e-16)).collect();
        let (y_lo, y_hi) = positive_range(ys.iter().copied().chain(std::iter::once(floor)));
        let mut chart = ChartBuilder::on(area)
            .caption(title, ("sans-serif", 16))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(70)
            .build_cartesian_2d((x_lo..x_hi).log_scale(), (y_lo..y_hi).log_scale())
            .map_err(pe)?;
        chart
            .configure_mesh()
            .x_desc(x_desc)
            .y_desc("|F_FV - F_exact| at end time")
            .x_labels(6)
            .y_labels(6)
            .x_label_formatter(&sci)
            .y_label_formatter(&sci)
            .light_line_style(TRANSPARENT)
            .draw()
            .map_err(pe)?;
        chart
            .draw_series(LineSeries::new(
                points.iter().zip(&ys).map(|(p, y)| (p.0, *y)),
                BLUE.stroke_width(2),
            ))
            .map_err(pe)?
            .label("FV error")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE.stroke_width(2)));
        chart
            .draw_series(
                points
                    .iter()
                    .zip(&ys)
                    .map(|(p, y)| Circle::new((p.0, *y), 4, BLUE.filled())),
            )
            .map_err(pe)?;
        // Reference slope anchored at the first point.
        let (x0, y0) = (points[0].0, ys[0]);
        let x1 = points[points.len() - 1].0;
        let guide = vec![(x0, y0), (x1, y0 * (x1 / x0).powf(slope))];
        chart
            .draw_series(LineSeries::new(
                guide.into_iter().filter(|(_, y)| *y >= y_lo && *y <= y_hi),
                BLACK.stroke_width(1),
            ))
            .map_err(pe)?
            .label(format!("slope {slope:+.0} guide"))
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLACK));
        chart
            .draw_series(LineSeries::new(
                vec![(x_lo, floor), (x_hi, floor)],
                RED.mix(0.7).stroke_width(1),
            ))
            .map_err(pe)?
            .label("analytical accuracy floor")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED.mix(0.7)));
        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::LowerLeft)
            .background_style(WHITE.mix(0.85))
            .border_style(BLACK)
            .draw()
            .map_err(pe)?;
        Ok(())
    }

    /// Spatial (left) and temporal (right) error against the exact series.
    pub fn convergence(path: &Path, case: &str, study: &ConvergenceStudy) -> Result<()> {
        let root = SVGBackend::new(path, (1100, 480)).into_drawing_area();
        root.fill(&WHITE).map_err(pe)?;
        let (left, right) = root.split_horizontally(550);
        let pts = |v: &[ConvergencePoint], x: fn(&ConvergencePoint) -> f64| -> Vec<(f64, f64)> {
            v.iter().map(|p| (x(p), p.abs_error)).collect()
        };
        loglog_panel(
            &left,
            &format!("Spatial refinement ({case})"),
            "total FV cells",
            &pts(&study.spatial, |p| p.cells as f64),
            -2.0,
            study.accuracy_floor,
        )?;
        loglog_panel(
            &right,
            &format!("Timestep refinement ({case})"),
            "timestep dt (s)",
            &pts(&study.temporal, |p| p.dt_s),
            1.0,
            study.accuracy_floor,
        )?;
        root.present().map_err(pe)?;
        Ok(())
    }

    /// Final concentration profile with the four material interfaces marked.
    pub fn profile(
        path: &Path,
        case: &str,
        radius: &[f64],
        conc: &[f64],
        layers: &[(String, f64)],
    ) -> Result<()> {
        let root = SVGBackend::new(path, (900, 480)).into_drawing_area();
        root.fill(&WHITE).map_err(pe)?;
        let r_max = radius.last().copied().unwrap_or(1.0) * 1.02;
        let c_max = conc.iter().copied().fold(0.0f64, f64::max).max(1e-300) * 1.1;
        let mut chart = ChartBuilder::on(&root)
            .caption(
                format!("Final concentration profile ({case})"),
                ("sans-serif", 20),
            )
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(70)
            .build_cartesian_2d(0.0f64..r_max, 0.0f64..c_max)
            .map_err(pe)?;
        chart
            .configure_mesh()
            .x_desc("radius (m)")
            .y_desc("concentration (mol/m^3)")
            .x_label_formatter(&|v| format!("{v:.1e}"))
            .y_label_formatter(&|v| format!("{v:.2e}"))
            .light_line_style(TRANSPARENT)
            .draw()
            .map_err(pe)?;
        chart
            .draw_series(LineSeries::new(
                radius.iter().copied().zip(conc.iter().copied()),
                BLUE.stroke_width(2),
            ))
            .map_err(pe)?;
        let mut inner = 0.0;
        for (name, outer) in layers {
            chart
                .draw_series(LineSeries::new(
                    vec![(*outer, 0.0), (*outer, c_max)],
                    RED.mix(0.6),
                ))
                .map_err(pe)?;
            chart
                .draw_series(std::iter::once(Text::new(
                    name.clone(),
                    (0.5 * (inner + outer), c_max * 0.97),
                    ("sans-serif", 13).into_font().color(&RED),
                )))
                .map_err(pe)?;
            inner = *outer;
        }
        root.present().map_err(pe)?;
        Ok(())
    }
}

/// Skip the refinement study when it would need more than this many cell-steps.
const STUDY_COST_LIMIT: f64 = 4.0e9;

fn flag(args: &mut Vec<String>, name: &str) -> bool {
    let before = args.len();
    args.retain(|a| a != name);
    args.len() != before
}

fn main() -> Result<()> {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let fv_only = flag(&mut args, "--fv-only");
    let no_study = flag(&mut args, "--no-convergence");
    let path = args
        .first()
        .cloned()
        .unwrap_or_else(|| "synthetic_five_layer.toml".to_string());
    let cfg = Config::load(&path)?;
    let mesh = fv::build_mesh(&cfg)?;
    let (rows, final_c) = fv::run(&cfg, &mesh)?;
    fs::create_dir_all(&cfg.output.directory)?;
    let out = Path::new(&cfg.output.directory);

    let ts_path = out.join("timeseries.csv");
    let mut w = csv::Writer::from_path(&ts_path)?;
    w.write_record([
        "time_s",
        "inventory_mol",
        "cumulative_release_mol",
        "release_fraction",
        "release_rate_mol_s",
        "balance_residual_mol",
        "min_concentration_mol_m3",
    ])?;
    for (t, d) in &rows {
        w.serialize((
            t,
            d.inventory,
            d.cumulative_release,
            d.release_fraction,
            d.release_rate,
            d.balance_residual,
            d.min_concentration,
        ))?;
    }
    w.flush()?;

    let profile_path = out.join("final_profile.csv");
    let mut p = csv::Writer::from_path(&profile_path)?;
    p.write_record(["cell", "material", "radius_m", "concentration_mol_m3"])?;
    for (i, (cell, c)) in mesh.cells.iter().zip(&final_c).enumerate() {
        p.serialize((i, cell.material, cell.centroid, c))?;
    }
    p.flush()?;

    let last = &rows.last().expect("initial row").1;
    println!("case={}", cfg.case_name);
    println!("cells={}", mesh.cells.len());
    println!("explicit_dt_bound_s={:.12e}", fv::explicit_dt_bound(&mesh));
    println!("final_release_fraction={:.12e}", last.release_fraction);
    println!("final_inventory_mol={:.12e}", last.inventory);
    println!(
        "max_final_balance_residual_mol={:.12e}",
        last.balance_residual.abs()
    );
    println!("min_final_concentration={:.12e}", last.min_concentration);
    println!("timeseries={}", ts_path.display());
    println!("profile={}", profile_path.display());

    let radii: Vec<f64> = mesh.cells.iter().map(|c| c.centroid).collect();
    let layers: Vec<(String, f64)> = cfg
        .regions
        .iter()
        .map(|r| (r.name.clone(), r.outer_radius_m))
        .collect();
    let fig = out.join("final_profile.svg");
    plots::profile(&fig, &cfg.case_name, &radii, &final_c, &layers)?;
    println!("figure={}", fig.display());

    if fv_only {
        return Ok(());
    }

    // Analytical cross-check. A failure here never invalidates the FV output above.
    match compare::compare_release(&cfg, &rows) {
        Ok(cmp) => {
            let cmp_path = out.join("comparison_fv_vs_analytical.csv");
            let mut c = csv::Writer::from_path(&cmp_path)?;
            c.write_record(["time_s", "release_fv", "release_analytical", "abs_error"])?;
            for r in &cmp.rows {
                match r.exact {
                    Some(e) => c.serialize((r.time_s, r.fv, e, (r.fv - e).abs()))?,
                    None => {
                        let (t, f) = (r.time_s.to_string(), r.fv.to_string());
                        c.write_record([t.as_str(), f.as_str(), "", ""])?
                    }
                }
            }
            c.flush()?;
            let fig = out.join("release_fv_vs_analytical.svg");
            plots::release_comparison(&fig, &cfg.case_name, &cmp)?;
            println!("analytical_modes={}", cmp.modes);
            println!("analytical_min_flux_ratio={:.3e}", cmp.min_flux_ratio);
            println!("analytical_accuracy_floor_abs={:.3e}", cmp.accuracy_floor);
            if cmp.max_abs_error < 3.0 * cmp.accuracy_floor {
                println!(
                    "WARNING: FV-analytical differences are within 3x of the analytical series accuracy floor; they are not resolved."
                );
            }
            println!("analytical_valid_from_s={:.6e}", cmp.min_valid_time_s);
            println!("max_abs_error_vs_analytical={:.6e}", cmp.max_abs_error);
            println!("final_abs_error_vs_analytical={:.6e}", cmp.final_abs_error);
            println!("comparison={}", cmp_path.display());
            println!("figure={}", fig.display());
        }
        Err(e) => {
            println!("analytical_cross_check=SKIPPED ({e})");
            return Ok(());
        }
    }

    if no_study {
        return Ok(());
    }
    let cost = compare::study_cost(&cfg);
    if cost > STUDY_COST_LIMIT {
        println!(
            "convergence_study=SKIPPED (estimated {cost:.2e} cell-steps; reduce end_time/dt or use fewer cells)"
        );
        return Ok(());
    }
    match compare::convergence_study(&cfg) {
        Ok(study) => {
            let study_path = out.join("convergence_study.csv");
            let mut s = csv::Writer::from_path(&study_path)?;
            s.write_record([
                "study",
                "total_cells",
                "dt_s",
                "fv_release",
                "abs_error_vs_exact",
            ])?;
            for (name, pts) in [("spatial", &study.spatial), ("temporal", &study.temporal)] {
                for q in pts {
                    s.serialize((name, q.cells, q.dt_s, q.fv_release, q.abs_error))?;
                }
            }
            s.flush()?;
            let fig = out.join("convergence.svg");
            plots::convergence(&fig, &cfg.case_name, &study)?;
            println!("exact_final_release_fraction={:.12e}", study.exact_final);
            let fmt = |v: Vec<f64>| {
                v.iter()
                    .map(|o| format!("{o:.2}"))
                    .collect::<Vec<_>>()
                    .join(",")
            };
            let note = |ok: bool| {
                if ok {
                    ""
                } else {
                    " (WARNING: finest errors are within 3x of the analytical accuracy floor, orders may be unreliable)"
                }
            };
            println!(
                "spatial_observed_orders={}{}",
                fmt(compare::ConvergenceStudy::orders(&study.spatial)),
                note(study.resolved(&study.spatial))
            );
            println!(
                "temporal_observed_orders={}{}",
                fmt(compare::ConvergenceStudy::orders(&study.temporal)),
                note(study.resolved(&study.temporal))
            );
            println!("convergence_study={}", study_path.display());
            println!("figure={}", fig.display());
        }
        Err(e) => println!("convergence_study=SKIPPED ({e})"),
    }
    Ok(())
}
