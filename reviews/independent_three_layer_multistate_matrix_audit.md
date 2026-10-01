# Independent Audit — Controlled Three-Layer Multistate Markov-Renewal Matrix Verification

**Review date:** 1 October 2026  
**Canonical branch:** main  
**Repository:** `weewzr/Derivation-of-TRISO-Source-Term-Modelling`  
**Reviewed main tip:** `dafa2d9e33577e0c32daee31d99e503218f00cd8`  
**Research commit producing the successful experiment:** `9b4bc635eedbf7c581184f96a2f8607624e152fc`  
**Supervisor reference used by the successful run:** `8d31482d127e211614ebb3f66b1076e1ed6dea98`  
**Successful run:** `36815849244`  
**Artifact:** `three-layer-multistate-matrix-results`  
**Artifact ID:** `11142115303`  
**Artifact SHA-256:** `e26ec7bd4bc6d153f3780a4891bf98693b83041b42e05e8ebae4543181e051e8`

## Scope

This is an independent audit of the controlled three-layer multistate Markov-renewal matrix verification.

It does not authorize the hard five-layer Cs-137 calculation merely because this controlled test succeeds.

No derivation, implementation, workflow, or supervisor source was modified during this review.

# Final gate decision

## **B. CONTROLLED MULTISTATE VERIFICATION PASSES WITH NON-BLOCKING FINDINGS — CLEARED FOR BOUNDED HARD FIVE-LAYER MATRIX DIAGNOSTIC**

The controlled experiment provides credible independent evidence for a genuine four-state Markov-renewal composition, not merely a two-state shortcut.

Specifically:

- the state topology is internally complete;
- the matrix equation correctly represents the documented renewal decomposition;
- explicit Process-B and deterministic matrix Process-C transforms agree at Monte-Carlo precision;
- all predeclared normalization checks pass;
- linear residuals are far below the declared threshold;
- the controlled matrix has been conditioned/measured at every predeclared transform point;
- the FV reference shows strong spatial/temporal refinement and near-machine-precision inventory closure;
- Process-B CDF values are compatible with the resolved FV reference at the declared controlled precision;
- benchmark parameters were predeclared before the successful numerical result;
- the two failed workflow runs were execution/compiler failures and did not generate scientific numerical results.

The controlled result therefore clears the requested next stage: a **bounded diagnostic of the actual 8x8 hard five-layer matrix**.

It does **not** validate the hard five-layer release CDF.

---

# 1. Current repository state

The requested canonical branch `main` currently points to:

`dafa2d9e33577e0c32daee31d99e503218f00cd8`

The supplied prior observation is therefore verified.

The successful controlled experiment was executed from research commit:

`9b4bc635eedbf7c581184f96a2f8607624e152fc`

The successful workflow recorded the supervisor source at:

`8d31482d127e211614ebb3f66b1076e1ed6dea98`

The branch remains the canonical active research branch. No use of `ray/triso-foundation` was required for this audit.

---

# 2. Runs and artifacts inspected

Inspected:

- `36815332644` — failed;
- `36815342086` — failed;
- `36815849244` — successful canonical run;
- successful artifact `11142115303`.

The successful artifact hash independently matches the hash recorded in the repository evidence:

`e26ec7bd4bc6d153f3780a4891bf98693b83041b42e05e8ebae4543181e051e8`.

The successful workflow actually executed the released optimized binary and produced the reported transform, CDF and FV values.

---

# 3. Failed-run provenance

## Run 36815332644

The workflow reached compilation of the harness and failed with Rust compiler errors including:

- shorthand floating-point literals such as `.5`, `.75`, `.4`, `.2`;
- an unrelated accidental `c64` type in the FV routine.

The failure occurred before numerical execution.

No scientific result was produced.

## Run 36815342086

The preceding compile typo was corrected, but the harness still contained shorthand floating-point literals such as:

`.02`, `.05`, `.10`, `.20`, `.40`, `.80`

and corresponding shorthand literals in other functions.

Compilation again failed before numerical execution.

No scientific result was produced.

## Successful correction

Commit `9b4bc635eedbf7c581184f96a2f8607624e152fc` changed the harness syntax to valid Rust float literals and removed the accidental invalid type.

The comparison of the failed → successful commits shows no changes to:

- benchmark radii;
- diffusivities;
- `K`;
- `epsilon`;
- `alpha`;
- `N=20,000`;
- predeclared transform points;
- predeclared CDF times;
- matrix residual criterion.

Therefore the successful run is not the result of scientific parameter retuning after seeing failed numerical results.

**Verdict: execution failures only; no scientific defect exposed.**

---

# 4. Benchmark-freeze / predeclaration audit

The benchmark gate was committed before the executable experiment.

The relevant order is:

1. `f6b40c99f0c512e0fa26e8660e92dfdc6f086697` — predeclare controlled three-layer gate;
2. `afd2960452b57bd55fcaa1b21d9ec6d1aa1a0301` — add verification harness;
3. failed execution attempts;
4. `9b4bc635eedbf7c581184f96a2f8607624e152fc` — syntax correction;
5. successful run 36815849244;
6. evidence persisted at `dafa2d9e33577e0c32daee31d99e503218f00cd8`.

The frozen values were:

[
R=(50,75,100)~mu m
]

[
D=(10^{-9},2	imes10^{-9},5	imes10^{-9})~m^2/s
]

[
K=1,quad
epsilon=100~nm,quad
alpha=2,quad
N=20,000.
]

Predeclared transform points:

[
s=(0,1,2,5,10,20)~s^{-1}.
]

Predeclared CDF times:

[
t=(0.02,0.05,0.10,0.20,0.40,0.80)~s.
]

The residual criterion was:

[
10^{-10}.
]

Normalization criterion:

[
|Phi_i(0)-1|<10^{-10}
]

and

[
|Phi_{init}(0)-1|<10^{-10}.
]

The FV resolution plan was also declared before successful execution.

**Verdict: predeclaration integrity PASS.**

---

# 5. Multistate topology audit

The controlled problem has four transient states:

- (S_0): material-1 side of (I_0);
- (S_1): material-2 side of (I_0);
- (S_2): material-2 side of (I_1);
- (S_3): material-3 side of (I_1);

plus absorbing release (A).

The documented non-zero transitions are complete.

For (S_0):

[
S_0ightarrow S_0,
qquad
S_0ightarrow S_1.
]

For (S_1):

[
S_1ightarrow S_0, S_1, S_2, S_3.
]

For (S_2):

[
S_2ightarrow S_0, S_1, S_2, S_3.
]

For (S_3):

[
S_3ightarrow S_2, S_3, A.
]

The topology correctly represents both possible shell exits from material 2 and both transmission/reflection outcomes at each internal interface.

No duplicated or reversed transition was found.

The state space is genuinely greater than two states because (S_1) and (S_2) are different post-interface states in the same material but at different interfaces/reinsertion radii, producing different shell-exit kernels.

**Verdict: PASS.**

---

# 6. Independent matrix reconstruction

The renewal construction is:

[
Phi_i(s)
=
sum_jK_{ij}(s)Phi_j(s)+B_i(s).
]

Hence:

[
(I-K)Phi=B
]

and:

[
oxed{Phi=(I-K)^{-1}B}.
]

The implemented 4x4 matrix follows this structure.

For (S_0):

[
K_{00}=H_Kp_{11},
qquad
K_{01}=H_Kp_{12}.
]

For (S_1,S_2), the shell transforms (G^-) and (G^+) are multiplied by the appropriate transmission/reflection probabilities and mapped to the four correct destination states.

For (S_3):

[
K_{32}=G^-p_{32},
qquad
K_{33}=G^-p_{33},
]

with:

[
B_3=G^+.
]

This is a genuine multistate first-step decomposition, not a disguised 2-state shortcut.

The matrix code independently reconstructs each of those terms.

**Verdict: PASS.**

---

# 7. B versus C transform verification

The successful run reports:

| (s) | Matrix C | Explicit B | SE | (B-C) | (z) |
|---:|---:|---:|---:|---:|---:|
| 0 | 1.000000000000 | 1.000000000000 | 0 | (sim-1.11	imes10^{-15}) | 0 |
| 1 | 0.610894564738 | 0.610257673058 | 0.00143408 | (-6.37	imes10^{-4}) | -0.444 |
| 2 | 0.414440336362 | 0.413544255439 | 0.00162945 | (-8.96	imes10^{-4}) | -0.550 |
| 5 | 0.174553542011 | 0.173615255224 | 0.00128713 | (-9.38	imes10^{-4}) | -0.729 |
| 10 | 0.063769458570 | 0.063274883544 | 0.00075746 | (-4.95	imes10^{-4}) | -0.653 |
| 20 | 0.015602361812 | 0.015478161280 | 0.00031454 | (-1.24	imes10^{-4}) | -0.395 |

Independent recalculation gives:

[
|z|_{max}=0.72898.
]

Thus the maximum discrepancy is less than one Monte-Carlo standard error.

The matrix/exact-renewal compatibility is therefore strongly supported at this controlled precision.

This establishes:

[
oxed{Bapprox C}
]

at the declared stochastic precision.

It does not establish:

[
oxed{A=B}
]

for finite-capture production WOS, nor direct equivalence to the continuum PDE.

**Verdict: PASS.**

---

# 8. Normalization and absorption

At (s=0), the reported state transforms are all one to floating-point precision.

The initial transform is also one.

Independent structural reasoning confirms eventual absorption:

- all (D_i>0);
- every internal transmission probability is strictly positive;
- there exists a finite positive-probability path from every transient state to (A);
- therefore there is no closed nonabsorbing communicating class.

Consequently:

[
ho(K(0))<1
]

for this finite controlled state chain, and:

[
Phi_i(0)=1.
]

The normalization criterion therefore passes.

**Verdict: PASS.**

---

# 9. Linear-system residuals

Reported residuals are approximately:

[
1.12	imes10^{-16},
1.24	imes10^{-16},
5.59	imes10^{-17},
4.00	imes10^{-17},
5.83	imes10^{-17},
1.39	imes10^{-17}.
]

All are vastly below:

[
10^{-10}.
]

The numerical solves therefore satisfy the declared residual criterion.

**Verdict: PASS.**

---

# 10. Conditioning terminology

The implementation computes:

[
|A|_F|A^{-1}|_F
]

for:

[
A=I-K.
]

That is a Frobenius-norm condition estimate.

It is **not** the exact spectral 2-norm condition number:

[
kappa_2(A)=|A|_2|A^{-1}|_2.
]

For independent confirmation, reconstructing the controlled matrices gives approximate true 2-norm condition numbers:

| (s) | reported (|A|_F|A^{-1}|_F) | independent (kappa_2(A)) |
|---:|---:|---:|
| 0 | 392.48 | 269.04 |
| 1 | 316.99 | 213.05 |
| 2 | 273.42 | 180.65 |
| 5 | 206.93 | 131.59 |
| 10 | 159.81 | 98.30 |
| 20 | 120.20 | 72.62 |

Thus the numerical values reported by the harness are internally consistent with a Frobenius estimate but must not be called `cond_2`.

These controlled values do not indicate a blocker for the 4x4 system.

They do **not** predict the conditioning of the hard 8x8 five-layer matrix.

### Finding R2-m01 — MINOR

**Affected locations:** `verification/three_layer_multistate_matrix_gate.md` and `verification/two_layer_matrix_reconciliation_evidence.md`.

**Issue:** The predeclared/evidence tables call the conditioning diagnostic `cond_2`, while the implementation actually computes the Frobenius product.

**Consequence:** Misleading numerical terminology and potentially incorrect comparison with a future spectral 2-norm calculation.

**Remediation:** Rename the recorded metric consistently as:

[
kappa_F^{est}=|A|_F|A^{-1}|_F.
]

If a true (kappa_2) is desired later, compute it separately.

**Closure evidence:** Updated labels and an explicit definition of the norm.

**Stage:** Documentation cleanup; does not block the hard diagnostic.

---

# 11. FV reference audit

The successful execution genuinely used:

- 50 cells/layer;
- 100 cells/layer;
- 200 cells/layer;
- explicit Euler;
- (q=0.4);
- (q=0.2);
- aligned material interfaces;
- conservative interface face resistance;
- absorbing outer boundary;
- normalized initial kernel condition.

Reported finest temporal step sizes:

[
Delta t_{0.4}=4.163770	imes10^{-7} s
]

and:

[
Delta t_{0.2}=2.081885	imes10^{-7} s.
]

Maximum temporal change:

[
1.345984	imes10^{-7}.
]

Maximum inventory residual:

approximately:

[
4.9	imes10^{-15}.
]

These values are independently consistent with the output.

### Spatial refinement correction

The recorded CDF values are:

50 cells/layer:
[
[1.04513	imes10^{-6},0.001749264,0.030573552,0.164784059,0.447241842,0.774625872]
]

100 cells/layer:
[
[1.05388	imes10^{-6},0.001755930,0.030610222,0.164836649,0.447271998,0.774633238]
]

200 cells/layer:
[
[1.05642	imes10^{-6},0.001757616,0.030619409,0.164849797,0.447279535,0.774635079].
]

Independent calculation of the 100→200 maximum change gives:

[
oxed{1.31488	imes10^{-5}}.
]

Therefore the evidence statement “below approximately (9.2	imes10^{-6})” is not reproduced by the actual recorded arrays.

### Finding R2-m02 — MINOR

**Affected location:** `verification/three_layer_multistate_matrix_evidence.md`, FV refinement section.

**Issue:** The stated maximum 100→200 spatial refinement change is numerically inconsistent with the recorded CDF arrays.

**Consequence:** The conclusion that FV discretisation uncertainty is small remains unchanged, but the exact reported refinement statistic is wrong.

The corrected value,

[
1.315	imes10^{-5},
]

is still much smaller than the approximately (10^{-3}) Monte-Carlo standard errors at the nontrivial observation times.

**Remediation:** Replace the reported (9.2	imes10^{-6}) value with the directly reproducible (1.31488	imes10^{-5}), or state the precise statistic that produced the former value if it was intended to represent something different.

**Closure evidence:** Regenerated refinement summary from the stored run output.

**Stage:** Documentation/evidence integrity; does not block the hard diagnostic.

---

# 12. FV versus stochastic CDF

Using the successful run's fine FV values, independent differences are:

| (t) s | B | FV | (B-FV) | B SE | z |
|---:|---:|---:|---:|---:|---:|
| 0.02 | 0 | (1.05705	imes10^{-6}) | (-1.05705	imes10^{-6}) | 0 | not meaningful |
| 0.05 | .001650 | .001757686 | (-1.07686	imes10^{-4}) | .00028699 | -0.375 |
| 0.10 | .031900 | .030619544 | (1.28046	imes10^{-3}) | .00124263 | 1.030 |
| 0.20 | .164200 | .164849815 | (-6.49815	imes10^{-4}) | .00261953 | -0.248 |
| 0.40 | .445500 | .447279439 | (-1.77944	imes10^{-3}) | .00351447 | -0.506 |
| 0.80 | .775750 | .774634983 | (1.11502	imes10^{-3}) | .00294925 | 0.378 |

Independent calculations reproduce:

[
mathrm{RMS}=1.039478	imes10^{-3}
]

and:

[
max|Delta F|
=
1.779439	imes10^{-3}.
]

At (t=0.02), the plug-in sample SE is zero because no B history released.

That does **not** imply exact equality.

The FV probability predicts only:

[
20,000(1.05705	imes10^{-6})
approx0.0211
]

expected releases.

So observing zero releases in 20,000 histories is entirely compatible with that very small probability.

The remaining five nonzero-time comparisons are all within approximately 1.03 B standard errors.

The signed discrepancies alternate rather than showing a coherent one-sided trend.

**Verdict: controlled B/FV compatibility PASS.**

---

# 13. What this experiment establishes

The evidence supports the following hierarchy.

### A. 2x2 matrix correctness

Already supported by the preceding controlled two-layer work.

### B. Genuine >2-state Markov-renewal composition

**Established at the controlled level.**

The present 4x4 system contains distinct post-interface states (S_0,S_1,S_2,S_3), different shell kernels, and multiple interacting renewal paths.

This is genuinely more than the previous 2x2 case.

### C. Compatibility with a resolved continuum FV reference

**Supported at controlled precision.**

The independent Process-B CDF and FV values are compatible at the stated Monte-Carlo precision.

### D. Hard five-layer Cs-137 hierarchy

**Not established.**

The controlled diffusivity contrasts are:

[
D_2/D_1=2,
qquad
D_3/D_2=2.5.
]

The hard TRISO hierarchy contains much more severe diffusivity separation and an 8-state matrix.

Therefore:

[
A,B,C 
otRightarrow D.
]

This distinction is essential.

---

# 14. Critical findings

**None.**

No defect was found that invalidates the controlled three-layer Markov-renewal mathematics or the executed B/C verification.

---

# 15. Major findings

**None.**

The remaining issues are evidence/documentation limitations, not failures of the controlled mathematical experiment.

---

# 16. Moderate findings

**None.**

The FV reference precision is somewhat overstated in one reported spatial statistic, but the corrected value is still comfortably below stochastic uncertainty and therefore remains a minor evidence-record issue.

---

# 17. Minor findings

### R2-m01 — conditioning metric mislabeled as cond_2

The harness computes:

[
|A|_F|A^{-1}|_F
]

not (kappa_2(A)).

### R2-m02 — incorrect stated 100→200 FV refinement maximum

Recorded output supports:

[
1.31488	imes10^{-5},
]

not approximately:

[
9.2	imes10^{-6}.
]

Neither finding changes the controlled gate.

---

# 18. R2-WOS-02 status

## OPEN

The controlled matrix experiment does not establish complete equivalence between the finite-capture production WOS process and the continuum process.

That remains outside this gate and must continue into later verification.

---

# 19. R2-B01 status

## OPEN

The hard five-layer production release calculation remains a separate problem.

The controlled three-layer experiment only supplies the requested authorization for a **bounded hard five-layer matrix diagnostic**.

It does not close R2-B01.

---

# 20. Authorization for next stage

The requested choice is:

### **B. CONTROLLED MULTISTATE VERIFICATION PASSES WITH NON-BLOCKING FINDINGS — CLEARED FOR BOUNDED HARD FIVE-LAYER MATRIX DIAGNOSTIC**

Before any inverse Laplace release curve is trusted, the hard 8x8 matrix diagnostic must independently inspect:

1. **(K(0)) probability topology**
   - every shell exit probability;
   - every interface transmission/reflection probability;
   - direct outer absorption;
   - no missing or duplicate state transitions.

2. **Normalization**
   [
   Phi_i(0)=1
   ]
   for all eight transient states and:
   [
   Phi_{init}(0)=1.
   ]

3. **Absorption / spectral radius**
   [
   ho(K(0))<1.
   ]

4. **Conditioning**
   Use a correctly named norm and predeclared transform grid.
   Do not import the controlled 4x4 conditioning numbers as evidence for the 8x8 problem.

5. **Linear residual**
   Verify:
   [
   rac{|(I-K)Phi-B|}{	ext{appropriate scale}}
   ]
   against a predeclared tolerance.

6. **Transform bounds and monotonicity**
   For real (sge0):
   [
   0lePhi_{init}(s)le1
   ]
   and:
   [
   s_2>s_1
   Rightarrow
   Phi(s_2)lePhi(s_1).
   ]

7. **Initial-radius quadrature**
   Demonstrate convergence of the kernel initial-condition integral independently of the matrix solve.

8. **Extreme-(D) numerical stability**
   Test hyperbolic-function evaluation and cancellation under the actual frozen five-layer diffusivity hierarchy.

9. **Floating-point strategy**
   Check whether direct (sinh) evaluation, scaled hyperbolic functions, or equivalent stable formulations are needed.

10. **Only after these checks**
    consider any inverse Laplace or release-CDF reconstruction.

The inverse-Laplace step is therefore **not yet authorized as a trusted physical release curve merely because the 8x8 matrix can be inverted**.

---

# 21. Why the gate is B rather than A

The controlled scientific result itself passes.

The non-blocking findings arise from evidence-record terminology rather than from the stochastic model:

- the conditioning diagnostic was mislabeled in the gate/evidence tables;
- one stated FV spatial-refinement number does not match the stored values.

Because these are real reproducibility issues, the strict independent classification is:

[
oxed{	ext{B rather than A}}.
]

They are straightforward to correct and do not justify additional controlled experiments before the bounded 8x8 diagnostic.

---

# 22. Recommended next action

Run the **bounded hard five-layer 8x8 matrix diagnostic only**.

Do not yet perform:

- inverse Laplace release-curve production;
- final Cs-137 release claims;
- physical validation claims;
- Method 3;
- changes to the supervisor WOS implementation.

The immediate mathematical object to audit is:

[
oxed{
Phi(s)
=
[I-K_{8	imes8}(s)]^{-1}B(s)
}
]

over a predeclared real-(s) grid.

The first output should be a matrix diagnostic table containing:

[
s,quad
ho(K),quad
kappa,quad
Phi_i,quad
Phi_{init},quad
	ext{residual}.
]

Only after that diagnostic passes should release-time reconstruction be considered.

---

# Final result

**Controlled three-layer multistate matrix verification: PASS WITH NON-BLOCKING FINDINGS.**

**Hard five-layer 8x8 matrix diagnostic: AUTHORIZED, BOUNDED.**

**Hard five-layer release CDF: NOT YET AUTHORIZED.**

**R2-WOS-02: OPEN.**

**R2-B01: OPEN.**

**Highest-value next action:** execute and independently audit the bounded 8x8 hard five-layer matrix diagnostic before attempting any inverse Laplace release curve.
