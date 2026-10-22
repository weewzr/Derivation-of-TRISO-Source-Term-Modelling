# Independent Final Supervisor Delivery Audit

**Repository:** `weewzr/Derivation-of-TRISO-Source-Term-Modelling`  
**Canonical branch:** `main`  
**Reviewed current main:** `8d3525bdd00fcc9e290a202f1f4eb1c58c0b5d13`  
**Final-delivery workflow:** `39711910260`  
**Final-delivery execution/source commit:** `803bfd4a8c44b981406290253cbbaa2c3314433f`  
**Final-delivery artifact:** `final-supervisor-delivery-evidence` / `12396634301`  
**Artifact digest:** `sha256:19062d34e795cc053ebccec863d2249c0826ace6853352009a1d68babe85c8a2`  
**Supervisor source inspected:** `theodoreOnzGit/outram-park-backend/main` at `8d31482d127e211614ebb3f66b1076e1ed6dea98`  
**Scope:** final supervisor delivery / mathematical, numerical, reproducibility and documentation audit  
**Supervisor repository modified:** No  
**Scientific project files modified by this review:** No

# Final gate

## B. FINAL SUPERVISOR DELIVERY ACCEPTED WITH NON-BLOCKING FINDINGS

The final deterministic FV supervisor package is acceptable for handoff at the stated MVP scope.

The core mathematical derivation, conservative spherical FV formulation, Rust implementation, executable synthetic five-layer case, conservation/positivity diagnostics, spatial and temporal convergence evidence, literature-input placeholder strategy, build/test workflow, and PDF delivery are substantively consistent.

The production WOS / Process-C research track remains correctly qualified as a separate evidence stream. **R2-WOS-02 and R2-B01 remain OPEN** and are not silently closed by the FV package or Process-C inversion result.

Two moderate documentation/provenance findings and two minor repository-status findings remain. None invalidates the deterministic FV solver or makes the current supervisor package scientifically unusable.

---

# 1. Current repository state

The current remote `main` ref resolves to:

`8d3525bdd00fcc9e290a202f1f4eb1c58c0b5d13`.

This is later than the source commit exercised by the final supervisor-delivery workflow:

`803bfd4a8c44b981406290253cbbaa2c3314433f`.

The six commits between those points were inspected. They modify delivery/provenance material and the final workflow's persistence list; they do not modify `triso-rust/src` scientific solver code.

Therefore the final numerical source actually exercised by run `39711910260` remains the scientific source present at current `main`.

The current `main` is the canonical active branch. No branch switch or feature-branch modification was performed.

---

# 2. Canonical final-delivery workflow

Workflow `.github/workflows/final-supervisor-delivery.yml` was independently inspected.

It performs, in a clean GitHub Actions checkout:

1. stable Rust installation with rustfmt/clippy;
2. `cargo fmt` and `cargo fmt --check`;
3. `cargo clippy --all-targets -- -D warnings`;
4. `cargo test`;
5. `cargo build --release`;
6. numerical verification executable;
7. executable five-layer synthetic case;
8. Rust toolchain/provenance recording;
9. two-pass `pdflatex -halt-on-error`;
10. PDF metadata/render generation;
11. numerical CSV copying;
12. artifact upload;
13. evidence persistence.

The numerical steps use `set -o pipefail`, so a failure from the Rust executable is not hidden by the `tee`.

The only visible `|| true` is attached to the LaTeX warning-reporting grep, not to PDF compilation or any scientific numerical gate.

The workflow therefore has meaningful failure propagation.

**Workflow verdict:** PASS.

---

# 3. Final-delivery run

Run `39711910260` completed successfully.

Job `119604958839` shows all intended stages completed successfully:

- checkout: PASS;
- stable Rust toolchain: PASS;
- rustfmt: PASS;
- clippy: PASS;
- cargo test: PASS;
- release build: PASS;
- numerical verification: PASS;
- five-layer executable: PASS;
- provenance recording: PASS;
- LaTeX compilation/render: PASS;
- CSV evidence: PASS;
- artifact upload: PASS;
- evidence persistence: PASS.

Artifact `12396634301` is still available and has the recorded SHA-256 digest above.

The artifact was independently retrieved and the PDF was independently rendered from the artifact. Pages 1, 2 and 3 were visually inspected at 160 dpi. No clipping, overlap, page truncation or broken mathematical glyphs were observed.

**Final execution verdict:** PASS.

---

# 4. Historical final-delivery failure reconciliation

The project prompt identified a historical failure near 800 numerical cells, approximately:

[
F=1.000000001874,
qquad
mathrm{balance}_{rel}approx1.874	imes10^{-9},
]

against an active acceptance requirement below (10^{-9}).

Repository history shows this was not resolved by simply relaxing that numerical assertion.

The relevant sequence was:

- `fe54c36...`: transient convergence horizon change;
- `4ae177a...`: informative synthetic release horizon;
- `23361ae...`: explicit numerical-failure propagation in delivery workflow;
- `803bfd4...`: correction of five-layer convergence horizon replacements.

Current `verify.rs` still contains the active assertion:

[
|mathrm{balance}_{rel}|<10^{-9}.
]

The final executed result is:

- 800 cells: (8.763	imes10^{-10});
- 1600 cells: (6.332	imes10^{-10}).

Thus the current run passes the unchanged scientific threshold.

The convergence-horizon changes affect the simulation horizon used by the convergence experiment; they are not a waiver of the balance criterion.

**Verdict:** historical failure was genuinely remediated at the acceptance level; no post-hoc relaxation of the (10^{-9}) balance gate was found.

---

# 5. Final mathematical derivation audit

The final Markdown and LaTeX derivations preserve the required chain:

[
	ext{inventory conservation}
ightarrow
	ext{local conservation}
ightarrow
	ext{Fick law}
ightarrow

ablacdot(D
abla c)+S
ightarrow
	ext{spherical multilayer PDE}.
]

The five-layer model retains:

- Kernel;
- Buffer;
- IPyC;
- SiC;
- OPyC.

The derivation explicitly distinguishes the main FV release benchmark:

- initial kernel inventory;
- no continuing source after (t=0);
- absorbing outer boundary;

from the separate analytical benchmark:

- continuous generation;
- Robin outer boundary.

That distinction is scientifically important and correctly preserved.

The finite-volume equations use exact spherical shell volumes,

[
V_j=rac{4pi}{3}
(r_{j+1/2}^3-r_{j-1/2}^3),
]

and the conservative series-resistance face conductance,

[
G_f=
rac{A_f}
{delta_P/D_P+delta_E/D_E}.
]

Dimensions are consistent:

[
[G_f]=mathrm{m^3/s},
]

so (G_f c) has units of mol/s for concentration in mol/m(^3).

The centre treatment uses zero west-face area rather than a ghost node, which is consistent with the integral spherical control-volume formulation.

The absorbing outer flux uses the OPyC diffusivity and the last cell centroid-to-boundary resistance.

Backward Euler is assembled as a tridiagonal system with positive diagonal and non-positive off-diagonal entries.

The Forward-Euler condition is correctly labelled verification-only.

**Mathematical derivation verdict:** PASS.

---

# 6. FV implementation audit

Current `triso-rust/src/fv.rs` independently maps onto the derivation.

## Geometry

`build_mesh()`:

- accepts per-region cell counts;
- aligns each physical interface with a face;
- computes exact shell volumes;
- computes volume-centroid radii;
- assigns physical material indices;
- stores layer diffusivities per cell.

## Interface conductance

The implementation computes:

[
G_f=
rac{4pi r_f^2}
{(r_f-r_P)/D_P+(r_E-r_f)/D_E}.
]

This is the declared series-resistance formulation.

## Centre

For the first cell:

[
G_W=0.
]

No singular radial expression is evaluated numerically at (r=0).

## Outer boundary

The final cell uses:

[
G_R=
rac{4pi R^2D_5}{R-r_N}.
]

The accumulated release uses the same boundary flux:

[
dot M_{rel}=G_R c_N.
]

## Time integration

Backward Euler is implemented through a Thomas-type tridiagonal solver.

The implementation rejects invalid timestep/state dimensions and singular pivots.

## Conservation

Cumulative release is separately accumulated from the implicit boundary flux and uses compensated summation to reduce long-run floating-point drift.

The resulting residual is independently exposed rather than inferred only from (1-M/M_0).

**FV implementation verdict:** PASS at the delivered MVP scope.

---

# 7. Rust usability

The repository contains:

`triso-rust/Cargo.toml)

and:

`triso-rust/Cargo.lock`.

The supervisor README provides an executable command sequence:

`cargo fmt --check`

`cargo clippy --all-targets -- -D warnings`

`cargo test`

`cargo build --release`

`cargo run --release -- config/synthetic_five_layer.toml`

`cargo run --release --example verify`.

The synthetic configuration is runnable without source editing.

The literature template is intentionally non-runnable until physical inputs are supplied.

No hidden dependence on the supervisor's WOS backend is required by the FV MVP.

The workflow itself demonstrates that a clean runner can build and execute the project.

The delivered Rust toolchain evidence records:

`rustc 1.99.1`

`cargo 1.99.1`.

**Usability verdict:** PASS.

---

# 8. Numerical verification

The final executed numerical evidence gives:

## Homogeneous analytical comparison

[
F_{FV}=0.9953254095,
qquad
F_{exact}=0.9953532113,
]

absolute difference:

[
2.780	imes10^{-5}.
]

This is within the declared verification threshold.

## Controlled two-layer comparison

[
F_{FV}=0.9999870002,
qquad
F_{ref}=0.99998700,
]

absolute difference:

[
1.847	imes10^{-10}.
]

## Five-layer spatial refinement

Release fractions are:

- 100 cells: (0.4324121657447);
- 200 cells: (0.4324286068846);
- 400 cells: (0.4324327175498);
- 800 cells: (0.4324337451508);
- 1600 cells: (0.4324340024425).

Successive differences are approximately:

[
1.6441	imes10^{-5},
4.1107	imes10^{-6},
1.0276	imes10^{-6},
2.5729	imes10^{-7}.
]

Independent recalculation gives observed orders approximately:

[
1.9999,quad2.0001,quad1.9978.
]

Thus the reported approximately second-order spatial behaviour is independently confirmed.

## Temporal refinement

Successive differences are approximately:

[
2.4270	imes10^{-4},
1.2147	imes10^{-4},
6.0766	imes10^{-5}.
]

Independent recalculation gives observed order approximately:

[
0.999,
]

consistent with Backward Euler's expected first-order temporal convergence.

The final numerical evidence therefore supports the stated spatial/time convergence claims for this synthetic benchmark.

**Verification verdict:** PASS.

---

# 9. Positivity and conservation

Minimum final concentrations remain positive across the delivered spatial sequence.

The 800-cell and 1600-cell balance residuals remain below the active (10^{-9}) relative gate.

The five-layer executable reports:

[
F=0.4324286068846,
]

[
M_{final}=1.901949993686	imes10^{-11} mathrm{mol},
]

[
|mathrm{balance residual}|
=
8.69677	imes10^{-22} mathrm{mol}.
]

This is approximately (4.57	imes10^{-11}) relative to the initial inventory, consistent with the reported pass.

The use of the same outer boundary flux for both release and conservation is mathematically appropriate.

**Verdict:** PASS.

---

# 10. Literature-input strategy

The template:

`triso-rust/config/literature_inputs.template.toml`

does not invent physical parameters.

It explicitly places:

- `INSERT_VALUE_HERE`;
- adjacent source/citation slots;
- explanatory comments;
- SI units.

The synthetic configuration is separate and explicitly labelled non-literature data.

This is scientifically preferable to embedding unverified Cs-137 or TRISO material numbers in the production solver.

**Verdict:** PASS.

---

# 11. Analytical reference limitation

The package's `analytical.rs` implements:

- homogeneous Robin eigenvalue roots;
- homogeneous-layer transfer building blocks.

It is **not** a verbatim reconstruction of the user's original full analytical Rust upload, and the supervisor handoff explicitly states this limitation.

That wording is correct and avoids an unsupported provenance claim.

The final deterministic FV acceptance does not depend on the analytical Rust module being a complete reproduction of the unavailable uploaded source.

**Verdict:** PASS WITH EXPLICIT LIMITATION.

---

# 12. Hard five-layer Process-C transform

The prior independent hard-matrix closure audit established:

- correct 8-state topology;
- (ho(K(0))<1);
- high-precision normalization;
- high-precision residuals;
- stable transform evaluation;
- quadrature convergence;
- hard-matrix conditioning;
- 50/80/120 digit convergence.

The high-precision reference gave approximately:

[
kappa_2(I-K(0))=5.20	imes10^{10},
]

with:

[
ho(K(0))
=
0.9999999999499492888ldots.
]

This explains the historical f64 normalization failure without requiring a physical/topological defect.

The independent hard-matrix closure audit cleared the controlled release-time recovery/inverse-Laplace stage.

**Verdict:** PASS within Process-C scope.

---

# 13. Inverse-Laplace / final five-layer Process-C release CDF

The independent inverse-Laplace audit accepted the Process-C five-layer release-time recovery with non-blocking findings.

The production run used:

`37008548408`

and an 81-point physical time grid.

The audit established:

- Stehfest/de Hoog agreement;
- 50/80/120 precision checks;
- source-factor validation;
- complex-s continuation checks;
- CDF/survival closure;
- monotonicity and bounds over the resolved grid.

The audit explicitly retained the limitation that the high-s forward-transform reconstruction diagnostic is coarse and should not be treated as a pointwise inversion certification.

It also explicitly did **not** close the finite-capture production WOS question.

Therefore the Process-C CDF is acceptable as a separately qualified research result, not as proof that the direct production WOS path is equivalent.

**Verdict:** PASS WITH NON-BLOCKING LIMITATION.

---

# 14. R2-WOS-02

## OPEN

The deterministic FV supervisor package does not close R2-WOS-02.

The project itself correctly continues to describe direct finite-capture production WOS as a separate verification path with unresolved computational/continuum-equivalence limitations.

No evidence inspected here justifies silently changing that status.

---

# 15. R2-B01

## OPEN

The final Process-C release CDF does not close R2-B01.

The project has demonstrated a highly credible accelerated/matrix Process-C result, but that is not equivalent to demonstrating a completed integrated five-layer finite-capture production-WOS release-time estimator.

This distinction is correctly preserved in the final-facing documents.

---

# 16. Final supervisor-document audit

The following intended deliverables are present:

- `deliverables/TRISO_Final_Mathematical_Derivation.md`;
- `deliverables/TRISO_Final_Mathematical_Derivation.tex`;
- `deliverables/TRISO_Final_Mathematical_Derivation.pdf`;
- `deliverables/TRISO_FV_Pseudocode.md`;
- `deliverables/SUPERVISOR_HANDOFF.md`;
- `triso-rust` Cargo project;
- literature-input template;
- executable synthetic configuration;
- numerical CSV evidence;
- parity and verification records.

The main derivation and LaTeX are scientifically synchronized.

The supervisor handoff accurately reports current scope and limitations.

---

# 17. Finding RFD-M01 — MODERATE

**Affected file:** `deliverables/TRISO_FV_Pseudocode.md`

**Finding:** The pseudocode file's displayed mathematics is malformed in the current stored Markdown. The text retrieved from the repository contains control-sequence corruption such as:

`rac`, `left`, `
ight`, `tle`

instead of valid GitHub Markdown LaTeX such as:

`\frac`, `\left`, `\right`, `\le`.

The algorithmic pseudocode itself is understandable, but the mathematical equation section does not render reliably.

**Consequence:** A supervisor viewing the pseudocode on GitHub cannot rely on the mathematical equations being rendered as intended.

**Required remediation:** Replace the malformed Markdown math escapes with valid GitHub-renderable LaTeX. Do not change the underlying equations.

**Closure criterion:** The equations render correctly in GitHub Markdown and remain mathematically identical to the final derivation.

**Stage:** documentation cleanup; does not block the executable FV package.

---

# 18. Finding RFD-M02 — MODERATE

**Affected files:** `paper/main.tex`, `paper/references.bib`

**Finding:** The manuscript states specific frozen five-layer Cs-137 radii and diffusivity values, but the current bibliography contains only the NIST DLMF reference.

The final delivery package itself correctly avoids presenting those values as verified physical inputs and requires literature values to be entered through the user template. However, the manuscript's hard-benchmark parameter claims therefore lack complete durable bibliography provenance in its current form.

**Consequence:** The manuscript is not publication-ready as a fully sourced physical-data document.

**Required remediation:** Add authoritative literature citations for the specific physical parameter claims before treating the manuscript as a publication-facing scientific document.

**Closure criterion:** Every substantive physical parameter/correlation claim has a stable source or is explicitly labelled synthetic/illustrative.

**Stage:** documentation/provenance; does not block the deterministic synthetic FV supervisor package.

---

# 19. Finding RFD-m01 — MINOR

**Affected files:** current main status documentation and final-delivery provenance.

**Finding:** The final verification workflow that exercised the scientific source ran at commit `803bfd4...`, whereas current `main` is `8d3525b...`.

The six later commits do not modify the FV scientific source, but the final workflow's latest execution is therefore not literally at the current HEAD.

**Consequence:** Provenance requires one extra comparison step, which was independently performed in this audit.

**Required remediation:** Future delivery records should identify both:
- the exact scientifically executed source commit;
- the later evidence/persistence commits.

A rerun on every metadata-only change is not scientifically necessary.

**Closure criterion:** Current delivery notes consistently distinguish execution source from later evidence-persistence commits.

**Stage:** repository provenance.

---

# 20. Finding RFD-m02 — MINOR

**Affected files:** `paper/main.tex`, `verification/hard_five_layer_high_precision_evidence.md`

**Finding:** Some historical wording still says the hard high-precision result or matrix stage is “pending independent review”, even though the corresponding independent reviews have subsequently been completed.

**Consequence:** A reader could misunderstand the current evidence status by opening historical documents directly.

**Required remediation:** Retain the historical evidence unchanged where it is explicitly intended as an immutable record, but add/update a clear current-status pointer in active documents so historical “pending” statements are visibly superseded.

**Closure criterion:** Active navigation documents point to the completed independent reviews and no active-status page treats the hard matrix/inversion as still awaiting the already completed review.

**Stage:** documentation hygiene.

---

# 21. Notes, not findings

### Configuration validation

`Config::validate()` checks positive radii/diffusivities, non-zero cells and positive time controls, but does not explicitly reject NaN/infinite floating-point values or validate temperature against a scientific domain.

This is reasonable implementation hardening for a later code audit and does not invalidate the current controlled synthetic package.

### Explicit Euler statement

The final derivation correctly labels the Forward-Euler stability restriction as verification-only; Backward Euler is the production default.

### WOS scope

No WOS result was promoted to validate the deterministic FV model, and no FV result was promoted to close WOS findings.

---

# 22. Traceability / reproducibility verdict

The principal deterministic delivery claims have a reproducible chain:

[
	ext{equation}
ightarrow
	ext{FV implementation}
ightarrow
	ext{verification executable}
ightarrow
	ext{GitHub Actions run}
ightarrow
	ext{artifact}
ightarrow
	ext{persisted result}
ightarrow
	ext{handoff}.
]

For the main FV quantities, this chain is sufficiently explicit.

The Process-C transform/CDF claims likewise have a separate chain through the dedicated high-precision and inverse-Laplace workflows.

The project does **not** collapse these evidence streams into a single claim of “the code is physically validated”.

**Traceability verdict:** PASS.

---

# 23. Overall findings count

**CRITICAL:** 0  
**MAJOR:** 0  
**MODERATE:** 2  
**MINOR:** 2  
**NOTE:** implementation-hardening observations only

No finding invalidates the final deterministic FV numerical package.

---

# 24. Final gate

## **B. FINAL SUPERVISOR DELIVERY ACCEPTED WITH NON-BLOCKING FINDINGS**

The deterministic five-physical-layer arbitrary-N Rust FV package is ready for supervisor handoff at the explicitly stated MVP scope.

The package demonstrates:

- a coherent first-principles derivation;
- conservative spherical FV mathematics;
- executable arbitrary per-region mesh allocation;
- Backward-Euler transient integration;
- analytical/homogeneous and controlled two-layer checks;
- approximately second-order spatial convergence on the declared synthetic benchmark;
- approximately first-order temporal convergence;
- positive concentrations;
- inventory/release conservation;
- an executable five-layer synthetic run;
- reproducible build/test commands;
- a user-editable literature-input template;
- PDF compilation and visual inspection;
- explicit limitations.

The package does **not** claim that direct finite-capture production WOS is validated, and that restraint is scientifically correct.

R2-WOS-02 and R2-B01 remain OPEN.

---

# 25. Single highest-value next action

**Complete the two active documentation fixes—especially the malformed `TRISO_FV_Pseudocode.md` math—and then hand the deterministic FV package to the supervisor without reopening the mathematical derivation.**

The next scientific research gate should remain separate from the delivery package: continue resolving the Method-2 WOS integration questions only when their evidence is strong enough to justify a new independent review.

STOP.
