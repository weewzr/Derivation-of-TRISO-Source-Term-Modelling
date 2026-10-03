# Independent Reviewer Prompt — Controlled Process-C Inverse-Laplace / Release-Time Recovery

Continue as the Independent Reviewer for the existing TRISO source-term derivation project.

DO NOT restart the project.
DO NOT repeat Reviews 1 or 2.
DO NOT redo the literature review.
DO NOT begin Method 3.
DO NOT modify Main Research's scientific implementation.

The canonical state is the CURRENT GitHub repository:

`weewzr/Derivation-of-TRISO-Source-Term-Modelling`

branch:

`main`

First recover the current live repository state yourself.

The dedicated review request is:

`reviews/independent_inverse_laplace_release_recovery_request.md`

Conduct the independent controlled Process-C inverse-Laplace / release-time-recovery audit requested there.

At minimum independently inspect:

- `reviews/independent_inverse_laplace_release_recovery_request.md`
- `reviews/independent_hard_five_layer_matrix_closure_audit.md`
- `verification/inverse_laplace_method_selection.md`
- `verification/inverse_laplace_runtime_cost_model.md`
- `verification/inverse_laplace_il1_executed_evidence.md`
- `verification/inverse_laplace_il2_plan.md`
- `verification/inverse_laplace_il2_executed_evidence.md`
- `verification/inverse_laplace_il3_plan.md`
- `verification/inverse_laplace_il3_executed_evidence.md`
- `verification/inverse_laplace_synthetic_executed_36991554900.json`
- `verification/inverse_laplace_il1_executed_37006915675.json`
- `verification/inverse_laplace_il2_executed_37007436157.json`
- `verification/inverse_laplace_il3_executed_37008548408.json`
- `verification/hard_five_layer_inverse_laplace.py`
- `verification/inverse_laplace_il1_smoke.py`
- `verification/inverse_laplace_il2_pilot.py`
- `verification/inverse_laplace_il3_production.py`

and the relevant GitHub Actions runs, especially:

`37008548408`

## Independence requirement

Do not merely confirm Main Research's interpretation.

Independently reconstruct and challenge the numerical reasoning.

Where appropriate, consult primary numerical-analysis literature and authoritative method references.

Distinguish executed evidence, mathematical derivation, numerical interpretation, and unverified inference.

## Process scope

Preserve the established distinction:

- Process A = original finite-capture production WOS.
- Process B = exact-interface accelerated stochastic process.
- Process C = deterministic Markov-renewal / matrix reduction of Process B.

This review concerns the Process-C release-time recovery.

Do NOT infer A = B = C merely from successful Process-C inversion.

## Required questions

1. Is `Phi(s) = E[exp(-sT)]` being inverted correctly? In particular, independently verify the identities used for CDF F(t), survival S(t), and PDF if applicable.

2. Are Gaver-Stehfest and de Hoog sufficiently numerically distinct to provide a meaningful cross-check?

3. Are the synthetic benchmark tests sufficient to establish implementation correctness?

4. Is the complex-s continuation used by de Hoog mathematically legitimate for every component of the Process-C transform? Inspect branch choices and numerical evaluation of quantities such as `sqrt(s/D)`.

5. Independently verify the analytical initial-source factor used by the physical transform, including `Q(s) = 3[z coth(z)-1]/z^2` where applicable.

6. Examine the early-time Stehfest negative values. The largest reported negative magnitude is approximately `5.3e-87`. Determine whether the interpretation as alternating numerical cancellation around an unresolved-zero CDF is justified. Do not accept this merely because de Hoog is positive.

7. Examine the complete 81-point physical CDF. Determine whether bounds, monotonicity and long-time normalization are sufficiently demonstrated.

8. Independently assess `F(t) + S(t) - 1` closure. Reported maximum errors are approximately `1.6e-80` for Stehfest and `1.6e-74` for de Hoog.

9. Independently assess the reported maximum method-to-method CDF discrepancy of approximately `2.2e-60`. Determine what this actually establishes and what it does NOT establish.

10. Pay PARTICULAR attention to the forward-transform reconstruction diagnostic.

Reported relative reconstruction errors are approximately:

- `s=1e-9 -> 9.1e-5`
- `s=1e-8 -> 2.8e-4`
- `s=1e-7 -> 1.56e-3`
- `s=1e-6 -> 9.99e-3`
- `s=1e-5 -> 9.41e-2`

Main Research attributes the increasing high-s error to the deliberately coarse 0.1-decade recovered-CDF grid and finite `1e2-s` lower window used in the post-hoc reconstruction diagnostic.

Do NOT simply accept this explanation.

Independently determine whether:

A. this explanation is quantitatively credible;

B. the reconstruction diagnostic itself is too coarse to serve as a strong verification test;

C. a refined reconstruction-only quadrature/grid check is required;

D. the ~9.4% high-s discrepancy indicates a deeper inversion problem.

This is a particularly important review question.

11. Determine whether any additional targeted numerical experiment is required BEFORE accepting the controlled five-layer release CDF. Do not request additional computation merely for completeness. Require it only if it materially resolves a remaining uncertainty.

12. Determine the correct status of `R2-WOS-02` and `R2-B01`. Do not close either finding unless its actual closure criterion is satisfied.

## Project-direction check

Also assess whether this inverse-Laplace work remains properly scoped within the project's original Method-2 verification objective.

The project's active method structure remains:

- Method 1 — analytical diffusion;
- Method 2 — computational diffusion using FV and WOS / explicitly distinguished accelerated verification processes;
- Method 3 — semi-empirical mechanistic / reduced-order modelling.

Do not allow successful Process-C inversion to silently replace the unresolved production-WOS question.

## Required classification

Return exactly one overall classification:

A. VERIFIED — controlled five-layer release CDF accepted;

B. VERIFIED WITH NON-BLOCKING FINDINGS — release CDF accepted with stated limitations;

C. NOT YET VERIFIED — specific additional numerical evidence required;

D. BLOCKED — substantive mathematical/numerical defect.

Do not select the classification before completing the audit.

## Findings

List every finding as:

- CRITICAL
- MAJOR
- MODERATE
- MINOR
- NOTE

For every nontrivial finding provide:

- exact affected file/evidence;
- scientific reason;
- consequence;
- required remediation if any;
- closure criterion.

## Final report

Report:

1. repository tip reviewed;
2. workflow runs reviewed;
3. inversion identity verdict;
4. Stehfest assessment;
5. de Hoog assessment;
6. numerical independence assessment;
7. synthetic benchmark assessment;
8. complex-s continuation assessment;
9. source-factor assessment;
10. precision-convergence assessment;
11. early-time negative-value assessment;
12. CDF bounds assessment;
13. monotonicity assessment;
14. F+S closure assessment;
15. method-agreement assessment;
16. forward-transform reconstruction assessment;
17. whether additional targeted computation is required;
18. Process A/B/C scope assessment;
19. R2-WOS-02 disposition;
20. R2-B01 disposition;
21. Method-3 status;
22. all findings by severity;
23. final classification A/B/C/D;
24. single highest-value next action.

Persist the independent review in the repository as a new immutable review artifact.

Do NOT alter previous independent-review records.

STOP after the independent review.

Do NOT perform Main Research remediation yourself.
