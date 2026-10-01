# Manuscript Traceability

This table maps the canonical manuscript to repository derivation, executed evidence and independent review. Pending results are explicitly marked.

| Manuscript section / result | Canonical derivation / contract | Verification evidence | Workflow run | Independent review / status |
|---|---|---|---|---|
| Geometry, PDE, centre/interface/outer BCs | docs/triso/13_canonical_equation_register.md | canonical equation register | — | continuous foundation passed |
| Method 1 homogeneous and multilayer modal structure | docs/triso/13_canonical_equation_register.md | canonical analytical derivations registered there | — | verified/qualified per equation status |
| Conservative spherical FV | canonical DIS/ACC equations in equation register | verification/fv_convergence_results.txt and accuracy/convergence resolution | post-remediation FV run 36661249962 | independent accuracy/convergence audit passed for deterministic reference |
| Frozen production WOS contract | docs/triso/21_frozen_production_wos_contract.md | verification/r2_b01_supervisor_integration.rs and WOS diagnostics | multiple; see evidence files | R2-B01 OPEN |
| Controlled WOS/FV finite-epsilon verification | controlled two-layer benchmark/evidence | verification/two_layer_wos_high_power_evidence.md | canonical runs recorded in evidence | independent interface closure audit passed with non-blocking findings |
| Exact shell first-passage transforms | verification/spherical_shell_joint_first_passage_derivation.md | verification/spherical_shell_kernel_evidence.md; verification/spherical_shell_cdf_evidence.md | 36741901064; 36743542042 | prerequisite chain accepted |
| Accelerated two-layer renewal | exact ball/shell derivation + production interface contract | verification/two_layer_accelerated_coupling_evidence.md | 36746018907 canonical corrected run | independently verified for controlled two-layer use |
| Five-layer explicit accelerated pathology | verification/five_layer_accelerated_diagnostic_gate.md | verification/five_layer_accelerated_diagnostic_evidence.md | 36801138769 | computational-usability gate failed; R2-WOS-02/R2-B01 OPEN |
| Interface-state Markov-renewal equation Phi=(I-K)^-1 B | verification/interface_state_markov_renewal_derivation.md | verification/two_layer_matrix_reconciliation_evidence.md | 36807696742 | independent closure audit substantially closed controlled remediation |
| Genuine three-layer multistate matrix | verification/three_layer_multistate_matrix_gate.md | verification/three_layer_multistate_matrix_evidence.md | 36815849244 | independent three-layer audit passed; hard diagnostic authorized |
| Hard five-layer 8x8 transform | verification/hard_five_layer_matrix_gate.md | verification/hard_five_layer_matrix_evidence.md | 36866849598 | f64 numerical gate NOT closed |
| High-precision hard-matrix remediation | verification/hard_five_layer_high_precision_gate.md | verification/hard_five_layer_high_precision_evidence.md | 36869649217 | independently verified with non-blocking findings; controlled inverse-Laplace verification authorized |
| Final five-layer release CDF | — | **DOES NOT EXIST** | — | not authorized; R2-B01 OPEN |


## Deliverable-view policy

This traceability file is a human-readable view of canonical evidence. Canonical derivations and evidence remain in `docs/`, `verification/` and `reviews/`. The supervisor repository is read-only and was not modified.

## Hard high-precision numerical gate

Run `36869649217` (commit `fdb62bd203bf2c618eb58efe026f0a3bf239a480`) used Python 3.12.14, mpmath 1.3.0 and NumPy 2.3.3 at 50/80/120 decimal digits. The 80-digit reference gives `Phi_init(0)=1`, `rho(K(0))=0.99999999994994928880658084574605124`, and residuals far below the original criteria. Evidence: `verification/hard_five_layer_high_precision_evidence.md`. Status: **independently verified with non-blocking findings; controlled inverse-Laplace verification authorized but not yet executed**.


## Detailed-manuscript reconstruction

- Dependency tree: `ray to outram park/EQUATION_DEPENDENCY_TREE.md`.
- Equation parity index: `ray to outram park/EQUATION_PARITY.md`.
- Detailed manuscript backbone: `docs/triso/00_consolidated_mathematical_foundation.md`.
- Original intended derivation path: `notes/raw/TRISO Fuel Derivation Ray V1.tex` (historical/raw; not overwritten).
- Stable equation IDs are preserved for canonical foundation equations; new stochastic/renewal equations use TRISO-FPT, TRISO-WOS, TRISO-MR, TRISO-VER and TRISO-HARD families.
- Current independent hard-matrix gate: VERIFIED WITH NON-BLOCKING FINDINGS; only a separately predeclared controlled transform-to-time verification stage is authorized.


## Final manuscript presentation baseline

The equation-led manuscript retains 1481 stable TRISO equation IDs with one-to-one Markdown/LaTeX parity. Inline mathematics is generated as genuine TeX math rather than escaped control-sequence prose. The standalone LaTeX uses the verified bibliography file and breakable repository paths. Final render/visual QA is controlled by the Outram Park manuscript render audit workflow.


## Equation-to-code function map

| Physical/numerical element | Equation family | Repository implementation | Principal function(s) | Verification |
|---|---|---|---|---|
| Spherical FV mesh/flux/conservation | TRISO-FV / TRISO-ACC | `verification/fv_convergence.rs` | `mesh`, `steady`, `dtmax`, `integrate` | FV convergence and conservation study |
| Two-layer transient FV reference | TRISO-FV | `verification/two_layer_fv_refinement.rs` | `mesh`, `dtmax`, `curve` | two-layer FV refinement |
| Production release/censor semantics | TRISO-WOS | `verification/r2_b01_supervisor_integration.rs` | `five_layer_release_time`, `empirical_cdf` | R2-B01 contract tests |
| Shell exit probability / conditional time | TRISO-FPT-020..065 | `verification/spherical_shell_kernel_verification.rs` | `exact_p_outer`, `exact_cond_mean`, `sample_conditional`, `direct_shell` | shell-kernel evidence |
| Shell conditional CDF | TRISO-FPT | `verification/spherical_shell_cdf_verification.rs` | `analytic_cond_cdf`, `sample_cond`, `direct` | shell-CDF evidence |
| Accelerated ball/shell renewal | TRISO-FPT / TRISO-WOS | `verification/two_layer_accelerated_coupling.rs` | `ball_radius`, `ball_exit_time`, `shell_exit` | accelerated two-layer evidence |
| Two-state matrix reduction | TRISO-MR / TRISO-VER-300..303 | `verification/two_layer_matrix_reconciliation.rs` | `ball_lt`, `shell_lt`, `solve2`, `integrate_init`, `explicit` | B/C reconciliation |
| Hard 8-state transform | TRISO-MR / TRISO-HARD | `verification/hard_five_layer_matrix.rs` | `ball`, `shell`, `mat`, `solve`, `rho`, `init` | hard-matrix diagnostic |
| High-precision hard solve | TRISO-HARD | `verification/hard_five_layer_high_precision.py` | `matrix`, `init_phi`, `evaluate` | run 36869649217 + independent closure audit |


## Validated-candidate promotion — run 36888699218

Canonical validation run **36888699218** at source head `0d63a6215e45a0bde5d737f2b90beeeba96beb41` completed successfully under the permanent two-stage architecture.

Executed evidence:

- Stage A `fast-manuscript-validation`: **PASS**.
- Validator self-test and serializer fixture: **PASS**.
- Stable equation IDs: **1481 Markdown / 1481 LaTeX**, unique and synchronized.
- Structured diff: **0 equations added, 0 removed, 0 equation IDs changed**.
- Strict KaTeX 0.16.22: **1498 PASS / 0 FAIL**.
- Authorized serialization restorations only: **TRISO-ANA-270, TRISO-MR-023, TRISO-MR-024**.
- Scientific mathematics changed: **NO**.
- Stage B `full-manuscript-render`: **PASS**.
- LuaLaTeX/BibTeX repeated build and final reference/citation audit: **PASS**.
- PDF artifact: **143 pages**.

The validated candidate from this run was promoted to the canonical Markdown and LaTeX deliverables without an additional normalization or regex-cleanup pass. The historical failure evidence above is retained.

The programming-escape/control-character defect class demonstrated by the prior `0x09` / `0x08` corruptions is closed for the promoted candidate: the canonical promoted targets contain no forbidden control characters.

The permanent build architecture remains Stage A fast validation followed by dependency-gated Stage B full render.

R2-WOS-02 remains **OPEN**.

R2-B01 remains **OPEN**.
