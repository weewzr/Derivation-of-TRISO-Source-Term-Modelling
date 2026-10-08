# Primary Next Gate — Production-Grade Arbitrary-N TRISO FV Solver

## Why this gate exists

The repository already contains executed proof that the canonical conservative spherical FV formulation works on an aligned five-layer verification benchmark:

- arbitrary N in the existing driver for N divisible by five;
- N = 25, 50, 100, 200, 400 executed;
- approximately second-order spatial convergence on that benchmark;
- approximately first-order Forward-Euler temporal convergence;
- generation/release and transient inventory balances close at floating-point scale.

Therefore the project must NOT restart FV derivation from zero.

However, the existing convergence driver is still a verification driver, not yet the final high-resolution TRISO simulator. It hard-codes equal-width synthetic radii [0,1,2,3,4,5], synthetic diffusivities, a steady kernel source and Robin outer boundary, and uses explicit Forward Euler for the transient check.

The next task is to generalise the reviewed mathematics into a production-quality arbitrary-N five-physical-layer solver.

## Scientific target

Keep exactly five physical material regions:

Kernel | Buffer | IPyC | SiC | OPyC.

Allow an arbitrary number of numerical control volumes across them.

The mesh must preserve the true physical interfaces as faces.

The first production transient should use the already-frozen initial-value release problem:

- initial inventory distributed in the kernel according to the frozen benchmark;
- no continuing source after t=0;
- piecewise material diffusivities frozen for the benchmark;
- ideal interface concentration/flux continuity under the frozen assumptions;
- absorbing outer OPyC boundary for the matched release benchmark.

Do not mix this with the older continuously forced + Robin benchmark except as a separate verification case.

## Phase FV-0 — recover and freeze the existing verified core

Before new code, map the already-reviewed generic FV equations to the existing Rust driver.

Record:

- cell-average unknown;
- spherical shell volume;
- volume-centroid coordinate;
- face area;
- two-point resistance/harmonic face conductance;
- centre treatment;
- material-interface face treatment;
- outer-boundary treatment;
- semidiscrete matrix form;
- conservation identity.

Create one concise math-to-code contract. Do not rederive hundreds of manuscript equations.

## Phase FV-1 — general mesh

Replace the synthetic equal-region assumption with a mesh builder taking the five physical radii and a configurable cell allocation.

Requirements:

- arbitrary total N subject only to each physical region receiving a valid number of cells;
- exact alignment of the four physical interfaces with cell faces;
- per-region cell counts configurable independently;
- support for nonuniform spacing within a region or a clearly isolated extension point for it;
- exact spherical shell volumes;
- robust centroid calculation;
- explicit material index for every cell and face.

Test mesh geometry independently before solving diffusion.

## Phase FV-2 — generic conservative operator

Assemble the FV diffusion operator from mesh/material data rather than hard-coded arrays.

For every internal face use the reviewed resistance form.

Require machine-checkable conservation: every internal face flux enters neighbouring cells with equal magnitude and opposite sign.

Keep boundary contributions separate from internal conductances.

## Phase FV-3 — time integration suitable for high resolution

The existing Forward-Euler verification is useful for convergence evidence but is not a good default high-resolution production integrator because its stability limit scales approximately with the square of cell size.

For hundreds/thousands of radial cells, implement a stable implicit production path, preferably:

- Backward Euler as the first robust baseline;
- optionally Crank-Nicolson only after positivity/oscillation behaviour is assessed.

Exploit the radial nearest-neighbour structure with a tridiagonal solver.

Retain explicit Euler only as a verification/reference option.

Derive the discrete implicit equation directly from the existing semidiscrete FV balance before coding it.

## Phase FV-4 — physical transient release observable

For the frozen absorbing-boundary initial-value problem compute:

- c_i(t) cell averages;
- total remaining inventory M(t);
- cumulative released inventory R(t);
- released fraction F(t)=R(t)/M(0);
- outer release flux/rate;
- conservation residual M(t)+R(t)-M(0).

Do not infer cumulative release only from 1-M/M0 if an independently accumulated boundary-flux integral can also be maintained; compare both as a conservation diagnostic.

## Phase FV-5 — verification ladder

Before the full physical five-layer case:

1. homogeneous sphere with analytical reference;
2. synthetic aligned five-layer steady benchmark already used by the repository;
3. controlled two-layer transient benchmark already used for WOS/FV comparison;
4. full frozen five-layer transient.

Reuse existing references rather than inventing easier tests.

## Phase FV-6 — spatial convergence

Run a predeclared refinement sequence. Example:

N approximately 50, 100, 200, 400, 800, 1600

with interface-aligned per-region allocations.

The exact sequence may be adjusted to physical layer thicknesses, but it must be declared before inspecting the final convergence result.

Compare at common physical times:

- F(t);
- M(t);
- selected concentration profiles;
- interface-adjacent concentrations/fluxes;
- release rate if sufficiently resolved.

Report N versus 2N differences and observed order where meaningful.

Do not call 1000 cells 'accurate' without this evidence.

## Phase FV-7 — temporal convergence

Separate spatial and temporal error.

At a sufficiently fine fixed mesh, refine the production time step/tolerance.

For Backward Euler expect first-order temporal convergence. If Crank-Nicolson is later adopted, verify rather than assume second order for the actual discontinuous-property problem and observables.

The previously open transient release-rate convergence item should be revisited here if release rate becomes a claimed output.

## Phase FV-8 — performance scaling

Record runtime and memory versus N.

The tridiagonal implicit solve should scale O(N) per time step.

Demonstrate that N~1000 and beyond is computationally practical rather than assuming it.

## Phase FV-9 — adaptive/nonuniform resolution only after baseline convergence

After uniform/per-region baseline convergence is established, investigate whether cells should be concentrated near:

- material interfaces;
- thin physical layers;
- steep concentration gradients;
- the SiC barrier.

Compare accuracy at equal computational cost.

Do not introduce adaptivity before the simple arbitrary-N solver is verified.

## Phase FV-10 — variable properties only after base solver passes

Only after the frozen piecewise-constant benchmark is converged should the solver be extended to scientifically justified property variation such as:

D = D(material, T, fluence, nuclide)

and possibly T=T(r,t).

At that point many numerical cells within one physical material can resolve property gradients without pretending they are new physical layers.

## WOS relationship

WOS remains a separate verification track.

Rare-event acceleration is non-blocking.

Do not delay FV-0 through FV-8 waiting for R2-WOS-02 or R2-B01.

Once the high-resolution FV result is mature, compare common observables against:

- analytical references where available;
- accepted Process-C release CDF at its reviewed scope;
- ordinary/validated WOS where computationally feasible.

Agreement must be interpreted within each method's assumptions; FV does not automatically close WOS findings.

## Immediate stop/go rule

The next Main Research pass should execute only FV-0 through the implementation plan for FV-4, then implement and run the verification ladder through at least the controlled two-layer case.

If those pass, proceed to a bounded full five-layer convergence run.

If a generic-operator or conservation test fails, stop before large-N production runs.

## Primary deliverables

Create or update:

- a concise arbitrary-N FV math-to-code contract;
- reusable Rust mesh/material/operator modules or a clearly staged standalone implementation;
- unit tests for geometry, interface alignment, conductance and conservation;
- implicit tridiagonal time integrator;
- transient release observable implementation;
- machine-readable convergence outputs;
- a dedicated fast CI workflow separate from manuscript rendering.

## Project priority

This is now the PRIMARY implementation track.

Rare-event WOS research may continue in parallel but must not consume the next-action slot unless specifically needed for a cross-method verification gate.


## Refinement addendum — 2026-10-08

The core FV verification driver already exists; do not restart derivation. The first implementation pass must produce a reusable library-style Rust solver rather than another single-purpose convergence executable.

Hard scientific distinctions:
- Preserve five physical materials; numerical cells are subdivisions, not new material layers.
- Do not conflate the existing continuously generated Robin steady benchmark with the zero-source, initial-inventory absorbing transient release problem.
- Treat interface partition conditions carefully: concentration continuity is only valid for the frozen interface model; do not silently impose it when a partition coefficient is specified.
- For interface-aligned cells, use the reviewed spherical FV conductance and separately test any proposed exact spherical-resistance improvement before substituting it.
- Distinguish conservation error, spatial discretisation error, temporal error and parameter/model uncertainty.
- Avoid reporting release-rate convergence from inventory/CDF convergence alone.
- At the absorbing outer boundary, implement the reviewed face resistance rather than applying a boundary value at the final cell centroid.
- For implicit Euler, verify positivity, diagonal structure and time-step refinement; tridiagonal solve failure must be explicit.
- Include a mesh feasibility rule requiring at least one cell in every physical region, and preserve exact interface-face alignment under refinement.
- Run the homogeneous and two-layer reference gates before expensive five-layer production.

Acceptance is based on executable numerical evidence, not code existence or manuscript appearance. Archive raw inputs, outputs, toolchain versions and reproducible commands. A workflow success is only infrastructure success until scientific metrics are checked.

### Parallel-track decision
The Process-A stopping-time splitting workflow run 37504699223 completed successfully at the CI level. Its scientific gate must be reconciled from its executed output and independently reviewed before any five-layer WOS claim. It does not block the primary arbitrary-N FV development.

### Next action
Main Research should freeze a concise FV math-to-Rust contract, implement generic mesh/operator/implicit transient release solver, and execute bounded homogeneous/two-layer tests. Stop for diagnosis on any failed numerical gate; otherwise proceed to a predeclared five-layer spatial/time convergence study. Do not begin Method 3 or new rare-event algorithm research during this FV pass.
