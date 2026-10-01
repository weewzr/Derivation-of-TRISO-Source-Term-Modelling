# Independent Closure Audit — Hard Five-Layer 8x8 Markov-Renewal Transform Diagnostic

**Repository:** weewzr/Derivation-of-TRISO-Source-Term-Modelling
**Canonical branch:** main
**Reviewed repository tip:** 7948002dc342fa5a008e787dd7bf0d6321807c68
**Hard f64 run:** 36866849598
**Hard f64 research commit:** 5171a9ae214881db3c3a87696b9aa04af0c56dc5
**High-precision run:** 36869649217
**High-precision research commit:** fdb62bd203bf2c618eb58efe026f0a3bf239a480
**Supervisor commit used by execution:** 8d31482d127e211614ebb3f66b1076e1ed6dea98
**Artifacts:** hard-five-layer-matrix-results (11164242042); hard-five-layer-high-precision-results (11166566883)
**Scope:** independent mathematical/numerical closure audit of transform-space hard five-layer matrix only
**Inverse Laplace:** not reviewed as a release calculation and not authorized by this audit
**Supervisor repository modified:** No

# Exact gate decision

## B. VERIFIED WITH NON-BLOCKING FINDINGS — CLEARED FOR CONTROLLED RELEASE-TIME RECOVERY / INVERSE-LAPLACE VERIFICATION

The frozen 8x8 transform system passes its original normalization and residual gates under independently credible arbitrary-precision arithmetic. The high-precision 50/80/120-digit sequence is stable far beyond the original thresholds, and the 80-digit result is independently supported by NumPy float64 SVD at the same conditioning scale.

The ordinary f64 failures were real and were not waived. They are quantitatively explained by severe conditioning of I-K at s=0 and near s=0.

The hard matrix topology, stable transform formulas, probability constraints, monotonicity, quadrature convergence, and high-precision linear solve are all consistent with the frozen Process-B/C formulation.

Two non-blocking verification-record issues remain: the high-precision script's automated gate checks are stronger for Phi_init than for every individual state transform, and its reported precision-convergence table is likewise centered on Phi_init. These do not overturn the numerical result, but the distinction should be preserved before calling the diagnostic completely self-checking.

R2-WOS-02 and R2-B01 remain OPEN.

---

# 1. Reviewed evidence and execution provenance

Remote main was independently recovered at:

7948002dc342fa5a008e787dd7bf0d6321807c68.

Hard f64 diagnostic run 36866849598 completed successfully from research commit 5171a9ae214881db3c3a87696b9aa04af0c56dc5.

High-precision run 36869649217 completed successfully from research commit fdb62bd203bf2c618eb58efe026f0a3bf239a480.

The high-precision workflow installs Python 3.12.14, NumPy 2.3.3 and mpmath 1.3.0. Its execution log explicitly records supervisor commit 8d31482d127e211614ebb3f66b1076e1ed6dea98.

The preceding high-precision attempt 36869168929 failed before scientific execution because mpmath 1.3.0 did not expose mp.one. The correction replaces only that API usage with mp.mpf(1). Repository history shows no benchmark or acceptance-criterion change accompanying that fix.

The earlier f64 compilation failures 36865761217 and 36866203368 occurred before numerical execution and therefore are not scientific failures.

Verdict: execution provenance PASS.

---

# 2. Frozen benchmark

The hard f64 program prints and uses:

R = [2.125e-4, 3.125e-4, 3.525e-4, 3.875e-4, 4.275e-4] m,
D = [1.2502982636347968e-13, 1e-8, 4.062299125614697e-14, 9.227773168241615e-17, 4.062299125614697e-14] m2/s,
delta = 2e-7 m,
s = [0, 1e-9, 1e-8, 1e-7, 1e-6, 1e-5, 1e-4] 1/s.

The gate and manuscript preserve:

epsilon = 100 nm, alpha = 2, K = 1, uniform kernel-volume birth, absorbing OPyC exterior.

The high-precision Python script uses the same R, D, delta, and s lists. No physical parameter is changed between f64 and arbitrary precision.

Verdict: PASS.

---

# 3. Eight-state topology

The hard implementation has exactly the intended eight transient states:

S0 Kernel side of I0;
S1 Buffer side of I0;
S2 Buffer side of I1;
S3 IPyC side of I1;
S4 IPyC side of I2;
S5 SiC side of I2;
S6 SiC side of I3;
S7 OPyC side of I3;
A = Released.

I independently reconstructed each block of the code against the persisted transition table.

S0 contains the kernel inward/reflection and outward/transmission branches.
S1 and S2 each contain both Buffer shell exits and their corresponding interface outcomes.
S3 and S4 contain both IPyC exits and the I1/I2 interface outcomes.
S5 and S6 contain both SiC exits and the I2/I3 outcomes.
S7 contains inward OPyC-to-SiC resolution plus direct outer absorption.

No missing nonzero path, duplicate path, row/column reversal, or G+/G- reversal was found.

Each transform is multiplied by the probability associated with the material reached from the source state.

Verdict: PASS.

---

# 4. Interface probabilities

For K=1 the implementation uses:

p(i -> j) = D_j/(D_i + D_j).

The executed values are:

I0 Kernel -> Buffer = 9.9998749717368629e-1;
I0 Buffer -> Kernel = 1.2502826313727662e-5;
I1 Buffer -> IPyC = 4.0622826234075476e-6;
I1 IPyC -> Buffer = 9.9999593771737660e-1;
I2 IPyC -> SiC = 2.2664158674722207e-3;
I2 SiC -> IPyC = 9.9773358413252777e-1;
I3 SiC -> OPyC = 9.9773358413252777e-1;
I3 OPyC -> SiC = 2.2664158674722207e-3.

Independent substitution of the frozen D values reproduces these probabilities.

Reflection is 1-p and the transmission plus reflection identity holds to floating-point precision.

These values also explain the near-recurrent state chain, especially the Buffer/Kernel and SiC/OPyC directional asymmetries.

Verdict: PASS.

---

# 5. Stable transform evaluation

The f64 ratio implementation uses direct sinh for moderate arguments and the mathematically equivalent scaled form

exp(x-y) * (1-exp(-2x))/(1-exp(-2y))

for large y.

That identity follows by factoring exp(y) from sinh(y) and exp(x) from sinh(x), so it is not an ad hoc numerical approximation.

The executed diagnostic gives:

(1e-8, 2e-8) -> 0.5;
(2, 3) -> 0.362038898880996;
(1000, 1200) -> 1.3838965267367376e-87.

The last result is finite and correctly avoids overflow.

The high-precision Python implementation uses the same stable algebra.

The centered-ball large-z formula

2 z exp(-z)/(1-exp(-2z))

is likewise algebraically equivalent to z/sinh(z).

Verdict: PASS.

---

# 6. Predeclaration integrity

The high-precision gate was committed before the high-precision execution. Repository history records the predeclaration commit:

c4971c28e9c601a4f85f5fce8f2ec628abd91a91

with message:

docs(triso): predeclare hard matrix high-precision remediation.

The numerical script was subsequently added and then corrected for the mpmath API incompatibility without changing the frozen physics, s grid, precision levels, or original gates.

The original f64 gate had already fixed:

- benchmark;
- s points;
- normalization threshold 1e-9;
- residual threshold 1e-10;
- transform bounds;
- monotonicity;
- quadrature levels 1k/10k/100k;
- probability topology checks.

The high-precision design fixed:

- 50/80/120 digits;
- 80-digit primary reference;
- high-precision residual;
- high-precision SVD condition number;
- NumPy float64 SVD cross-check.

No post-result threshold relaxation was found.

Verdict: PASS.

---

# 7. Spectral radius at s=0

The high-precision run gives:

rho(K(0)) = 0.99999999994994928880658084574605124...

and:

1-rho = 5.0050711193419154e-11.

This is strictly below one and preserves the finite-state absorption argument.

The gap is extremely small, which is exactly what is expected from the extreme interface probabilities. It also explains why f64 is ill-conditioned at s=0.

The value is reproducible at 50, 80 and 120 digits to the displayed precision.

Verdict: PASS.

---

# 8. Normalization

The original f64 calculation gave:

Phi_init(0) = 0.9999998878466840,

an error of about 1.1215e-7, which genuinely fails the original 1e-9 criterion.

The high-precision calculation gives:

Phi_init(0) = 1.0.

The state transforms at s=0 are also unity at the high-precision reference.

The 50-, 80- and 120-digit calculations agree at s=0 far beyond the gate.

Therefore the f64 failure is a numerical-conditioning failure, not evidence of a probability/topology defect.

Verdict: PASS under reliable arithmetic.

---

# 9. Precision convergence

The 50/80/120-digit initial-transform convergence is extremely strong.

At s=0:

|Phi_init(50)-Phi_init(80)| = 7.41e-42.
|Phi_init(80)-Phi_init(120)| = 7.91e-72.

Across the complete declared s grid, the reported 50/80/120 Phi_init values agree to many tens of digits and the 80/120 values are effectively identical at the displayed precision.

The 80-digit reference is therefore defensible for the primary diagnostic quantity Phi_init.

One verification-record limitation remains: the automated Python script does not calculate a formal 50->80 and 80->120 difference table for every individual state Phi_i; it does so for Phi_init. The f64 state values are all within the declared bounds, and the high-precision state solves are consistent, so this is non-blocking, but it means the claim 'full state-vector precision convergence' is not independently automated.

Verdict: PASS for Phi_init; non-blocking scope qualification for individual state transforms.

---

# 10. Linear residual

Original f64 residuals include:

s=0: 1.286e-13;
s=1e-9: 1.435e-13;
s=1e-8: 1.536e-13;
s=1e-7: 1.420e-13;
s=1e-6: 5.184e-13;
s=1e-5: 1.138e-10;
s=1e-4: 4.337e-19.

The s=1e-5 residual genuinely exceeds the predeclared 1e-10 threshold.

The high-precision residuals are approximately:

s=0: 7.51e-85;
s=1e-9: 1.17e-84;
s=1e-8: 7.60e-85;
s=1e-7: 3.59e-85;
s=1e-6: 2.91e-85;
s=1e-5: 3.49e-85;
s=1e-4: 2.95e-85.

Therefore the original residual gate passes under reliable arithmetic.

Verdict: PASS.

---

# 11. Conditioning

The high-precision SVD gives at s=0:

kappa_2(I-K) = 5.2020352891e10,
sigma_min = 2.71854703581e-11,
sigma_max = 1.41419776154.

NumPy float64 SVD independently gives approximately:

kappa_2 = 5.2020486864e10,
sigma_min = 2.7185400345e-11.

The agreement is excellent relative to the scale of the condition number.

Across s the high-precision condition number decreases from roughly 5.20e10 at s=0 to 1.49e7 at 1e-4 s^-1.

This provides a quantitative mechanism for the observed f64 loss of normalization/residual accuracy.

High condition number does not invalidate the mathematical system. It means ordinary f64 is not a reliable primary arithmetic for the hardest points.

Verdict: PASS.

---

# 12. f64 versus high precision

Absolute differences in Phi_init are:

s=0: 1.1215e-7;
s=1e-9: 4.6364e-6;
s=1e-8: 3.0409e-7;
s=1e-7: 9.0800e-8;
s=1e-6: 8.2952e-11;
s=1e-5: 3.1464e-11;
s=1e-4: 6.2261e-29.

The pattern is largest where kappa_2 is largest and becomes negligible as conditioning improves.

At s=1e-9, for example, kappa_2 is about 4.98e10 and the f64/high-precision absolute difference is 4.64e-6. By s=1e-4, kappa_2 is about 1.49e7 and the difference has fallen to 6.23e-29 for the tiny transform.

This pattern is consistent with conditioning-driven f64 error.

Verdict: PASS.

---

# 13. Transform bounds

The hard f64 execution reports bounds=true at every declared s.

The printed state values are all finite and lie in [0,1] for every s, and Phi_init is likewise within bounds.

The high-precision Phi_init values are also in [0,1].

No transform-bound violation was observed.

One non-blocking audit note is that the high-precision script does not itself contain an explicit assertion over every individual Phi_i; it asserts the Phi_init bound. The state-vector values are nonetheless numerically inspected through the solve output and remain physical.

Verdict: PASS with reporting qualification.

---

# 14. Monotonicity

The hard f64 run reports monotone=true at every ordered s interval.

The high-precision Phi_init sequence decreases:

1.0 -> 0.955021769... -> 0.675651450... -> 0.147639152... -> 0.00405621033... -> 3.22458722e-7 -> 1.06756672e-19.

This is consistent with Phi_init(s)=E[e^{-sT}] for nonnegative release time.

The state transforms also show the expected non-increasing behaviour in the executed f64 diagnostic.

This is a necessary diagnostic, not a proof of complete monotonicity.

Verdict: PASS.

---

# 15. Quadrature

The f64 initial-radius midpoint quadrature uses:

Nq = 1,000, 10,000, 100,000.

The 10k->100k changes are:

s=1e-9: 2.364e-9;
s=1e-8: 1.674e-9;
s=1e-7: 3.685e-10;
s=1e-6: 1.088e-11;
s=1e-5: 1.454e-15;
s=1e-4: 2.225e-27;
s=0: 0.

These are negligible relative to the f64 matrix-solve discrepancy at the hard points.

The high-precision script uses adaptive mpmath quadrature for s>0 and the exact factor 1 for s=0. The high-precision result is therefore not contaminated by the coarse 1k/10k/100k f64 midpoint choice.

Verdict: PASS.

---

# 16. Positive-s rho output

The original f64 log displays rho=NaN for positive s.

Independent inspection of the source shows this is deliberate:

if s==0 { rho(k) } else { f64::NAN }

Therefore positive-s rho was NOT EVALUATED.

The high-precision implementation reports NOT_EVALUATED for s>0 as well.

This has no bearing on the matrix solution because positive-s spectral radius was not part of the original gate; only rho(K(0)) mattered.

Future reporting should continue to use NOT_EVALUATED rather than NaN to avoid implying numerical failure.

Verdict: PASS.

---

# 17. Process-scope distinction

The present hard 8x8 matrix is Process C:

C = deterministic matrix reduction of the exact-interface accelerated Process B.

Process A is the original finite-capture production WOS.

The current evidence does not claim:

A = B = C pathwise,

nor:

C is already proven equivalent to the ideal epsilon->0 continuum PDE.

The previous controlled two-layer reconciliation established empirical A-versus-B compatibility at the declared precision. The present hard-matrix audit concerns B/C only.

That epistemic separation is preserved in the manuscript and traceability files.

Verdict: PASS.

---

# 18. Manuscript consistency

The current Markdown and LaTeX manuscript both state that:

- the hard five-layer matrix has been constructed and diagnosed;
- ordinary f64 is affected by near-recurrent interface dynamics;
- the predeclared high-precision check passes the original numerical gates;
- the result is pending independent review;
- no final five-layer release CDF is claimed;
- R2-WOS-02 and R2-B01 remain open.

No final five-layer release CDF or inverse-Laplace result is claimed.

The traceability table likewise labels the hard high-precision result as pending independent review.

One wording point follows from this audit: after this review is persisted, the phrase 'pending independent review' will become stale and should be updated by Main Research in its next documentation pass. That is documentation synchronization, not a scientific defect.

Verdict: PASS with routine status-update requirement.

---

# 19. Findings

## CRITICAL

None.

## MAJOR

None.

## MODERATE

### R3-HM01 — Automated high-precision bounds/precision convergence are centered on Phi_init rather than the complete eight-state vector

Affected: verification/hard_five_layer_high_precision.py and the corresponding high-precision gate.

Reason: the script explicitly asserts the initial-transform normalization/bounds and reports convergence of Phi_init, but does not assert every Phi_i bound or produce a formal per-state 50->80->120 convergence table.

Consequence: the primary release observable is strongly verified, but the automated high-precision evidence is not maximally complete at the state-vector level.

Required remediation: add explicit state-vector assertions/convergence reporting in a future evidence-refresh pass if full state-vector certification is desired.

Closure criterion: automated checks over every Phi_i at every predeclared s.

Status at this gate: NON-BLOCKING.

### R3-HM02 — Hard-matrix conditioning cannot be extrapolated from controlled lower-order systems

Affected: interpretation of kappa_2 and sigma_min.

Reason: the hard 8x8 conditioning is now measured directly, but no assertion should be made that the 4x4/2x2 historical conditioning predicts it.

Consequence: none for the current result; this is a scope boundary for future inverse-Laplace work.

Required remediation: use the directly measured hard-matrix conditioning in all subsequent numerical analysis.

Closure criterion: no extrapolative conditioning claim is made.

Status: NON-BLOCKING.

## MINOR

### R3-HM03 — Positive-s spectral-radius field should be labelled NOT_EVALUATED, not NaN

Affected: historical f64 result presentation.

The implementation itself is correct; the display is potentially ambiguous.

### R3-HM04 — Manuscript status text is now stale after this independent review

Before this audit the manuscript correctly said pending independent review. After the audit, Main Research should synchronize that status.

---

# 20. Exact gate interpretation

The scientific hard-matrix diagnostic has passed the original gates under reliable arithmetic:

Normalization: PASS.
Residual: PASS.
rho(K(0))<1: PASS.
Bounds: PASS.
Monotonicity: PASS.
Quadrature: PASS.
Stable transform evaluation: PASS.
Conditioning: quantified and independently cross-checked.
50/80/120 precision: PASS for the primary Phi_init observable.
Frozen benchmark: PASS.
Topology/probability structure: PASS.

The f64 failures are retained as evidence of the need for higher precision; they are not erased from project history.

Therefore the correct gate is:

# B. VERIFIED WITH NON-BLOCKING FINDINGS — CLEARED FOR CONTROLLED RELEASE-TIME RECOVERY / INVERSE-LAPLACE VERIFICATION

This is not the same as saying the final physical release distribution has been validated.

---

# 21. R2-WOS-02

## OPEN

This audit does not establish a completed finite-capture production WOS release calculation.

# 22. R2-B01

## OPEN

No final five-layer release CDF exists yet and no transform inversion has been independently verified.

---

# 23. What is now authorized

The project may proceed to a **controlled release-time recovery / inverse-Laplace verification stage** for Process C.

The next stage must be separately predeclared.

It should not immediately be reported as a final physical five-layer release curve.

The inverse-Laplace stage must preserve:

- the frozen hard-matrix transform;
- high-precision arithmetic where conditioning requires it;
- independent numerical cross-checks;
- explicit inversion error assessment;
- comparison against any feasible independent time-domain benchmark;
- no retroactive tuning of transform points or acceptance criteria.

---

# 24. Single highest-value next action

Build and predeclare a controlled **inverse-Laplace recovery benchmark** for the hard 8x8 transform, beginning with a small set of methods rather than selecting one by convenience alone.

The first implementation should compare at least two numerically distinct inversion strategies on the same high-precision transform, with synthetic/known-transform tests and internal consistency checks before producing any physical five-layer release CDF.

The purpose is to determine whether the extremely long physical timescale associated with the SiC barrier and the sharply varying transform near s=0 can be inverted stably.

Do not change the hard-matrix benchmark or its transform values.

Do not begin Method 3.

---

# 25. Final independent conclusion

## B. VERIFIED WITH NON-BLOCKING FINDINGS — CLEARED FOR CONTROLLED RELEASE-TIME RECOVERY / INVERSE-LAPLACE VERIFICATION

The hard five-layer 8x8 transform system is mathematically and numerically credible under reliable high-precision arithmetic.

The ordinary f64 failures were real but are quantitatively explained by a condition number of about 5.2e10 at s=0 and the resulting loss of precision in ordinary double arithmetic.

The high-precision results satisfy the original acceptance criteria without changing the scientific benchmark or thresholds.

No inverse-Laplace release CDF has been validated by this audit.
R2-WOS-02 remains OPEN.
R2-B01 remains OPEN.
Method 3 remains outside scope.