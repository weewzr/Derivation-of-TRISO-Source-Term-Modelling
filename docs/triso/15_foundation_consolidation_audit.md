# Foundation Consolidation Audit

## Audit basis

Source notebook: TRISO Fuel Derivation Ray V1.tex

Source length: 881 lines.

Substantive displayed equations identified: 33.

Substantive equations explicitly mapped in 13_central_equation_status_register.md: 33.

Equation audit coverage: 33/33 = 100%.

All displayed equations from the source notebook were inspected by label and content. In addition, the derivational prose around the equations was checked for hidden algebraic jumps, assumptions, notation, units, signs and scope.

## Verified equation families

- Initial condition and centre regularity under the stated benchmark assumptions.
- Constant-D Fickian spherical diffusion reduction.
- Product-rule expansion of the spherical radial operator within a constant-D region.
- Homogeneous Part-I governing equation.
- Part-I steady integration and Robin surface relation.
- Independent steady global mass balance.
- Homogeneous transient deviation formulation.
- Corrected spatial/temporal eigenvalue separation.
- Corrected Robin eigencondition.
- Centre limit of sin(k r)/r.
- Weighted eigenfunction projection.
- Part-I FTCS interior algebra.
- Centre ghost-point and factor-of-six treatment.
- Robin ghost-point algebra.
- Surface FTCS algebra.
- Exact steady five-layer shell-by-shell resistance formulation under ideal assumptions.
- Five-layer transient layerwise eigen-equation and determinant framework.

## Remaining DERIVATION GAP items

1. Full numerical enumeration and convergence of the five-layer transient determinant F(Lambda).
   Depends on: final multilayer analytical verification and any transient continuum reference used for WOS.

2. Closed evaluation and convergence of five-layer transient modal coefficients.
   Depends on: item 1 and the fully specified physical interface model.

3. Complete spectral stability analysis of the Part-I FTCS amplification matrix.
   Depends on: whether FTCS remains a required benchmark and the chosen time discretisation definition.

4. Full mathematical proof that the stochastic WOS process converges to the chosen divergence-form diffusion PDE.
   Depends on: physical interface model and the exact production WOS algorithm.

5. Full decay/trapping stochastic-to-PDE derivation.
   Depends on: selected species and physical source/reaction model.

## POSSIBLE ERROR items

1. Original eigenvalue notation: lambda was assigned s^-1 while used as a spatial wave-number-squared quantity.

2. Original FTCS stability wording: non-negative coefficients were described as though necessary and sufficient for boundedness. The corrected text labels this only as a sufficient coefficient-positivity/monotonicity condition.

3. Original FTCS stability discussion omitted the centre-node coefficient condition Fo <= 1/6.

4. Original source decomposition did not define signs or constitutive laws for decay and trapping.

5. A single Part-II Biot number reference diffusivity was not fully defined.

6. Blanket concentration continuity at every physical interface remains unverified for every species/material combination.

## CODE MISMATCH items

1. Original notebook numerical method = FTCS finite difference.

2. Supervisor production diffusion method = Lagrangian first-passage Walk-on-Spheres.

The mismatch is intentional and documented. FTCS is retained as a transparent deterministic benchmark; WOS is the actual supervisor implementation. Neither is being silently substituted for the other.

## QUESTION FOR SUPERVISOR items

1. Canonical first fission-product species.
2. Whether radioactive decay is part of the first physical model.
3. Whether trapping/release is part of the first physical model.
4. Whether interfaces use concentration continuity or species-specific partition coefficients.
5. Whether any explicit interfacial resistance is intended.
6. Whether D_i are constants, temperature dependent, fluence dependent, or state dependent in the canonical run.
7. Whether c_infty = 0 is a physical approximation or only a benchmark.
8. Whether both Gaussian and WOS diffusion paths should be retained for comparative verification.

## Exact implementation provenance

Supervisor repository: theodoreOnzGit/outram-park-backend.

Default branch: main.

Ray branch: ray-triso-derivation.

Ray branch tip inspected: fd9d5c287ce77ab04a89fa4764fcf0a6b9b3e90e.

The branch is 2 commits ahead of main and 0 behind; the branch-specific commits are the user's derivations/ray/notes.md and derivations/ray/notes.tex.

The scientific WOS implementation is inherited from main. Path-specific history for the interface, WOS and sphere-FPT modules identifies commit 159d1fc4bd56916f5d598da80fa45a2311d6e594 as the relevant repository state containing those modules.

Implementation paths:

1. crates/boon-lay/src/lagrangian_decay_simulator/lagrangian_diffusion/single_particle_simulator/constructive_solid_geometry/mod.rs
   TrisoCell::new; TrisoCell::new_crp6_geometry; TrisoCell::get_triso_region; TrisoCell::try_get_diffusion_coefficient.

2. crates/boon-lay/src/lagrangian_decay_simulator/lagrangian_diffusion/first_passage/walk_on_spheres.rs
   WoSWalker; step_multilayer; walk_until_released; nearest_interface_distance; shell_bounds; sample_uniform_in_ball.

3. crates/boon-lay/src/lagrangian_decay_simulator/lagrangian_diffusion/first_passage/interface.rs
   does_transmit.

4. crates/boon-lay/src/lagrangian_decay_simulator/lagrangian_diffusion/first_passage/sphere_fpt.rs
   homogeneous first-passage distribution and lookup/interpolation.

5. verification_and_validation/crp6_case1_kernel_release_vs_crank.md
   existing single-layer WOS versus Crank verification record.

6. docs/buffer_clt_failure_analysis.md
   legacy Gaussian interface-overshoot analysis.

## Master Instructions compliance

First-principles completeness: SUBSTANTIALLY COMPLETE through the continuous model and benchmark discretisation; remaining physical and stochastic closure items are explicitly listed.

One-operation-per-step granularity: IMPROVED and applied to the consolidated derivation. Some source notebook lines were intentionally compressed for presentation but are all explicitly accounted for in the mapping register.

Assumptions: EXPLICIT for spherical symmetry, representative particle, piecewise homogeneous layers, ideal interfaces and benchmark boundary choices.

Units: AUDITED; eigenvalue dimensions corrected in the consolidated derivation.

Signs: AUDITED; decay/trapping sign convention identified as unresolved in the original source.

Variable-D treatment: CORRECTED in the consolidated foundation by retaining D inside the divergence until layer specialisation.

Centre treatment: AUDITED and corrected/expanded.

Interface treatment: CONTINUUM IDEAL MODEL DERIVED; physical partition/resistance choice remains unresolved.

Original-note traceability: 100% of 33 substantive displayed equations mapped.

Literature provenance: established for the governing TRISO diffusion framework; specific material/source correlations still require parameter-level provenance.

Branch isolation: remote Ray repository work has been confined to ray/triso-foundation. No main branch commit was created by this pass.

Supervisor repository safety: no production WOS source file was modified by this pass.

## Review-1 decision

The continuous mathematical foundation is now materially stronger and is no longer blocked by a disconnected collection of partial notes.

However, Review 1 should still wait.

The single prerequisite is an independent mathematical audit of the consolidated foundation against the original 33-equation notebook and the status register. This reviewer should focus on mathematical correctness and continuity only; WOS expansion is not required for that review.