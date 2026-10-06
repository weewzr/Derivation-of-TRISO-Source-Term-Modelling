# Independent Review — Controlled Process-C Inverse-Laplace / Release-Time Recovery

**Repository:** `weewzr/Derivation-of-TRISO-Source-Term-Modelling`
**Canonical branch:** `main`
**Reviewed repository tip:** `7948002dc342fa5a008e787dd7bf0d6321807c68`
**Primary production workflow:** `37008548408`
**Primary production commit:** `9668f76c6339871e6fc60d915242fcaa44ae3fef`
**Artifact:** `inverse-laplace-il3-production-results` (11226802949)
**Hard-matrix prerequisite review:** `reviews/independent_hard_five_layer_matrix_closure_audit.md`
**Scope:** independent controlled Process-C inverse-Laplace / release-time recovery audit
**Process A/B/C distinction preserved:** yes
**Supervisor repository modified:** No

# Overall classification

## B. VERIFIED WITH NON-BLOCKING FINDINGS — RELEASE CDF ACCEPTED WITH STATED LIMITATIONS

The controlled Process-C five-layer release-time recovery is accepted at the current verification level.

The acceptance is based on the independently reviewed high-precision 8x8 transform, two numerically distinct inversion methods, 81-point physical-grid agreement, precision checks, source-factor validation, complex-s continuation checks, bounds/monotonicity evidence, and CDF/survival closure.

The acceptance does NOT mean:

- Process A finite-capture production WOS has been validated against this result;
- Process A = Process B = Process C pathwise;
- epsilon -> 0 continuum convergence has been proven by this inversion;
- the forward-reconstruction diagnostic has negligible discretization error at all s;
- R2-WOS-02 or R2-B01 is closed;
- Method 2 is complete in the broader production-integration sense.

---

# 1. Evidence inspected

Primary repository request and prerequisite evidence were inspected, including:

- `reviews/independent_inverse_laplace_release_recovery_request.md`;
- `reviews/independent_hard_five_layer_matrix_closure_audit.md`;
- `verification/inverse_laplace_method_selection.md`;
- `verification/inverse_laplace_runtime_cost_model.md`;
- IL-1, IL-2 and IL-3 plans/evidence;
- `verification/hard_five_layer_inverse_laplace.py`;
- executed JSON evidence for IL-1, IL-2 and IL-3;
- full IL-3 GitHub Actions execution `37008548408` and artifact `11226802949`.

The IL-3 workflow completed all 81-point production inversions and uploaded the result artifact successfully.

The synthetic production workflow `36991554900` timed out before the physical step and therefore did not provide physical CDF evidence. This is correctly treated as an execution-cost event, not a scientific failure.

---

# 2. Inversion identities

Let

`Phi(s) = E[exp(-sT)]`

for nonnegative release time T with eventual release probability one.

Then:

`L{F}(s) = Phi(s)/s`,

and:

`L{S}(s) = (1-Phi(s))/s`

for the CDF F(t)=P(T<=t) and survival S(t)=P(T>t).

Independent derivation:

`integral_0^infinity exp(-st) F(t) dt = E[integral_T^infinity exp(-st) dt] = E[exp(-sT)]/s`

for s>0.

The survival identity follows from F+S=1 and L{1}=1/s.

The identities used by the implementation are therefore correct.

**Verdict: PASS.**

---

# 3. Stehfest and de Hoog independence

Stehfest uses positive real transform evaluations of the Gaver-Stehfest type. de Hoog uses complex-s evaluations on a Bromwich/Fourier representation followed by nonlinear acceleration.

They therefore exercise materially different numerical mechanisms:

- real-axis only versus complex continuation;
- alternating real coefficients versus Fourier/continued-fraction acceleration.

Both methods nevertheless share the exact same Process-C transform evaluator. Thus they are not fully independent implementations of the physical model; they are independent inversion routes applied to the same transform.

The distinction is scientifically sufficient for a controlled inversion cross-check.

**Verdict: meaningful numerical independence, not model independence.**

---

# 4. Synthetic benchmark evidence

The synthetic artifact `36991554900` contains exponential, two-exponential mixture, long-timescale exponential and centered-ball first-passage benchmarks at 50/80/120 digits.

Maximum errors are extremely small. For example, the centered-ball Stehfest error falls from about `4.74e-34` at 50 digits to `2.77e-80` at 120 digits; de Hoog is comparably accurate.

These are meaningful inversion-unit tests because the exact transforms and time-domain references are known independently.

However, synthetic agreement does not prove correctness for the hard TRISO transform itself because the physical transform has severe near-zero conditioning and a nontrivial matrix structure.

The synthetic tests establish implementation competence, not physical-case correctness by themselves.

**Verdict: PASS with appropriate scope.**

---

# 5. Complex-s continuation

The physical transform uses:

`lambda = sqrt(s/D)`

and hyperbolic ratios.

For the de Hoog contour in the right half-plane, the principal square root is analytic away from its standard branch cut on the negative real axis. For Re(s)>0, the principal square root has positive real part, which is consistent with the Laplace-domain solution.

The implemented complex ratio switches to the algebraically equivalent scaled formula when Re(y) is large:

`exp(x-y) (1-exp(-2x))/(1-exp(-2y))`.

This is algebraically the same as sinh(x)/sinh(y) and avoids overflow.

IL-1 explicitly checked:

- positive-real agreement with the frozen reviewed transform at 1e-9, 1e-7 and 1e-5 s^-1;
- conjugacy `Phi(conj(s)) = conj(Phi(s))` at three complex points;
- principal sqrt usage.

Reported real-axis discrepancies are about `3.06e-16` or smaller and conjugacy discrepancy is displayed as zero.

This is sufficient evidence that the complex continuation used by de Hoog is mathematically and numerically consistent over the tested points.

It is not a proof of analyticity over the entire contour for arbitrary future parameters.

**Verdict: PASS for the controlled inversion.**

---

# 6. Analytical source factor

For uniform volume birth in the kernel:

`Q(s) = integral_0^R 3 r^2/R^3 * H(r,s) dr`.

With `z = R sqrt(s/D)`, the analytical evaluation is:

`Q(s) = 3 [z coth(z) - 1] / z^2`,

with `Q(0)=1`.

Independent differentiation/integration gives this expression.

IL-1 directly compared the closed form to historical adaptive quadrature at real and complex s. Maximum reported absolute differences are about `1e-78` to `3e-81`.

Thus the source factor is not merely plausible; it is strongly cross-checked against an independent quadrature path.

**Verdict: PASS.**

---

# 7. IL-1 and IL-2 early-time Stehfest negatives

IL-1 at 1e4 s produced approximately `-7.37e-73` at 50 digits.

IL-2 then showed:

- 50 digits: `-7.37e-73`;
- 80 digits: `+4.49e-99`;
- 120 digits: `-1.05e-130`.

de Hoog gave positive values:

- 50 digits: about `9.08e-107`;
- 80 digits: about `9.08e-147`;
- 120 digits: about `2.62e-182`.

At 1e2 s the values are even smaller.

The sign alternates while the magnitude collapses rapidly with precision. This is exactly the pattern expected from cancellation in an inversion of a CDF that is far below the meaningful physical scale.

The largest negative in IL-3 is:

`5.2882749324e-87`.

That magnitude is negligible relative to any physical release-fraction precision represented by the project.

No clipping is applied in machine-readable output, which is correct.

Human-readable interpretation as 'numerically unresolved from zero / effectively zero' is justified.

**Verdict: PASS.**

---

# 8. Complete 81-point CDF: bounds and monotonicity

The IL-3 artifact contains 81 log-spaced times from 1e2 to 1e10 s.

de Hoog:

- all CDF values in [0,1];
- monotone nondecreasing over the entire grid.

Stehfest:

- strict bounds/monotonicity fail only because of the 10 tiny negative early-tail values;
- the largest negative is 5.29e-87;
- resolved values from the physically relevant release region onward are monotone and positive.

Representative de Hoog values include:

1e4 s: 9.08e-147;
1e5 s: 2.04e-21;
1e6 s: 5.849128692e-4;
1e7 s: 0.1594036656;
1e8 s: 0.8872792421;
1e9 s: 0.999999999788;
1e10 s: 1.

This is consistent with a nonnegative release-time distribution and the independently established long-time absorption probability.

**Verdict: PASS with the explicit early-tail Stehfest qualification.**

---

# 9. CDF/survival closure

The production run independently inverts both:

`Phi(s)/s`

and:

`[1-Phi(s)]/s`.

Maximum absolute closure error:

Stehfest: `1.5813296915e-80`.
de Hoog: `1.5955319297e-74`.

These are far below the 80-digit working precision and demonstrate that the two independently inverted representations agree with the identity F+S=1 to the numerical precision of the calculation.

**Verdict: PASS.**

---

# 10. Method-to-method agreement

The maximum Stehfest/de Hoog CDF difference over all 81 points is:

`2.2e-60`

at:

`125892.5411794166 s`.

From the physically resolved region onward, the two methods agree to the stored precision.

This is very strong numerical cross-validation.

However, method-to-method agreement does not prove truth: two numerical methods can, in principle, share a common transform-evaluation error.

Here that concern is reduced by the independently audited transform mathematics, source-factor check, hard-matrix high-precision closure, and complex-s gate.

Therefore the evidence is strong enough for the present controlled release-CDF acceptance.

**Verdict: PASS.**

---

# 11. Precision convergence of the inversion

At the representative IL-3 verification points, 80-digit values agree with 50/120-digit evaluations to extremely high precision.

Examples:

At 1e6 s, Stehfest |F50-F80| is about `1.85e-37` and |F80-F120| is zero at stored precision.

At 1e10 s, Stehfest |F50-F80| is about `7.12e-38`; de Hoog is about `5.4e-44`.

At the 1e2 s unresolved tail, all methods/precisions push the magnitude rapidly toward zero.

Thus 80 digits is defensible as the primary inversion precision.

**Verdict: PASS.**

---

# 12. Forward-transform reconstruction diagnostic

The IL-3 post-hoc reconstruction uses the 81 recovered CDF points:

`Phi_rec(s) ~= sum exp(-s sqrt(t_i t_{i-1})) [F_i-F_{i-1}]`.

Reported relative errors versus direct Phi are:

1e-9: 9.105e-5;
1e-8: 2.772e-4;
1e-7: 1.556e-3;
1e-6: 9.987e-3;
1e-5: 9.412e-2.

Independent inspection confirms that this is a **post-hoc quadrature diagnostic**, not the inversion algorithm itself.

The high-s 9.4% relative discrepancy is not evidence that Stehfest and de Hoog disagree: their recovered CDF grids are effectively identical, so the same reconstruction produces the same value.

The lower time limit at 1e2 s is not quantitatively capable of explaining the high-s discrepancy by itself: the physical CDF at 1e2 s is astronomically small (approximately 1e-253 to 1e-340 in the earlier controlled inversions).

The dominant plausible source is therefore the coarse 0.1-decade quadrature representation of the recovered CDF over a transform whose relevant time scale moves toward 1/s. At s=1e-5, the relevant time region is around 1e5 s, where a 0.1-decade grid has only about 26% multiplicative spacing.

That makes the project's explanation quantitatively credible, but the present diagnostic alone does not prove the reconstruction error is purely quadrature error.

Because the actual inversion has two mutually consistent methods, 50/80/120 precision convergence, and exact F+S closure, the reconstruction discrepancy is not sufficient to reject the CDF.

### R4-m01 — MODERATE

**Issue:** the forward-reconstruction check is too coarse to serve as a strong independent verification diagnostic at the highest s values.

**Consequence:** its 9.4% relative error should not be presented as evidence of inversion accuracy or inaccuracy.

**Required remediation:** none to the accepted CDF. For a stronger future check, rerun reconstruction on a substantially finer log-time grid or with an independently controlled quadrature/interpolant, and report absolute error as well as relative error when Phi(s) is extremely small.

**Closure criterion:** refined reconstruction demonstrates that the high-s absolute error is quadrature-controlled.

**Status:** non-blocking.

---

# 13. Additional targeted computation before accepting the CDF

After considering the whole evidence chain, I do **not** require additional computation before accepting the controlled Process-C five-layer release CDF.

The remaining uncertainty in the forward-reconstruction diagnostic is methodological weakness of that check, not evidence of a contradictory release curve.

The inversion itself has stronger evidence:

- two distinct inversion methods;
- 50/80/120 precision checks;
- CDF and survival closure;
- stable complex continuation;
- independently verified source factor;
- independently closed hard-matrix arithmetic.

Therefore an additional reconstruction-only run is optional, not a release-CDF blocker.

---

# 14. Process A/B/C scope

The present result is explicitly Process C.

Process C is the deterministic Markov-renewal transform reduction of Process B.

Process A remains the finite-capture production WOS.

Successful Process-C inversion does not establish A=B=C.

The earlier controlled A-versus-B evidence established empirical compatibility at finite epsilon for the controlled two-layer benchmark. That does not become five-layer pathwise equivalence by inversion.

The present audit therefore accepts only the following statement:

`Process-C hard five-layer release-time CDF is numerically verified at the current controlled transform/inversion level.`

That statement is appropriate.

---

# 15. R2-WOS-02

## OPEN

The earlier finding concerns the unresolved integrated finite-capture production-WOS problem. Process-C inversion does not close it.

# 16. R2-B01

## OPEN

The controlled Process-C release CDF does not by itself close the broader Method-2 production verification blocker.

The final physical production claim remains appropriately qualified.

# 17. Method-3 status

## NOT STARTED / OUT OF SCOPE

The successful Process-C inverse-Laplace recovery does not justify beginning Method 3.

Method 2 remains the active route for the remaining production-integration questions.

# 18. Findings summary

## CRITICAL
None.

## MAJOR
None.

## MODERATE
R4-m01 — Forward-transform reconstruction is too coarse to be a strong high-s verification diagnostic; its 9.4% relative discrepancy is not evidence of inversion failure.

## MINOR
None.

## NOTE
N1 — The manuscript/traceability status that says the inverse-Laplace result is 'pending independent review' should be synchronized after this review.

N2 — Near-zero Stehfest negatives should remain in raw machine-readable evidence and only be described as numerically unresolved from zero in human-facing summaries.

# 19. Final classification

# B. VERIFIED WITH NON-BLOCKING FINDINGS — RELEASE CDF ACCEPTED WITH STATED LIMITATIONS

The controlled five-layer Process-C release CDF is accepted at the current numerical-verification level.

The accepted claim is bounded:

- accepted for Process C;
- 81-point grid from 1e2 to 1e10 s;
- 80-digit primary arithmetic with 50/120-digit checks at representative times;
- cross-validated by Stehfest and de Hoog;
- early Stehfest values below numerical resolution are not interpreted as physical negative probabilities;
- high-s forward reconstruction is a coarse diagnostic and not a certification of pointwise transform accuracy;
- no claim is made for Process A pathwise identity;
- R2-WOS-02 and R2-B01 remain OPEN.

# 20. Single highest-value next action

Proceed to the next **Method-2 integration/verification stage** focused on reconciling the accepted Process-C release CDF with the broader production-WOS question, while keeping Process A/B/C distinctions explicit.

Do not spend the next stage on brute-force inverse-Laplace refinement unless a specific downstream requirement needs a tighter time-grid or transform-reconstruction error bound.

Do not begin Method 3 from this result alone.