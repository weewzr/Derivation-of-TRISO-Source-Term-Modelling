# Project Workflow — First-Principles TRISO to High-Resolution Numerical Simulation

## Purpose

This document records the clarified implementation objective of the project.

The physical TRISO model remains a **five-material-layer** model:

1. fuel kernel;
2. Buffer;
3. IPyC;
4. SiC;
5. OPyC.

Numerical accuracy is NOT obtained by inventing hundreds or thousands of new physical materials. It is obtained by subdividing the five physical regions into an arbitrary number N of **numerical radial control volumes** and demonstrating convergence as N increases.

The primary engineering objective is therefore:

**first principles -> five-layer continuous model -> arbitrary-N conservative discretisation -> Rust implementation -> grid/time convergence -> release observables.**

WOS remains an independent stochastic verification stream. Rare-event acceleration is an optional WOS verification tool and must not block development of the deterministic high-resolution solver.

## Track 0 — First-principles physics [substantially complete]

Derive and preserve:

species conservation
-> Fick's law
-> conservative diffusion equation
-> spherical symmetry
-> piecewise material properties
-> centre regularity
-> material-interface conditions
-> outer boundary
-> initial/source conditions.

The mathematics should continue only until the physical model is unambiguous.

## Track 1 — Five-physical-layer continuous TRISO model [substantially complete]

The canonical physical geometry is five concentric regions.

The governing equation is solved with the actual material interfaces retained exactly.

Physical-layer count and numerical-cell count are distinct concepts.

Do not replace the five physical regions with 1000 fictitious material layers merely to increase resolution.

## Track 2 — Arbitrary-N finite-volume formulation [PRIMARY IMPLEMENTATION TRACK]

Generalise the already-derived conservative spherical FV formulation to an arbitrary radial mesh.

Requirements:

- N is a runtime/configuration parameter, not hard-coded;
- each control volume knows its physical material;
- physical interfaces coincide with FV faces wherever practical;
- conservative face fluxes are used;
- spherical face areas and exact shell volumes are retained;
- nonuniform radial spacing is supported or explicitly planned;
- material-dependent diffusivity is evaluated consistently;
- centre and outer boundary treatments remain conservative;
- release flux / cumulative release observables are defined from the same balance law.

The mathematical deliverable is a generic cell/face equation valid for arbitrary N. Once that is frozen, further increases in N are a code/execution task, not a new derivation.

## Track 3 — Independent Rust FV solver [NEXT CORE DEVELOPMENT]

Develop a clean Rust implementation independent of manuscript-generation machinery.

Suggested architecture:

- geometry / five physical regions;
- materials / diffusivity;
- mesh / arbitrary-N radial control volumes;
- FV face coefficients and interface fluxes;
- time integrator / linear solve;
- initial and boundary conditions;
- release flux and cumulative release;
- diagnostics and conservation;
- convergence runner.

The implementation must support automated mesh sequences such as:

N = 25, 50, 100, 200, 400, 800, 1600

or another predeclared refinement sequence.

Do not assume N=1000 is intrinsically sufficient. Determine adequacy from convergence.

## Track 4 — Deterministic convergence and accuracy [CORE ACCEPTANCE]

For each refinement level report at minimum:

- concentration profile c(r,t);
- total remaining inventory;
- cumulative released fraction F(t);
- outer release flux where reliable;
- global mass-balance error;
- differences between N and 2N;
- temporal-step sensitivity separately from spatial refinement;
- runtime and memory.

Use analytical/simple benchmarks first, then the full five-layer model.

The target is a **grid-converged solution**, not a particular cell count.

If 500, 1000 and 2000 cells agree within the predeclared tolerance, that is stronger evidence than merely choosing 1000 cells.

## Track 5 — Spatially varying properties [AFTER BASE FV CONVERGENCE]

Once the piecewise-constant five-layer solver is verified, extend the numerical property model where scientifically justified.

Examples:

D = D(material, T, fluence, nuclide)

and, if a temperature field is available,

T = T(r,t)
-> D = D(r,t).

This is a major reason arbitrary-N resolution can be useful: properties may vary within one physical material even though the particle still has only five physical material regions.

Do not introduce spatially varying correlations before the base constant-property solver passes convergence and conservation gates.

## Track 6 — Ordinary WOS [SECONDARY INDEPENDENT VERIFICATION TRACK]

Retain the mathematically defined ordinary production WOS as an independent stochastic implementation of the diffusion problem.

Validation ladder:

homogeneous sphere
-> controlled two-layer problem
-> five-layer problem where computationally feasible.

FV success does not prove WOS correctness, and WOS success does not replace FV convergence.

The ordinary WOS implementation may be developed independently now; its unresolved five-layer computational efficiency does not block the FV/Rust track.

## Track 7 — Rare-event WOS acceleration [OPTIONAL / NON-BLOCKING]

Stopping-time splitting, weighted ensemble, Process B/C and related rare-event methods exist to address the computational difficulty of the five-layer stochastic route.

They are NOT additional physical models.

They must preserve or explicitly relate to the target stochastic law before being used as verification evidence.

Current rare-event research must not block:

- arbitrary-N FV development;
- deterministic convergence;
- high-resolution five-layer simulation;
- property-model development after FV verification.

Continue rare-event work only when it provides useful independent WOS verification evidence.

## Track 8 — Cross-method verification

Where common observables exist, compare:

Method 1 analytical reference
vs
Method 2 FV
vs
Method 2 ordinary/validated accelerated WOS.

Use simple geometries first and the five-layer case only after each route passes its own internal gates.

Keep numerical uncertainty, Monte-Carlo uncertainty and model-form assumptions separate.

## Track 9 — Method 3

Method 3 remains the semi-empirical/reduced-order route:

experimentally fitted diffusivities
-> D(T, ...)
-> Booth / breakthrough / release-to-birth formulations
-> release fraction/rate.

Method 3 is not required before implementing the arbitrary-N first-principles FV solver.

Its eventual comparison should be against the verified first-principles computational result, with assumptions and calibration clearly separated.

## Immediate priority order

1. Freeze/document the arbitrary-N FV mathematical contract from the already-reviewed FV derivation.
2. Implement or refactor a clean arbitrary-N Rust FV solver.
3. Verify conservation and simple analytical benchmarks.
4. Run spatial and temporal convergence studies.
5. Execute the five-physical-layer high-resolution model over a refinement sequence.
6. Determine the minimum mesh giving the required accuracy; do not preselect 1000 as truth.
7. Only then extend to within-material spatially varying properties if required.
8. Maintain WOS as an independent verification stream in parallel; rare-event acceleration is non-blocking.
9. Reassess Method 3 after the deterministic first-principles solver is mature.

## Workflow-controller rule

For future next-action decisions, prefer progress on the PRIMARY arbitrary-N FV/Rust track whenever rare-event WOS research is not required to establish the next deterministic milestone.

Do not allow unresolved WOS rare-event efficiency to stall the core high-resolution solver.

Do not spend another manuscript-formatting cycle unless a scientific deliverable actually requires it.

## Acceptance philosophy

The project is complete at the numerical-resolution level only when increasing N and refining time resolution no longer changes the required observables beyond predeclared tolerances while conservation remains satisfied.

The correct statement is therefore:

**five physical layers, arbitrarily many numerical cells, convergence determines sufficient resolution.**
