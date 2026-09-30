# Independent Review — Deterministic FV Accuracy, Convergence, and Numerical Verification

**Repository:** `weewzr/Derivation-of-TRISO-Source-Term-Modelling`  
**Branch:** `ray/triso-foundation`  
**Current reviewed branch tip:** `63880a6af4e996f5ddd32cff7f12033956bce92b`  
**Scope:** deterministic finite-volume accuracy, convergence, conservation, and executed numerical verification  
**Review type:** independent milestone audit  
**WOS comparison:** not performed  
**Repository modifications during review:** none

## Final gate

# **PASS FOR DETERMINISTIC REFERENCE / WOS COMPARISON**

The canonical five-layer finite-volume scheme has credible executed evidence for:

- steady spatial convergence on the stated aligned benchmark;
- first-order temporal convergence of explicit Euler;
- discrete inventory conservation to floating-point scale;
- implementation agreement with the canonical FV equations;
- reproducible execution through GitHub Actions.

The observed second-order steady spatial convergence is real for this benchmark.

It must **not** be interpreted as a universal second-order theorem for arbitrary discontinuous-(D) meshes or boundary/interface configurations.

The earlier FTCS Robin finding `R2-D01` remains open and does **not** block the FV reference track.

---

# 1. Review basis and current execution state

The current branch tip was obtained directly before review:

`63880a6af4e996f5ddd32cff7f12033956bce92b`

The requested GitHub Actions run is:

`36654437525`

Its recorded head commit is:

`039095e3b9419f6700950dbb426eb2a6210310fb`

The workflow completed:

- checkout;
- Rust compilation;
- convergence study execution;
- artifact upload;

with overall conclusion `success`.

The driver file at the run commit and the current branch has the same blob SHA:

`861aa163b3175a1023beb7ab651b68039567cfe4`.

Therefore the executed study used the same driver currently present on the reviewed branch.

The workflow log independently confirms that the run checked out commit `039095e...`, compiled `verification/fv_convergence.rs`, executed it, and produced the exact persisted output values.

---

# 2. Implementation ↔ mathematical derivation

## Independent result: VERIFIED

The current `verification/fv_convergence.rs` implements the reviewed FV construction rather than an unrelated approximation.

### Spherical geometry

The code uses

[
V_P=
rac{4pi}{3}(r_e^3-r_w^3),
]

matching the canonical control-volume equation.

It defines the spherical volume centroid as

[
r_P
=
rac34
rac{r_e^4-r_w^4}{r_e^3-r_w^3},
]

which is the correct centroid.

### Face conductances

For an interface-aligned face, it uses

[
G_f
=
4pi r_f^2
left(
rac{delta r_P}{D_P}
+
rac{delta r_E}{D_E}
ight)^{-1},
]

matching the resistance-weighted FV derivation.

For same-material regions this reduces to the expected constant-(D) conductance.

### Centre

The central cell uses zero west-face area and a single east conductance, consistent with the spherical control-volume derivation.

### Source

The code marks all volume-centroid cells inside (r<1) as source cells.

Because every tested mesh satisfies (Nequiv0pmod5), the interfaces (r=1,2,3,4) are exact mesh faces.

Thus the source is aligned exactly with the first material interface.

### Robin boundary

The code uses

[
h_{m eff}
=
left(
rac{R-r_P}{D_5}
+
rac1H
ight)^{-1}
]

and

[
G_R=4pi R^2h_{m eff},
]

which directly matches the reviewed cell-centred Robin closure.

### Linear steady solve

The assembled tridiagonal system has:

[
-G_w C_W+(G_w+G_e)C_P-G_eC_E=S_PV_P,
]

which is the correct rearrangement of the conservative steady FV equation.

### Transient update

The explicit update in `integrate()` is:

[
C_P^{n+1}
=
C_P^n+
rac{Delta t}{V_P}
left[
G_wC_W^n-(G_w+G_e)C_P^n+G_eC_E^n
ight]
+
S_PDelta t,
]

with the outer boundary replacing (G_eC_E) by zero and retaining (G_R C_P) in the diagonal term.

This matches the canonical explicit FV system.

**Conclusion:** the executed program is implementing the reviewed mathematical FV scheme.

---

# 3. Benchmark geometry and parameters

The executed study uses:

[
(r_1,r_2,r_3,r_4,R)
=
(1,2,3,4,5)
]

and

[
(D_1,D_2,D_3,D_4,D_5)
=
(1,0.5,2,0.25,1.5),
]

with

[
S_0=1,
qquad
h=0.8,
qquad
c_infty=0.
]

These are clearly treated as normalized mathematical verification parameters, not physical TRISO material data.

For

[
N=25,50,100,200,400,
]

there are exactly:

[
N/5
]

cells in each material.

Thus:

| Total cells | Cells/layer | (Delta r) |
|---:|---:|---:|
| 25 | 5 | 0.2 |
| 50 | 10 | 0.1 |
| 100 | 20 | 0.05 |
| 200 | 40 | 0.025 |
| 400 | 80 | 0.0125 |

All interfaces are exactly face-aligned.

**Independent result:** VERIFIED.

---

# 4. Analytical reference implementation

The convergence driver does **not** compare cell averages against arbitrary point samples.

It defines `exact_steady(r)` from the already independently verified five-layer analytical solution and then computes:

[
oxed{
ar c_P
=
rac1{V_P}
int_{r_w}^{r_e}
c_{m exact}(r),4pi r^2,dr
}.
]

This is the correct comparison quantity for the FV unknown.

The numerical integration uses eight-point Gauss-Legendre quadrature.

This is effectively exact for this benchmark because within each aligned cell:

### Kernel

[
c(r)=A-Br^2,
]

so

[
c(r)r^2=Ar^2-Br^4,
]

a polynomial of degree 4.

### Coating

[
c(r)=A+rac Br,
]

so

[
c(r)r^2=Ar^2+Br,
]

a polynomial of degree 2.

Eight-point Gauss-Legendre integration therefore introduces only floating-point error for these cell integrals.

**Independent result:** the reference quantity is appropriate and the numerical cell-average construction is credible.

---

# 5. Spatial error norm

The code defines:

[
E_V
=
left(
sum_PV_P
(C_P-ar c_P)^2
ight)^{1/2}.
]

This is the unnormalised volume-weighted (L^2)-type norm derived in the canonical mathematics.

Its units are those of concentration times (sqrt{	ext{volume}}), rather than concentration itself.

That is acceptable because it is an error norm.

Any constant normalization would not alter the observed convergence order.

**Independent result:** VERIFIED.

---

# 6. Independent reproduction of spatial errors

The persisted values are:

[
E_{25}=1.832317223290	imes10^{-2},
]

[
E_{50}=4.621922061067	imes10^{-3},
]

[
E_{100}=1.158126274643	imes10^{-3},
]

[
E_{200}=2.896983800097	imes10^{-4},
]

[
E_{400}=7.243504635747	imes10^{-5}.
]

I independently recomputed the values from the stated FV equations and analytical cell-average reference.

The results agree to essentially the displayed precision.

The independently recomputed orders are:

[
1.9871044572,
]

[
1.9967003679,
]

[
1.9991690419,
]

[
1.9997918245.
]

The repository reports:

[
1.987104,quad
1.996700,quad
1.999169,quad
1.999792.
]

Therefore the reported spatial orders are correct.

The data show a clear asymptotic regime approaching:

[
oxed{p_xapprox2}.
]

This is strong evidence for approximately second-order **global steady volume-weighted convergence for this particular aligned five-layer benchmark**.

---

# 7. Interface-order paradox

This is not a contradiction.

The analytical consistency study correctly says the interface flux evaluated from the exact solution is generically only

[
J_I^h-J_I=O(Delta r).
]

I independently confirmed this.

For example, at the (r=2) interface, the resistance-weighted flux error from exact cell averages decreases approximately by a factor of two when the mesh is halved.

So the local interface flux really is first-order.

Yet the global steady FV solution error behaves as approximately second-order.

These statements can coexist because the **steady discrete equations enforce exact conservation of the integrated flux**.

For the source-driven steady benchmark:

[
dot N_{m gen}
=
rac{4pi S_0r_1^3}{3}.
]

At every source-free control volume, the steady balance requires equality of incoming and outgoing amount rate.

Consequently the numerical amount rate crossing every source-free face is forced to the globally conserved steady rate.

The interface conductance error therefore primarily changes the concentration drop required to support that flux.

Since the interface resistance itself is (O(Delta r)), an (O(Delta r)) resistance/flux reconstruction error can produce an (O(Delta r^2)) concentration-level perturbation in this special steady problem.

That mechanism is entirely compatible with:

[
	ext{local interface flux error}=O(Delta r)
]

and

[
	ext{global steady concentration error}=O(Delta r^2).
]

Therefore I do **not** consider the observed second-order convergence suspicious or contradictory.

The repository's warning that the observed rate is benchmark-specific is appropriate.

---

# 8. Spatial convergence scope

The result establishes:

[
oxed{
E_V=O(Delta r^2)
}
]

for the executed aligned benchmark.

It does **not** establish:

- second-order convergence for non-aligned interfaces;
- arbitrary diffusivity contrasts;
- arbitrary layer thicknesses;
- absorbing Dirichlet boundaries;
- arbitrary source distributions;
- transient spatial convergence;
- universal second-order interface flux accuracy.

The repository does not overclaim these points.

**Independent result:** appropriate scope.

---

# 9. Temporal refinement

The temporal experiment fixes:

[
N=200
]

and integrates to:

[
t=0.1.
]

The requested timestep sequence is approximately divided by two at every refinement:

[
3.90605	imes10^{-5},
1.95303	imes10^{-5},
9.76513	imes10^{-6},
4.88257	imes10^{-6},
2.44128	imes10^{-6}.
]

The successive numerical-solution differences are:

[
4.481502492252	imes10^{-6},
]

[
2.241216677982	imes10^{-6},
]

[
1.120779441064	imes10^{-6},
]

[
5.603640282583	imes10^{-7},
]

[
2.801893399962	imes10^{-7}.
]

The reported orders are:

[
0.999700,quad
0.999780,quad
1.000066,quad
0.999962.
]

---

# 10. Independent temporal-order correction

## R3-m01 — MINOR

**Location:** `verification/fv_convergence.rs`, temporal-order calculation.

The code computes the observed order using:

[
p=
rac{log(E_k/E_{k+1})}{log2}.
]

However, the actual timestep is:

[
Delta t
=
rac{t_{m end}}
{lceil t_{m end}/Delta t_{m requested}ceil}.
]

Therefore the actual timestep ratios are not exactly 2 because of the ceiling operation.

I independently recomputed the orders using:

[
p_k
=
rac{
log(E_k/E_{k+1})
}{
log(Delta t_k/Delta t_{k+1})
}.
]

This gives approximately:

[
0.999982,
]

[
0.999921,
]

[
1.000066,
]

[
0.999997.
]

Thus the reported conclusion

[
oxed{p_t=1}
]

is unchanged.

**Consequence:** no scientific invalidation.

**Remediation:** use the actual timestep ratios in the reported order formula.

**Closure evidence:** regenerated temporal table using actual (Delta t), not requested (Delta t).

This is a reporting/precision issue only.

---

# 11. Temporal-order interpretation

The experiment compares successive numerical solutions rather than comparing to an exact transient analytical solution.

That is a valid **self-convergence/order study** when:

- the spatial discretisation is fixed;
- the spatial error is time-independent to leading order;
- the solutions are in the temporal asymptotic regime.

The five observed orders essentially equal one, providing strong evidence of first-order Forward Euler time convergence.

However, this is not an absolute temporal-error measurement.

It does not independently establish the error constant or absolute transient error relative to the exact continuum solution.

The repository currently describes this limitation appropriately.

**Independent result:** credible temporal-order evidence.

---

# 12. Stability during executed runs

The driver computes:

[
Delta t_{max}
=
min_P
rac{V_P}
{G_w+G_e},
]

with the appropriate centre and Robin-boundary forms.

For this benchmark I independently obtain approximately:

[
oxed{
Delta t_{max}
=
1.56242159175	imes10^{-4}.
}
]

The first requested step is approximately:

[
3.90605	imes10^{-5},
]

and the actual executed step is slightly smaller because of the ceiling operation.

All subsequent steps are smaller still.

The code also explicitly asserts:

[
Delta tleDelta t_{max}(1+10^{-12}).
]

Therefore every temporal refinement run satisfies the previously derived sufficient FV positivity/(ell_infty)-stability condition.

**Independent result:** VERIFIED.

---

# 13. Conservation evidence

The exact generation rate is:

[
oxed{
dot N_{m gen}
=
rac{4pi}{3}
=
4.1887902047863905ldots
}
]

The spatial study reports release rates around:

[
4.188790204786.
]

I independently reproduce the same value from the analytical global balance.

The steady residuals reported by the run span approximately:

[
2.7	imes10^{-15}
]

to

[
2.9	imes10^{-12}.
]

Those magnitudes are fully consistent with floating-point linear-solve/arithmetic error.

The transient residual remains below approximately:

[
4	imes10^{-10}.
]

The structure of the transient residual calculation is also correct for the explicit method: the inventory difference is divided by (Delta t), while the release term is evaluated from the old state, matching the flux actually used in the Forward Euler update.

As (Delta t) becomes smaller, subtraction of nearly equal inventories is followed by division by a smaller number, so absolute roundoff in the residual can increase.

Thus:

[
oxed{
	ext{increasing residual does not imply conservation drift}
}
]

for this experiment.

The current evidence is consistent with exact algebraic discrete conservation plus floating-point evaluation error.

**Independent result:** VERIFIED.

---

# 14. Exact versus floating-point conservation

The scheme itself is exactly conservative at the algebraic level because the same face conductance/flux appears with opposite signs in adjacent control volumes.

Therefore:

[
sum_P
left[
G_{P-1/2}(C_{P-1}-C_P)
+
G_{P+1/2}(C_{P+1}-C_P)
ight]
]

cancels internally.

Only the external Robin boundary remains.

This is mathematically distinct from the finite-precision residual printed by the program.

The project correctly makes this distinction.

---

# 15. Formal consistency analysis

The `TRISO-ACC-*` derivation was independently checked.

## Cell-average representation

The exact cell average is:

[
C_P
=
rac1{V_P}
int_{Omega_P}c,dV.
]

At the spherical volume centroid (r_P), the first central moment vanishes:

[
int_{Omega_P}(r-r_P)dV=0.
]

Consequently:

[
oxed{
C_P=c(r_P)+O(Delta r^2)
}
]

for smooth fields.

This derivation is correct.

---

# 16. Smooth same-material face consistency

The two-point centroid-to-centroid gradient has the expected second-order behaviour under the stated geometric symmetry assumptions.

The important subtlety is correctly identified:

[
oxed{
O(Delta r^2)	ext{ face-flux error alone does not automatically imply }
O(Delta r^2)	ext{ cell-divergence error}.
}
]

The cell divergence divides a difference of face errors by a cell volume of (O(Delta r)).

Therefore cancellation of the leading face-error field is needed.

The canonical derivation explicitly identifies this requirement rather than silently assuming it.

**Independent result:** mathematically sound under its stated conditional assumptions.

---

# 17. Interface consistency

The derivation gives:

[
J_I^h
=
J_I+O(Delta r)
]

generically for unequal diffusivities.

I independently verified that scaling using the benchmark's analytical cell averages.

This is important:

[
oxed{
	ext{conservative}

otRightarrow
	ext{second-order locally}.
}
]

The repository maintains that distinction correctly.

The executed second-order global steady result must therefore be understood as an **observed property of this benchmark**, not as a proof that the interface flux is second-order.

**Independent result:** VERIFIED.

---

# 18. Robin FV boundary consistency

For finite physical transfer coefficient (h>0), the resistance closure

[
J_R^h
=
rac{C_P-c_infty}
{delta r_R/D_5+1/h}
]

is correctly derived.

The analysis gives:

[
J_R^h-J_R=O(Delta r^2)
]

for fixed finite (h).

I independently checked the outer-flux scaling numerically.

Therefore the physical boundary release rate is second-order consistent in the finite-(h) benchmark.

The document also correctly distinguishes this from the pointwise residual in the outer control volume, which can remain only:

[
O(Delta r).
]

It also correctly refuses to transfer the finite-(h) result automatically to the absorbing Dirichlet limit.

**Independent result:** VERIFIED.

---

# 19. Global energy/convergence argument

The error equation

[
Vdot e=Ke-V	au
]

and energy identity

[
rac12rac{d}{dt}|e|_V^2
=
e^TKe-langle e,	auangle_V
]

are correct.

Since:

[
e^TKele0,
]

Cauchy-Schwarz gives:

[
rac{d}{dt}|e|_V
le
|	au|_V.
]

The resulting estimate

[
oxed{
|e(t)|_V
le
|e(0)|_V+
int_0^t|	au(s)|_V,ds
}
]

is correct.

The interface residual analysis then yields the conservative bound:

[
oxed{
|	au_h|_V=O(Delta r^{1/2})
}
]

under the presently established local estimates, producing:

[
oxed{
|e(t)|_V
le
CtDelta r^{1/2}.
}
]

This is properly labelled as an upper bound, not an observed convergence prediction.

The subsequently observed (O(Delta r^2)) benchmark convergence does not contradict this bound.

**Independent result:** VERIFIED.

---

# 20. Why the observed (O(h^2)) result can exceed the provable bound

The present proof is deliberately conservative.

It treats the interface-adjacent local residual using worst-case scaling.

But the executed steady problem has additional structure:

- source is piecewise constant;
- interfaces are exactly aligned;
- the source region is exactly represented by cell volumes;
- steady flux is globally constrained by exact discrete conservation;
- the outer Robin resistance is explicitly represented;
- the analytical solution has simple (A+B/r) shell structure.

Thus the worst-case local residual bound need not be sharp.

This is a standard distinction:

[
oxed{
	ext{provable upper bound}

eq
	ext{observed asymptotic convergence rate}.
}
]

The repository's present wording is appropriately cautious.

---

# 21. Execution provenance

## Independent result: VERIFIED

The CI run:

`36654437525`

has:

- correct workflow name;
- correct feature branch;
- exact head SHA `039095e...`;
- successful checkout;
- successful compilation;
- successful execution;
- successful artifact upload.

The workflow log itself contains the same numerical values stored in:

`verification/fv_convergence_results.txt`.

Therefore the persisted results were not merely manually invented after the fact.

The current driver source has the same blob SHA as the driver used by the run.

This is strong execution provenance.

---

# 22. Reproducibility

## R3-m02 — MINOR

**Location:** `.github/workflows/fv-convergence.yml`, `verification/fv_convergence.rs`.

The workflow gives the executable command:

```
rustc -O verification/fv_convergence.rs -o /tmp/fv_convergence
/tmp/fv_convergence
```

and the benchmark/results file records the mesh and parameter sequences.

However, the workflow does not pin a Rust toolchain version. It uses whatever `rustc` is available on `ubuntu-latest`.

Therefore future reruns may use a different compiler/runtime environment.

This is unlikely to affect the present double-precision numerical result materially, but it weakens strict reproducibility.

**Required remediation:** pin a Rust toolchain or record the exact compiler version in the execution manifest.

**Closure evidence:** workflow run with explicitly recorded `rustc --version`.

---

# 23. Release-rate evidence scope

## R3-m03 — MINOR

**Location:** `verification/fv_convergence.rs`, `spatial()` and `integrate()`.

The program explicitly calculates the steady release rate:

[
dot N_R=G_R C_{m outer},
]

but the temporal refinement study does not report a time-resolved release-rate convergence curve.

The release rate appears inside the transient conservation residual calculation, but it is not itself used as a temporal convergence observable.

This is not a problem for the current stated claim of **first-order temporal concentration convergence**.

It simply means the study has not yet independently established first-order temporal convergence of the transient release-rate observable.

**Required remediation:** none for the current gate.

**Future evidence:** a release-rate/time-series temporal refinement study if release-rate convergence is needed as its own observable.

---

# 24. Status of FTCS finding R2-D01

## R2-D01 remains OPEN

The previous deterministic discrete audit identified:

**R2-D01 — MAJOR**

concerning the retained FTCS Robin boundary.

The current FV study does **not** close it.

The present study concerns the conservative FV scheme.

It therefore cannot establish the required FTCS Robin grid-refinement evidence.

The correct status remains:

[
oxed{	ext{R2-D01 OPEN}}
]

and it remains isolated from the canonical FV accuracy/convergence route.

To close R2-D01, a dedicated FTCS Robin benchmark must be run using the actual FTCS scheme, with:

- fixed analytical Robin benchmark;
- mesh refinement;
- appropriate error quantity;
- actual observed order;
- explicit comparison against the corrected (O(Delta r)) local boundary-consistency analysis.

No such experiment was found in the current executed FV evidence.

---

# 25. Equation-register status

The canonical register has been updated consistently with the newer ACC/FV material.

No mathematical collision in the reviewed FV ranges was found to invalidate the derivation.

The previously repaired overlapping FV IDs are now separated.

The remaining bookkeeping weakness is that the register is very large and some grouped ranges make individual equation lookup cumbersome, but no correctness defect follows from this.

---

# 26. Scope discipline

The project correctly keeps these tracks separate:

[
oxed{
	ext{continuous analytical model}
leftrightarrow
	ext{deterministic FV reference}
}
]

versus

[
oxed{
	ext{analytical/deterministic reference}
leftrightarrow
	ext{production WOS}
}.
]

The FV convergence run does **not** claim to validate WOS.

The README explicitly preserves this distinction.

No scope violation found.

---

# 27. Independently reproduced numerical results

### Steady source generation

[
oxed{
dot N_{m gen}
=
4.1887902047863905
}
]

### Spatial errors

| (N) | Independently reproduced (E_V) |
|---:|---:|
| 25 | (1.83231722328999	imes10^{-2}) |
| 50 | (4.62192206106726	imes10^{-3}) |
| 100 | (1.15812627463269	imes10^{-3}) |
| 200 | (2.89698380001099	imes10^{-4}) |
| 400 | (7.24350463503003	imes10^{-5}) |

The tiny differences from the persisted last digits are floating-point evaluation-order differences.

### Spatial orders

[
oxed{
1.98710446,;
1.99670037,;
1.99916904,;
1.99979182
}
]

### Temporal orders using actual timestep ratios

[
oxed{
0.99998205,;
0.99992059,;
1.00006614,;
0.99999750
}
]

### Stability limit

[
oxed{
Delta t_{max}
approx1.56242159175	imes10^{-4}
}
]

### Conservation

Steady release:

[
oxed{
4.188790204786ldots
}
]

matching the exact generation rate.

The persisted conservation residual ranges are consistent with floating-point evaluation.

---

# 28. Findings summary

## MAJOR

**None affecting the FV gate.**

## MODERATE

**None affecting the FV gate.**

## MINOR

### R3-m01 — Temporal order calculation uses nominal factor 2 rather than actual timestep ratios

Conclusion remains (p_t=1).

### R3-m02 — Rust toolchain is not pinned

Execution is reproducible now, but strict future reproducibility is weaker than it could be.

### R3-m03 — Transient release-rate convergence is not independently measured

This does not affect the current concentration-based temporal-order claim.

---

# 29. Remaining limitations

These are limitations, not failures:

### Not established

- universal second-order convergence for arbitrary discontinuous-(D) meshes;
- non-interface-aligned convergence;
- absorbing Dirichlet boundary convergence;
- arbitrary parameter robustness;
- transient spatial convergence;
- transient release-rate convergence;
- WOS agreement.

### Still correctly deferred

- R2-D01 FTCS Robin closure;
- production WOS validation;
- full deterministic ↔ WOS comparison.

---

# 30. Final gate

# **PASS FOR DETERMINISTIC REFERENCE / WOS COMPARISON**

The current deterministic FV route is sufficiently credible to serve as the numerical reference for the next verification stage.

The important evidence chain is now:

[
oxed{
	ext{continuous PDE}
ightarrow
	ext{conservative FV equations}
ightarrow
	ext{implemented FV driver}
ightarrow
	ext{successful CI execution}
ightarrow
	ext{independently reproduced convergence}
}
]

with:

[
oxed{
p_xapprox2
}
]

for the tested aligned steady five-layer benchmark,

[
oxed{
p_tapprox1
}
]

for explicit Euler temporal self-convergence, and

[
oxed{
	ext{discrete conservation verified to floating-point scale}.
}
]

This is sufficient to move the deterministic reference into the subsequent **deterministic ↔ analytical ↔ WOS verification stage**.

It does **not** constitute WOS validation.

The next independent verification should therefore address the deterministic reference against the production WOS process, while keeping the open FTCS finding R2-D01 separate.
