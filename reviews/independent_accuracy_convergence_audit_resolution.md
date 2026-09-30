# Independent Accuracy/Convergence Audit — Reconciliation and Closure

**Repository:** `weewzr/Derivation-of-TRISO-Source-Term-Modelling`  
**Branch:** `ray/triso-foundation`  
**Reconciliation type:** audit reconciliation / remediation only  
**Immutable audit:** `reviews/independent_accuracy_convergence_audit.md`  
**Audit gate:** **PASS FOR DETERMINISTIC REFERENCE / WOS COMPARISON**

## Scope

This document reconciles the persisted independent accuracy/convergence audit against the current repository state. The audit report itself is immutable evidence and is not modified here.

No WOS comparison is performed in this pass.

The project evidence layers remain distinct:

**continuous analytical reference → deterministic FV reference → production WOS verification**

FV convergence evidence is not WOS validation.

## R3-m01 — VALID — CLOSED

The audit correctly identified that the temporal study forms the actual executed timestep as

[
Delta t_k =
rac{t_{mathrm{end}}}
{leftlceil t_{mathrm{end}}/Delta t_{mathrm{requested},k}ightceil},
]

so successive actual timestep ratios are not guaranteed to be exactly 2.

The verification driver has been remediated so the observed temporal order is calculated from the actual executed timesteps:

[
p_k =
rac{log(E_k/E_{k+1})}
{log(Delta t_k/Delta t_{k+1})}.
]

The driver now retains both requested and actual timesteps in the temporal refinement records and reports both values.

No numerical values are hand-edited.

Closure evidence: GitHub Actions run `36661249962` completed successfully at driver commit `13e9d37a59082c809e21446b631621444023152c`. The artifact reports actual-ratio temporal orders `0.999841`, `0.999780`, `1.000101`, and `0.999980`, confirming approximately first-order temporal convergence:

[
p_t approx 1.
]

## R3-m02 — VALID — CLOSED

The audit correctly identified that the prior workflow relied on the runner's ambient `rustc` without pinning or recording its version.

The workflow now explicitly installs the stable Rust toolchain with `dtolnay/rust-toolchain@stable`, compiles with that toolchain, and records `rustc --version` into `verification/fv_convergence_results.txt` before the numerical output.

Closure evidence: GitHub Actions run `36661249962` completed successfully using `rustc 1.98.1 (48a229cea 2026-09-01)` and persisted that version in the uploaded artifact.

## R3-m03 — VALID — OPEN / NON-BLOCKING

The audit's finding remains valid.

The existing temporal experiment uses concentration-vector self-convergence as its observable. Although release enters the conservation residual, the study does not independently measure temporal convergence of the transient release-rate observable.

No such evidence is invented during this reconciliation.

Therefore R3-m03 remains **OPEN — NON-BLOCKING** and is deferred until release-rate convergence is explicitly required.

## R2-D01 — OPEN

R2-D01 remains open exactly as established by the earlier deterministic discrete audit.

It concerns the retained FTCS Robin benchmark. Closure still requires a dedicated FTCS grid-refinement study using the actual FTCS scheme, a suitable analytical Robin reference, an appropriate error quantity, measured observed order, and comparison with the corrected local (O(Delta r)) boundary-consistency analysis.

The FV convergence study does not close R2-D01.

## R2-B01 — OPEN / SEPARATE

R2-B01 remains a separate production-WOS implementation-verification finding. It is not modified or closed by this FV accuracy/convergence reconciliation.

## Preserved validated evidence

The reconciliation preserves the independently validated conclusions that:

- the FV implementation matches the canonical conservative equations;
- the aligned five-layer steady benchmark exhibits approximately second-order volume-weighted spatial convergence;
- explicit Euler exhibits approximately first-order temporal self-convergence;
- FV inventory conservation closes at floating-point scale;
- the approximately second-order steady result is benchmark-specific rather than a universal theorem for arbitrary discontinuous-(D) meshes or boundary/interface configurations.

No unrelated scientific code is changed.

## Equation register

No new equation identifier is required. The existing `TRISO-ACC-328–329` entry remains the canonical mapping for successive-step temporal Richardson orders.

No stable equation IDs are renamed, duplicated, or renumbered.

The execution-provenance evidence is `verification/fv_convergence.rs`, post-remediation workflow run `36661249962`, and `verification/fv_convergence_results.txt`. The run used driver commit `13e9d37a59082c809e21446b631621444023152c` and Rust `1.98.1`.

## Final reconciliation state

**Blocking FV findings:** none. R3-m01 and R3-m02 are CLOSED by post-remediation GitHub Actions run `36661249962`.

**Non-blocking/open findings:** R3-m03; R2-D01 remains open but isolated to the retained FTCS track; R2-B01 remains separately open for the production WOS verification track.

**Gate preserved:** **PASS FOR DETERMINISTIC REFERENCE / WOS COMPARISON**

The deterministic FV model is cleared to serve as the reference for the subsequent WOS-verification stage only after the remediation workflow run is checked. This reconciliation itself does not begin WOS comparison.
