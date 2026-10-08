# PRIMARY DELIVERY SPRINT — Verified Rust TRISO FV Simulator

## Immediate objective

Deliver a **working, reproducible, numerically verified Rust simulator** for the five-physical-layer TRISO diffusion/release problem to the supervisor as quickly as scientifically responsible.

This is now the controlling short-term project priority. The deliverable is executable code, not another long mathematical manuscript.

## Current starting point

The research repository has a reviewed standalone Rust FV verification driver at `verification/fv_convergence.rs` with executed convergence evidence at `verification/fv_convergence_results.txt`. There is not yet a reusable Cargo-based production FV package in the repository.

Do not restart derivation or claim the existing driver is already a deliverable.

## MVP scope — mandatory

1. Create a clean Cargo project/crate in the research repository, with a runnable binary and reusable library modules.
2. Model exactly five physical regions: Kernel, Buffer, IPyC, SiC, OPyC.
3. Accept five physical region radii, per-region diffusivities, per-region numerical cell allocations and time controls through a documented configuration interface. Default configuration must be executable without editing source code.
4. Use interface-aligned spherical finite volumes, exact shell volumes and conservative face fluxes from the reviewed FV mathematics.
5. Use Backward Euler with a robust tridiagonal solver for transient evolution.
6. First production case: frozen initial kernel inventory, no continuing source, absorbing outer boundary, with correct initial inventory normalisation.
7. Output time series for total inventory, cumulative released inventory/fraction, release flux/rate and conservation residual, plus selected concentration profiles in CSV.
8. Support N approximately 100, 200, 400, 800, 1600 (or a comparable predeclared interface-aligned refinement sequence).
9. Provide a single documented build/run/test command sequence for the supervisor.
10. No dependence on GitHub manuscript compilation, LaTeX, WOS or the supervisor's backend to execute this standalone FV MVP.

## Explicit non-goals for first handoff

- No Method 3.
- No new rare-event or WOS method development.
- No full manuscript rewrite.
- No GUI or digital twin front end.
- No temperature/fluence dependence until base solver is verified.
- No premature promise of full TRISO experimental validation.
- No requirement to resolve R2-WOS-02 or R2-B01 to ship FV code.

## Implementation and verification order

**D0 — Recover mathematical contract.** Map the reviewed FV equations and existing convergence driver to a concise implementation specification. Confirm physical benchmark geometry, interface laws, units, initial and outer boundary conditions. No new derivation cycle.

**D1 — Compile and unit-test.** Implement geometry/mesh, material mapping, conductances, conservative operator, tridiagonal solver and Backward Euler. Ensure `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` and `cargo build --release` pass.

**D2 — Physical/numerical verification.** Test exact mesh/interface alignment, homogeneous analytical benchmark, existing synthetic steady five-layer reference, existing controlled two-layer transient FV reference, nonnegative concentration (within numerical tolerance), mass conservation, and a timestep-refinement check. Diagnose failures rather than tuning tolerances.

**D3 — Five-layer demonstration.** Execute the frozen five-layer initial-release problem at a modest N first, then a bounded mesh refinement to approximately N=1000+. Record common physical observation times, release CDF and inventory; separate spatial from temporal error. Save actual numerical outputs.

**D4 — Supervisor handoff.** Write a short `README` and example configuration, produce a `SUPERVISOR_HANDOFF.md` containing purpose, assumptions, build/run commands, expected output files, verification results, known limitations and precise source commit. Prefer a self-contained repository directory the supervisor can copy or integrate; do not push to Theodore's backend without write permission.

## Scientific guardrails

- Keep physical material count = 5; N counts numerical cells.
- Maintain physical interface alignment.
- Use the correct interface condition for the frozen model; do not silently replace partition physics.
- Compare accumulated boundary release against inventory loss independently.
- Ensure no negative inventory or CDF outside [0,1] beyond justified numerical tolerances.
- Never claim experimental/physical validation from numerical convergence alone.
- The Process-C benchmark can be a secondary comparison only if all model assumptions and initial/boundary conditions are matched.
- Document the limitations of any unmatched comparison.

## CI and acceptance

Create a dedicated fast Cargo CI workflow, separate from manuscript and WOS workflows.

**Minimum acceptance for supervisor handoff:**

- Rust code committed and builds cleanly;
- automated tests pass;
- a real five-layer run completes;
- release/inventory CSVs exist;
- conservation diagnostics pass predeclared tolerances;
- at least a small mesh/time convergence study completes;
- commands are independently reproducible;
- a human-readable verification report records executed evidence and limitations.

If any mandatory gate fails, do not label the package VERIFIED or supervisor-ready. Report the exact blocker.

## Time management

Prioritize a minimum viable scientifically credible deliverable over extra features. Do not let advanced mesh adaptivity, rare-event WOS, analytical completeness, manuscript formatting or Method 3 delay the first Rust handoff.

## Required end-of-pass report

1. Current commit and Rust toolchain;
2. Cargo project location;
3. build/test/CI run status;
4. exact physical and numerical configuration;
5. benchmark results;
6. conservation and positivity results;
7. mesh/time convergence evidence;
8. actual five-layer output paths;
9. run instructions;
10. supervisor handoff document;
11. known limitations;
12. ready-to-send YES/NO and precise blockers if NO;
13. single highest-value next action.

Stop once a verified first-version supervisor package exists, or immediately when a blocking failure needs a targeted correction.
