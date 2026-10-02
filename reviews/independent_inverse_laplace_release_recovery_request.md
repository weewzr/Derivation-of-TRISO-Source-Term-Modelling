# Independent Review Request — Controlled Process-C Inverse-Laplace Recovery

## Scope

Review the completed Method-2 release-time recovery evidence only. Do not reopen the independently verified hard five-layer 8x8 transform unless a concrete inversion inconsistency points back to it. Do not begin Method 3.

Primary executed evidence:
- verification/inverse_laplace_synthetic_executed_36991554900.json
- verification/inverse_laplace_il1_executed_37006915675.json
- verification/inverse_laplace_il2_executed_37007436157.json
- verification/inverse_laplace_il3_executed_37008548408.json
- verification/inverse_laplace_il3_executed_evidence.md
- verification/inverse_laplace_method_selection.md
- verification/inverse_laplace_runtime_cost_model.md
- verification/inverse_laplace_il3_plan.md
- verification/inverse_laplace_il3_production.py
- reviews/independent_hard_five_layer_matrix_closure_audit.md

## Questions requiring independent disposition

1. Is the inversion identity correct: Phi(s)/s for CDF and [1-Phi(s)]/s for survival?
2. Are Stehfest and de Hoog sufficiently independent numerical routes for this verification?
3. Does the synthetic benchmark evidence adequately establish implementation correctness?
4. Is the complex-s continuation sufficiently justified for de Hoog?
5. Is the analytical source factor Q(s)=3[z coth(z)-1]/z^2 correctly derived and sufficiently cross-checked?
6. Is the IL-2/IL-3 interpretation of tiny alternating early-time Stehfest values as numerical cancellation around an unresolved-zero CDF justified by the precision and de Hoog evidence?
7. Are the 81-point de Hoog bounds/monotonicity PASS and the Stehfest resolved-region agreement sufficient for a controlled release-CDF claim?
8. Is F+S-1 closure at ~1e-74 to 1e-80 adequate?
9. Does the maximum method discrepancy 2.2e-60 materially support method agreement?
10. Is the forward-reconstruction diagnostic correctly interpreted as a coarse finite-grid/window quadrature diagnostic rather than a 9.4% inversion-method error at high s?
11. Are any additional targeted numerical checks required before accepting the five-layer release CDF?
12. Can R2-WOS-02 and/or R2-B01 be closed on this evidence, or must either remain open?

## Required classification

Return one:
A. VERIFIED — controlled five-layer release CDF accepted;
B. VERIFIED WITH NON-BLOCKING FINDINGS — release CDF accepted with stated limitations;
C. NOT YET VERIFIED — specific additional numerical evidence required;
D. BLOCKED — substantive mathematical/numerical defect.

List every finding with severity and exact evidence. Do not alter immutable prior review records.
