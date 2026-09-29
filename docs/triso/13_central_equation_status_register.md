# Central Equation Status and Dependency Register

## Scope

The supplied TRISO Fuel Derivation Ray V1.tex contains 33 substantive displayed equations. All 33 are explicitly inventoried below.

Status vocabulary:
- VERIFIED
- CONDITIONALLY VERIFIED
- UNVERIFIED
- DERIVATION GAP
- POSSIBLE ERROR
- CODE MISMATCH
- SOURCE NEEDED
- QUESTION FOR SUPERVISOR

| Original label | Project ID | Status | Finding | Downstream dependency |
|---|---|---|---|---|
| eq:b:ic | TRISO-BC-008 | CONDITIONALLY VERIFIED | Zero initial concentration is a benchmark assumption, not a universal physical history. | All transient benchmarks. |
| eq:b:bc0 | TRISO-BC-003 | VERIFIED | Regular spherical symmetry gives zero radial derivative at the centre. | Centre analysis and numerics. |
| eq:b:bcR | TRISO-BC-006 | CONDITIONALLY VERIFIED | Correct for zero coolant concentration and constant outer D; physical five-layer form should use D5 and c_infty. | Robin analytical solution and boundary discretisation. |
| eq:b:biot | TRISO-VER-009 | CONDITIONALLY VERIFIED | Correct Part-I Biot number. Part-II reference diffusivity is not yet fixed. | Dimensionless eigenproblem. |
| eq:fick | TRISO-GOV-012 | CONDITIONALLY VERIFIED | Correct only when D is constant. General multilayer form requires div(D grad c). | Full five-layer PDE. |
| eq:fick-spherical | TRISO-SPH-008 | CONDITIONALLY VERIFIED | Correct constant-D spherical Laplacian form. | Spherical reduction. |
| eq:radial-conservative | TRISO-SPH-009 | CONDITIONALLY VERIFIED | Correct constant-D radial equation, incomplete for discontinuous multilayer D. | Piecewise PDE. |
| eq:product-rule | TRISO-SPH-010 | VERIFIED | Product-rule expansion is correct inside a constant-D region. | Part-I PDE and FTCS. |
| eq:simplified-triso | TRISO-GOV-013 | CONDITIONALLY VERIFIED | Constant-D radial equation only. Must not be treated as the general five-layer PDE. | Part-I benchmark. |
| eq:kernel-source | TRISO-GOV-015 | CONDITIONALLY VERIFIED | Kernel-source assumption is explicit; S0 still needs physical provenance. | Source model. |
| eq:source-decomposition | TRISO-GOV-028 | UNVERIFIED / POSSIBLE ERROR | Decay and trapping signs/forms are undefined; the total-source convention is not frozen. | Source/decay/transmutation model. |
| eq:b:pde | TRISO-GOV-029 | VERIFIED | Correct homogeneous Part-I benchmark PDE. | Steady, transient and FTCS derivations. |
| eq:b:pde-divergence | TRISO-GOV-030 | VERIFIED | Equivalent divergence form for constant D. | Sturm-Liouville analysis. |
| eq:b:pqsigma | TRISO-GOV-031 | CONDITIONALLY VERIFIED | Operator structure correct; eigenvalue dimensions in the original notebook are inconsistent with later use. | Eigenfunction expansion. |
| eq:b:w-firstint | TRISO-GOV-032 | VERIFIED | First steady integration and boundedness condition are algebraically correct. | Steady solution. |
| eq:b:w-secondint | TRISO-GOV-033 | CONDITIONALLY VERIFIED | Algebra correct for c_infty = 0. | Steady solution. |
| eq:b:w | TRISO-GOV-034 | VERIFIED | Correct steady profile under Part-I assumptions. | Transient deviation and checks. |
| eq:b:massbalance | TRISO-VER-010 | VERIFIED | Independent steady inventory/flux check. | Steady verification. |
| eq:b:vproblem | TRISO-GOV-035 | VERIFIED | Correct homogeneous transient problem under Part-I assumptions. | Separation of variables. |
| eq:b:sl-ode | TRISO-VER-011 | POSSIBLE ERROR | Original notation mixes spatial and temporal eigenvalues. | Entire modal solution. |
| eq:b:harmonic | TRISO-VER-012 | POSSIBLE ERROR | u'' + lambda u = 0 is dimensionally wrong if lambda is in s^-1. | Eigenfunction. |
| eq:b:eigfun | TRISO-VER-013 | POSSIBLE ERROR | sin(sqrt(lambda) r)/r is only dimensionally valid if lambda is a spatial wavenumber squared. | Robin eigencondition. |
| eq:b:eigcond-raw | TRISO-VER-014 | VERIFIED AFTER NOTATION REPAIR | Raw Robin equation is correct when mu = kR and Bi = hR/D. | Homogeneous reference. |
| eq:central-2nd | TRISO-DIS-001 | CONDITIONALLY VERIFIED | Second-order central differences in smooth interior only. | Interior FTCS. |
| eq:explicit-coeffs | TRISO-DIS-002 | CONDITIONALLY VERIFIED | Algebra correct for constant D, uniform grid and i >= 1. | Part-I FTCS. |
| eq:centre-ghost | TRISO-BC-011 | VERIFIED | Correct even-extension symmetry construction. | Centre stencil. |
| eq:centre-stencil | TRISO-DIS-003 | CONDITIONALLY VERIFIED | Correct ghost-node central derivative approximation. | Centre update. |
| eq:lhopital | TRISO-BC-012 | VERIFIED | Correct regular-solution limit; must be treated as a limit. | Centre operator. |
| eq:centre-curvature-limit | TRISO-DIS-004 | CONDITIONALLY VERIFIED | Correct factor-of-six spherical centre operator for this FTCS construction. | Centre update and stability. |
| eq:centre-update | TRISO-DIS-005 | CONDITIONALLY VERIFIED | Algebra correct; coefficient positivity additionally requires Fo <= 1/6. | Global FTCS stability. |
| eq:b:ftcs-ghost | TRISO-BC-013 | CONDITIONALLY VERIFIED | Algebra correct for the stated constant-D, zero-coolant, central-gradient boundary treatment. | Surface update. |
| eq:b:ftcs-surface | TRISO-DIS-006 | CONDITIONALLY VERIFIED | Algebra correct for the ghost construction. | Surface stability. |
| eq:b:ftcs-stability | TRISO-VER-015 | POSSIBLE ERROR / INCOMPLETE | This is a sufficient coefficient-positivity condition, not a complete spectral stability proof, and it omits the centre restriction. | Any FTCS timestep claim. |

## Corrected FTCS coefficient-positivity condition

The interior centre coefficient is 1 - 2 Fo, giving Fo <= 1/2.

The centre-node update is
C0^(j+1) = (1 - 6 Fo) C0^j + 6 Fo C1^j + S0 Delta t.
Therefore coefficient positivity requires Fo <= 1/6.

The surface update gives the additional sufficient bound
Fo <= 1 / [2(1 + kappa(1 + 1/N))].

Therefore a global sufficient coefficient-positivity condition is

Fo <= min{ 1/6, 1 / [2(1 + kappa(1 + 1/N))] }.

This is a monotonicity/non-negative-coefficient condition. It must not be described as a complete spectral stability theorem.

## Major unresolved equation families

1. Source/decay/trapping constitutive model.
2. Physical five-layer analytical eigenproblem and full multilayer transfer formulation.
3. Interface model: concentration continuity versus partition coefficient/interfacial resistance.
4. Part-I modal notation and coefficient derivation.
5. Full FTCS stability analysis beyond coefficient positivity.
6. Variable-D numerical formulation.
7. Equation-to-code traceability for the production WOS implementation.

## Implementation provenance

The WOS implementation referenced by later Ray notes is from repository theodoreOnzGit/outram-park-backend.

Default branch: main.
Ray feature branch: ray-triso-derivation.
Current Ray branch tip: fd9d5c287ce77ab04a89fa4764fcf0a6b9b3e90e.

The Ray branch is 2 commits ahead of main and 0 behind; those two branch-specific commits are the user's derivations/ray/notes.md and derivations/ray/notes.tex.

The WOS scientific implementation is inherited from main. Path-specific history for the current interface, WOS and sphere-FPT modules traces to repository commit 159d1fc4bd56916f5d598da80fa45a2311d6e594, the 2026-09-05 squash merge into main.

Relevant locations:
- crates/boon-lay/src/lagrangian_decay_simulator/lagrangian_diffusion/single_particle_simulator/constructive_solid_geometry/mod.rs
  - TrisoCell::new
  - TrisoCell::new_crp6_geometry
  - TrisoCell::get_triso_region
  - TrisoCell::try_get_diffusion_coefficient
- crates/boon-lay/src/lagrangian_decay_simulator/lagrangian_diffusion/first_passage/walk_on_spheres.rs
  - WoSWalker
  - step_multilayer
  - walk_until_released
  - nearest_interface_distance
  - shell_bounds
  - sample_uniform_in_ball
- crates/boon-lay/src/lagrangian_decay_simulator/lagrangian_diffusion/first_passage/interface.rs
  - does_transmit
- crates/boon-lay/src/lagrangian_decay_simulator/lagrangian_diffusion/first_passage/sphere_fpt.rs
  - sphere first-passage distribution and lookup/interpolation
- verification_and_validation/crp6_case1_kernel_release_vs_crank.md
  - single-layer WOS versus Crank verification record
- docs/buffer_clt_failure_analysis.md
  - legacy Gaussian interface-overshoot failure analysis

## Questions for supervisor

1. Which fission-product species is the canonical first target?
2. Does the first model include radioactive decay?
3. Does it include trapping/release?
4. Is concentration continuous at all interfaces, or are partition coefficients required?
5. Is explicit interfacial resistance required?
6. Are D_i constants, temperature-dependent, fluence-dependent, or fully state-dependent?
7. Is c_infty = 0 an intended physical approximation or only a benchmark?
8. Should both legacy Gaussian diffusion and WOS be retained for method comparison?

## Review-1 gate

The continuous foundation is not yet ready for Review 1.

The single blocking prerequisite is the central consolidation document: it must integrate the corrected governing equation, assumptions, all Part-I analytical steps, corrected eigenvalue notation, centre treatment, Robin ghost derivation, and corrected FTCS stability logic into one continuous derivation that can be audited without reconstructing the argument from separate notes.
