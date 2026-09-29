use boon_lay::{
    lagrangian_decay_simulator::lagrangian_diffusion::first_passage::{
        walk_on_spheres::{sample_uniform_in_ball, WalkParams, WoSWalker},
    },
    lagrangian_decay_simulator::lagrangian_diffusion::single_particle_simulator::constructive_solid_geometry::TrisoCell,
    Nuclide,
};
use uom::si::{
    f64::{Length, Time},
    length::{meter, nanometer},
    time::second,
    };
use uom::ConstZero;

#[derive(Debug, Clone, Copy, PartialEq)]
enum ReleaseOutcome {
    Released(Time),
    CensoredMaxSteps,
}

fn five_layer_release_time(seed: u64, params: &WalkParams) -> ReleaseOutcome {
    let cell = TrisoCell::new_crp6_geometry();
    let mut master = boon_lay::lagrangian_decay_simulator::lagrangian_diffusion::central_limit_theorem::oorandom_rng::OoRng64::from_u64(seed);
    let start = sample_uniform_in_ball(&mut master.0, cell.get_fuel_radius());
    let child = boon_lay::lagrangian_decay_simulator::lagrangian_diffusion::central_limit_theorem::oorandom_rng::OoRng64::from_u64(master.next_u64());
    let mut walker = WoSWalker::new(start, Nuclide::Cs137, child);
    match walker.walk_until_released(&cell, params) {
        Some(t) => ReleaseOutcome::Released(t),
        None => ReleaseOutcome::CensoredMaxSteps,
    }
}

fn empirical_cdf(outcomes: &[ReleaseOutcome], t: Time) -> Result<f64, &'static str> {
    if outcomes.iter().any(|o| matches!(o, ReleaseOutcome::CensoredMaxSteps)) {
        return Err("release-time empirical CDF requires zero censored histories");
    }
    let released = outcomes
        .iter()
        .filter(|o| matches!(o, ReleaseOutcome::Released(tau) if *tau <= t))
        .count();
    Ok(released as f64 / outcomes.len() as f64)
}

fn empirical_cdf_infty(outcomes: &[ReleaseOutcome]) -> Result<f64, &'static str> {
    empirical_cdf(outcomes, Time::new::<second>(f64::INFINITY))
}

#[test]
fn initial_kernel_births_are_inside_and_volume_uniform() {
    let cell = TrisoCell::new_crp6_geometry();
    let radius = cell.get_fuel_radius();
    let radius_m = radius.get::<meter>();
    let n = 20_000usize;
    let mut seed = 0x1357_9BDF_2468_ACE0_u64;
    let mut sum_r3 = 0.0;

    for _ in 0..n {
        let p = sample_uniform_in_ball(&mut seed, radius);
        let r_m = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2])
            .sqrt()
            .get::<meter>();
        assert!(r_m <= radius_m, "birth point escaped kernel");
        sum_r3 += r_m.powi(3);
    }

    let observed = sum_r3 / n as f64 / radius_m.powi(3);
    let expected = 0.5;
    assert!(
        (observed - expected).abs() < 0.02,
        "E[(r/R)^3] = {observed:.4}, expected {expected:.4} for a volume-uniform ball"
    );
}

#[test]
fn five_layer_release_path_records_actual_release_time_or_explicit_censoring() {
    let params = WalkParams {
        capture_eps: Length::new::<nanometer>(10.0),
        reinsert_factor: 2.0,
        partition_k: 1.0,
        max_steps: 200_000,
    };

    let outcome = five_layer_release_time(0xB01, &params);
    match outcome {
        ReleaseOutcome::Released(t) => {
            assert!(t > Time::ZERO, "release time must be positive");
        }
        ReleaseOutcome::CensoredMaxSteps => {
            // Explicitly allowed here: the test proves censoring is not silently
            // converted into a known non-release state.
        }
    }
}

#[test]
fn fixed_time_cdf_excludes_late_releases() {
    let t1 = Time::new::<second>(1.0);
    let t3 = Time::new::<second>(3.0);
    let outcomes = [
        ReleaseOutcome::Released(t1),
        ReleaseOutcome::Released(t3),
        ReleaseOutcome::Released(t3),
    ];

    let f1 = empirical_cdf(&outcomes, Time::new::<second>(1.5)).unwrap();
    let f2 = empirical_cdf(&outcomes, Time::new::<second>(2.5)).unwrap();
    assert_eq!(f1, 1.0 / 3.0);
    assert_eq!(f2, 1.0 / 3.0);
    assert!(f2 >= f1);
}

#[test]
fn max_step_censoring_is_not_known_nonrelease() {
    let params = WalkParams {
        capture_eps: Length::new::<nanometer>(10.0),
        reinsert_factor: 2.0,
        partition_k: 1.0,
        max_steps: 0,
    };

    assert_eq!(
        five_layer_release_time(0xC0DE_5EED, &params),
        ReleaseOutcome::CensoredMaxSteps
    );

    let outcomes = [ReleaseOutcome::CensoredMaxSteps];
    assert!(empirical_cdf(&outcomes, Time::new::<second>(1.0)).is_err());
}

#[test]
fn empirical_cdf_is_monotone() {
    let outcomes = [
        ReleaseOutcome::Released(Time::new::<second>(0.5)),
        ReleaseOutcome::Released(Time::new::<second>(1.5)),
        ReleaseOutcome::Released(Time::new::<second>(4.0)),
        ReleaseOutcome::Released(Time::new::<second>(6.0)),
    ];

    let mut previous = 0.0;
    for t in [0.0, 0.5, 1.0, 2.0, 5.0, 10.0] {
        let value = empirical_cdf(&outcomes, Time::new::<second>(t)).unwrap();
        assert!(value >= previous, "CDF decreased at t={t}");
        previous = value;
    }
}

#[test]
fn no_censoring_gives_full_release_at_infinity_in_absorbing_benchmark() {
    let params = WalkParams {
        capture_eps: Length::new::<nanometer>(10.0),
        reinsert_factor: 2.0,
        partition_k: 1.0,
        max_steps: 5_000_000,
    };

    let n = 20usize;
    let outcomes: Vec<_> = (0..n)
        .map(|i| five_layer_release_time(0xF000_0000 + i as u64, &params))
        .collect();

    assert!(
        outcomes.iter().all(|o| matches!(o, ReleaseOutcome::Released(_))),
        "benchmark run must have zero max-step censoring before F(infinity) is reported"
    );

    let f_inf = empirical_cdf_infty(&outcomes).unwrap();
    assert_eq!(f_inf, 1.0);
}
