> HISTORICAL ARCHIVE: superseded working document. Do not use as the current project source.

# 24 — Review-2 R2-B01 Remediation / Reconciliation

## Review-2 blocker

R2-B01: the documented five-layer fixed-time release estimator was not represented by one concrete implementation path.

Independent reproduction of the review finding: CONFIRMED.

## Evidence

Supervisor source reviewed:

- repository: theodoreOnzGit/outram-park-backend
- reviewed source commit: 8d31482d127e211614ebb3f66b1076e1ed6dea98
- first_passage/ensemble.rs
- first_passage/live.rs
- first_passage/walk_on_spheres.rs

parallel_kernel_release_fraction uses walk_to_absorbing_sphere with a bare kernel radius and therefore is not a five-layer estimator.

LiveEnsemble::advance_frame(until) calls diffuse_until, which may complete a WOS hop whose sampled event time passes beyond until; the frame timestamp is then advanced to until. Therefore frame release state is not identical to the release-time event tau_m.

The repository already has the scientifically relevant five-layer primitive WoSWalker::walk_until_released, which repeatedly calls step_multilayer and returns Some(self.time) on outer-surface release or None at the step cap.

## Remediation

A focused feature-branch verification wrapper was added to the Ray repository:

verification/r2_b01_supervisor_integration.rs

It is not production WOS code.

The wrapper constructs one actual five-layer supervisor path:

uniform kernel birth
→ TrisoCell::new_crp6_geometry()
→ sample_uniform_in_ball
→ WoSWalker::new
→ walk_until_released
→ step_multilayer
→ does_transmit at internal interfaces
→ HopOutcome::Released at the OPyC surface
→ Some(Time) containing the actual accumulated simulated release time.

No interface probability or WOS mechanics were changed.

## Frozen estimator

For history m:

tau_m = first simulated time at which the history reaches the absorbing outer OPyC boundary.

For N equal-weight complete histories:

F_hat_N(t) = (1/N) sum_m 1[tau_m <= t].

This is the empirical CDF of first-release times.

The estimator is intentionally independent of LiveEnsemble::advance_frame or animation-frame state.

## Censoring semantics

The wrapper defines:

- Released(Time)
- CensoredMaxSteps

walk_until_released returning None is mapped to CensoredMaxSteps.

A censored history is NOT treated as known unreleased.

For the ordinary empirical release-time CDF, the required condition is:

number of CensoredMaxSteps = 0.

Only then is F_hat_N(t) = #{tau_m <= t}/N a complete empirical CDF.

No survival-analysis/censored-data estimator is introduced in this remediation.

## Focused tests

The test file checks:

1. kernel birth points are inside the kernel and have the expected volume-uniform radial statistic;
2. the five-layer release-time wrapper uses the supervisor multilayer walk path;
3. released histories return actual positive simulated times;
4. release times later than an observation time are excluded from the CDF;
5. max-step censoring is explicit and prevents use of the complete-sample CDF;
6. the empirical CDF is monotone;
7. a zero-censoring absorbing run gives F_hat(infinity)=1.

These tests do not attempt to close R2-M01, R2-m01, R2-m02, or R2-N01–N04.

## Execution path

The current Ray runtime cannot resolve GitHub and has no Rust compiler, so the tests were not executed locally.

A dedicated workflow:

.github/workflows/r2-b01-supervisor-integration.yml

checks out the exact supervisor commit in a temporary CI workspace, injects the focused integration test, and runs:

cargo test -p boon-lay --test r2_b01_supervisor_integration -- --nocapture

No supervisor repository branch is modified by this workflow.

Therefore current status is:

R2-B01 = REMEDIATION IMPLEMENTED, EXECUTION EVIDENCE PENDING.

It must NOT be marked closed until the focused CI run succeeds.

## Deferred Review-2 findings

R2-M01: Review 4.
R2-m01: Review 4.
R2-m02: Review 4.
R2-N01–N04: Review 3 where applicable.

Passing the focused estimator tests would establish the release-time observable/path contract, not full WOS-to-PDE equivalence.

## Supervisor questions

No new supervisor question is required to resolve R2-B01 itself.

Existing physical-model questions remain in docs/triso/21_frozen_production_wos_contract.md.

## Closure criterion

R2-B01 is closed only when all four agree:

1. mathematical observable;
2. concrete five-layer implementation path;
3. explicit censoring semantics;
4. passing focused tests against the actual supervisor source.

Until then, the blocker remains open.