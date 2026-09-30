# Active Method Scope Freeze

## Purpose

This document freezes the active scientific project to three reproducible methods. The criterion is that the core method can be derived, implemented, executed, inspected, and independently verified within this project or against the authoritative production WOS source.

## METHOD 1 — FIRST-PRINCIPLES ANALYTICAL DIFFUSION

Conservation → Fick's law → spherical multilayer PDE → interfaces and boundaries → analytical/modal solution.

This is the continuous first-principles reference layer.

## METHOD 2 — FIRST-PRINCIPLES COMPUTATIONAL DIFFUSION

The same governing diffusion physics is solved computationally through two distinct numerical evidence streams:

1. deterministic conservative finite volume (FV);
2. stochastic production Walk-on-Spheres (WOS).

FV accuracy does not by itself validate WOS. Production WOS verification proceeds against a matched deterministic/analytical reference with explicit Monte-Carlo and interface-resolution uncertainty.

### Staged WOS verification execution

- **Stage A — smoke / contract:** deliberately small fixed ensemble. Verifies that the authoritative production estimator executes, preserves Released versus CensoredMaxSteps semantics, records geometry/material/interface parameters, and measures runtime/steps. It is not validation.
- **Stage B — pilot Monte Carlo:** sample count fixed before comparison with FV. Measures censoring, runtime distribution and empirical release probabilities/variance. Production sample size is then chosen from a declared Monte-Carlo uncertainty target using the binomial standard-error relation, not from observed agreement.
- **Stage C — production verification:** uses the pre-specified sample size and matched deterministic transient reference to compare F_WOS(t) with F_FV(t), including confidence intervals. Capture/reinsertion sensitivity is treated separately to distinguish Monte-Carlo sampling error from interface-discretisation bias.

R2-B01 remains open until executed evidence satisfies its exact closure requirement.

## METHOD 3 — SEMI-EMPIRICAL MECHANISTIC / REDUCED-ORDER

Experimentally fitted diffusivities → D(T) → Booth / breakthrough / release-to-birth formulations → release fraction/rate.

Method 3 is in project scope but is not started by this Stage-A execution-design pass.

## External / archived references, not active methods

BISON, PARFUME, CFDT, FRESCO-II, COPA, STACY and other external packages/models are retained only as external or archived references for possible future benchmarking. They are not additional active methods in the current project.

## Scope rule

Do not add a fourth active method without an explicit project-scope decision. External software may support contextual comparison or later benchmarking, but it does not replace the three-method reproducible core.
