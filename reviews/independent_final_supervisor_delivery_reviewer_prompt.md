# Independent Reviewer Prompt — Final Supervisor Delivery Verification

## Role

Act as the **independent final mathematical, numerical, reproducibility, and delivery auditor** for `weewzr/Derivation-of-TRISO-Source-Term-Modelling`.

Canonical branch: **main only**.

This prompt is stored before the final delivery workflows have necessarily settled. **Do not begin substantive review until you have recovered current remote state and all relevant Final supervisor delivery verification runs have completed or their canonical status is unambiguous.**

Do not trust conversational summaries, commit messages, workflow names, or green/red badges by themselves. Inspect the repository, workflow logs, artifacts, derivations, code, and accepted independent reviews. Do not modify the supervisor repository.

## 1. Recover the actual current state first

Before reviewing:

1. fetch current remote `main` and record its SHA;
2. inspect recent commits;
3. inspect all recent workflows named **Final supervisor delivery verification**;
4. determine which run/commit is the canonical final-delivery candidate;
5. distinguish superseded runs, CI/compiler-only failures, genuine numerical failures, and successful runs;
6. inspect actual logs and artifacts.

At prompt preparation, multiple overlapping final-delivery runs existed and at least one run exposed an FV verification failure at the finest spatial test. Treat that only as historical context. Re-establish current facts yourself.

If workflows are still actively changing the canonical candidate, **STOP and report that independent review is premature**.

## 2. Review the full evidence chain

Read the current controlling derivations/evidence for:

- continuous and discrete/FV mathematical foundations;
- equation register / derivation ledger / status;
- shell first-passage derivation/evidence;
- interface-state Markov-renewal derivation;
- hard five-layer matrix and high-precision evidence;
- IL-1, IL-2, IL-3 inverse-Laplace evidence;
- accepted Process-C machine-readable release CDF;
- final supervisor deliverables and verification outputs.

Read relevant independent reviews, including controlled WOS/interface closure, accelerated first-passage, interface-state Markov-renewal and closure, controlled multistate matrix, hard five-layer matrix closure, and inverse-Laplace release-recovery audit/resolution.

## 3. Preserve Process A / B / C scope

Audit all final-facing documents for epistemic correctness:

- **A:** finite-capture production WOS;
- **B:** accelerated exact-interface renewal;
- **C:** deterministic Markov-renewal matrix reduction of B.

Do not permit language implying `A=B=C` pathwise without new evidence. The accepted hard five-layer release CDF is a **Process-C** result at its reviewed verification level. Do not promote finite-epsilon evidence to proof of the ideal epsilon->0 continuum limit.

## 4. Audit the mathematical derivation

Independently check:

conservation -> Fick -> spherical multilayer PDE -> boundary/interface conditions -> conservative FV -> first passage -> shell joint transforms -> interface renewal -> finite-state Markov-renewal -> hard 8x8 transform -> inverse Laplace.

Audit:
[
\Phi(s)=[I-K(s)]^{-1}B(s),
]
[
\mathcal L\{F\}(s)=\Phi(s)/s,
\qquad
\mathcal L\{S\}(s)=[1-\Phi(s)]/s.
]

Check dimensions, signs, boundary conditions, direction conventions, normalization and physical-time preservation.

## 5. Mandatory FV final-delivery audit

Inspect current canonical `triso-rust` FV implementation and final verification harness.

Verify:

- homogeneous analytical comparison;
- controlled two-layer benchmark;
- spatial/temporal refinement;
- positivity;
- conservation/inventory closure;
- interface conductance;
- timestep/stability;
- outer boundary.

### Historical failure requiring reconciliation

A recent final-delivery run reported at 800 cells approximately:
[
F=1.000000001874,
\qquad
\text{balance\_rel}\approx1.874\times10^{-9},
]
while the active assertion required:
[
\text{balance\_rel}<10^{-9}.
]

Do not waive this automatically. Determine whether it was solver error, floating-point accumulation, an inappropriate metric/tolerance, a harness defect, or already remediated. If tolerance changed, audit scientific justification and reject post-hoc relaxation merely to turn CI green. Confirm current CI propagates numerical failures.

## 6. Rust supervisor usability

From clean checkout check:

- cargo check;
- cargo test;
- release build;
- documented invocation;
- configuration/input handling;
- numerical output/error handling;
- promised source-term/literature placeholders;
- equation-to-code correspondence;
- absence of hidden research-harness dependencies.

Determine whether a supervisor can actually use the delivered code from repository instructions.

## 7. Hard five-layer transform

Verify frozen parameters, interface probabilities, 8-state topology, rho(K(0))<1, normalization, bounds/monotonicity, source factor/quadrature, stable hyperbolic evaluation, conditioning, residuals, 50/80/120-digit convergence, and the documented distinction between f64 limitations and high-precision results.

## 8. Accepted inverse-Laplace / release CDF

Inspect production run `37008548408` and artifact if still canonical, or identify its replacement.

Verify:

- full 81-point grid/provenance;
- Stehfest vs de Hoog;
- precision checks;
- bounds/monotonicity;
- CDF/survival closure;
- complex-s continuation;
- analytical source factor;
- tiny early Stehfest negatives;
- forward-transform reconstruction limitation.

Confirm final deliverables preserve the independent review's limitations.

## 9. Supervisor-facing documents

Audit every intended final delivery file. Check equation correctness/parity, symbols, assumptions, units, pseudocode/code correspondence, user placeholders, current verification status, removal of stale "pending review" wording, open findings, Process-C limitations, and absence of Method-3 claims. If Markdown and LaTeX both exist, check mathematical parity.

## 10. Traceability/reproducibility

For each major final claim verify a trace:
claim -> derivation -> implementation -> verification -> workflow/run -> artifact/evidence -> independent review.

Check machine-readable versus prose evidence and commit/run/artifact provenance.

## 11. Open blockers

Explicitly reassess:

- **R2-WOS-02:** do not close merely because Process C succeeds.
- **R2-B01:** read its current definition and determine OPEN / PARTIALLY RESOLVED / CLOSED from current evidence.
- **R4-m01 and later findings:** audit current status rather than inheriting old labels.

## 12. Audit final CI itself

Inspect **Final supervisor delivery verification**. Verify it checks intended files, builds/tests the intended Rust project, executes numerical verification, propagates failures, contains no scientific-check masking such as `|| true`, regenerates rather than merely trusting stale outputs, and is reproducible from clean checkout.

A green workflow is evidence only if the workflow itself is meaningful.

## 13. Findings

Classify new findings as CRITICAL, MAJOR, MODERATE, MINOR, NOTE.

For each substantive finding provide identifier, affected file/code/workflow, evidence, reasoning, consequence, required remediation and closure criterion. Do not manufacture findings.

## 14. Final gate

Choose exactly one:

**A. FINAL SUPERVISOR DELIVERY ACCEPTED**

**B. FINAL SUPERVISOR DELIVERY ACCEPTED WITH NON-BLOCKING FINDINGS**

**C. REMEDIATION REQUIRED BEFORE SUPERVISOR DELIVERY**

**D. REJECTED**

Do not choose A/B merely because latest CI is green.

## 15. Persistence

Persist the audit under `reviews/` on `main`, preferably:

`reviews/independent_final_supervisor_delivery_audit.md`

Commit/push **only independent review evidence**. Do not modify derivations, Rust source, verification harnesses, workflows, deliverables or prior reviews. If remediation is required, leave it to Main Research.

## 16. Final report

Report:

1. reviewed main SHA;
2. canonical final-delivery run;
3. superseded/failing runs inspected;
4. final CI verdict;
5. mathematical derivation verdict;
6. FV verification verdict;
7. Rust implementation verdict;
8. hard-matrix verdict;
9. inverse-Laplace/CDF verdict;
10. supervisor-document verdict;
11. traceability/reproducibility verdict;
12. R2-WOS-02 status;
13. R2-B01 status;
14. other finding statuses;
15. finding counts by severity;
16. exact final gate;
17. single highest-value next action.

**STOP after persisting the independent audit.**
