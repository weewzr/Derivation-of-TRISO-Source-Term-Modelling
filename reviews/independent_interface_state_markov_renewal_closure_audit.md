# Independent Closure Audit — Finite-Capture / Exact-Interface / Matrix-Renewal Remediation

Repository: weewzr/Derivation-of-TRISO-Source-Term-Modelling
Canonical branch: main
Reviewed repository commit: 9c3cc785d4032787b8cab87eeae2105c1de6c2cd
Supervisor commit: 8d31482d127e211614ebb3f66b1076e1ed6dea98
Run: 36807696742
Workflow: Two-layer finite-capture matrix reconciliation
Research commit used by run: 95ff1e99e6aa9df1567ba30a3a8072f770266d0a
Artifact: two-layer-matrix-reconciliation-results
Artifact ID: 11137889673
Scope: independent closure audit of the controlled A/B/C reconciliation remediation
Supervisor repository modified: No

# Exact gate decision

# B. REMEDIATION SUBSTANTIALLY CLOSED WITH NON-BLOCKING FINDINGS — CLEARED FOR CONTROLLED MULTISTATE MATRIX IMPLEMENTATION/VERIFICATION

The remediation successfully closes the central prior process-definition objection at the controlled two-layer level.

The key distinction is now correct:

A = finite-capture production WOS;
B = explicit accelerated exact-interface renewal;
C = deterministic matrix reduction of B.

C is mathematically equivalent to B under the stated exact-interface process definition. A is not claimed pathwise identical to B/C. A versus B is treated as an empirical compatibility question.

The controlled B-versus-C numerical reconciliation, normalization, residual, and conditioning checks all pass their predeclared criteria. The A-versus-B CDF difference is within the previously accepted controlled-benchmark precision scale, but it remains empirical compatibility rather than an identity theorem.

R2-B01 remains OPEN.
The hard five-layer Cs-137 matrix calculation is NOT authorized by this audit. Only controlled multistate matrix implementation/verification is cleared.

# 1. Process-definition audit

## A — direct finite-capture production WOS

Ordinary production WOS stops a region excursion when the current nearest interface is within epsilon_capture. The interface decision is resolved there with zero event time and the walker is reinserted alpha*epsilon on the chosen side.

Thus the interface-event location is generally inside the material, not at the exact interface radius.

## B — explicit accelerated exact-interface renewal

The accelerated shell/ball kernel propagates to the physical interface radius, accumulates the corresponding first-passage time, applies the same production transmission/reflection probability with zero interface-event time, and reinserts at alpha*epsilon on the selected side.

## C — matrix reduction

The matrix applies first-step decomposition to B and sums all repeated transient renewals in transform space.

The current documents explicitly state that A != B != C as a pathwise identity. Instead:

C = B is the exact mathematical target;
A versus B is empirical compatibility.

**Verdict: PASS.**

# 2. Finite-capture difference

For a=50 um and epsilon=0.1 um, uniform-volume initial mass in the capture shell is:

P(a-epsilon <= r <= a) = 1 - (1-epsilon/a)^3.

With epsilon/a = 0.002:

P = 1 - 0.998^3 = 0.005988008.

So the affected initial mass is:

0.5988008%.

This independently confirms the repository value.

The interpretation is correct: a production finite-capture history born in this region can resolve the interface without first reaching the exact physical interface, while B/C assign the exact-interface first-passage time.

The documents correctly retain this as a real semantic distinction rather than setting it to zero by assumption.

**Verdict: PASS.**

# 3. Matrix structure

The controlled 2x2 process uses:

S0 = inner-side reinsertion state at a-alpha*epsilon;
S1 = outer-side reinsertion state at a+alpha*epsilon;
A = absorbing release at R.

Let H(s) be the inner centered-ball first-exit transform, and G^-(s), G^+(s) the outer-shell joint exit transforms from the outer-side reinsertion radius.

The equations are:

Phi0 = H [p00 Phi0 + p01 Phi1]
Phi1 = G^- [p10 Phi0 + p11 Phi1] + G^+.

Thus:

K = [[H p00, H p01], [G^- p10, G^- p11]],
B = [0, G^+]^T.

The implementation uses p00=D1/(D1+D2), p01=D2/(D1+D2), p10=D1/(D1+D2), p11=D2/(D1+D2), which is correct for the stated K=1 transmission law in both directions.

The initial transform correctly weights the initial radius by 3r^2/a^3 and applies the centered-ball transform before the interface outcome.

Every K entry is dimensionless; B and Phi are dimensionless Laplace transforms.

**Verdict: PASS.**

# 4. Predeclaration integrity

The finite-capture reconciliation gate was committed at:

8b10fb5b5853ba3b2c40b3c836822b7ed108afb6

message: docs(triso): predeclare finite-capture reconciliation gate

The matrix reconciliation implementation was added later at:

3c800881c999dc3cf32479a16f750c75936ed12c

and the workflow execution used:

95ff1e99e6aa9df1567ba30a3a8072f770266d0a.

Therefore the benchmark, s-points, normalization criterion, residual criterion, conditioning diagnostics, and comparison metrics were specified before results.

The declared transform points were:

s = [0, 0.25, 0.5, 1, 2, 4] s^-1.

N = 20,000 for explicit Process B.

The acceptance criteria were not changed after observing the outputs.

**Verdict: PASS.**

# 5. B versus C transform reconciliation

The execution log independently reports:

| s | C Phi_init | B mean | B SE | B-C | z | cond2(I-K) | residual |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1.000000000000 | 1.000000000000 | 0 | ~1.45e-14 | 0 | 230.63 | 0 |
| 0.25 | 0.565271047887 | 0.565176155688 | 0.00172154 | -9.49e-5 | -0.055 | 195.04 | 1.12e-16 |
| 0.5 | 0.378686243232 | 0.378694937890 | 0.00182016 | +8.69e-6 | +0.0048 | 173.39 | 3.47e-18 |
| 1 | 0.209451869140 | 0.209666100098 | 0.00153950 | +2.14e-4 | +0.139 | 146.08 | 3.47e-18 |
| 2 | 0.090790236649 | 0.091358859363 | 0.00102223 | +5.69e-4 | +0.556 | 116.44 | 2.79e-17 |
| 4 | 0.028729861774 | 0.029244528324 | 0.00051285 | +5.15e-4 | +1.004 | 88.75 | 4.34e-19 |

The maximum standardized discrepancy is:

|z|max = 1.004.

Every other |z| is <= 0.556.

These discrepancies are fully compatible with the explicit-renewal Monte-Carlo uncertainty.

This is strong evidence that C correctly implements B.

It does not prove that the numerical floating-point matrix solve is exact; it demonstrates consistency with an independently simulated B at the declared precision.

**Verdict: PASS.**

# 6. Normalization

The predeclared criterion is:

|Phi_init(0)-1| < 1e-10.

The execution gives:

Phi0(0)=1,
Phi1(0)=1,
Phi_init(0)=1

to floating-point precision.

Independent reasoning also confirms that at s=0 all exit probabilities sum to one and all interface probabilities sum to one.

**Verdict: PASS.**

# 7. Spectral-radius / eventual-absorption argument

At s=0, D1,D2>0 implies all relevant interface transmission probabilities are strictly positive.

S0 has a positive-probability route to S1.

From S1 there is positive probability G^+(0)>0 of reaching the absorbing outer boundary.

Thus every transient state has a finite path of positive probability to absorption.

For a finite-state substochastic transition matrix this excludes a closed nonabsorbing communicating class. Equivalently, some finite power K^m has row sums strictly below one for every state, implying:

rho(K(0)) < 1.

Hence:

Phi(0) = (I-K(0))^-1 B(0)

is finite and the eventual release probability is one.

The numerical Phi(0)=1 test provides independent computational confirmation.

**Verdict: PASS.**

# 8. Linear-system residual

The predeclared criterion is:

relative residual < 1e-10.

Actual residuals are between 0 and approximately:

1.12e-16.

The worst observed value is about eight orders of magnitude smaller than the criterion.

**Verdict: PASS.**

# 9. Conditioning

Reported 2-norm condition numbers are:

s=0: 230.63
s=0.25: 195.04
s=0.5: 173.39
s=1: 146.08
s=2: 116.44
s=4: 88.75

No post-hoc acceptance threshold was declared.

These values are moderate for a 2x2 system and are consistent with the very small linear residuals.

They do not establish conditioning of the eventual 8x8 five-layer system, where the TRISO diffusivity contrasts are much stronger.

Therefore there is no controlled-matrix blocker here.

**Verdict: PASS for the controlled benchmark.**

# 10. A versus B controlled compatibility

Previously executed N=10,000 CDFs at epsilon=100 nm were:

A = [0.0089, 0.0692, 0.2329, 0.5005, 0.7619, 0.9439, 0.9966, 1.0].
B = [0.0091, 0.0752, 0.2390, 0.4971, 0.7621, 0.9447, 0.9958, 1.0].

B-A = [+0.0002,+0.0060,+0.0061,-0.0034,+0.0002,+0.0008,-0.0008,0].

RMS = 0.003281.
Maximum absolute discrepancy = 0.0061.

The largest difference is below the previously accepted controlled-benchmark precision scale of approximately 0.01.

The sign changes across time and do not show a coherent one-sided bias.

However, these A/B data come from independent Monte-Carlo experiments. They therefore demonstrate controlled statistical compatibility, not a pathwise identity or a rigorous deterministic error bound.

**Verdict: PASS for the declared compatibility target.**

# 11. Physical-time semantics

The remediation correctly acknowledges:

A stops when entering the finite capture region.
B/C stop at the exact interface.

Therefore A and B/C need not have identical interface-event times.

The initial capture-shell mass is 0.5988%, and the net A/B release-CDF discrepancy has been explicitly quantified.

This is sufficient for the stated empirical compatibility target. It is not a proof that the time difference is pointwise negligible for arbitrary parameters.

**Verdict: PASS WITH SCOPE LIMITATION.**

# 12. Three-level epistemic distinction

The repository now cleanly separates:

LEVEL 1: exact mathematical equivalence B=C;
LEVEL 2: controlled empirical compatibility A versus B/C;
LEVEL 3: independent continuum epsilon->0 relevance.

None of the evidence at one level is promoted to another.

R3-M02 is therefore resolved.

**Verdict: RESOLVED.**

# 13. Toy model

The toy document is explicitly labelled:

ALGEBRAIC SANITY CHECK ONLY — NOT STOCHASTIC VALIDATION.

That is the correct epistemic classification.

R3-m01 is resolved.

# 14. Eight-state transition table

The eight-state table was independently checked for topology and transform-direction consistency.

Every material layer has the two possible bounding exits; each internal-interface outcome maps to the correct post-interface state; OPyC S7 has one inward-interface renewal branch and one direct absorbing outer branch.

The table correctly distinguishes I0, I1, I2 and I3 and uses G^- for the inner shell boundary and G^+ for the outer shell boundary.

R3-m02 is resolved.

---

# 15. Finding status

### R3-M01 — RESOLVED AT CONTROLLED TWO-LAYER COMPATIBILITY LEVEL

The exact A=B identity was removed. The finite-capture difference is quantified and its net release-CDF effect is shown to be within the declared controlled precision scale.

It remains empirical, not theorem-level.

### R3-M02 — RESOLVED

The three-level distinction between exact B=C, empirical A~B/C compatibility, and continuum epsilon->0 relevance is now explicit.

### R3-M03 — RESOLVED

The finite-state absorption argument and numerical Phi(0)=1 evidence establish the controlled-chain normalization.

### R3-M04 — RESOLVED FOR CONTROLLED MATRIX

Conditioning and residuals were explicitly measured at all predeclared transform points.

Hard five-layer conditioning remains a future controlled implementation/verification issue.

### R3-m01 — RESOLVED

Toy model is explicitly algebraic sanity evidence only.

### R3-m02 — RESOLVED

The eight-state transition table is present and internally consistent.

---

# 16. R2-WOS-02

## OPEN

R2-WOS-02 concerns the five-layer direct-production censoring problem. The present two-layer matrix reconciliation cannot close it.

# 17. R2-B01

## OPEN

The present controlled two-layer reconciliation cannot close the five-layer release calculation blocker.

# 18. Implementation authorization

# B. REMEDIATION SUBSTANTIALLY CLOSED WITH NON-BLOCKING FINDINGS — CLEARED FOR CONTROLLED MULTISTATE MATRIX IMPLEMENTATION/VERIFICATION

Only the next controlled multistate implementation/verification stage is authorized.

The hard five-layer Cs-137 production release calculation is not authorized by this audit.

# 19. Remaining non-blocking limitations

1. The exact finite-capture production process A is not mathematically identical to the exact-interface process B. This is intentionally retained as an empirical compatibility distinction.

2. Controlled 2x2 conditioning does not predict conditioning of the eventual 8x8 five-layer system.

3. The matrix has not yet been validated against a genuine multistate (>2-state) numerical renewal simulation.

4. R2-WOS-02 and R2-B01 remain open.

# 20. Single highest-value next action

Run a **controlled three-layer / reduced-contrast multistate matrix verification** before attempting the hard five-layer Cs-137 case.

The purpose is to test genuine multistate composition:

shell kernel -> interface state -> second shell/interface -> matrix reduction

using a parameter regime where explicit accelerated renewal remains computationally feasible.

Compare:

- explicit multistate accelerated renewal;
- deterministic matrix transform;
- deterministic FV reference where available;
- normalization;
- residual;
- selected release-time moments/CDF points.

This provides the missing validation step that a 2x2 matrix cannot supply before the 8x8 five-layer matrix is trusted.

# 21. Final closure decision

# B. REMEDIATION SUBSTANTIALLY CLOSED WITH NON-BLOCKING FINDINGS — CLEARED FOR CONTROLLED MULTISTATE MATRIX IMPLEMENTATION/VERIFICATION

The remediation is closed sufficiently for the next controlled implementation stage.

The result does not close the five-layer production problem and does not authorize the hard five-layer Cs-137 release calculation.