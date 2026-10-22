# Main Research — Final TRISO Derivation and Verified Rust Supervisor Package

## Authority and objective

Continue from CURRENT remote `main` of `weewzr/Derivation-of-TRISO-Source-Term-Modelling`. Recover the live repository and all current scientific/implementation evidence first. Do not restart first-principles research.

**Immediate goal:** Deliver a concise, mathematically complete, supervisor-ready TRISO derivation plus a working and numerically verified Rust finite-volume simulator.

**Five requested deliverables:**
1. Final mathematical derivation PDF.
2. Matching mathematical derivation Markdown.
3. Concise pseudocode.
4. Separate analytical and FV Rust code files.
5. Separate user-editable literature-value input/configuration file.

Include the LaTeX source, executable example, numerical evidence and handoff instructions as supporting artifacts.

Read at minimum:
- `DELIVERY_SPRINT_RUST_FV.md`
- `WORKFLOW_HIGH_RESOLUTION.md`
- `verification/primary_arbitrary_n_fv_gate.md`
- `verification/rust_analytical_baseline.md`
- `verification/rust_fv_pseudocode_handoff.md`
- `verification/fv_convergence.rs`
- `docs/triso/00_consolidated_mathematical_foundation.md`
- `docs/triso/13_canonical_equation_register.md`
- relevant independent mathematics, FV and convergence reviews;
- the user's analytical Rust reference, if available in the repository or accessible attached source. If it is not accessible, explicitly state this; do not claim to have reconstructed it verbatim.

## 1. Final mathematical derivation

Create a concise submission-ready derivation. Remove the unnecessary opening paragraph and other nonessential introductory prose. Remove repeated explanations, redundant equations, historical remediation, GitHub workflow/debugging notes, WOS rare-event research, obsolete alternatives and unrelated experiments.

**Preserve every necessary mathematical intermediate step**, including:
species conservation -> integral balance -> differential balance -> Fick's law -> spherical-coordinate reduction -> five physical layers -> centre/interface/outer boundary conditions -> initial/source conditions -> analytical/modal formulation where relevant -> conservative FV control-volume integration -> shell volumes and face areas -> interface face flux/resistance -> coefficient collection -> discrete equations -> implicit timestep -> stability/positivity condition -> inventory, boundary release and release fraction.

Never jump directly from PDE to stencil. Clearly state assumptions and distinguish:
- analytical continuously generated/Robin-boundary benchmark;
- numerical initial-inventory/no-continuing-source/absorbing-boundary release case.

Preserve verified scientific reasoning and existing stable equation IDs where appropriate. Do not add unsupported claims or hide derivation steps merely to shorten the document.

Deliver:
- `deliverables/TRISO_Final_Mathematical_Derivation.md`
- `deliverables/TRISO_Final_Mathematical_Derivation.tex`
- `deliverables/TRISO_Final_Mathematical_Derivation.pdf`

Markdown and LaTeX must be scientifically synchronized. Compile the PDF, inspect actual rendered pages, check labels/references and equation readability, and report exact render evidence. If the execution environment cannot compile LaTeX, use the existing GitHub Actions route and do not claim a compile that did not occur. Preserve the original full manuscripts.

## 2. Concise supervisor-facing pseudocode

Include ONLY the final governing PDE, conservative FV cell equation, Backward-Euler equation, explicit Forward-Euler stability bound (labelled verification-only), release/inventory conservation equations and algorithm sequence. The formal mathematical derivation retains the intermediate mathematics; pseudocode does not repeat it.

Sequence:
read inputs -> construct five-region interface-aligned mesh -> assign properties -> compute shell volumes/face conductances -> initialize -> assemble implicit tridiagonal system -> solve -> compute boundary release and inventory -> check conservation/positivity -> write CSV -> refine mesh/time and report convergence.

Deliver: `deliverables/TRISO_FV_Pseudocode.md`.

## 3. Analytical Rust code

Use the supplied analytical Rust example as a structural reference, not as FV code. Keep homogeneous Robin sphere and five-layer transfer-matrix/eigenfunction calculations in a **separate analytical module**. Prefer `uom` for SI units, `csv` for outputs, `plotters` for optional figures. Verify roots, interface matching, homogeneous reduction and modal checks at the claimed scope. Do not use illustrative example radii/diffusivities as literature-verified TRISO properties.

If the analytical component cannot yet be fully validated, state that limitation and do not delay the primary verified FV deliverable. Do not compare analytical and FV results with mismatched source/initial/boundary assumptions.

## 4. Primary numerical FV Rust code

Build a real, runnable Cargo project in `triso-rust/` with reusable analytical and FV modules.

Implement:
- five physical materials: Kernel, Buffer, IPyC, SiC, OPyC;
- arbitrary radial numerical cell counts and per-region allocations;
- physical interfaces aligned with mesh faces;
- exact spherical shell volumes and consistent representative radii;
- reviewed conservative face conductances and correct interface law;
- centre and absorbing outer boundary;
- Backward Euler with a robust tridiagonal solver;
- concentration, inventory, cumulative boundary release, release fraction/rate and mass-balance residual;
- CSV outputs and optional plots;
- configurable mesh refinement to about 1000+ numerical cells.

Use the already-reviewed FV equations; do not invent a new discretization. Preserve an explicit Euler mode only as a verification reference if needed.

Run:
`cargo fmt --check`
`cargo clippy -- -D warnings`
`cargo test`
`cargo build --release`

Execute homogeneous analytical, controlled two-layer and five-layer tests. Record actual outputs, positivity, mass conservation, spatial convergence, temporal convergence, runtime and exact input configuration. A green CI check without numerical evidence is insufficient.

## 5. Separate user-editable literature input file

Create `triso-rust/config/literature_inputs.template.toml` (or an equally clear dedicated configuration file) for user-supplied physical values, without modifying solver logic.

Include, where supported:
- five outer radii;
- five diffusivities;
- species/nuclide and temperature;
- initial inventory or concentration;
- volumetric source magnitude, material/radial distribution and source on/off schedule;
- outer boundary type and Robin coefficient if implemented;
- timestep, end time, per-region cell counts and output times.

For **every missing literature-dependent value**, put one explanatory comment above, the `INSERT_VALUE_HERE` placeholder, and one closing comment below. Preserve SI units and a citation/source slot. Example:

```toml
# INSERT LITERATURE-VERIFIED KERNEL DIFFUSIVITY (m^2/s) BELOW
kernel_diffusivity_m2_s = "INSERT_VALUE_HERE"
# END LITERATURE-VERIFIED KERNEL DIFFUSIVITY
```

Never invent literature values. Never misrepresent the illustrative values in the supplied Rust reference as physical validation.

Create a **separate fully executable synthetic benchmark configuration**, clearly labelled synthetic. The placeholder template may intentionally be non-runnable until filled, but the synthetic configuration must build and execute.

If source schedules, temperature dependence, Robin conditions or other input fields are unsupported in the MVP, mark them clearly unsupported/deferred. Do not pretend they are implemented.

## 6. Supervisor handoff

Create `deliverables/SUPERVISOR_HANDOFF.md` describing the solver, assumptions, directory structure, exact build/run/test commands, literature-value editing instructions, CSV interpretation, executed verification, known limitations, GitHub commit and workflow run IDs.

Do not push to Theodore's repository. Work in the research repository's `main` only.

## 7. Scientific scope and acceptance

The primary objective is the deterministic arbitrary-N FV Rust simulator and concise finalized derivation. No new WOS acceleration, Method 3, CFD, GUI or unrelated manuscript expansion. Retain WOS/Process C as separate verification tracks, not proof of FV/WOS equivalence.

**Do not declare supervisor-ready unless:**
- PDF actually compiles and representative pages are visually inspected;
- Markdown/LaTeX scientific parity is checked;
- pseudocode agrees with the implemented equations;
- Rust builds, lints and tests pass;
- a five-layer numerical simulation executes;
- conservation/positivity checks pass;
- spatial and temporal convergence evidence is stored;
- literature template and executable synthetic config exist;
- handoff instructions and provenance are complete.

If a mandatory check fails, report the precise blocker rather than manufacturing a PASS. Commit all source, documents and reproducible results to `main`.

## End-of-pass report

Report:
1. current commit;
2. PDF path/page count;
3. Markdown path;
4. LaTeX path;
5. pseudocode path;
6. analytical Rust source paths;
7. FV Rust source paths;
8. literature-input template;
9. executable synthetic configuration;
10. build/lint/test status;
11. analytical benchmark;
12. FV numerical verification;
13. five-layer execution;
14. spatial/time convergence;
15. conservation/positivity;
16. CSV/plot paths;
17. supervisor handoff path;
18. workflow run IDs;
19. known limitations;
20. supervisor-ready YES/NO;
21. single highest-value next action.

**STOP** when the verified package is complete, or report the exact blocking issue. Do not proceed into new research.
