> HISTORICAL ARCHIVE: superseded working document. Do not use as the current mathematical or project-status source.

# Two-Layer WOS Verification — Test Audit and Next Experimental Design

## Current state

The supervisor repository has a two-region equilibrium test embedded in `first_passage/walk_on_spheres.rs`. It uses a 10:1 diffusivity contrast and checks whether the measured inner-region time fraction is close to the inner volume fraction.

This is a useful diagnostic, but it is not yet a complete continuum-verification test.

## What the existing equilibrium test actually checks

The test constructs:

- inner radius 50 µm;
- outer radius 100 µm;
- inner diffusivity (10^{-8},mathrm{m^2/s});
- outer diffusivity (10^{-9},mathrm{m^2/s});
- unit partition ratio;
- finite interface distance 20 nm;
- reinsertion distance 60 nm;
- a reflecting outer wall.

For a uniform equilibrium concentration and spherical geometry, the expected fraction of time spent in the inner region is its volume fraction:

$$
f_{in}=\frac{a^3}{b^3}=\left(\frac{50}{100}\right)^3=0.125.
$$

The test records first-passage time in each material only during ordinary interface-free WOS hops. The interface-resolution operation itself is treated as zero simulated time.

Therefore the current statistic tests the implemented bulk WOS residence-time mechanism plus the interface crossing rule, but does not measure any finite time associated with the artificial reinsertion operation.

## Important limitation

The existing test comment says that the result fixes the transmission rule. That conclusion is stronger than the test alone supports.

A passing equilibrium occupancy test demonstrates that the tested implementation produces the expected stationary volume weighting under this particular geometry and parameter set. It does not by itself prove:

- the transient diffusion equation;
- the correct interface flux for arbitrary non-equilibrium states;
- the correct partition coefficient for arbitrary K;
- convergence as capture epsilon tends to zero;
- convergence as reinsertion distance tends to zero;
- correctness over arbitrary diffusivity ratios;
- correctness for curved interfaces beyond the tested spherical case.

Status: [UNVERIFIED] beyond the tested equilibrium observable.

## Exact two-layer continuum target

For an inner source region and an absorbing outer boundary, the steady continuum solution is already derived in `05_two_layer_spherical_benchmark.md`.

That solution provides direct targets for concentration and flux.

At the interface:

$$
c_1(a)=c_2(a)
$$

and

$$
-D_1c_1'(a)=-D_2c_2'(a).
$$

The interface flux equals:

$$
J(a)=\frac{S_0a}{3}.
$$

These quantities are preferable to a final release fraction alone because they test the physical interface conditions directly.

## Proposed verification sequence

### Test A — equilibrium occupancy

Keep the existing reflecting two-region geometry.

Repeat over several diffusivity ratios, for example:

$$
\frac{D_2}{D_1}\in\{10,1,0.1,10^{-2},10^{-3}\}.
$$

Measure the inner time fraction and compare with (a^3/b^3).

This tests whether the interface rule maintains the desired equilibrium density over a range of contrasts.

### Test B — partition-ratio equilibrium

Repeat for:

$$
K\in\{0.25,0.5,1,2,4\}.
$$

Compare the measured equilibrium concentration ratio with the imposed partition ratio.

This is required because the production interface API exposes K rather than only K=1.

### Test C — epsilon convergence

Hold physical geometry fixed and vary:

$$
\epsilon/L\in\{10^{-1},10^{-2},10^{-3},10^{-4}\}.
$$

where (L) is the thinnest relevant material length scale.

Measure an observable such as equilibrium occupancy error or release-time statistic.

The objective is to demonstrate that the numerical error decreases as epsilon decreases rather than merely assuming it.

### Test D — reinsertion convergence

At fixed epsilon, vary the reinsertion factor.

Check whether the measured observable approaches a stable limit.

### Test E — transient two-layer release

Use a two-layer sphere with:

- source only in the inner region;
- specified (D_1,D_2);
- absorbing outer boundary;
- a continuum solution or high-accuracy numerical reference.

Compare the cumulative release curve and, where reconstructable, the interface flux.

### Test F — rule diagnostic

For controlled research only, run identical tests with:

1. the repository D-linear rule;
2. a square-root diagnostic rule;
3. the continuum reference.

The purpose is not to select a preferred stochastic formula in advance. The purpose is to determine which process reproduces the chosen continuum model.

## Approximation budget

The benchmark should report uncertainty from each source separately:

$$
\text{total discrepancy}
=
\text{Monte-Carlo error}
+
\text{first-passage table error}
+
\text{finite-epsilon error}
+
\text{reinsertion error}
+
\text{interface-model error}.
$$

This is a conceptual decomposition rather than an assertion that the terms are independent or exactly additive.

## Code verification constraint

The current execution environment cannot clone the supervisor repository because outbound DNS access to GitHub is unavailable. Therefore no local cargo test result is claimed in this Ray pass.

The remote source code was inspected directly through the repository connector. Any future claimed numerical test result must come from an actually executed run or an existing repository record.

## Current conclusion

The next scientific work should be an implementation-side two-layer benchmark, but it should be added only after deciding how to measure a continuum observable from the Lagrangian histories.

No production interface rule should be changed yet.

No claim of full WOS-to-PDE equivalence should be made yet.

Review 1 remains premature.