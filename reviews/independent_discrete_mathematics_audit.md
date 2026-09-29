# Independent Review — Deterministic Discrete TRISO Mathematics

**Branch:** `ray/triso-foundation`  
**Reviewed commit:** `34d450649db4fe70e9c8ade8d4222d63f3d3fa1f`  
**Scope:** Independent deterministic discrete-mathematics milestone review  
**Decision:** **PASS FOR ACCURACY / CONVERGENCE STUDY**

## Executive determination

The current deterministic discrete mathematical foundation is sufficiently correct and internally consistent to proceed into formal accuracy/convergence analysis.

The review found:

- **0 CRITICAL**
- **1 MAJOR**
- **1 MODERATE**
- **2 MINOR**

The canonical five-layer finite-volume formulation is mathematically sound enough to proceed. The main outstanding technical defect is isolated to the claimed boundary consistency order of the retained FTCS Robin benchmark.

---

# 1. Most important finding

## R2-D01 — MAJOR
### FTCS Robin boundary is not generically (O(\Delta r^2)) as currently claimed

**Location**

`docs/triso/00_consolidated_mathematical_foundation.md`

Section 16.9, equations:

- `TRISO-DIS-452`–`TRISO-DIS-456`

In particular the statement:

[
\boxed{
\text{surface local consistency}
=
O(\Delta t)+O(\Delta r^2)
}
]

is not justified by the derivation given.

### Independent derivation

The ghost relation is constructed from

[
-D\frac{C_{N+1}-C_{N-1}}{2\Delta r}
=
hC_N.
]

Let

[
\beta=\frac{h}{D}.
]

The resulting ghost value is

[
C_{N+1}^{g}
=
C_{N-1}-2\beta\Delta r\,C_N.
]

For the exact smooth solution satisfying

[
c'(R)=-\beta c(R),
]

Taylor expansion gives:

[
c(R+\Delta r)
=
c(R)-\beta\Delta r\,c(R)
+\frac{\Delta r^2}{2}c''(R)
+\frac{\Delta r^3}{6}c'''(R)
+O(\Delta r^4),
]

whereas the ghost construction gives

[
c^g(R+\Delta r)
=
c(R)-\beta\Delta r\,c(R)
+\frac{\Delta r^2}{2}c''(R)
-\frac{\Delta r^3}{6}c'''(R)
+O(\Delta r^4).
]

Therefore

[
c^g(R+\Delta r)-c(R+\Delta r)
=
-\frac{\Delta r^3}{3}c'''(R)
+O(\Delta r^4).
]

So the ghost value is only accurate to

[
O(\Delta r^3).
]

But that ghost value is inserted into the second-derivative stencil, where it is divided by

[
\Delta r^2.
]

Hence the resulting boundary approximation to the spherical Laplacian contains a generic error

[
O(\Delta r).
]

So the document's current argument

> “the Robin derivative is second-order, therefore the complete surface update is second-order”

is incomplete.

### Consequence

The FTCS benchmark's claimed boundary consistency order is currently overstated.

This does **not** invalidate:

- the continuous Robin condition;
- the ghost algebra;
- the centre stencil;
- the finite-volume five-layer formulation.

But it does matter before using the FTCS solution as a formal second-order spatial convergence benchmark.

### Remediation

Replace the current boundary-order claim with a direct truncation-error derivation of the **complete boundary PDE closure**.

Then determine empirically/analytically whether the resulting global scheme is:

- first-order;
- second-order;
- or has some special boundary superconvergence.

Do not assume second order from the boundary derivative alone.

### Closure evidence

A Taylor expansion of the complete surface operator, followed by an actual grid-refinement study.

**Downstream:** FTCS accuracy/convergence work.

---

# 2. Finite-volume unknown/mesh definition is not sufficiently frozen for formal accuracy

## R2-D02 — MODERATE

**Location**

`docs/triso/00_consolidated_mathematical_foundation.md`

Section 18, particularly:

- `TRISO-FV-118`
- `TRISO-FV-124`–`TRISO-FV-128`
- `TRISO-FV-141`
- `TRISO-FV-177`
- `TRISO-FV-180`

### Issue

The derivation defines

[
C_P
=
\frac1{V_P}
\int_{r_w}^{r_e}c(r)\,4\pi r^2dr,
]

so (C_P) is an **exact cell-average concentration**.

But the face approximation subsequently uses

[
\frac{C_E-C_P}{r_E-r_P},
]

as though (C_P) and (C_E) were point values located at (r_P) and (r_E).

The derivation never fully freezes:

[
\boxed{\text{what exactly is }r_P?}
]

Possible interpretations include:

- arithmetic midpoint of the cell;
- volume centroid;
- some prescribed cell-centre coordinate;
- point value rather than cell average.

These choices are not equivalent for spherical cells.

### Consequence

The conservative balance remains valid, but a formal accuracy analysis cannot yet start from an unambiguous discrete unknown definition.

This is particularly relevant for:

- same-material face gradients;
- interface half-cell distances;
- outer Robin half-cell distance;
- proving spatial order.

### Remediation

Explicitly define the numerical representation, for example:

[
C_P=\text{cell average}
]

with a precisely defined representative coordinate (r_P), or instead define

[
C_P=c(r_P)
]

as a point-centred method.

Then state the face geometry:

[
r_{P+1/2},\qquad r_P,\qquad
r_{P+1}-r_P,
]

and their relationships.

### Closure evidence

One complete mesh-definition subsection from which every (G_{P+1/2}), (delta r_P), and boundary distance can be reconstructed unambiguously.

**Downstream:** formal spatial-order analysis.

---

# 3. Material-interface treatment: mathematically sound, but accuracy remains unproved

## R2-D03 — NOTE

**Location**

`TRISO-DIS-333`–`TRISO-DIS-345` and `TRISO-FV-129`–`TRISO-FV-146`.

I independently checked the resistance construction:

[
J_e
=
\frac{C_P-C_E}
{\delta r_P/D_P+\delta r_E/D_E}.
]

The resulting effective diffusivity

[
D_e^{\rm eff}
=
\frac{\delta r_P+\delta r_E}
{\delta r_P/D_P+\delta r_E/D_E}
]

is correct.

For equal distances it reduces to

[
\boxed{
D_e^{\rm eff}
=
\frac{2D_PD_E}{D_P+D_E}
}.
]

The important distinction is that this is a **discrete two-point flux approximation**, not the exact continuous spherical resistance of a finite radial interval.

For example, the exact source-free spherical resistance between two radii contains

[
\frac1{4\pi D}
\left(
\frac1{r_a}-\frac1{r_b}
\right),
]

whereas the two-point FV face construction uses a local face-area/centre-distance approximation.

That is perfectly legitimate for a finite-volume scheme, but it is why accuracy must be established separately.

### Status

Conservation: **verified**.

Interface flux continuity: **verified**.

Harmonic/resistance weighting: **verified**.

Global spatial order across discontinuous (D): **not yet established**.

No blocking defect.

---

# 4. Exact finite-volume conservation is correctly derived

## Independently verified

The control-volume balance

[
V_P\frac{dC_P}{dt}
=
A_wJ_w-A_eJ_e+S_PV_P
]

is correct.

Summing all cells cancels shared internal face fluxes pairwise.

The result

[
\boxed{
\frac{d}{dt}
\left(
\sum_P V_PC_P
\right)
=
\sum_P S_PV_P
-
G_R(C_{M-1}-c_\infty)
}
]

is correct for the stated Robin boundary.

This is one of the strongest parts of the discrete derivation.

---

# 5. Centre finite-volume treatment is correct

## Independently verified

For the central cell,

[
r_w=0
]

so

[
A_w=4\pi r_w^2=0.
]

Therefore there is no artificial flux through the origin.

The centre balance naturally becomes

[
V_0\frac{dC_0}{dt}
=
G_{1/2}(C_1-C_0)+S_0V_0.
]

This is conceptually distinct from the factor-six nodal FTCS construction, as it should be.

No defect found.

---

# 6. Kernel source integration is correct

For an interface-aligned mesh,

[
\sum_{P\in\text{kernel}}V_P
=
\frac{4\pi r_1^3}{3},
]

so

[
\boxed{
\sum_P S_PV_P
=
\frac{4\pi S_0r_1^3}{3}
}.
]

This exactly matches the continuous generation rate.

The alignment assumption needs to remain explicit.

No defect found.

---

# 7. Cell-centred Robin FV closure is correct

## Independently verified

The derivation correctly separates:

- cell-centre concentration (C_P);
- physical surface concentration (C_R);
- external concentration (c_\infty).

The two resistances are:

[
\frac{\delta r_R}{D_5}
]

and

[
\frac1h.
]

Thus

[
\boxed{
J_R
=
\frac{C_P-c_\infty}
{\delta r_R/D_5+1/h}
}
]

and

[
\boxed{
h_{\rm eff}
=
\left(
\frac{\delta r_R}{D_5}
+
\frac1h
\right)^{-1}.
}
]

The limits were also checked:

[
h\to0
\Rightarrow
J_R\to0,
]

and

[
h\to\infty
\Rightarrow
h_{\rm eff}\to\frac{D_5}{\delta r_R}.
]

This is mathematically consistent.

---

# 8. Global finite-volume matrix is correct

## Independently verified

The assembled system

[
\boxed{
\mathbf V\dot{\mathbf C}
=
\mathbf K\mathbf C
+
\mathbf V\mathbf S
+
\mathbf b_\infty
}
]

has the expected structure.

I independently checked:

[
K_{P,P-1}=G_{P-1/2},
]

[
K_{P,P}=-(G_{P-1/2}+G_{P+1/2}),
]

[
K_{P,P+1}=G_{P+1/2}.
]

The outer row correctly includes

[
-(G_{M-3/2}+G_R).
]

The important structural result

[
\boxed{\mathbf K=\mathbf K^T}
]

is correct.

Therefore

[
\mathbf L=\mathbf V^{-1}\mathbf K
]

is self-adjoint under

[
\langle x,y\rangle_V=x^T\mathbf Vy.
]

No defect found.

---

# 9. Conductance quadratic form and spectrum are correct

## Independently verified

The derivation gives

[
\boxed{
x^TKx
=
-\sum_PG_{P+1/2}(x_{P+1}-x_P)^2
-G_Rx_{M-1}^2
}.
]

Therefore

[
x^TKx\le0.
]

For

[
G_R>0,
]

the matrix is negative definite.

For a closed Neumann boundary, the expected constant zero mode appears.

Thus the generalized eigenproblem

[
Kx=\lambda Vx
]

satisfies

[
\boxed{\lambda\le0}
]

and, with positive external loss,

[
\boxed{\lambda<0}.
]

This is a strong and useful mathematical foundation for the later time-discretisation analysis.

---

# 10. Finite-volume explicit-Euler bound is correctly characterized as sufficient

The derived bound

[
\boxed{
\Delta t
\le
\min
\left\{
\frac{V_0}{G_{1/2}},
\min_P
\frac{V_P}{G_{P-1/2}+G_{P+1/2}},
\frac{V_{M-1}}{G_{M-3/2}+G_R}
\right\}
}
]

does ensure non-negative update coefficients.

Together with the substochastic row sums, this gives

[
\|\mathbf A_{\rm FV}\|_\infty\le1
]

and consequently

[
\rho(\mathbf A_{\rm FV})\le1.
]

This is correctly described as a **sufficient monotonicity/\ell_\infty-stability condition**, not as the necessary-and-sufficient spectral stability boundary.

No defect found.

---

# 11. FTCS interior derivation is correct

The derivation

[
C_i^{j+1}
=
\mathrm{Fo}
\left(1-\frac1i\right)C_{i-1}^j
+
(1-2\mathrm{Fo})C_i^j
+
\mathrm{Fo}
\left(1+\frac1i\right)C_{i+1}^j
+
S_i^j\Delta t
]

is correct for a smooth constant-(D), uniform-grid interior node.

The stated interior truncation

[
O(\Delta t)+O(\Delta r^2)
]

is also correct **away from centre, interfaces, and boundaries**.

The factor-six centre formulation was independently verified as second-order spatially.

The original FTCS stability correction is also now properly limited to:

- coefficient positivity;
- \ell_infty non-amplification;
- sufficient spectral-radius bound.

No issue there.

---

# 12. Stable-ID bookkeeping

## R2-D04 — MINOR

**Location:** `docs/triso/13_canonical_equation_register.md`.

The register is much cleaner than earlier versions, but the FV ranges are not completely non-overlapping.

For example, the register assigns:

- `TRISO-FV-181–185` to ordinary-cell positivity; and
- `TRISO-FV-181–185` again to the outer Robin closure.

The underlying equations themselves are different.

This does not affect mathematics, but it violates the stated “single source of truth” purpose of the stable IDs.

### Remediation

Assign unique ranges to the outer-boundary and positivity equations.

---

# 13. Repository status consistency

## R2-D05 — MINOR

**Location:** `docs/triso/00_consolidated_mathematical_foundation.md`, final status text.

The document still contains an obsolete statement equivalent to:

> “Review 1 is not yet the next step.”

That is now historically incorrect because the independent continuous-mathematics audit has already been completed and the current task is the discrete milestone.

This is bookkeeping rather than a scientific defect.

---

# 14. Independently verified discrete results

I independently verified the following:

### FTCS

[
C_i^{j+1}
=
\mathrm{Fo}
\left(1-\frac1i\right)C_{i-1}
+
(1-2\mathrm{Fo})C_i
+
\mathrm{Fo}
\left(1+\frac1i\right)C_{i+1}
+
S_i\Delta t.
]

✓ Correct.

### Centre

[
\mathcal L[c](0)
=
3c_{rr}(0)
]

and

[
\mathcal L_h[c](0)
=
\frac{6(C_1-C_0)}{\Delta r^2}.
]

✓ Correct.

### Interface resistance

[
J_e
=
\frac{C_P-C_E}
{\delta r_P/D_P+\delta r_E/D_E}.
]

✓ Correct as a two-point discrete approximation.

### Equal-spacing harmonic mean

[
D_e^{\rm eff}
=
\frac{2D_PD_E}{D_P+D_E}.
]

✓ Correct.

### Finite-volume conservation

[
\frac{d}{dt}
\sum_PV_PC_P
=
\sum_PS_PV_P-\text{outer release}.
]

✓ Correct.

### Matrix symmetry

[
K=K^T.
]

✓ Correct.

### Weighted self-adjointness

[
\langle x,Ly\rangle_V
=
\langle Lx,y\rangle_V.
]

✓ Correct.

### Negative-definite Robin operator

[
G_R>0
\Rightarrow
K<0.
]

✓ Correct.

### Explicit-Euler sufficient bound

[
\Delta t
\le
\min_P
\frac{V_P}{\sum G_{\text{adjacent}}}
]

with appropriate centre/outer special cases.

✓ Correct as a sufficient coefficient-positivity condition.

---

# 15. What is *not* yet established

The project is right not to claim these yet:

- global spatial order for discontinuous (D);
- fully discrete convergence rate;
- exact spectral stability limit of explicit Euler;
- numerical grid/time convergence;
- interface convergence rate;
- comparison against the analytical five-layer transient solution.

Those belong to the next stage.

The key distinction is:

[
\boxed{
\text{the discrete equations are now closed}
}
]

but

[
\boxed{
\text{their accuracy has not yet been established}
}.
]

That is exactly where the project should be.

---

# Final assessment

### Findings

**1 MAJOR**

- **R2-D01:** FTCS Robin boundary's complete surface truncation error is incorrectly claimed to be (O(\Delta r^2)); the current derivation only proves the Robin derivative approximation is second-order.

**1 MODERATE**

- **R2-D02:** FV cell-average versus cell-centre representation is not sufficiently explicit for a formal accuracy proof.

**1 MINOR**

- **R2-D04:** FV stable-ID ranges overlap.

**1 MINOR**

- **R2-D05:** stale Review-1 workflow wording remains.

No CRITICAL findings.

### Gate

# **PASS FOR ACCURACY / CONVERGENCE STUDY**

The **five-layer conservative finite-volume formulation is sufficiently mathematically reliable to proceed into formal accuracy and convergence analysis**.

Main Research should **not redesign the FV scheme before that study**. The right next step is to make the mesh/unknown definition explicit, then perform the consistency/order analysis.

The FTCS boundary issue should be corrected concurrently, but it does **not block the canonical FV route**.

The next mathematical stage should therefore be:

[
\boxed{
\text{finite-volume consistency}
\rightarrow
\text{interface consistency}
\rightarrow
\text{boundary consistency}
\rightarrow
\text{stability}
\rightarrow
\text{convergence}
}
]

rather than another derivation of the basic FV coefficients.
