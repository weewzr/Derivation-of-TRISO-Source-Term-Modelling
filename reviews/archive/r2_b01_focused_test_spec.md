> HISTORICAL ARCHIVE: superseded working document. Do not use as the current project source.

# R2-B01 Focused Contract Tests

These tests specify the exact semantics required of the supervisor-side wrapper in reviews/r2_b01_minimal_supervisor_patch.md.

## Birth distribution

All starts must be inside the fuel kernel. The radial CDF for a volume-uniform birth is:

P(r <= x) = (x/r1)^3.

Use the existing sample_uniform_in_ball primitive.

## Five-layer path

The test path must call:

sample_uniform_in_ball -> WoSWalker::new -> WoSWalker::walk_until_released.

This proves the estimator uses the existing multilayer WOS path rather than the bare-kernel helper.

## Release time

Released outcomes must preserve the Time returned by walk_until_released.

## Fixed-time semantics

With completed outcomes at 1 s, 2 s and 4 s, the estimator at 2 s must be 2/3.

An outcome exactly at t is counted; an outcome after t is not counted.

## Censoring

CensoredMaxSteps is not an ordinary unreleased observation.

Any non-zero censoring count must make the ordinary empirical-CDF function return an explicit error/count.

## Monotonicity

For zero-censored data and t2 > t1:

F_hat_N(t2) >= F_hat_N(t1).

## Long-time completion

For a completed zero-censored sample and t >= max release time:

F_hat_N(t) = 1.

## Frame separation

LiveEnsemble::advance_frame must not be used as the source of the release-time estimator.

## Execution status

These are closure tests for the supervisor-side wrapper. The Ray environment does not contain a Rust compiler and cannot execute the supervisor crate locally, so no execution result is claimed in this repository.