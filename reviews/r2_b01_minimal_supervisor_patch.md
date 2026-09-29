# R2-B01 Minimal Supervisor-Branch Patch Proposal

Target repository: theodoreOnzGit/outram-park-backend
Target supervisor feature branch: ray-triso-derivation

This patch is intentionally not applied to the supervisor repository from the Ray project. The user controls publication to Theodore's repository.

## Purpose

Add one concrete five-layer release-time ensemble path using the existing production WOS primitives:

uniform kernel birth -> TrisoCell -> WoSWalker::walk_until_released -> Released(Time) or CensoredMaxSteps -> empirical release CDF.

Do not change step_multilayer, interface probability, first-passage sampler, or geometric logic.

## Proposed changes

### 1. Explicit outcome type in first_passage/ensemble.rs

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReleaseOutcome {
    Released { time: Time },
    CensoredMaxSteps,
}
```

### 2. Concrete five-layer release-time ensemble

```rust
pub fn parallel_multilayer_release_times(
    nuclide: Nuclide,
    triso_cell: &TrisoCell,
    params: &WalkParams,
    config: &EnsembleConfig,
) -> Vec<ReleaseOutcome> {
    (0..config.n_histories)
        .into_par_iter()
        .map(|i| {
            let mut master = OoRng64::from_u64(history_seed(config.base_seed, i));
            let start = sample_uniform_in_ball(
                &mut master.0,
                triso_cell.get_fuel_radius(),
            );
            let child = OoRng64::from_u64(master.next_u64());
            let mut walker = WoSWalker::new(start, nuclide, child);

            match walker.walk_until_released(triso_cell, params) {
                Some(time) => ReleaseOutcome::Released { time },
                None => ReleaseOutcome::CensoredMaxSteps,
            }
        })
        .collect()
}
```

This reuses the actual production five-layer geometry, diffusivity dispatch, multilayer WOS step, interface treatment and absorbing OPyC release.

### 3. Explicit fixed-time estimator

```rust
pub fn release_fraction_at(
    outcomes: &[ReleaseOutcome],
    time: Time,
) -> Result<f64, usize> {
    let mut released = 0usize;
    let mut censored = 0usize;

    for outcome in outcomes {
        match outcome {
            ReleaseOutcome::Released { time: tau } if *tau <= time => {
                released += 1;
            }
            ReleaseOutcome::Released { .. } => {}
            ReleaseOutcome::CensoredMaxSteps => {
                censored += 1;
            }
        }
    }

    if censored != 0 {
        return Err(censored);
    }

    if outcomes.is_empty() {
        return Ok(0.0);
    }

    Ok(released as f64 / outcomes.len() as f64)
}
```

This implements the exact empirical CDF only when the release-time sample contains zero censored histories.

### 4. Focused tests

Test the kernel birth distribution using the same sample_uniform_in_ball path; assert all starts are inside the kernel and check the volume law:

P(r <= x) = (x/r1)^3.

Test the five-layer release path by calling parallel_multilayer_release_times and asserting that every history is either Released{time} or CensoredMaxSteps.

Test fixed-time semantics with controlled outcomes:

- Released at 1 s;
- Released at 2 s;
- CensoredMaxSteps.

Verify that a release at exactly t is counted, a release after t is not counted, and censoring returns an explicit error.

Test CDF monotonicity for zero-censored outcome vectors.

Test long-time completion using the deterministic recorded sample: at any t greater than or equal to the maximum recorded release time, the empirical CDF equals 1.

## Why this patch is minimal

No WOS scientific mechanics are changed.

Untouched:

- step_multilayer;
- does_transmit;
- first-passage distribution;
- interface probability;
- capture epsilon;
- reinsertion;
- geometry routines.

The patch only exposes the release time already returned by walk_until_released and prevents max-step censoring from being silently interpreted as known non-release.

## Exact equation-to-code chain after application

X_0 -> (X_n,T_n) -> tau_m_release -> ReleaseOutcome::Released{time} -> release_fraction_at -> F_hat_N(t).

Mapping:

- X_0: sample_uniform_in_ball
- X_n,T_n: WoSWalker
- tau_m_release: walk_until_released
- outcome classification: ReleaseOutcome
- estimator: release_fraction_at

## Application status

This patch has not been applied to the supervisor repository because the Ray project does not automatically publish changes to Theodore's repository.

Therefore R2-B01 is **not yet closed**. It is remediated at the design/patch level, but closure requires the actual supervisor feature branch to contain and execute this wrapper plus its focused tests.