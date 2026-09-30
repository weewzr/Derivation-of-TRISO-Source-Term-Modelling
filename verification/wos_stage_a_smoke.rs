use std::time::Instant;

use boon_lay::{
    lagrangian_decay_simulator::lagrangian_diffusion::{
        central_limit_theorem::oorandom_rng::OoRng64,
        first_passage::walk_on_spheres::{
            sample_uniform_in_ball, HopOutcome, WalkParams, WoSWalker,
        },
        single_particle_simulator::constructive_solid_geometry::TrisoCell,
    },
    Nuclide,
};
use uom::si::{
    diffusion_coefficient::square_meter_per_second,
    f64::Length,
    length::{meter, nanometer},
    time::second,
};

#[derive(Debug)]
enum Outcome {
    Released { time_s: f64, steps: u64 },
    CensoredMaxSteps { steps: u64 },
}

fn main() {
    const N: usize = 24;
    const MAX_STEPS: u64 = 5_000_000;
    const PROGRESS_EVERY: usize = 4;

    let cell = TrisoCell::new_crp6_geometry();
    let radii = [
        cell.get_fuel_radius().get::<meter>(),
        cell.get_buffer_radius().get::<meter>(),
        cell.get_ipyc_radius().get::<meter>(),
        cell.get_sic_radius().get::<meter>(),
        cell.get_opyc_radius().get::<meter>(),
    ];
    let probes = [
        0.5 * radii[0],
        0.5 * (radii[0] + radii[1]),
        0.5 * (radii[1] + radii[2]),
        0.5 * (radii[2] + radii[3]),
        0.5 * (radii[3] + radii[4]),
    ];
    let mut diffusivities = [0.0_f64; 5];
    for i in 0..5 {
        let p = [
            Length::new::<meter>(probes[i]),
            Length::new::<meter>(0.0),
            Length::new::<meter>(0.0),
        ];
        diffusivities[i] = cell
            .try_get_diffusion_coefficient(p, Nuclide::Cs137)
            .expect("Cs137 diffusivity must exist in every TRISO layer")
            .get::<square_meter_per_second>();
    }

    let params = WalkParams {
        capture_eps: Length::new::<nanometer>(10.0),
        reinsert_factor: 2.0,
        partition_k: 1.0,
        max_steps: MAX_STEPS,
    };

    println!("WOS_STAGE_A_SMOKE_CONTRACT");
    println!("purpose=execution_semantics_and_runtime_not_validation");
    println!("N={N} max_steps={MAX_STEPS} progress_every={PROGRESS_EVERY}");
    println!("capture_eps_nm=10 reinsert_factor=2 partition_k=1");
    println!("radii_m={radii:?}");
    println!("D_m2_s={diffusivities:?}");

    let wall = Instant::now();
    let mut outcomes = Vec::with_capacity(N);
    let mut released = 0usize;
    let mut censored = 0usize;
    let mut total_steps = 0u64;
    let mut max_steps_seen = 0u64;

    for i in 0..N {
        let mut master = OoRng64::from_u64(0xA110_0000_u64 + i as u64);
        let start = sample_uniform_in_ball(&mut master.0, cell.get_fuel_radius());
        let child = OoRng64::from_u64(master.next_u64());
        let mut walker = WoSWalker::new(start, Nuclide::Cs137, child);

        let mut outcome = Outcome::CensoredMaxSteps { steps: MAX_STEPS };
        for step in 1..=MAX_STEPS {
            if walker.step_multilayer(&cell, &params) == HopOutcome::Released {
                outcome = Outcome::Released {
                    time_s: walker.time.get::<second>(),
                    steps: step,
                };
                break;
            }
        }

        let steps = match outcome {
            Outcome::Released { steps, .. } => {
                released += 1;
                steps
            }
            Outcome::CensoredMaxSteps { steps } => {
                censored += 1;
                steps
            }
        };
        total_steps += steps;
        max_steps_seen = max_steps_seen.max(steps);
        outcomes.push(outcome);

        if (i + 1) % PROGRESS_EVERY == 0 || i + 1 == N {
            println!(
                "progress completed={} released={} censored={} elapsed_s={:.3} mean_steps={:.1} max_steps={}",
                i + 1,
                released,
                censored,
                wall.elapsed().as_secs_f64(),
                total_steps as f64 / (i + 1) as f64,
                max_steps_seen
            );
        }
    }

    let release_times: Vec<f64> = outcomes
        .iter()
        .filter_map(|o| match o {
            Outcome::Released { time_s, .. } => Some(*time_s),
            Outcome::CensoredMaxSteps { .. } => None,
        })
        .collect();

    println!(
        "summary N={} released={} censored={} censor_fraction={:.8} elapsed_s={:.3} mean_steps={:.1} max_steps={}",
        N,
        released,
        censored,
        censored as f64 / N as f64,
        wall.elapsed().as_secs_f64(),
        total_steps as f64 / N as f64,
        max_steps_seen
    );
    if !release_times.is_empty() {
        let min_t = release_times.iter().copied().fold(f64::INFINITY, f64::min);
        let max_t = release_times.iter().copied().fold(0.0_f64, f64::max);
        let mean_t = release_times.iter().sum::<f64>() / release_times.len() as f64;
        println!("released_time_s min={min_t:.12e} mean={mean_t:.12e} max={max_t:.12e}");
    }
    println!("stage_a_conclusion=execution_only_no_WOS_FV_validation_claim");
}
