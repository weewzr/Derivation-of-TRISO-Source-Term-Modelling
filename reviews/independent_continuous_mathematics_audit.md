# Independent Review — Current Continuous TRISO Mathematical Foundation

**Branch:** `ray/triso-foundation`  
**Reviewed commit:** `c77cd9fadb127455928fc94b3dd445b9b9a1cf61`  
**Scope:** Continuous mathematical foundation only  
**Decision:** **PASS FOR DISCRETISATION**

---

## 1. Executive determination

The current continuous mathematical foundation is sufficiently correct, explicit, and internally consistent to permit progression into detailed discrete derivation.

I found:

- **0 CRITICAL findings**
- **0 MAJOR findings**
- **2 MODERATE findings**
- **2 MINOR findings**

The important point is that neither MODERATE finding invalidates the continuous PDE, analytical solution, or five-layer eigenvalue framework. They concern theorem/provenance precision and equation-ID bookkeeping.

The core mathematical chain is now substantially sound:

\[
\text{conservation}
\rightarrow
\text{Fick}
\rightarrow
\text{variable-}D\text{ PDE}
\rightarrow
\text{spherical reduction}
\rightarrow
\text{layer/interface conditions}
\rightarrow
\text{analytical solutions}
\rightarrow
\text{five-layer eigenproblem}
\rightarrow
\text{global orthogonality}
\rightarrow
\text{modal projection}.
\]

---

# 2. Conservation and Fickian formulation

## Independent result: VERIFIED

The project begins from

\[
N_V(t)=\int_V c\,dV
\]

and obtains

\[
\frac{\partial c}{\partial t}
+
\nabla\cdot\mathbf J
=
S.
\]

With

\[
\mathbf J=-D\nabla c,
\]

substitution gives

\[
\frac{\partial c}{\partial t}
-\nabla\cdot(D\nabla c)
=
S,
\]

hence

\[
\boxed{
\frac{\partial c}{\partial t}
=
\nabla\cdot(D\nabla c)+S
}.
\]

The sign convention is correct for outward-positive flux.

The units are also consistent:

\[
[c]=\mathrm{mol\,m^{-3}},
\qquad
[D]=\mathrm{m^2\,s^{-1}},
\qquad
[S]=\mathrm{mol\,m^{-3}\,s^{-1}},
\]

and

\[
[-D\nabla c]
=
\mathrm{mol\,m^{-2}\,s^{-1}}.
\]

Importantly, the derivation correctly keeps \(D\) inside the divergence until constant-within-layer assumptions are explicitly introduced.

That is essential for discontinuous multilayer diffusivity.

---

# 3. Spherical reduction

## Independent result: VERIFIED

The derivation correctly establishes

\[
\nabla c
=
\mathbf e_r\frac{\partial c}{\partial r}
\]

under spherical symmetry and obtains

\[
\boxed{
\frac{\partial c}{\partial t}
=
\frac1{r^2}
\frac{\partial}{\partial r}
\left(
r^2D(r,t)\frac{\partial c}{\partial r}
\right)
+
S(r,t)
}.
\]

For constant \(D_i\) within layer \(i\),

\[
\frac{\partial c_i}{\partial t}
=
D_i
\left(
c_{i,rr}
+
\frac2r c_{i,r}
\right)
+
S_i.
\]

The product-rule expansion is explicitly shown rather than hidden.

The restriction that the expanded equation is only a **within-layer equation** is correctly stated.

No defect found.

---

# 4. Centre treatment

## Independent result: VERIFIED

The derivation establishes

\[
c_r(0,t)=0
\]

through even radial extension.

The apparently singular quantity

\[
\frac{2}{r}c_r
\]

is correctly treated by a limit.

For a sufficiently smooth radial field,

\[
\lim_{r\to0}\frac{c_r}{r}=c_{rr}(0,t),
\]

so

\[
\boxed{
\lim_{r\to0}
\left(
c_{rr}+\frac2r c_r
\right)
=
3c_{rr}(0,t)
}.
\]

This is mathematically correct.

The document also correctly separates this continuous result from the later discrete factor-of-six construction.

No defect found.

---

# 5. Source and reaction mathematics

## Independent result: VERIFIED, with conditional physical models

The bookkeeping equation

\[
S_i
=
S_{i,\mathrm{gen}}
+
S_{i,\mathrm{other}}
+
S_{i,\mathrm{release}}
-
S_{i,\mathrm{decay}}
-
S_{i,\mathrm{trap}}
\]

is a valid definition provided each term is understood as a positive magnitude with the displayed sign convention.

The decay derivation correctly gives

\[
\frac{dN}{dt}
=
-\lambda_dN
\]

and hence

\[
\boxed{
S_{i,\mathrm{decay}}
=
\lambda_{d,i}c_i
}
\]

with the minus sign entering the PDE.

The dimensional check is correct.

The trapping/release formulation is appropriately kept abstract:

\[
S_{\mathrm{release}}-S_{\mathrm{trap}},
\]

without inventing an unsupported constitutive law.

That is the correct epistemic treatment.

Most importantly, the project now cleanly distinguishes:

### Problem A — continuing generation

\[
c(r,0)=0,
\qquad
S_1=S_0.
\]

### Problem B — initial inventory release

\[
c_1(r,0)=c_0,
\qquad
c_{2..5}(r,0)=0,
\qquad
S_i=0.
\]

These are genuinely different initial-value problems and are not being conflated.

No mathematical defect found.

---

# 6. Centre/interface/outer conditions

## Independent result: VERIFIED

### Centre

\[
c_1'(0,t)=0
\]

is correct.

### Interfaces

The zero-thickness conservation argument correctly produces

\[
J_k^-=J_k^+
\]

for zero interfacial storage and no interfacial production.

Using Fick:

\[
\boxed{
-D_kc_k'(r_k^-)
=
-D_{k+1}c_{k+1}'(r_k^+)
}.
\]

This is correctly identified as a **conservation consequence**.

The project correctly does not claim that concentration continuity follows from conservation.

Instead it treats

\[
c_k(r_k,t)=c_{k+1}(r_k,t)
\]

as an ideal-interface assumption.

Likewise it correctly distinguishes:

\[
c_{k+1}=K_kc_k
\]

for partitioning and an additional interfacial-resistance constitutive law as separate physical models.

### Outer boundary

The Robin condition

\[
\boxed{
-D_5c_5'(R,t)
=
h[c_5(R,t)-c_\infty(t)]
}
\]

has the correct sign and dimensions.

The two benchmark choices remain separate:

- Robin for the deterministic source-driven benchmark;
- absorbing Dirichlet for the production WOS release benchmark.

No defect found.

---

# 7. Homogeneous analytical benchmark

## Independent result: VERIFIED

The steady equation is correctly reduced to

\[
\frac{d}{dr}(r^2w')=-\frac{S_0}{D}r^2.
\]

The first integration gives

\[
r^2w'
=
-\frac{S_0r^3}{3D}+A.
\]

Centre regularity gives

\[
A=0.
\]

Second integration gives

\[
w=B-\frac{S_0r^2}{6D}.
\]

The Robin boundary then gives

\[
B=
\frac{S_0R}{3h}
+
\frac{S_0R^2}{6D}.
\]

Hence:

\[
\boxed{
w(r)
=
\frac{S_0R}{3h}
+
\frac{S_0}{6D}(R^2-r^2)
}.
\]

I independently checked:

- differentiation;
- centre boundedness;
- Robin boundary;
- dimensions;
- positivity/sign structure;
- global generation.

The global steady balance

\[
\frac{4\pi R^3S_0}{3}
=
4\pi R^2hw(R)
\]

correctly gives

\[
w(R)=\frac{S_0R}{3h}.
\]

The analytical solution agrees with that independently derived balance.

No defect found.

---

# 8. Homogeneous transient/eigenproblem

## Independent result: VERIFIED

The transformation

\[
v=c-w
\]

correctly removes the steady source contribution.

The resulting PDE is

\[
v_t
=
D
\left(
v_{rr}+\frac2r v_r
\right).
\]

The Robin condition becomes

\[
-Dv_r(R,t)=hv(R,t),
\]

and the initial condition is

\[
v(r,0)=-w(r).
\]

For

\[
v=\phi(r)T(t),
\]

the separation produces

\[
\boxed{
\phi''
+\frac2r\phi'
+k^2\phi=0
}
\]

with

\[
\Lambda=Dk^2.
\]

The dimensions are correctly separated:

\[
[k]=\mathrm{m^{-1}},
\qquad
[\Lambda]=\mathrm{s^{-1}}.
\]

The transformation

\[
u=r\phi
\]

was independently checked and indeed gives

\[
\boxed{
u''+k^2u=0
}.
\]

Centre regularity correctly eliminates the cosine branch.

The regular mode may be written

\[
\phi(r)
=
C\frac{\sin(kr)}{kr}.
\]

The Robin characteristic condition is correctly derived as

\[
\boxed{
\sin\mu-\mu\cos\mu
=
\mathrm{Bi}\sin\mu
}
\]

with

\[
\mu=kR,
\qquad
\mathrm{Bi}=\frac{hR}{D}.
\]

Therefore:

\[
\boxed{
\mu\cot\mu=1-\mathrm{Bi}
}
\]

when \(\sin\mu\neq0\).

The project explicitly returns to the undivided condition and checks the excluded case:

\[
\sin\mu=0
\]

which gives

\[
-\mu\cos\mu=0.
\]

For positive integer multiples of \(\pi\), this is not satisfied.

Therefore the division does not lose positive eigenvalues.

The decay rate

\[
\boxed{
\Lambda_n
=
D\frac{\mu_n^2}{R^2}
}
\]

is dimensionally correct.

No defect found.

---

# 9. Homogeneous orthogonality and projection

## Independent result: VERIFIED

The self-adjoint form

\[
-\frac{d}{dr}
\left(
r^2\phi_n'
\right)
=
k_n^2r^2\phi_n
\]

correctly identifies

\[
p(r)=r^2,
\qquad
w(r)=r^2.
\]

The direct Lagrange-identity derivation is valid.

The boundary term at \(R\) vanishes because both modes obey the same Robin condition.

The centre term vanishes because regular modes remain bounded while \(r^2\to0\).

Therefore

\[
\boxed{
\int_0^Rr^2\phi_m\phi_n\,dr=0,
\qquad m\neq n.
}
\]

The projection coefficient

\[
\boxed{
A_n
=
-
\frac{
\int_0^R r^2w\phi_n\,dr
}{
\int_0^R r^2\phi_n^2\,dr
}
}
\]

is consistent with

\[
v(r,0)=-w(r).
\]

No defect found.

---

# 10. Five-layer steady TRISO solution

## Independent result: VERIFIED

For the kernel:

\[
c_1(r)
=
A_1-\frac{S_0r^2}{6D_1}.
\]

This is correct.

The source-free coating layers have:

\[
\boxed{
c_i(r)=A_i+\frac{B_i}{r},
\qquad i=2,\ldots,5.
}
\]

The total kernel generation is

\[
\boxed{
\dot N
=
\frac{4\pi S_0r_1^3}{3}
}.
\]

For all \(r>r_1\),

\[
\boxed{
J_r(r)
=
\frac{S_0r_1^3}{3r^2}
}.
\]

This gives the correct outward-positive flux and correct \(1/r^2\) decay.

The shell concentration drop is correctly obtained:

\[
c_i(r)-c_i(r_i)
=
\frac{S_0r_1^3}{3D_i}
\left(
\frac1r-\frac1{r_i}
\right).
\]

Hence

\[
B_i=\frac{S_0r_1^3}{3D_i}.
\]

The external resistance

\[
\boxed{
\mathcal R_h
=
\frac1{4\pi R^2h}
}
\]

has the correct dimensions, and the shell resistance

\[
\boxed{
\mathcal R_i
=
\frac1{4\pi D_i}
\left(
\frac1{r_{i-1}}-\frac1{r_i}
\right)
}
\]

is correct.

I also checked that the source-containing kernel is **not** incorrectly folded into the source-free shell resistance series.

No defect found.

---

# 11. Five-layer transient eigenproblem

## Independent result: VERIFIED

For the restricted constant-\(D_i\), reaction-free analytical benchmark,

\[
v_i=\phi_i e^{-\Lambda t}
\]

is mathematically appropriate.

The layer equation becomes

\[
\boxed{
\phi_i''
+\frac2r\phi_i'
+
k_i^2\phi_i
=
0
}
\]

with

\[
\boxed{
k_i^2=\frac{\Lambda}{D_i}
}.
\]

Thus one global temporal decay rate \(\Lambda\) produces layer-dependent spatial wavenumbers \(k_i\).

The \(u_i=r\phi_i\) transformation is correct.

The centre condition gives

\[
B_1=0,
\]

leaving nine coefficients.

The nine-dimensional coefficient vector is correctly counted.

The interface rows are correctly constructed from:

\[
u_i=u_{i+1}
\]

and

\[
D_i
\left(
u_i'-\frac{u_i}{r_i}
\right)
=
D_{i+1}
\left(
u_{i+1}'-\frac{u_{i+1}}{r_i}
\right).
\]

I independently checked the signs in the sine/cosine transformed flux coefficients:

\[
F_{ij}^{(s)}
=
D_i
\left(
k_i\cos(k_ir_j)-\frac{\sin(k_ir_j)}{r_j}
\right)
\]

and

\[
F_{ij}^{(c)}
=
D_i
\left(
-k_i\sin(k_ir_j)-\frac{\cos(k_ir_j)}{r_j}
\right).
\]

These are correct.

The outer Robin row is also correct:

\[
D_5u_5'
+
\left(
h-\frac{D_5}{R}
\right)u_5
=0.
\]

The explicit \(9\times9\) matrix therefore has the correct:

- unknown count;
- row count;
- interface structure;
- sign convention;
- coefficient definitions;
- outer-boundary structure.

The condition

\[
\boxed{
\det\mathbf M(\Lambda)=0
}
\]

is the correct non-trivial-mode condition.

No mathematical defect found.

---

# 12. Global multilayer orthogonality

## Independent result: VERIFIED

This was the most important part of the present review.

The project correctly starts from

\[
-\frac{d}{dr}
\left(
r^2D_i\phi_i'
\right)
=
\Lambda r^2\phi_i.
\]

Thus the piecewise coefficient is

\[
\boxed{
p(r)=r^2D(r)
}
\]

while the eigenvalue weight is

\[
\boxed{
w(r)=r^2
}.
\]

I independently reproduced the layerwise Lagrange identity.

At each interface, the boundary form is

\[
r_i^2D_i
\left(
\phi_i^{(m)}\phi_i^{(n)\prime}
-
\phi_i^{(n)}\phi_i^{(m)\prime}
\right).
\]

Using both:

\[
\phi_i=\phi_{i+1}
\]

and

\[
D_i\phi_i'
=
D_{i+1}\phi_{i+1}',
\]

the contribution from the inner side equals the contribution from the outer side.

Therefore:

\[
\mathcal B_i(r_i)-\mathcal B_{i+1}(r_i)=0.
\]

All four internal interface terms cancel.

At \(r=0\), regularity gives a vanishing contribution.

At \(r=R\), the common Robin condition gives a vanishing contribution.

Therefore:

\[
\boxed{
(\Lambda_m-\Lambda_n)
\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2\phi_i^{(m)}\phi_i^{(n)}\,dr
=
0.
}
\]

For distinct eigenvalues:

\[
\boxed{
\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2\phi_i^{(m)}\phi_i^{(n)}\,dr
=
0.
}
\]

Thus the project's result

\[
\boxed{w(r)=r^2}
\]

is independently verified.

This does **not** require equal diffusivities.

The discontinuity remains in \(p(r)=r^2D(r)\), while the weight remains \(r^2\).

No defect found.

---

# 13. Modal projection

## Independent result: VERIFIED CONDITIONALLY

Once the modal expansion is assumed,
\[
-c_{i,\mathrm{ss}}(r)
=
\sum_nA_n\phi_i^{(n)}(r),
\]

the global orthogonality immediately yields

\[
\boxed{
A_m
=
-
\frac{
\displaystyle
\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2c_{i,\mathrm{ss}}\phi_i^{(m)}\,dr
}{
\displaystyle
\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2[\phi_i^{(m)}]^2\,dr
}
}.
\]

The normalization/scaling argument is also correct.

So:

- orthogonality: **verified**;
- projection algebra: **verified**;
- expansion existence/completeness: still theorem-dependent.

That distinction is appropriate.

---

# 14. Completeness

## R1-MOD-01 — MODERATE

**Location:** `docs/triso/00_consolidated_mathematical_foundation.md`, §12.22; canonical register entries for `TRISO-ML-554` and the completeness discussion.

The current derivation is correct to avoid claiming that a standard **regular** Sturm–Liouville completeness theorem automatically proves completeness here.

The centre is a singular endpoint because

\[
p(r)=r^2D(r)\to0
\]

as \(r\to0\).

The coefficient \(D(r)\) is also piecewise discontinuous, with transmission conditions.

Therefore a specialized spectral theorem or self-adjoint operator argument is needed for a rigorous completeness statement.

However, this omission does **not** block discretisation.

There is mathematical literature specifically treating singular Sturm–Liouville operators and transmission problems, including completeness via self-adjoint/compact-resolvent constructions. DLMF explicitly distinguishes the regular and singular Sturm–Liouville settings and discusses eigenfunction expansions for self-adjoint operators; specialized transmission problems can establish completeness under additional operator hypotheses.

### Consequence

The current wording:

> orthogonality verified; completeness theorem-dependent

is scientifically defensible.

The only improvement needed before treating the modal series as a fully rigorous spectral solution is to identify the appropriate theorem and state its hypotheses.

### Remediation

Add a theorem-level provenance item covering the specific singular/transmission operator, or explicitly keep the series labelled:

\[
\text{FORMAL / CONDITIONALLY VERIFIED}.
\]

### Closure evidence

A source/theorem showing completeness for the stated operator domain and transmission conditions.

**This is not a blocker for discretisation.**

---

# 15. Canonical equation-ID bookkeeping

## R1-MOD-02 — MODERATE

**Location:** `docs/triso/13_canonical_equation_register.md`.

The repository now correctly contains the original 33-equation inventory, and I independently verified the raw LaTeX still contains:

- 30 `equation` environments;
- 1 `align` environment containing 3 labelled equations;

giving:

\[
\boxed{33}
\]

substantive labelled equations.

So the **33/33 coverage claim is verified**.

However, the stable-ID register still contains identifier collisions.

### Collision 1

`TRISO-GOV-008` is assigned to the original notebook's `eq:b:w-secondint`, but later is also assigned to the general conservative PDE.

These are different mathematical objects.

### Collision 2

`TRISO-BC-005` is assigned to the original notebook's `eq:lhopital`, but later is also used for the general outer Robin condition.

Again, these are different equations.

### Consequence

The mathematics is not invalidated, but the requirement that stable IDs provide an unambiguous bidirectional trace is violated.

### Remediation

Give the later replacement equations distinct IDs and preserve the original 33 mappings.

For example, the existing 100-series IDs can be used consistently for the new expansions.

### Closure evidence

A register in which every stable ID maps to exactly one mathematical object.

---

# 16. Original-note traceability

## Independent result: VERIFIED

The raw notebook is preserved at:

`notes/raw/TRISO Fuel Derivation Ray V1.tex`

and the branch includes it.

The original 33 labels remain represented.

The current derivation does not silently pretend the old notation was correct; it explicitly records repairs such as the \(k/\Lambda\) distinction.

That is good scientific provenance.

No substantive traceability defect beyond the stable-ID collisions above.

---

# 17. Derivational granularity

## Independent result: VERIFIED GENERALLY

The current derivation is much more explicit than the original notebook and now generally follows approximately one meaningful operation per displayed mathematical step.

Particularly strong sections include:

- conservation;
- spherical reduction;
- centre limit;
- steady homogeneous integration;
- \(v=c-w\);
- \(u=r\phi\);
- five-layer steady recursion;
- five-layer matrix assembly;
- global orthogonality.

I did not find a remaining hidden mathematical jump that is serious enough to affect correctness.

There are places where several elementary algebraic manipulations could be split even further, but that is not scientifically consequential.

No finding raised.

---

# 18. Important independently verified results

The following were independently reproduced:

### Governing model

\[
\boxed{
c_t=\nabla\cdot(D\nabla c)+S
}
\]

### Spherical model

\[
\boxed{
c_t=
\frac1{r^2}(r^2Dc_r)_r+S
}
\]

### Constant-layer specialization

\[
\boxed{
c_{i,t}
=
D_i
\left(
c_{i,rr}+\frac2r c_{i,r}
\right)
+
S_i
}
\]

### Centre limit

\[
\boxed{
\lim_{r\to0}
\left(
c_{rr}+\frac2r c_r
\right)
=
3c_{rr}(0)
}
\]

### Homogeneous steady profile

\[
\boxed{
w(r)
=
\frac{S_0R}{3h}
+
\frac{S_0}{6D}(R^2-r^2)
}
\]

### Homogeneous eigencondition

\[
\boxed{
\sin\mu-\mu\cos\mu
=
\mathrm{Bi}\sin\mu
}
\]

and therefore

\[
\boxed{
\mu\cot\mu=1-\mathrm{Bi}
}
\]

for \(\sin\mu\ne0\).

### Homogeneous modal decay

\[
\boxed{
\Lambda_n=D\frac{\mu_n^2}{R^2}
}
\]

### Five-layer kernel solution

\[
\boxed{
c_1=A_1-\frac{S_0r^2}{6D_1}
}
\]

### Five-layer source rate

\[
\boxed{
\dot N=\frac{4\pi S_0r_1^3}{3}
}
\]

### Five-layer outer flux

\[
\boxed{
J_r=\frac{S_0r_1^3}{3r^2}
}
\]

### Shell resistance

\[
\boxed{
\mathcal R_i
=
\frac1{4\pi D_i}
\left(
\frac1{r_{i-1}}-\frac1{r_i}
\right)
}
\]

### Five-layer eigen-wavenumber

\[
\boxed{
k_i^2=\frac{\Lambda}{D_i}
}
\]

### Five-layer transformed ODE

\[
\boxed{
u_i''+k_i^2u_i=0
}
\]

### Five-layer global eigencondition

\[
\boxed{
\det\mathbf M(\Lambda)=0
}
\]

### Global orthogonality

\[
\boxed{
\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2\phi_i^{(m)}\phi_i^{(n)}\,dr
=0
}
\]

for distinct eigenvalues.

---

# 19. What remains open, and where it belongs

## Must be resolved before discretisation

**None.**

The continuous mathematics is sufficiently established to begin detailed discrete derivation.

## May remain open during Review 2 / later numerical work

- numerical eigenvalue enumeration;
- modal truncation error;
- numerical conditioning of the determinant;
- discrete representation of discontinuous \(D\);
- discrete centre/interface/boundary treatment.

These belong to the next stage.

## Review 3

- complete code ↔ equation mapping;
- implementation details;
- parameter validation;
- runtime fallback behaviour;
- production implementation correctness.

## Review 4

- completeness/series convergence evidence;
- WOS/continuum equivalence;
- numerical convergence;
- transient benchmark comparison;
- published benchmark reproduction.

## Supervisor-dependent

- species selection;
- physical decay/trapping model;
- \(K_i\);
- interfacial resistance;
- final coolant boundary;
- state-dependent diffusivity.

These are appropriately left conditional.

---

# 20. Final gate decision

# **PASS FOR DISCRETISATION**

The continuous model is now sufficiently reliable to support the next derivation stage.

The two remaining substantive cleanup items are:

1. resolve the stable equation-ID collisions;
2. retain the completeness statement as theorem-dependent until a theorem matching the singular transmission problem is supplied.

Neither constitutes a mathematical blocker.

The important result is that I **did not find a hidden error in the new five-layer eigenproblem or the global \(r^2\)-weighted orthogonality argument**. Those were the two places where a superficially convincing derivation could most easily have concealed a real defect.

The project can therefore move to **detailed discrete derivation** without reopening the continuous mathematics.