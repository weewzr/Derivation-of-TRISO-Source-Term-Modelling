use anyhow::{bail, Context, Result};
use serde::Deserialize;
use std::{fs, path::Path};

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

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let raw = fs::read_to_string(path.as_ref())
            .with_context(|| format!("reading {}", path.as_ref().display()))?;
        let cfg: Self = toml::from_str(&raw).context("parsing TOML")?;
        cfg.validate()?;
        Ok(cfg)
    }

    pub fn validate(&self) -> Result<()> {
        if self.regions.len() != 5 {
            bail!("exactly five physical regions are required");
        }
        let mut last = 0.0;
        for r in &self.regions {
            if r.outer_radius_m <= last || r.diffusivity_m2_s <= 0.0 || r.cells == 0 {
                bail!("invalid region {}", r.name);
            }
            last = r.outer_radius_m;
        }
        if self.time.dt_s <= 0.0 || self.time.end_time_s <= 0.0 || self.time.output_every_steps == 0
        {
            bail!("invalid time controls");
        }
        if self.initial.kernel_concentration_mol_m3 < 0.0 {
            bail!("initial concentration must be nonnegative");
        }
        Ok(())
    }
}
