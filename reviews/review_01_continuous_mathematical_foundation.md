# Review 1 — Continuous Mathematical Foundation

**Reviewer:** Independent Reviewer  
**Date:** 29 September 2026  
**Feature branch:** `ray/triso-foundation`  
**Branch state:** 26 commits ahead of `main`, 0 behind  
**Review scope:** Continuous mathematical foundation only

## Final determination

**Review 1 PASS WITH NON-BLOCKING FINDINGS**

The continuous mathematical foundation is sufficiently correct and explicit to justify progression into the next numerical stage **for an explicitly selected conditional model**.

The core conservation law, Fickian constitutive law, conservative variable-diffusivity PDE, spherical reduction, centre condition, ideal-interface conditions, outer Robin condition, homogeneous Part-I steady solution, repaired Part-I eigenproblem, modal projection, and five-layer steady analytical framework are independently consistent.

No finding identified below invalidates the continuous PDE itself or prevents discretisation of a clearly frozen model.

The most important remaining issue is that the five-layer transient eigenframework uses separation of variables while the preceding model still permits time-dependent diffusivities. That is a conditional-model consistency problem, not a defect in the conservative PDE.

---

# 1. Current substantive state

The feature branch contains:

- a consolidated continuous mathematical foundation;
- a dedicated continuous five-layer model;
- an original-notebook audit and traceability register;
- a central equation status register;
- a foundation consolidation audit;
- a two-layer continuum analytical/reference framework;
- Rust verification/reference programs;
- CI configuration.

The branch is exactly 26 commits ahead of `main` at this review point.

The original LaTeX notebook supplied with the project remains an external evidence stream rather than a repository file. Its labels were independently inspected below.

The repository therefore contains substantial mathematical work, but the original LaTeX source is not itself present in the feature-branch tree.

---

# 2. Original-derivation coverage audit

## Independent equation count

I independently parsed `TRISO Fuel Derivation Ray V1.tex`.

It contains:

- 30 `equation` environments;
- 1 `align` environment containing 3 labelled mathematical equations;
- therefore **33 labelled substantive displayed equations**.

The 33 equation labels, in source order, are:

1. `eq:b:ic`
2. `eq:b:bc0`
3. `eq:b:bcR`
4. `eq:b:biot`
5. `eq:fick`
6. `eq:fick-spherical`
7. `eq:radial-conservative`
8. `eq:product-rule`
9. `eq:simplified-triso`
10. `eq:kernel-source`
11. `eq:source-decomposition`
12. `eq:b:pde`
13. `eq:b:pde-divergence`
14. `eq:b:pqsigma`
15. `eq:b:w-firstint`
16. `eq:b:w-secondint`
17. `eq:b:w`
18. `eq:b:massbalance`
19. `eq:b:vproblem`
20. `eq:b:sl-ode`
21. `eq:b:harmonic`
22. `eq:b:eigfun`
23. `eq:b:eigcond-raw`
24. `eq:central-2nd`
25. `eq:explicit-coeffs`
26. `eq:centre-ghost`
27. `eq:centre-stencil`
28. `eq:lhopital`
29. `eq:centre-curvature-limit`
30. `eq:centre-update`
31. `eq:b:ftcs-ghost`
32. `eq:b:ftcs-surface`
33. `eq:b:ftcs-stability`

The 33/33 equation inventory claim is therefore **verified**.

The displayed mathematics does not conceal an additional unlabelled substantive equation beyond these 33. The two literal `\\[` matches in the source are title-spacing commands rather than mathematical displays.

### Original equation locations

The 33 labelled equations occur at source lines:

```
220, 221, 222, 241,
262, 286, 310, 331, 350, 374, 396,
431, 455, 474, 492, 510, 530, 550, 570,
592, 610, 625, 642,
671, 692, 712, 727, 745, 760, 781,
813, 840, 865
```

The source also contains substantive prose assumptions around these equations. I found that the consolidated foundation preserves the material assumptions that matter to the continuous model:

- concentric spherical layers;
- representative particle;
- radial symmetry;
- kernel-confined generation;
- piecewise material diffusivity;
- ideal-interface assumptions;
- finite-resistance coolant-side transfer;
- zero initial concentration as a benchmark.

The consolidation also explicitly identifies changes to the original equation forms rather than silently pretending that the original notation was already correct.

## Traceability finding

**R1-m01 — MINOR**

**Location:** `docs/triso/04_existing_notebook_audit_and_traceability.md`, section “Original → stable equation mapping”; `docs/triso/13_central_equation_status_register.md`.

**Issue:** The 33-equation coverage itself is correct, but the stable-ID mapping is not yet completely internally uniform.

For example:

- `eq:b:pde` is listed in the register as `TRISO-GOV-029`;
- the consolidated foundation instead reconstructs the Part-I benchmark through `TRISO-GOV-025` and `TRISO-GOV-026`;
- `eq:b:biot` is listed as `TRISO-VER-009`, while the visible consolidated numbering jumps from `TRISO-VER-008) in the eigenfunction discussion to the later verification IDs.

**Consequence:** A reviewer can still recover the mathematics, but stable equation identifiers are not yet a completely reliable bidirectional trace key.

**Required remediation:** Freeze a single mapping table in one authoritative document and make the other status documents reference it rather than recreating mappings independently.

**Evidence required for closure:** One checked register in which every original label has exactly one authoritative stable destination or an explicit “superseded/repaired into” relationship.

---

# 3. Conservation derivation

## Independent reconstruction

Starting with total species inventory in a fixed control volume:

[
\frac{d}{dt}\int_V c\,dV
=
-\oint_{\partial V}\mathbf J\cdot\mathbf n\,dA
+
\int_V S\,dV.
]

Using the divergence theorem:

[
\oint_{\partial V}\mathbf J\cdot\mathbf n\,dA
=
\int_V\nabla\cdot\mathbf J\,dV.
]

Therefore:

[
\int_V
\left(
\frac{\partial c}{\partial t}
+
\nabla\cdot\mathbf J
-
S
\right)dV
=0.
]

For arbitrary fixed (V):

[
\boxed{
\frac{\partial c}{\partial t}
+
\nabla\cdot\mathbf J
=
S
}.
]

With the adopted Fickian law

[
\mathbf J=-D\nabla c,
]

substitution gives

[
\frac{\partial c}{\partial t}
-
\nabla\cdot(D\nabla c)
=
S,
]

and hence

[
\boxed{
\frac{\partial c}{\partial t}
=
\nabla\cdot(D\nabla c)+S
}.
]

The signs and units are correct.

The project consolidation reproduces this derivation in `TRISO-GOV-001` through `TRISO-GOV-012`.

**Independent result:** PASS.

## Important scope check

The original notebook's

[
\frac{\partial c}{\partial t}
=
D\nabla^2c+S
]

is valid only after the constant-(D) specialization.

The consolidation correctly identifies the conservative variable-(D) equation as the general form.

This is mathematically necessary for a multilayer TRISO model.

**Independent result:** PASS.

---

# 4. Spherical reduction

The consolidated model defines

[
J_r=-D(r,t)\frac{\partial c}{\partial r}
]

and

[
\nabla\cdot\mathbf J
=
\frac{1}{r^2}
\frac{\partial}{\partial r}
\left(r^2J_r\right).
]

Substitution therefore gives

[
\boxed{
\frac{\partial c}{\partial t}
=
\frac{1}{r^2}
\frac{\partial}{\partial r}
\left(
r^2D(r,t)\frac{\partial c}{\partial r}
\right)
+
S(r,t)
}.
]

That derivation is correct.

Inside a layer where (D_i) is spatially constant,

[
\frac{\partial c_i}{\partial t}
=
D_i
\frac{1}{r^2}
\frac{\partial}{\partial r}
\left(r^2\frac{\partial c_i}{\partial r}\right)
+
S_i.
]

Only there may the operator be expanded as

[
\frac{1}{r^2}\frac{d}{dr}
(r^2c_r)
=
c_{rr}+\frac{2}{r}c_r.
]

The consolidation correctly avoids differentiating a discontinuous (D) through an interface.

**Independent result:** PASS.

---

# 5. Centre treatment

The continuous condition

[
\left.c_r\right|_{r=0}=0
]

is correct for a regular spherically symmetric scalar field.

The even-extension argument used by Main Research is valid provided differentiability at the centre is assumed.

The apparent (1/r) singularity is removable for a regular solution.

For a Taylor expansion

[
c(r)=c_0+\frac12c_{rr}(0)r^2+O(r^4),
]

we have

[
c_r=c_{rr}(0)r+O(r^3),
]

so

[
\lim_{r\to0}\frac{2}{r}c_r
=
2c_{rr}(0).
]

Hence

[
\left[
c_{rr}+\frac{2}{r}c_r
\right]_{r=0}
=
3c_{rr}(0).
]

The project’s later factor-of-six FTCS centre stencil is consistent with this continuous limit.

The discrete stencil is a later-stage matter and is not treated as evidence for the continuous condition.

**Independent result:** PASS.

---

# 6. Material interfaces

For interface (r=r_k), with no interfacial storage, conservation requires equal radial flux:

[
J_{r,k}(r_k,t)=J_{r,k+1}(r_k,t).
]

With Fick's law:

[
-D_k
\left.c_k'\right|_{r_k^-}
=
-D_{k+1}
\left.c_{k+1}'\right|_{r_k^+}.
]

Equivalently,

[
D_kc_k'(r_k^-)
=
D_{k+1}c_{k+1}'(r_k^+).
]

The project then imposes

[
c_k(r_k,t)=c_{k+1}(r_k,t)
]

under an explicitly stated ideal-contact, single-concentration assumption.

This is an acceptable **conditional continuous model**.

It is not an identity that follows from conservation alone.

A partition law would instead be something of the form

[
c_{k+1}=K_kc_k
]

once (K_k) is defined with a clear direction convention.

An interfacial resistance would generally preserve flux continuity while introducing a concentration jump governed by an interface constitutive relation.

Main Research correctly leaves those alternatives conditional.

**Independent result:** PASS, conditional on the declared ideal-interface assumption.

No WOS verification is required for this Review-1 conclusion.

---

# 7. Outer boundary

The consolidated model gives

[
J_r(R,t)=h[c(R,t)-c_\infty(t)]
]

and therefore

[
\boxed{
-D_5c_5'(R,t)
=
h[c_5(R,t)-c_\infty(t)]
}.
]

This is dimensionally consistent:

- (D_5c_r): mol m(^{-2}) s(^{-1});
- (h(c-c_\infty)): mol m(^{-2}) s(^{-1}).

The use of (D_5) is correct for the outer OPyC region.

The original notebook's condition

[
-Dc_r(R,t)=hc(R,t)
]

is recovered when:

1. the domain is homogeneous with diffusivity (D);
2. (c_\infty=0).

The zero-coolant case is therefore a benchmark assumption, not the general boundary law.

**Independent result:** PASS.

---

# 8. Part-I steady analytical solution

For

[
Dc''+\frac{2D}{r}c'+S_0=0,
]

the integrated equation is

[
\frac{d}{dr}(r^2w_r)
=
-\frac{S_0}{D}r^2.
]

First integration:

[
r^2w_r
=
-\frac{S_0r^3}{3D}+A.
]

Boundedness at (r=0) requires (A=0).

Thus:

[
w_r=-\frac{S_0r}{3D}.
]

Second integration gives

[
w=B-\frac{S_0r^2}{6D}.
]

At (R),

[
-Dw_r(R)=\frac{S_0R}{3}.
]

The zero-coolant Robin condition gives

[
w(R)=\frac{S_0R}{3h}.
]

Hence:

[
\boxed{
w(r)
=
\frac{S_0R}{3h}
+
\frac{S_0}{6D}(R^2-r^2)
}.
]

Independent global generation/release balance:

[
\frac{4\pi R^3}{3}S_0
=
4\pi R^2h,w(R)
]

gives the same surface concentration.

Units and signs are consistent.

**Independent result:** PASS.

---

# 9. Part-I transient eigenproblem

The repaired notation is mathematically consistent.

With

[
v=c-w,
]

the transient equation is homogeneous:

[
v_t=D\left(v_{rr}+\frac2r v_r\right).
]

Assume

[
v(r,t)=\phi(r)T(t).
]

Separation gives

[
\frac{T'}{DT}
=
\frac{\phi''+2\phi'/r}{\phi}.
]

Set

[
\frac{T'}{DT}=-k^2.
]

Then:

[
T'=-Dk^2T
]

and

[
\phi''+\frac2r\phi'+k^2\phi=0.
]

With (u=r\phi),

[
u''+k^2u=0.
]

The regular centre solution is

[
u=A\sin(kr),
]

therefore

[
\phi=A\frac{\sin(kr)}{r}.
]

The centre limit

[
\lim_{r\to0}\frac{\sin(kr)}r=k
]

is finite.

Define

[
\mu=kR,
\qquad
\mathrm{Bi}=\frac{hR}{D}.
]

Applying the Robin condition gives

[
\sin\mu-\mu\cos\mu
=
\mathrm{Bi}\sin\mu.
]

Therefore, for positive admissible roots,

[
\boxed{
\mu\cot\mu=1-\mathrm{Bi}
}.
]

The temporal decay rate is

[
\boxed{
\Lambda=Dk^2=\frac{D\mu^2}{R^2}
}.
]

The dimensional distinction between (k,[\mathrm m^{-1}]) and (Lambda,[\mathrm s^{-1}]) is correct.

**Independent result:** PASS after the documented notation repair.

---

# 10. Eigenvalue-root handling

**R1-m02 — MINOR**

**Location:** `docs/triso/00_consolidated_mathematical_foundation.md`, §9 “Robin eigencondition”.

**Issue:** The text correctly says the cotangent form is obtained “away from (sin\mu=0)”, but it does not explicitly state the admissibility consequences.

Two special cases should be recorded:

1. (mu=0) makes the raw equation vanish algebraically, but it does not represent a non-zero eigenfunction for (h>0); the corresponding (sin(kr)/r) representation collapses in the (k\to0) parameterisation.
2. (sin\mu=0) with positive integer (mu=n\pi) does not satisfy the raw equation for positive (n), because the remaining term (-\mu\cos\mu) is nonzero.

Therefore no physically valid positive mode is lost by passing to (mu\cot\mu=1-\mathrm{Bi}), but the root-exclusion statement should be explicit.

**Consequence:** Prevents future eigenvalue enumeration from accidentally treating (mu=0) as a valid mode.

**Required remediation:** State explicitly that the eigenvalue set consists of positive admissible roots of the raw condition, excluding the spurious parameter root (mu=0).

**Evidence required for closure:** A short root-domain statement plus the first few numerical roots for representative positive (mathrm{Bi}).

---

# 11. Modal expansion and orthogonality

For the homogeneous Part-I operator with constant (D), the radial Sturm–Liouville weight is (r^2).

The orthogonality relation

[
\int_0^R r^2\phi_m\phi_n\,dr=0,
\qquad m\ne n,
]

is appropriate under the stated regular-centre and Robin boundary conditions.

The coefficient projection

[
A_n
=
-
\frac{
\int_0^R r^2w\phi_n\,dr
}{
\int_0^R r^2\phi_n^2\,dr
}
]

is consistent with expanding the initial deviation (-w(r)).

No mathematical error was found in the projection formula.

**Independent result:** PASS.

---

# 12. Five-layer steady analytical model

The kernel source produces total generation

[
\dot N_{gen}
=
4\pi\int_0^a S_0r^2dr
=
\frac{4\pi}{3}S_0a^3.
]

For (r>a), steady conservation gives constant total outward rate:

[
4\pi r^2J_r=\dot N_{gen}.
]

Therefore,

[
J_r=\frac{S_0a^3}{3r^2}.
]

Using

[
J_r=-D_i c_i',
]

gives

[
c_i'
=
-\frac{S_0a^3}{3D_ir^2}.
]

Integrating through a shell gives the stated radial concentration drop.

The shell resistance

[
\rho_i
=
\frac1{4\pi D_i}
\left(
\frac1{r_{i-1}}-\frac1{r_i}
\right)
]

has units s m(^{-3}).

Multiplying by total generation (mol s(^{-1})) gives concentration (mol m(^{-3})).

The surface relation

[
c_5(R)
=
\frac{S_0a^3}{3hR^2}
]

for (c_\infty=0) also follows directly from global balance.

**Independent result:** PASS.

---

# 13. Five-layer transient framework

The layerwise equation

[
\frac1{r^2}\frac{d}{dr}
\left(r^2D_i\phi_i'\right)
+
\Lambda\phi_i
=0
]

is correct for the source-free transient eigenproblem when (D_i) is spatially constant and time independent.

With

[
k_i^2=\frac{\Lambda}{D_i},
qquad
u_i=r\phi_i,
]

one obtains

[
u_i''+k_i^2u_i=0.
]

At the centre, regularity gives the kernel cosine coefficient (B_1=0).

At an ideal interface,

[
u_k=u_{k+1}
]

and

[
D_k
\left(
u_k'-\frac{u_k}{r_k}
\right)
=
D_{k+1}
\left(
u_{k+1}'-\frac{u_{k+1}}{r_k}
\right).
]

These are correctly transformed versions of concentration continuity and flux continuity.

At the outer boundary,

[
-D_5
\left(
u_5'-\frac{u_5}{R}
\right)
=
hu_5
]

is likewise correct after multiplying the Robin condition by (R).

A homogeneous coefficient system therefore has a non-zero solution only when its determinant vanishes:

[
F(\Lambda)=0.
]

**Independent result:** PASS as a **constant-(D_i), no-reaction eigenproblem**.

---

# 14. Five-layer transient eigenframework scope consistency

**R1-M01 — MAJOR**

**Location:** `docs/triso/03_continuous_five_layer_model.md`, §7 and §14; `docs/triso/00_consolidated_mathematical_foundation.md`, §12.

**Issue:** The general continuous model explicitly leaves

[
D_i=D_i(t)
]

open, and the document also allows reaction terms (R_i). But the separated transient eigenframework assumes

[
v_i(r,t)=\phi_i(r)e^{-\Lambda t}
]

with

[
k_i^2=\Lambda/D_i.
]

That separation requires the coefficients entering the operator to be time independent (or otherwise reduced to a special solvable form). With (D_i(t)), the spatial eigenfunctions generally become time-dependent and the simple exponential separation is no longer valid.

Similarly, if

[
R_i=-\lambda_{d,i}c_i
]

is included, the layer wavenumber relation changes to the corresponding reaction-diffusion form; (k_i^2=\Lambda/D_i) is no longer the complete relation.

**Consequence:** The five-layer transient eigenproblem is correct only for a narrower model than the preceding general PDE currently allows.

**Required remediation:** Mark §12 explicitly as a benchmark/eigenanalysis subcase:

- (D_i=) constant in time;
- no decay/reaction term in the eigenproblem;
- time-independent interface parameters;
- fixed (h) and (c_\infty=0) if the homogeneous modal boundary condition is being used.

Alternatively derive the corresponding eigenproblem for the selected reaction/diffusion model.

**Evidence required for closure:** An explicit model declaration immediately before the eigenproblem, plus confirmation that the chosen eigenproblem solves the same continuous PDE intended for that benchmark.

This does **not** block discretisation of the general conservative PDE.

---

# 15. Source/reaction model

**R1-M03 — MAJOR**

**Location:** `docs/triso/00_consolidated_mathematical_foundation.md`, §5; `docs/triso/03_continuous_five_layer_model.md`, §9 and §14; original `eq:source-decomposition` at LaTeX line 396.

**Issue:** The project still allows

[
S_i=S_{i,gen}+S_{i,other}-\lambda_{d,i}c_i
]

while the actual trapping/release model remains undefined.

This is acceptable as a conditional mathematical framework, but the document currently moves between:

- pure generation;
- generation + optional decay;
- possible trapping/release;
- generic (R_i);

without freezing which one belongs to the continuous model that will actually be discretised.

**Consequence:** A numerical implementation cannot legitimately choose one source/reaction form merely by copying the most convenient one from the notes.

**Required remediation:** Before implementing the physical case, freeze one explicit continuous model:

[
R_i=0,
]

or

[
R_i=-\lambda_{d,i}c_i,
]

or a fully defined mobile/trapped exchange model.

Keep alternatives as separate benchmark variants.

**Evidence required for closure:** A signed, dimensional source/reaction equation and explicit statement of which model is the canonical benchmark.

This is not a blocker for discretising the current **base transport model with (R_i=0)**.

---

# 16. Coolant concentration and interface physics

**R1-m04 — NOTE**

**Location:** `docs/triso/03_continuous_five_layer_model.md`, §§11–13.

The project correctly keeps:

- (c_\infty);
- partition coefficients;
- interfacial resistance;

as conditional choices.

This is the correct epistemic treatment for Review 1.

The outer Robin model with (c_\infty=0) is mathematically closed and suitable as a benchmark.

No physical defect is identified here.

**Closure evidence later required:** Supervisor selection of the canonical interface and coolant model.

---

# 17. Literature provenance

The external evidence checked during this review supports the general continuous modelling direction:

- IAEA documentation states that Fickian equations with effective diffusion coefficients are used for fission-product transport in reactor materials, and that effective coefficients are commonly temperature dependent. citeturn916212search36
- The BISON TRISO workshop explicitly frames the model around defining diffusion coefficient (D), decay (C), and source (S), with material-specific diffusion coefficients. citeturn916212search3
- BISON's stable/long-lived spherical verification case uses zero initial concentration, a constant source, spherical diffusion, and an analytical solution. citeturn916212search2
- BISON's out-of-pile decaying-product verification separately uses radioactive decay and a spherical analytical solution. citeturn916212search1
- The IAEA CRP-6 BISON documentation includes diffusion benchmarks ranging from simple coated geometries to realistic TRISO configurations and explicitly notes the importance of prescribed geometry, boundary conditions, material properties and constitutive relations. citeturn916212search0
- Hales et al. 2021 is the identified primary paper underlying BISON's TRISO fission-product diffusion modelling. citeturn916212search37

## Provenance finding

**R1-M05 — MODERATE**

**Location:** `docs/triso/03_continuous_five_layer_model.md`, References; `docs/triso/00_consolidated_mathematical_foundation.md`, References.

The source quality is broadly appropriate, but several repository citations are stored as ChatGPT-style citation tokens rather than repository-native bibliographic links. This weakens durable provenance.

The mathematical derivations themselves do not need external citations when they are derived in the project. The issue concerns sourced physical claims and parameters.

**Consequence:** A future reviewer cannot necessarily resolve every source token from the repository alone.

**Required remediation:** Replace transient citation tokens with permanent bibliographic references/URLs/DOIs in repository documents.

**Evidence required for closure:** Each substantive sourced physical claim points to a stable source entry.

---

# 18. Implementation provenance — Review-1-limited check

The foundation documents identify:

`theodoreOnzGit/outram-park-backend`

and the relevant WOS paths/functions.

I checked the cited upstream `interface.rs` directly. It exists on that repository's `main` branch and contains the cited `does_transmit` interface routine and associated transmission-probability tests.

Therefore the provenance claim that this upstream repository contains the cited interface implementation is supported.

The implementation itself was **not reviewed here for correctness**, because that is outside Review 1.

**Independent result:** provenance supported; implementation correctness deferred.

---

# 19. Repository/documentation traceability

**R1-M06 — MODERATE**

**Location:** `docs/triso/04_existing_notebook_audit_and_traceability.md`; `docs/triso/13_central_equation_status_register.md`; `docs/triso/15_foundation_consolidation_audit.md`.

Several statements are stale relative to the current feature branch:

- `04_existing_notebook_audit_and_traceability.md) says the GitHub repository contains only the bootstrap README and access-test file;
- `13_central_equation_status_register.md) describes an older upstream Ray branch and old commit tip;
- `15_foundation_consolidation_audit.md) likewise records the earlier repository state and says the branch was only two commits ahead.

Those statements are contradicted by the current feature branch, which is 26 commits ahead of `main) and contains the scientific documentation and Rust verification files listed above.

**Consequence:** The scientific mathematics is unaffected, but repository provenance and review reproducibility are weakened.

**Required remediation:** Refresh stale repository-state sections so that they describe the state at the commit to which the review applies, or clearly freeze them as historical snapshots.

**Evidence required for closure:** A commit SHA and date attached to each historical state statement.

---

# 20. Original LaTeX repository availability

**R1-m07 — MINOR**

**Location:** project repository tree versus supplied `TRISO Fuel Derivation Ray V1.tex`.

The original LaTeX document was independently audited because it was supplied as a project artifact, but it is not stored in the current feature-branch tree.

**Consequence:** A future repository-only reviewer cannot reproduce the 33-equation inventory directly from the repository.

**Required remediation:** Preserve the original notebook in an evidence path such as `notes/raw/` or an equivalent repository-standard location without altering its mathematical content.

**Evidence required for closure:** A byte-identical or provenance-tracked copy of the original source with its original filename preserved.

This is not a continuous-mathematics blocker.

---

# 21. FTCS scope check

The FTCS material was checked only to determine whether it reveals a defect in the underlying continuous equation.

The following continuous-to-discrete relationships are internally consistent:

- spherical interior stencil;
- factor-of-six centre treatment;
- Robin ghost algebra;
- surface update algebra.

The corrected documentation now distinguishes:

- coefficient positivity / monotonicity;
- full spectral stability.

The centre update

[
C_0^{j+1}
=
(1-6\mathrm{Fo})C_0^j
+
6\mathrm{Fo}C_1^j
+
S_0\Delta t
]

implies coefficient positivity requires

[
\mathrm{Fo}\le\frac16.
]

That is a valid observation.

A complete amplification-matrix spectral stability analysis is correctly deferred to the numerical review stage.

No FTCS issue identified here invalidates the continuous PDE.

**Review-1 result:** PASS, with numerical stability verification explicitly deferred.

---

# 22. Independently verified items

The following were independently reproduced and found mathematically consistent:

### Continuous foundation
- fixed-control-volume species conservation;
- divergence theorem step;
- sign convention;
- Fickian constitutive law;
- conservative variable-(D) PDE;
- spherical radial flux;
- divergence of radial flux;
- constant-(D_i) layer specialization;
- product-rule expansion.

### Boundary/interface mathematics
- centre regularity condition;
- removable spherical singularity;
- ideal-interface flux continuity;
- ideal-interface concentration continuity as an explicit assumption;
- partition-coefficient interpretation as a conditional alternative;
- Robin outer condition;
- correct use of (D_5);
- zero-(c_\infty) benchmark recovery.

### Analytical solutions
- homogeneous Part-I steady solution;
- global generation/release balance;
- (v=c-w) transient transformation;
- repaired (k/Lambda) eigenvalue notation;
- (u=r\phi) transformation;
- regular centre eigenfunction;
- Robin eigencondition;
- modal orthogonality weight;
- projection coefficient formula;
- five-layer steady generation rate;
- (1/r^2) flux decay outside the source region;
- shell resistance expression;
- five-layer layerwise transient ODE;
- interface equations in (u_i);
- outer Robin equation in (u_5);
- determinant eigencondition framework.

No fabricated defect was identified in these components.

---

# 23. Unresolved work classification

## Must be resolved before Review 1 can close

None of the remaining issues is a mathematical blocker to Review 1.

For the Review-1 gate to remain reproducible, however, the next commit should explicitly identify the intended continuous benchmark subcase used for numerical work.

## May remain open until Review 2 — discretisation

- complete FTCS spectral stability analysis;
- treatment of discontinuous diffusivity in the selected numerical scheme;
- discrete interface treatment;
- discrete centre/boundary formulation;
- timestep and mesh convergence;
- discrete conservation.

## May remain open until Review 3 — implementation

- full equation-to-code map;
- production WOS algorithm audit;
- code ↔ discrete-equation correspondence;
- implementation unit handling;
- WOS-specific interface algorithm validation.

## May remain open until Review 4 — verification/integration

- five-layer eigenvalue enumeration;
- modal coefficient convergence;
- WOS continuum equivalence;
- published benchmark reproduction;
- regression against the supervisor implementation.

## Requires supervisor input but may remain explicitly conditional

- canonical fission-product species;
- decay inclusion;
- trapping/release model;
- partition coefficients;
- explicit interfacial resistance;
- (D_i) dependence on temperature/fluence/burnup;
- whether (c_\infty=0) is physical or benchmark-only;
- initial inventory versus zero-inventory benchmark.

These questions do not justify inventing physics.

---

# 24. Recommended remediation order

1. **Freeze the continuous benchmark model used for numerical progression.**
   In particular, state whether the first numerical model is the stable/long-lived (R_i=0) case with prescribed time-independent (D_i), ideal interfaces, and (c_\infty=0).

2. **Constrain the five-layer eigenframework to its actual mathematical assumptions.**
   Make time-independent (D_i) and reaction-free separation explicit.

3. **Clean the stable equation-ID mapping.**
   Establish one authoritative mapping table.

4. **Refresh stale repository-state/provenance text.**

5. **Preserve the original LaTeX notebook inside the repository as raw evidence.**

6. **Then proceed to Review 2's numerical discretisation audit.**

---

# 25. Suggested focus of the next review

**Review 2 should begin from the now-frozen continuous model, not reopen the entire physical derivation.**

The highest-value numerical question is:

[
\boxed{
\text{Does the selected discretisation solve the verified continuous five-layer PDE}
\text{ while conserving inventory and treating centre/interface/boundary fluxes correctly?}
}
]

The Review-2 audit should therefore derive the actual discrete equations from the selected continuous model and test:

- spatial consistency;
- temporal consistency;
- conservation;
- centre treatment;
- discontinuous-(D_i) interface fluxes;
- boundary flux;
- stability;
- convergence.

The WOS implementation audit should remain a later stage unless a discrete issue exposes a direct contradiction with the continuous model.

---

# Final Review-1 decision

[
\boxed{\textbf{Review 1 PASS WITH NON-BLOCKING FINDINGS}}
]

The mathematical foundation is sufficiently trustworthy to justify progressing to discretisation **provided the numerical work explicitly freezes the conditional continuous model it is discretising**.

The important distinction is:

**The continuous PDE foundation passes.**

The remaining uncertainty is primarily **which physically selected member of the conditional model family is to be discretised**, plus consistency of the separately presented five-layer transient eigenbenchmark.

No continuous-model blocker was found.
