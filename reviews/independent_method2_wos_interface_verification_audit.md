# Independent Review — Method-2 WOS Interface Verification

Repository: weewzr/Derivation-of-TRISO-Source-Term-Modelling
Canonical branch: main
Reviewed repository commit: 288fa151701da5e86b78d09003195cb51f2a375d
Production WOS research commit: 3dd3fcfafb79e776055c67790e5cac2c189e4fd8
Supervisor source: theodoreOnzGit/outram-park-backend
Supervisor commit used by all WOS runs: 8d31482d127e211614ebb3f66b1076e1ed6dea98
Review scope: controlled two-layer WOS <-> FV transient interface verification with capture-epsilon refinement
Production supervisor repository modified: No

## Exact gate decision

# FAIL — REMEDIATION REQUIRED

The controlled two-layer experiment is a meaningful and technically useful verification result, but the principal scientific question of this milestone was whether the finite-epsilon WOS interface algorithm demonstrates convergence toward the deterministic FV solution as epsilon -> 0.

That requirement is NOT established.

The evidence instead shows:

- zero censoring across all 10,000 production histories;
- strong direct interface-event accounting;
- deterministic FV reference error much smaller than the WOS Monte-Carlo uncertainty over the important part of the time interval;
- WOS CDFs broadly compatible with FV at the tested epsilons;
- computational cost increasing at approximately inverse proportion to epsilon;
- no monotone or statistically resolved epsilon-convergence sequence from 200 -> 100 -> 50 -> 25 nm.

This is a verification-gate failure, not evidence that the WOS interface mathematics is wrong.

---

# 1. Executed evidence inspected

Deterministic reference:
Workflow: Two-layer FV transient refinement
Run: 36693875073
Conclusion: success
Head commit: e6b0deb1b1bcb181dc29f36e257cfcff56925dc1

WOS epsilon smoke:
Run: 36694765968
Research commit: 89b27c54779e40e5ce37b8e3b4137a5a02dee05a
Conclusion: success

WOS epsilon pilot:
Run: 36723835539
Research commit: c087fe9c85c916af5742a6adfcef3b799d2f8f08
Conclusion: success

WOS epsilon production:
Run: 36726533800
Research commit: 3dd3fcfafb79e776055c67790e5cac2c189e4fd8
Conclusion: success

All four executions were inspected at run and job-log level.

The review specification requires the central question to be actual epsilon -> 0 convergence and explicitly separates workflow success from verification success.

---

# 2. Benchmark mathematical consistency

The controlled benchmark is:

a = 50 um
R = 100 um

D1 = 1e-10 m^2/s
D2 = 1e-9 m^2/s
K = 1

Initial condition:
- uniform concentration in the inner sphere;
- zero concentration in the outer shell.

Outer boundary:

c(R,t) = 0.

Internal interface:

c1(a,t) = c2(a,t)

and

D1 c1'(a,t) = D2 c2'(a,t).

No continuing source is present after t = 0.

The FV and WOS experiments use the same physical geometry, diffusivities, initial distribution, and absorbing outer release observable.

The WOS birth point uses r = a U^(1/3) with isotropic direction, which is uniform in the inner volume.

The compared observable is the cumulative release fraction:

F(t) = P(tau_release <= t).

Verdict: PASS.

No benchmark-definition mismatch was found.

---

# 3. Deterministic FV reference

The executed FV study uses 10, 20, 40 and 80 cells per layer, with the interface exactly aligned.

The finest temporal run uses:

Delta t = 2.595074794135e-5 s.

Finest spatial change:

max_t |F_80(t) - F_40(t)| = 2.76872315e-4.

Finest temporal change:

max_t |F_Dt(t) - F_Dt/2(t)| = 3.42369925e-6.

Maximum inventory residuals are about 1e-15.

These are well below the WOS Monte-Carlo uncertainty at the principal intermediate-time points. Even near the saturated late-time endpoint, the deterministic spatial refinement remains below the WOS standard error.

The FV reference is therefore sufficiently refined for the present WOS comparison.

Verdict: PASS.

---

# 4. WOS production fidelity

The production experiment directly uses the supervisor implementation primitives at commit 8d31482d:

- first-passage sampler;
- isotropic direction sampler;
- does_transmit interface rule.

The controlled adapter provides:

- two-layer geometry;
- fixed D1 and D2;
- capture epsilon;
- reinsertion;
- absorbing outer release;
- release-time ensemble.

The adapter deliberately does not call the full five-layer WoSWalker::step_multilayer control path because the experiment is isolating the two-layer interface mechanism.

For this controlled interface experiment that is acceptable.

However:

the experiment therefore verifies the tested stochastic primitives and controlled transition process, not the complete integrated five-layer production control flow.

Verdict: PASS WITH SCOPE LIMITATION.

---

# 5. Predeclaration / experimental integrity

The controlled benchmark specification was frozen in commit:

d5a7e314bbcbc5b0d082b04419303332a7e2a5b7

That commit predates the production ensemble source commit:

23c14d17387233bbcbaa1042715b4e97ecd02866

The benchmark specification already declared:

epsilon = 200, 100, 50, 25 nm
alpha = 2
N = 2500 histories per epsilon

The N = 2500 target was derived from the predeclared worst-case binomial uncertainty condition:

1.96 sqrt(0.25/N) <= 0.02,

giving N >= 2401.

There is no evidence that epsilon values or N were selected after inspecting WOS/FV agreement.

Verdict: PASS.

---

# 6. Production censoring

The production run records:

2500/2500 released

for every epsilon.

Therefore:

censor fraction = 0

at all four epsilons.

All 10,000 production histories are therefore complete release-time histories.

This resolves the censoring problem for the controlled two-layer benchmark.

It does not resolve the earlier five-layer production censoring finding.

Verdict: PASS for the controlled benchmark.

---

# 7. Monte-Carlo statistics

For every epsilon the production program reports:

- N;
- released count;
- censored count;
- F_WOS(t);
- Monte-Carlo standard error;
- Wilson 95% interval.

The standard error is correctly computed as:

SE = sqrt(F_hat (1 - F_hat) / N).

For N = 2500 the worst-case standard-error is:

SE_max = 0.01.

The corresponding 95% normal half-width is:

1.96 * 0.01 = 0.0196.

Therefore N = 2500 satisfies the predeclared uncertainty target.

The Wilson interval calculations were independently checked, including the near-F = 1 cases where the normal standard error is zero but the Wilson interval remains non-degenerate.

Verdict: PASS.

---

# 8. WOS <-> FV discrepancies

The finest deterministic reference is:

F_FV =
[0.00983747, 0.06998542, 0.23413553, 0.49412611,
 0.76267512, 0.94225401, 0.99648779, 0.999986996]

at:

t = [0.25, 0.5, 1, 2, 4, 8, 16, 32] s.

Production WOS:

| t (s) | FV | 200 nm | 100 nm | 50 nm | 25 nm |
|---:|---:|---:|---:|---:|---:|
| 0.25 | 0.00984 | 0.0140 | 0.0068 | 0.0064 | 0.0068 |
| 0.5 | 0.06999 | 0.0776 | 0.0700 | 0.0700 | 0.0708 |
| 1 | 0.23414 | 0.2444 | 0.2284 | 0.2204 | 0.2400 |
| 2 | 0.49413 | 0.4984 | 0.5008 | 0.4776 | 0.5092 |
| 4 | 0.76268 | 0.7620 | 0.7616 | 0.7580 | 0.7696 |
| 8 | 0.94225 | 0.9524 | 0.9416 | 0.9356 | 0.9464 |
| 16 | 0.99649 | 0.9964 | 0.9964 | 0.9964 | 0.9960 |
| 32 | 0.99999 | 1.0000 | 1.0000 | 1.0000 | 0.9996 |

Signed Delta F = F_WOS - F_FV:

| t (s) | 200 nm | 100 nm | 50 nm | 25 nm |
|---:|---:|---:|---:|---:|
| 0.25 | +0.00416 | -0.00304 | -0.00344 | -0.00304 |
| 0.5 | +0.00761 | +0.00001 | +0.00001 | +0.00081 |
| 1 | +0.01026 | -0.00574 | -0.01374 | +0.00586 |
| 2 | +0.00427 | +0.00667 | -0.01653 | +0.01507 |
| 4 | -0.00068 | -0.00108 | -0.00468 | +0.00692 |
| 8 | +0.01015 | -0.00065 | -0.00665 | +0.00415 |
| 16 | -0.00009 | -0.00009 | -0.00009 | -0.00049 |
| 32 | +0.00001 | +0.00001 | +0.00001 | -0.00039 |

Independently recalculated aggregate metrics:

| epsilon | max |Delta F| | RMS |Delta F| | FV inside Wilson CI |
|---:|---:|---:|---:|
| 200 nm | 0.01026 | 0.00615 | 6/8 |
| 100 nm | 0.00667 | 0.00332 | 8/8 |
| 50 nm | 0.01653 | 0.00821 | 8/8 |
| 25 nm | 0.01507 | 0.00649 | 7/8 |

These reproduce the persisted production evidence.

---

# 9. Discrepancy relative to Monte-Carlo uncertainty

Representative cases:

At 200 nm, t = 8 s:

|Delta F| = 0.01015

with:

SE ~= 0.00426

so:

|Delta F|/SE ~= 2.38.

At 50 nm, t = 2 s:

|Delta F| = 0.01653

with:

SE ~= 0.00999

so:

|Delta F|/SE ~= 1.65.

At 25 nm, t = 2 s:

|Delta F| = 0.01507

with:

SE ~= 0.01000

so:

|Delta F|/SE ~= 1.51.

These individual deviations are not sufficient to establish systematic finite-epsilon bias.

Most of the remaining differences are of the same general scale as the Monte-Carlo uncertainty.

The appropriate conclusion is broad compatibility, not exact equality.

---

# 10. Epsilon-convergence audit

This is the decisive issue.

A clear convergence sequence would require the differences to decrease systematically as:

200 -> 100 -> 50 -> 25 nm.

That behaviour is not observed.

At t = 2 s:

+0.00427, +0.00667, -0.01653, +0.01507.

At t = 4 s:

-0.00068, -0.00108, -0.00468, +0.00692.

At t = 8 s:

+0.01015, -0.00065, -0.00665, +0.00415.

The sign changes are inconsistent with a monotone approach.

The smallest epsilon is also not consistently the closest result to FV.

Therefore:

A. monotonic convergence: NO.

B. convergence hidden by Monte-Carlo noise: PLAUSIBLE.

C. approximate epsilon-independent agreement: PLAUSIBLE.

D. non-convergent systematic bias: NOT DEMONSTRATED.

E. insufficient evidence: YES.

The central requirement:

epsilon -> 0 convergence

has therefore not been demonstrated.

R2-WOS-01 remains OPEN.

---

# 11. Computational cost scaling

Production mean steps/history:

200 nm: 615.1
100 nm: 1187.3
50 nm: 2564.7
25 nm: 4875.5

The ratios for each factor-of-two reduction in epsilon are approximately:

1.93, 2.16, 1.90.

Runtime ratios are approximately:

1.88, 2.15, 1.90.

Interface encounters scale similarly.

Thus the observed computational cost is approximately:

cost ~ 1/epsilon

over the tested range.

This directly supports the earlier hypothesis that smaller capture epsilon improves geometric resolution while increasing computational effort.

It does not establish that the smaller epsilon is more accurate.

---

# 12. Interface-event accounting

The production run directly records:

- interface encounters;
- transmissions;
- reflections;
- inner -> outer events;
- outer -> inner events;
- reinsertion events;
- zero-time interface events.

For every epsilon:

zero-dt events = interface encounters.

This agrees exactly with the implementation semantics.

The direct event-classification requirement of R2-WOS-04 is therefore satisfied.

R2-WOS-04 status:

RESOLVED.

This is a diagnostics closure only; it does not validate transient multilayer transport.

---

# 13. R2-WOS-01

Status:

OPEN.

Reason:

The four-epsilon production sequence does not demonstrate a statistically resolved convergence trend toward FV.

The experiment improves the evidence substantially but does not close the requested finite-interface convergence requirement.

---

# 14. R2-WOS-02

Status:

OPEN.

Reason:

The present controlled two-layer benchmark has zero censoring, but R2-WOS-02 concerns the earlier five-layer production problem.

The previous five-layer Stage-A experiment still had:

24/24 censored histories.

This two-layer experiment does not change that fact.

---

# 15. R2-WOS-03

Status:

PARTIALLY RESOLVED.

What is now established:

finite-epsilon two-layer transient WOS results are broadly compatible with the independently refined FV reference.

What remains unestablished:

convergence of that agreement as epsilon -> 0.

Therefore full transient interface PDE equivalence remains open.

---

# 16. R2-WOS-04

Status:

RESOLVED.

The controlled production experiment directly reports interface encounters, transmissions, reflections and crossing directions.

This closes the specific diagnostic-accounting deficiency.

---

# 17. R2-B01

Status:

OPEN.

The two-layer experiment does not provide a successful five-layer production release-time ensemble and therefore cannot close the original five-layer production verification blocker.

---

# 18. Findings

## CRITICAL

None.

## MAJOR

### R3-WOS-01 — finite-epsilon convergence is not demonstrated

Affected evidence:
two-layer production sequence 200, 100, 50, 25 nm.

Reasoning:
WOS/FV agreement is broad but the epsilon sequence is non-monotonic and statistically unresolved.

Consequence:
the central finite-interface convergence claim remains unsupported.

Required remediation:
an experiment with sufficient statistical power to distinguish residual epsilon bias from Monte-Carlo noise.

Closure evidence:
a statistically resolved epsilon-dependent observable demonstrating convergence toward FV, or a robust epsilon-independent plateau with an independently justified error bound.

## MODERATE

### R3-WOS-02 — controlled adapter bypasses complete integrated step_multilayer control flow

Affected evidence:
verification/two_layer_wos_epsilon_production.rs.

Reasoning:
the adapter uses the production stochastic primitives but reconstructs the controlled two-layer transition loop rather than invoking the complete production multilayer walker.

Consequence:
strong interface-process evidence, but not complete integrated five-layer production verification.

Required remediation:
none for the present interface-isolation experiment; this limitation must remain explicit.

Closure evidence:
direct production-walker execution under controlled two-layer geometry or equivalent behavioural proof.

### R3-WOS-03 — N=2500 cannot reliably resolve a weak epsilon trend

Affected evidence:
epsilon-convergence comparison.

Reasoning:
the worst-case 95% half-width is about 0.0196, while many epsilon-to-epsilon differences are comparable to one or two standard errors.

Consequence:
a small systematic finite-epsilon bias could remain hidden.

Required remediation:
increase statistical power without changing the acceptance criterion after seeing the results.

Closure evidence:
higher-power predeclared experiment that statistically separates epsilon dependence from sampling noise.

## MINOR

### R3-WOS-04 — near-saturated late-time points are weak convergence discriminators

At 16–32 s the CDF is essentially saturated near one.

These points provide little information about epsilon convergence and should not dominate the interpretation.

---

# 19. Independently verified results

FV:
- finest spatial change = 2.7687e-4;
- finest temporal change = 3.4237e-6;
- inventory residual ~1e-15.

WOS:
- 10,000/10,000 production histories released;
- zero censoring at all four epsilons;
- all reported Wilson intervals reproduce;
- all WOS/FV discrepancy tables reproduce.

Cost:
- mean step count approximately doubles for each factor-of-two epsilon refinement.

Predeclaration:
- benchmark freeze predates the production ensemble source;
- epsilon sequence and N=2500 are predeclared.

---

# 20. Exact scientific status

The evidence establishes:

1. homogeneous/controlled WOS primitives can be exercised;
2. the two-layer WOS release process is operational with zero censoring;
3. the deterministic FV reference is sufficiently resolved;
4. the WOS CDF is broadly compatible with FV at the four tested finite epsilons;
5. interface-event accounting is direct and reproducible;
6. computational cost increases approximately as 1/epsilon.

The evidence does NOT establish:

- epsilon -> 0 convergence;
- absence of systematic finite-epsilon bias;
- full five-layer production correctness;
- closure of R2-B01;
- complete production-walker integrated verification.

---

# 21. Single highest-value next action

Repeat the SAME controlled two-layer experiment with substantially higher predeclared statistical power.

Do not change:

epsilon = 200, 100, 50, 25 nm
alpha = 2
K = 1
geometry
D1, D2
observation times

The purpose is not to seek a new agreement threshold. It is specifically to determine whether the residual epsilon dependence is real or is hidden within Monte-Carlo noise.

A reinsertion-factor sensitivity experiment should follow only if the higher-power epsilon experiment reveals a persistent epsilon effect.

Do not return immediately to the five-layer ensemble, and do not begin Method 3.

---

# 22. Final gate

# FAIL — REMEDIATION REQUIRED

The two-layer WOS production experiment is a substantial positive result:

WOS and FV are broadly compatible, all histories release, the statistical accounting is sound, and the interface-event diagnostics now work.

But the central scientific requirement for this milestone was epsilon -> 0 convergence.

That convergence is not demonstrated by the current N=2500 sequence.

Therefore the correct state is:

WOS/FV agreement: SUPPORTED

epsilon-convergence: UNRESOLVED

R2-WOS-01: OPEN

R2-WOS-02: OPEN

R2-WOS-03: PARTIALLY RESOLVED

R2-WOS-04: RESOLVED

R2-B01: OPEN
