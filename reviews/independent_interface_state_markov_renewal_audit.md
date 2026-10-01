# Independent Mathematical Audit — Five-Layer Interface-State Markov-Renewal Reduction

Repository: weewzr/Derivation-of-TRISO-Source-Term-Modelling
Branch: main
Reviewed commit: c6c8fdbc33ba7650c94363105c63a83bcd6b0291
Supervisor commit underpinning the prerequisite WOS evidence: 8d31482d127e211614ebb3f66b1076e1ed6dea98
Scope: derivation/design audit only; no five-layer hard release calculation performed; no production supervisor source modified.

# Exact gate decision

C. REMEDIATION REQUIRED BEFORE IMPLEMENTATION

The eight-state interface-state Markov-renewal construction is mathematically promising and, as a renewal representation of the controlled radial accelerated process, is largely coherent. However, one central equivalence statement is not presently exact: the derivation claims equivalence to the frozen finite-capture production WOS process while its shell/ball kernels terminate at the ideal material interface radius and then apply the finite-epsilon reinsertion rule. The production WOS interface event occurs when the walker enters the capture region, not when it reaches the exact interface. The elapsed time and starting radius at capture therefore differ in general.

This can likely be repaired without changing the underlying reduction: explicitly define the target process as the previously verified accelerated epsilon-regularized process, or incorporate the finite-capture stopping surface/time into the renewal kernel. Until that distinction is repaired, implementation should not proceed from the present derivation as if exact finite-epsilon equivalence were already established.

R2-B01 remains OPEN.

---

# 1. State-space sufficiency

## Verdict: PASS for the controlled radial accelerated process

The proposed states

S0 Kernel side of I0
S1 Buffer side of I0
S2 Buffer side of I1
S3 IPyC side of I1
S4 IPyC side of I2
S5 SiC side of I2
S6 SiC side of I3
S7 OPyC side of I3

are sufficient for the stated concentric, time-homogeneous, fixed-D benchmark, provided each state is defined as a post-interface reinsertion state with deterministic radius.

Reason: within each homogeneous layer, the diffusion is rotationally invariant; shell boundaries depend only on radius; the interface transmission/reflection rule depends only on the adjacent material states and diffusivities; and the reinsertion radius is deterministic once the interface and selected side are known. The strong Markov property therefore permits restarting the process at each interface event using only the post-event state plus accumulated time.

No hidden angular or path-history state is required for the radial release observable in this benchmark.

Important limitation: this sufficiency is specific to the spherical fixed-property benchmark. It would not survive arbitrary angularly varying properties, non-spherical geometry, or history-dependent interface rules.

---

# 2. Shell transition kernels

## Verdict: PASS

For a homogeneous shell a<r<b, the audited transforms are:

G^+(r,s) = E_r[e^{-sT} 1{exit at b}] = (b/r) sinh(lambda(r-a))/sinh(lambda(b-a))

G^-(r,s) = E_r[e^{-sT} 1{exit at a}] = (a/r) sinh(lambda(b-r))/sinh(lambda(b-a))

with lambda=sqrt(s/D).

These follow directly from the backward equation

D(G''+2G'/r)=sG

and the appropriate unit/zero boundary values.

For the Buffer states the direction assignments are correct:

- S1 exits inward to I0 or outward to I1;
- S2 exits inward to I0 or outward to I1;
- transmission/reflection at I0 maps to S0/S1;
- transmission/reflection at I1 maps to S3/S2.

The same structural mapping is correct for IPyC, SiC and OPyC.

No omitted shell exit-side is present in the eight-state topology.

---

# 3. Interface probabilities

## Verdict: PASS

For K=1 the construction uses

p_{l->r}=D_r/(D_l+D_r)

and

p_{l->l}=D_l/(D_l+D_r).

The reverse-direction rule is the corresponding exchange of the diffusivities.

This matches the previously accepted production interface rule.

At each interface, the probabilities sum to one.

No transition-matrix orientation error was found in the documented S0-S7 construction.

---

# 4. Physical-time preservation

## Verdict: PASS within the stated accelerated-process model; NOT PASS for literal finite-capture production equivalence

The matrix entries correctly retain physical shell first-passage time through the Laplace factors G^-/G^+.

Interface transmission/reflection contributes no physical time, consistent with the frozen implementation semantics.

Repeated renewal paths therefore multiply Laplace transforms, so convolution of successive physical waiting times is represented correctly in transform space.

However, there is a central finite-capture mismatch, discussed in Finding R3-M01 below.

Production WOS triggers an interface event at distance epsilon_capture from an interface. The present shell reduction instead samples to the exact interface and then applies the interface decision. These are not literally the same stopping times.

The discrepancy exists both after a shell excursion and at initial kernel birth: a production history born inside the capture region can encounter the interface with zero WOS-hop time, while the present centered-ball factor H_K samples a positive first-passage time to the physical kernel radius.

Therefore the statement in the derivation that the matrix process is 'the same finite-epsilon stochastic process' is too strong without an explicit equivalence argument or a modified kernel.

---

# 5. Matrix renewal equation

## Verdict: PASS after resolving the process-definition distinction

The equation

Phi_i(s)=sum_j K_ij(s) Phi_j(s)+B_i(s)

is correctly derived from first-step decomposition.

In vector form:

Phi(s)=K(s)Phi(s)+B(s),

so whenever I-K(s) is invertible:

Phi(s)=[I-K(s)]^{-1}B(s).

The entries of K are dimensionless Laplace transforms of probability-weighted first-passage times, hence are dimensionless.

The source/release vector B has the correct interpretation as the direct absorbing contribution.

For real s>0, every genuine transient transition has transform strictly below its s=0 mass, so the discounted transition operator is a strict contraction under the finite-state absorbing construction. At s=0, the remaining issue is eventual absorption.

No indexing/orientation inconsistency was found.

---

# 6. Geometric path sum

## Verdict: PASS

The formal expansion

(I-K)^-1 = I + K + K^2 + ...

is the Neumann series when the spectral radius of K is below one.

Each factor K corresponds to one transient shell-exit plus interface-renewal event, and each matrix power enumerates an additional finite sequence of transient renewals.

No path class is omitted by the eight-state topology:

- kernel reflections remain S0;
- kernel transmission goes S1;
- buffer returns at I0 go S0/S1 depending on interface outcome;
- buffer visits to I1 go S2/S3;
- analogous mappings continue through IPyC and SiC;
- OPyC S7 either renews inward through I3 or exits directly through R.

Path probabilities and physical waiting-time transforms are multiplied exactly as required.

---

# 7. Absorbing release vector

## Verdict: PASS

Only S7 has direct access to the absorbing outer boundary.

Thus

B_7(s)=G_OPyC^+(r_7,s)

is correct for a state defined on the OPyC side of I3 at its deterministic reinsertion radius.

All other B_i are zero.

Outer release is therefore neither omitted nor double counted.

---

# 8. Initial kernel distribution

## Verdict: PASS, subject to the same finite-capture caveat

For uniform kernel-volume birth density:

f_R(r)=3r^2/R0^3

which is correctly normalized:

integral_0^R0 3r^2/R0^3 dr = 1.

The proposed expression

Phi_init(s)=integral f_R(r) H_K(r,s)[p_KK Phi_0+p_KB Phi_1] dr

is the correct first-step decomposition for an exact centered-ball kernel first exit, followed by the interface decision.

Physical time before the first interface event is retained through H_K.

Again, the literal finite-capture production process has a distinction when the birth point lies within capture epsilon of I0: production WOS can resolve the interface without a full trip to R0, while this reduction samples to R0. Therefore the expression is exact for the accelerated ideal-interface process, not yet proven identical to the finite-capture production process.

---

# 9. s=0 normalization

## Verdict: PASS for eventual absorption under the stated finite positive-D five-layer model

At s=0,

G^-+G^+=1

and transmission plus reflection equals one.

The resulting S0-S7 chain has positive probability of moving outward across every interface and positive probability of reaching the outer absorbing boundary from S7.

Because all diffusivities are strictly positive and all interface transmission probabilities are strictly positive for the stated finite-D benchmark, there is no closed nonabsorbing state class that is forced by the model.

A finite-state chain in which every transient state has a finite positive-probability path to absorption has spectral radius below one.

Therefore the eventual release probability is one:

Phi_i(0)=1

and the normalized initial mixture gives:

Phi_init(0)=1.

This should still be implemented as an explicit unit check before any large calculation.

---

# 10. PDE / stochastic connection

## Verdict: PASS WITH REQUIRED EPISTEMIC QUALIFICATION

The backward shell equation is the backward generator corresponding to the homogeneous radial diffusion operator inside each layer.

Stopping at successive shell boundaries and applying the strong Markov property is mathematically legitimate.

The matrix reduction is therefore a valid algebraic marginalisation of a Markov-renewal process built from those shell kernels and the frozen interface rule.

However, two different equivalence statements must be kept separate:

1. equivalence to the reduced/accelerated interface-renewal process defined by exact shell-to-interface stopping and deterministic alpha-epsilon reinsertion;
2. equivalence to the original finite-capture production WOS implementation.

The first is mathematically justified by the construction.
The second is not yet exact because of the finite-capture stopping discrepancy identified above.

Separately, neither statement is a theorem of equivalence to the ideal epsilon->0 divergence-form PDE.

The existing controlled two-layer WOS/FV plateau is empirical support for small practical discrepancy, not a pathwise identity.

---

# 11. Release-time transform

## Verdict: PASS

For nonnegative release time T:

Phi_init(s)=E[e^{-sT}]

is the Laplace-Stieltjes transform of the release-time law.

For the CDF F(t)=P(T<=t) and s>0:

integral_0^infinity e^{-st}F(t)dt
= E[integral_T^infinity e^{-st}dt]
= E[e^{-sT}]/s
= Phi_init(s)/s.

This requires the usual measurability/integrability condition; for a nonnegative T and s>0 the expression is well defined even when E[T] is not finite.

No mathematical error found.

---

# 12. Numerical recovery options

## Safest next route: controlled numerical transform evaluation plus independent explicit-renewal comparison

Option 1: numerical inverse Laplace
- mathematically direct;
- potentially sensitive to cancellation and conditioning for complex s;
- requires stable evaluation of hyperbolic functions and the 8x8 linear system over a suitable complex contour;
- deterministic and reproducible once the contour/inversion scheme is fixed.

Option 2: reduced macro-renewal sampling
- easiest to cross-check against explicit renewal simulation;
- preserves a direct stochastic interpretation;
- useful for validating the matrix before inverse-Laplace machinery is trusted.

Option 3: moments/quantiles
- valuable unit checks;
- insufficient as the sole release-distribution validation.

Recommended sequence: first implement the matrix transform and compare its moments/finite-time observables against explicit controlled renewal simulation; only then introduce numerical inverse Laplace recovery for the full CDF.

Do not use an inverse-Laplace result as the first implementation-level proof of correctness.

---

# 13. Toy-chain verification

## Verdict: PASS as an algebraic sanity check

For

Phi(s)=g(s)[p+q Phi(s)], q=1-p,

solving gives:

Phi(s)=p g(s)/(1-q g(s)).

The expansion

p g(s)[1+qg(s)+q^2g(s)^2+...]

corresponds exactly to zero, one, two, ... failed renewal cycles before release.

At s=0 with g(0)=1:

Phi(0)=p/(1-q)=1.

This verifies the algebraic interpretation of the matrix inverse.

It does not verify:

- the eight-state geometry;
- the shell joint first-passage law;
- interface probabilities;
- finite-capture semantics;
- numerical conditioning;
- initial-radius quadrature;
- the five-layer stochastic process.

Therefore it is useful and correctly scoped.

---

# 14. Validation hierarchy

## Verdict: sufficient, with one required addition

The proposed hierarchy is appropriate:

A. analytical toy-chain verification;
B. matrix transform versus explicit accelerated renewal in a feasible controlled regime;
C. controlled two-layer FV comparison;
D. controlled three-layer or reduced-contrast multilayer comparison;
E. Phi(0)=1 normalization;
F. release-time moments/distribution verification;
G. independent review.

One additional mandatory gate should be inserted between B/C and any hard five-layer use:

H. explicit finite-capture versus exact-interface process reconciliation.

This is necessary because the matrix currently claims to represent the finite-epsilon production stochastic process while actually starting shell segments at the physical interface and using alpha-epsilon reinsertion.

The gate can be small: either prove the difference is exactly zero under the adopted accelerated-process definition, or quantify the difference against the direct finite-capture process in the already feasible two-layer benchmark.

No other bureaucracy is necessary.

---

# 15. Findings

## CRITICAL

None.

## MAJOR

### R3-M01 — Literal finite-epsilon production equivalence is overstated

Affected documents: `verification/interface_state_markov_renewal_derivation.md`, especially Sections 5 and 9.

Evidence:
- production WOS triggers an interface event when the nearest interface is within finite capture epsilon;
- the proposed renewal kernel instead samples first passage to the exact physical interface;
- after that it applies the zero-time interface decision and alpha-epsilon reinsertion;
- the initial kernel factor similarly samples a full centered-ball exit to the physical kernel radius.

Independent reasoning:
The finite-capture stopping time is generally strictly earlier than the physical-interface hitting time. The resulting starting position and elapsed time entering the interface decision therefore differ. This is not a mere notation difference.

Consequence:
The matrix reduction is not yet proven to be exactly the same finite-epsilon production WOS process described in the frozen contract. The empirical two-layer agreement does not turn the two processes into an identity.

Required remediation:
Reframe the derivation explicitly as a reduction of the already verified accelerated exact-interface renewal process, or augment the kernel to reproduce the finite-capture stopping surface and its associated time distribution.

Closure evidence:
Either a mathematical equivalence proof under an explicitly redefined accelerated process, or a controlled two-layer quantitative reconciliation showing that the finite-capture difference is below the declared benchmark tolerance.

Later dependency:
Must be resolved before the matrix is presented as an exact reduction of production finite-epsilon WOS.

### R3-M02 — The convergence/verification hierarchy should distinguish process equivalence from empirical agreement

Affected document: `verification/interface_state_markov_renewal_derivation.md`, Sections 9 and 12.

Reasoning:
The current text correctly says epsilon->0 continuum equivalence is not automatic, but it moves too quickly from shell strong-Markov construction to 'same finite-epsilon stochastic process'. The previous controlled two-layer evidence supports agreement, not identity.

Consequence:
A reader could incorrectly treat the matrix result as an exact replacement for the production finite-capture walker.

Required remediation:
Use three explicit layers: exact accelerated-process equivalence; empirical agreement with finite-capture WOS; eventual continuum relevance.

Closure evidence:
Updated derivation language plus the finite-capture reconciliation gate.

Later dependency:
Before five-layer claims.

## MODERATE

### R3-M03 — Explicit spectral-radius/invertibility proof should be recorded at s=0

The text states the result Phi_i(0)=1 but does not explicitly construct the finite-state absorption argument in matrix terms.

The positive transition probability across each interface and positive outer release probability are enough to justify eventual absorption for the finite-state chain, but the proof should be written once rather than left implicit.

Required remediation:
Add a short theorem/proposition showing a positive-probability finite path from every S_i to release, hence rho(K(0))<1.

Closure evidence:
Formal finite-state absorption proof and Phi(0)=1 unit test.

### R3-M04 — Numerical conditioning is identified but not quantified

The matrix has state-transition probabilities approaching one when diffusivity contrasts are extreme. In the actual TRISO diffusivity hierarchy, some interfaces are highly reflective.

This does not invalidate the mathematics, but it makes conditioning at s near zero a real implementation concern.

Required remediation:
Before hard five-layer use, evaluate condition numbers or residuals of I-K(s) over the intended s range and include a conditioning diagnostic.

Closure evidence:
Predeclared conditioning study with residual thresholds.

## MINOR

### R3-m01 — Toy verification is algebraic only

Correctly scoped, but it should remain explicitly labelled as algebraic sanity evidence rather than stochastic validation.

### R3-m02 — The eight state definitions would benefit from a compact transition table

The derivation is correct but a full 8x8 nonzero-pattern table would make auditing easier and reduce the risk of later row/column mapping errors.

---

# 16. Previously established prerequisite status

Shell transform mathematics: PASS.
Shell exit-side/moment verification: PASS.
Conditional shell CDF verification: PASS.
Corrected two-layer accelerated coupling: PASS at controlled two-layer level.
High-power direct-WOS interface plateau: accepted at declared statistical precision.
Direct five-layer production WOS: computationally unusable under the frozen benchmark.
Five-layer shell-accelerated explicit renewal diagnostic: computationally unusable under the bounded renewal cap.

These establish why the matrix reduction is worth investigating. They do not by themselves validate the new eight-state matrix.

---

# 17. R2-B01

## OPEN

The present mathematical reduction cannot close R2-B01.

R2-B01 still requires a defensible integrated five-layer release-time calculation.

---

# 18. Final classification

# C. REMEDIATION REQUIRED BEFORE IMPLEMENTATION

The underlying idea is mathematically viable and the state-space/matrix construction is substantially correct.

The required remediation is narrow but important: correct the equivalence statement between the exact-interface accelerated renewal process and the original finite-capture production WOS process, and add the corresponding controlled reconciliation gate.

Once that distinction is repaired, the derivation can proceed to a small controlled matrix implementation/verification. The hard five-layer Cs-137 release calculation remains unauthorized.

---

# 19. Single highest-value next action

Perform the smallest controlled two-layer finite-capture reconciliation experiment for the proposed matrix process.

Use the already verified two-layer benchmark and compare three objects under identical physical parameters:

1. direct finite-capture production WOS;
2. explicit accelerated exact-interface renewal;
3. matrix-renewal transform result.

The sole purpose is to quantify the finite-capture-to-exact-interface difference that the current derivation treats as an identity.

If that reconciliation closes at the existing declared precision, the eight-state matrix formulation is ready for controlled implementation verification. If it does not, the kernel must be modified before any five-layer use.

Do not run the hard five-layer release calculation yet.