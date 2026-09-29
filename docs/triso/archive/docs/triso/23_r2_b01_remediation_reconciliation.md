> HISTORICAL ARCHIVE: superseded working document. Do not use as the current project source.

# 23 — Review-2 R2-B01 Remediation Reconciliation

## R2-B01

Status: REMEDIATED AT PATCH/DESIGN LEVEL; NOT YET CLOSED.

Independent reproduction confirmed:

1. parallel_kernel_release_fraction uses sample_uniform_in_ball but calls walk_to_absorbing_sphere for the bare kernel, so it is not a five-layer estimator.
2. LiveEnsemble::advance_frame(until) calls diffuse_until, which can complete a WOS hop whose event time exceeds until, so its frame release flag is not the same mathematical observable as the first-release-time CDF.
3. WoSWalker::walk_until_released already supplies the needed five-layer per-history release-time primitive and returns Some(Time) on release or None at max_steps.

Therefore the smallest coherent remediation is an ensemble wrapper around the existing primitive rather than a change to the WOS algorithm.

## Frozen verification observable

For history m:

tau_m = first simulated time at which the history reaches the absorbing outer OPyC surface.

For N equal-weight histories:

F_hat_N(t) = (1/N) sum_m 1[tau_m <= t].

This is the empirical CDF of first-release times.

The observable is not defined from an animation frame state.

## Required concrete path

sample_uniform_in_ball
-> WoSWalker::new
-> WoSWalker::walk_until_released
-> ReleaseOutcome::Released { time } / CensoredMaxSteps
-> release_fraction_at
-> F_hat_N(t).

## Censoring

walk_until_released returns None when max_steps is reached.

That outcome is classified explicitly as CensoredMaxSteps.

The ordinary empirical CDF is defined only when the completed sample contains zero censored histories.

If censoring is non-zero, the estimator returns an explicit error/count rather than silently putting censored histories in the denominator as known unreleased outcomes.

Thus the base verification run must satisfy:

censored_count = 0.

Only under that condition is:

F_hat_N(t) = #{tau_m <= t}/N

the ordinary empirical CDF of the sample.

## Minimal supervisor-side patch

The proposed wrapper and tests are documented in:

reviews/r2_b01_minimal_supervisor_patch.md

The focused closure test specification is in:

reviews/r2_b01_focused_test_spec.md

No production WOS mechanic is changed.

## Deferred findings

These are intentionally not closed here:

- R2-M01 — transient WOS/interface to PDE equivalence: Review 4.
- R2-m01 — first-passage table sensitivity: Review 4.
- R2-m02 — reinsertion bias/convergence: Review 4.
- R2-N01–N04: retain for Review 3 as applicable.

## Application limitation

The Ray project does not automatically publish changes to the supervisor repository. The minimal wrapper therefore exists as a patch specification, not as a change already present in the supervisor repository.

Consequently this reconciliation deliberately does not mark R2-B01 CLOSED.

## Branch state

All work remains on ray/triso-foundation in the Ray repository.

No main branch was modified.

## Next action

Apply the minimal wrapper and focused tests to the designated supervisor feature branch, run them, verify zero censoring in the selected base run, and then conduct the independent focused R2-B01 closure check.