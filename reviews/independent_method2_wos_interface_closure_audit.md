# Independent Closure Audit — High-Power Method-2 WOS Interface Remediation

Repository: weewzr/Derivation-of-TRISO-Source-Term-Modelling
Canonical branch: main
Reviewed repository commit: de81b814f6e90b4ba64de32764a0ab619e3e1ffd
High-power execution research commit: 47f1638d94c89ecb350180a063a0b11d63166ef6
Supervisor repository: theodoreOnzGit/outram-park-backend
Supervisor commit: 8d31482d127e211614ebb3f66b1076e1ed6dea98
Workflow: Two-layer WOS high-power epsilon verification
Run: 36730522197
Artifact: two-layer-wos-high-power-results (Artifact ID 11105626485)
Scope: independent closure audit of finite-epsilon Method-2 WOS interface remediation

# Exact gate decision

PASS WITH NON-BLOCKING FINDINGS

The high-power remediation successfully addresses the previous statistical-power deficiency and provides sufficient evidence to close the finite-epsilon interface finding through the second permitted closure route: an approximately epsilon-independent plateau around the independently resolved FV reference at the predeclared statistical precision.

It does not demonstrate a monotone or asymptotically resolved epsilon -> 0 convergence rate. The correct closure statement is therefore finite-epsilon plateau verified at declared statistical precision, not zero finite-epsilon bias proven.

# 1. Previous finding and closure criterion

The preceding independent audit concluded FAIL — REMEDIATION REQUIRED, with R3-WOS-01 (finite-epsilon convergence not demonstrated) as the principal unresolved finding.

The previous audit explicitly permitted closure through either statistically resolved epsilon-dependent convergence toward FV, or a robust epsilon-independent plateau compatible with FV within an independently justified error scale.

The high-power run was designed to test the second route without changing the physical benchmark.

# 2. Current repository state

The current remote main tip is de81b814f6e90b4ba64de32764a0ab619e3e1ffd.

History shows the remediation design was committed before its execution and that the latest evidence-only persistence commit is the current branch tip.

# 3. Executed high-power evidence

Run 36730522197 completed successfully.

Execution log independently confirms:

N = 10000 histories per epsilon
40000 total histories
epsilon = 200, 100, 50, 25 nm
alpha = 2
K = 1
zero censoring at every epsilon

The execution provenance explicitly records supervisor commit 8d31482d127e211614ebb3f66b1076e1ed6dea98.

Rust was rustc 1.98.1 (48a229cea 2026-09-01).

# 4. Frozen benchmark integrity

The high-power source and execution log preserve:

a = 50 um
R = 100 um
D1 = 1e-10 m2/s
D2 = 1e-9 m2/s
K = 1
alpha = 2
epsilon = 200, 100, 50, 25 nm
observation times = 0.25, 0.5, 1, 2, 4, 8, 16, 32 s

The deterministic FV reference remains the previously accepted result from run 36693875073.

Verdict: PASS.

# 5. Statistical predeclaration integrity

The predeclared N=10000 design was committed in 6cf984605bbd41cc6962c2ba2e281efc350c78d9 before the execution commit 47f1638d94c89ecb350180a063a0b11d63166ef6 and before the high-power run.

The design condition was:

1.96 sqrt(0.25/N) <= 0.01

which requires N >= 9604, so N=10000 is valid.

No evidence of post-hoc tuning of epsilon or N was found.

Verdict: PASS.

# 6. Deterministic FV reference

The accepted FV reference is the finest result from run 36693875073.

F_FV = [0.00983747, 0.06998542, 0.23413553, 0.49412611, 0.76267512, 0.94225401, 0.99648779, 0.99998700].

The previous deterministic review established finest spatial refinement change 2.7687e-4, finest temporal refinement change 3.4237e-6, and inventory residual approximately 1e-15.

These reference errors are small relative to the WOS sampling uncertainty over the informative time range.

Verdict: PASS.

# 7. Independently recalculated WOS-FV discrepancies

The independently reconstructed Delta F = F_WOS - F_FV values are:

| t (s) | 200 nm | 100 nm | 50 nm | 25 nm |
|---:|---:|---:|---:|---:|
| 0.25 | +0.000863 | -0.000937 | -0.001437 | +0.000163 |
| 0.5 | +0.003415 | -0.000786 | +0.001015 | +0.004915 |
| 1 | -0.000436 | -0.001236 | -0.002936 | +0.008164 |
| 2 | +0.000574 | +0.006374 | -0.001026 | +0.008774 |
| 4 | -0.002075 | -0.000775 | -0.002975 | +0.003425 |
| 8 | +0.001246 | +0.001646 | -0.002054 | +0.001246 |
| 16 | -0.001588 | +0.000112 | -0.000488 | -0.000088 |
| 32 | +0.000013 | +0.000013 | +0.000013 | -0.000087 |

Independent maxima of absolute Delta F:

200 nm: 0.003415
100 nm: 0.006374
50 nm: 0.002936
25 nm: 0.008774

Independent RMS values:

200 nm: 0.001632
100 nm: 0.002423
50 nm: 0.001805
25 nm: 0.004758

These reproduce the persisted high-power evidence.

# 8. Monte-Carlo uncertainty

For N=10000:

SE = sqrt(F_hat(1-F_hat)/N).

The worst-case SE is 0.005, giving a 95% normal half-width of 0.0098.

Thus the predeclared statistical precision target of 0.01 is achieved.

The Wilson intervals in the actual execution were independently checked and are correct, including near-F=1 cases.

Verdict: PASS.

# 9. MC uncertainty is not a deterministic epsilon-bias bound

This distinction remains essential.

The 0.01 target is a confidence-scale statement about Monte-Carlo sampling uncertainty. It is not a theorem that the finite-epsilon systematic bias is at most 0.01.

Therefore the scientifically valid statement is: no statistically resolved epsilon dependence is detected over 25–200 nm at the declared sampling precision.

A statement that finite-epsilon bias is mathematically bounded by 0.01 would overstate the evidence.

# 10. Monotonic epsilon convergence

Monotonic convergence is NOT demonstrated.

At t=2 s the discrepancies are +0.000574, +0.006374, -0.001026, +0.008774.
At t=4 s they are -0.002075, -0.000775, -0.002975, +0.003425.
At t=8 s they are +0.001246, +0.001646, -0.002054, +0.001246.

The sign changes and the fact that 25 nm is not consistently closest to FV rule out claiming a monotone convergence rate.

# 11. Plateau hypothesis

The plateau hypothesis is SUPPORTED.

The maximum cross-epsilon spread over the eight times is 0.0111 at t=1 s. The other informative-time spreads are 0.0057, 0.0098, 0.0064 and 0.0037 at 0.5, 2, 4 and 8 s.

The four curves remain in a narrow band around the independently resolved FV result and show no coherent signed epsilon drift.

The largest individual absolute WOS-FV discrepancy is 0.008774, below the predeclared 0.01 statistical-precision scale.

Some individual points are slightly more than two standard errors from FV, e.g. 200 nm at t=16 s. This does not establish systematic bias because the comparison contains many time points, the signs fluctuate, and the absolute deviations remain small.

Therefore the correct aggregate interpretation is a plateau at declared statistical precision, not pointwise equality or zero bias.

# 12. Residual epsilon dependence

The correct conclusion is:

no statistically detectable epsilon dependence at current power

not:

epsilon dependence is exactly zero.

The high-power experiment substantially reduces the possibility that the previous N=2500 behaviour was simply too noisy to interpret.

# 13. Computational scaling

Mean steps/history:

200 nm: 623.9
100 nm: 1207.2
50 nm: 2485.8
25 nm: 4834.0

The ratios for factor-of-two epsilon refinement are approximately 1.935, 2.059 and 1.945.

Interface encounters increase from 934102 to 1822890 to 3780360 to 7372585.

Runtime increases from approximately 0.549 s to 1.051 s to 2.163 s to 4.194 s.

The observed computational cost therefore scales approximately as 1/epsilon over the tested range.

Further brute-force epsilon reduction is unlikely to add proportional information unless a substantially smaller systematic-error target is required.

# 14. R3-WOS-01

Status: RESOLVED — plateau route.

The statistically resolved monotone-convergence route is not demonstrated, but the second permitted closure route is supported: an approximately epsilon-independent plateau around the resolved FV reference at the declared statistical precision.

# 15. R3-WOS-02

Status: OPEN.

The controlled adapter still reconstructs the two-layer loop around production stochastic primitives instead of exercising the complete integrated five-layer step_multilayer control path.

This is a scope limitation and does not invalidate the interface-isolation result.

# 16. R3-WOS-03

Status: RESOLVED.

N increased from 2500 to 10000 and the predeclared worst-case 95% half-width decreased from about 0.0196 to 0.0098.

# 17. R3-WOS-04

Status: RESOLVED.

Direct interface-event accounting is present in the high-power execution.

# 18. R2-WOS-01

Status: RESOLVED — controlled interface verification.

The finite-epsilon interface behaviour is now supported at the declared statistical precision through the plateau route.

# 19. R2-WOS-02

Status: OPEN.

The earlier finding concerns the five-layer production censoring problem. The two-layer benchmark does not close it.

# 20. R2-WOS-03

Status: RESOLVED at the controlled two-layer verification level.

The high-power run establishes broad transient WOS-FV compatibility with no statistically resolved epsilon dependence over the tested finite-epsilon interval.

This remains a controlled two-layer verification result, not a proof of full five-layer stochastic-to-PDE equivalence.

# 21. R2-WOS-04

Status: RESOLVED.

Direct interface encounters, transmissions, reflections, crossing directions, reinsertion events and zero-time events are reported.

# 22. R2-B01

Status: OPEN.

The high-power remediation does not exercise the complete five-layer release-time problem and therefore cannot close the original five-layer production blocker.

# 23. Findings

## CRITICAL
None.

## MAJOR
None.

## MODERATE

### R3-WOS-C01 — plateau precision must not be described as a deterministic finite-epsilon bias bound

Severity: MODERATE.

The 0.01 figure is an independently justified Monte-Carlo confidence/precision target, not a deterministic upper bound on systematic finite-epsilon bias.

Consequence: final project wording must preserve this epistemic distinction.

Closure evidence: final documentation states that the plateau is supported at declared statistical precision and does not claim zero or deterministically bounded finite-epsilon bias.

### R3-WOS-C02 — integrated production scope remains limited

Severity: MODERATE.

The controlled adapter validates the production stochastic primitives in a two-layer experiment but does not independently execute the complete integrated five-layer production control path.

Consequence: the interface milestone is closed, but integrated five-layer validation remains separate.

Closure evidence: direct controlled invocation of the integrated multilayer production path in a future implementation audit.

## MINOR

### R3-WOS-C03 — no monotone epsilon convergence rate is available

Severity: MINOR.

The epsilon sequence is non-monotone. This is not a failure of the plateau closure route, but the project must not report an empirical convergence order in epsilon.

# 24. What the remediation establishes

The high-power evidence establishes:

1. zero-censoring two-layer WOS release ensembles at all four finite epsilons;
2. adequate predeclared Monte-Carlo power for a 0.01 worst-case 95% precision target;
3. broad WOS-FV transient compatibility;
4. no statistically resolved epsilon dependence over 25–200 nm;
5. approximately inverse-epsilon computational scaling.

The remediation therefore closes the specific statistical-power problem and supports the finite-epsilon plateau interpretation.

# 25. What remains open

It does not establish:

- mathematically exact epsilon -> 0 convergence;
- zero finite-epsilon systematic bias;
- complete five-layer production correctness;
- five-layer computational practicality;
- closure of R2-WOS-02;
- closure of R2-B01.

# 26. Single highest-value next action

Return to the actual five-layer production verification problem, using the now-supported two-layer interface behaviour as a frozen verification input.

Do not spend the next stage on brute-force smaller epsilon values unless a stricter asymptotic error requirement is scientifically required.

The key unresolved Method-2 question is now whether the validated controlled interface process can operate successfully and produce a usable release-time ensemble in the actual five-layer TRISO geometry.

# 27. Final gate

# PASS WITH NON-BLOCKING FINDINGS

The high-power remediation satisfies the permitted epsilon-independent plateau closure route for the controlled two-layer Method-2 interface benchmark.

The correct scientific conclusion is:

WOS/FV compatibility and epsilon-independent behaviour are supported at the declared statistical precision over 25–200 nm.

It is not:

asymptotic epsilon -> 0 convergence proven.

The controlled two-layer interface verification milestone can therefore close, while the integrated five-layer production problem and R2-B01 remain open.