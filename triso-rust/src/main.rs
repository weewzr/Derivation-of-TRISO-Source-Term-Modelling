use anyhow::Result;
use std::{env, fs, path::Path};
use triso_rust::{config::Config, fv};

fn main() -> Result<()> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "config/synthetic_five_layer.toml".to_string());
    let cfg = Config::load(&path)?;
    let mesh = fv::build_mesh(&cfg)?;
    let (rows, final_c) = fv::run(&cfg, &mesh)?;
    fs::create_dir_all(&cfg.output.directory)?;

    let ts_path = Path::new(&cfg.output.directory).join("timeseries.csv");
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

    let profile_path = Path::new(&cfg.output.directory).join("final_profile.csv");
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
    Ok(())
}
