# Final TRISO Diffusion and Release Derivation

## 1. Scope and assumptions

This document gives the compact mathematical contract used by the verified Rust finite-volume (FV) solver. The physical particle contains five concentric regions: Kernel, Buffer, IPyC, SiC and OPyC, with outer radii \(R_1<\cdots<R_5=R\). Within each material, diffusivity \(D_i\) is constant for a specified nuclide and temperature. Spherical symmetry is assumed. Interfaces are ideal: concentration and normal diffusive flux are continuous. The FV release benchmark uses a frozen initial kernel inventory, no continuing source after \(t=0\), and an absorbing outer surface. The separate analytical benchmark may use continuous generation and a Robin boundary; these cases must not be compared unless source, initial and boundary assumptions are matched.

## 2. Species conservation

For a fixed control volume \(V\) with outward normal \(\mathbf n\),

\[
\frac{d}{dt}\int_V c\,dV
=
-\int_{\partial V}\mathbf J\cdot\mathbf n\,dA
+\int_V S\,dV .
\tag{TRISO-GOV-020}
\]

Applying the divergence theorem gives

\[
\int_V\left(
\frac{\partial c}{\partial t}
+\nabla\cdot\mathbf J-S
\right)dV=0,
\]

and because \(V\) is arbitrary,

\[
\frac{\partial c}{\partial t}
+\nabla\cdot\mathbf J=S.
\tag{TRISO-GOV-030}
\]

Fick's law is

\[
\mathbf J=-D\nabla c.
\tag{TRISO-GOV-034}
\]

Hence

\[
\frac{\partial c}{\partial t}
=
\nabla\cdot(D\nabla c)+S.
\tag{TRISO-GOV-040}
\]

For spherical symmetry in layer \(i\),

\[
\boxed{
\frac{\partial c_i}{\partial t}
=
\frac{1}{r^2}\frac{\partial}{\partial r}
\left(r^2D_i\frac{\partial c_i}{\partial r}\right)+S_i
}.
\tag{TRISO-FV-100}
\]

## 3. Five-layer conditions

At the centre, regularity and symmetry require

\[
\left.\frac{\partial c_1}{\partial r}\right|_{r=0}=0.
\]

At each material interface \(r=R_i\),

\[
c_i(R_i,t)=c_{i+1}(R_i,t),
\qquad
D_i\left.\frac{\partial c_i}{\partial r}\right|_{R_i^-}
=
D_{i+1}\left.\frac{\partial c_{i+1}}{\partial r}\right|_{R_i^+}.
\]

For the primary release calculation the outer boundary is absorbing,

\[
c_5(R,t)=0.
\]

A separate analytical benchmark may use the Robin condition

\[
-D_5\left.\frac{\partial c_5}{\partial r}\right|_R
=h[c_5(R,t)-c_\infty].
\]

For the initial-inventory release case,

\[
c(r,0)=
\begin{cases}
c_0,&0\le r<R_1,\\
0,&R_1<r<R,
\end{cases}
\qquad S_i(r,t)=0\quad(t>0).
\]

## 4. Analytical/modal reference

For a homogeneous source-free transient, write \(c(r,t)=\phi(r)e^{-\Lambda t}\). Then

\[
\frac{1}{r^2}\frac{d}{dr}\left(r^2D\frac{d\phi}{dr}\right)+\Lambda\phi=0.
\]

With \(u=r\phi\),

\[
u''+k^2u=0,\qquad k^2=\Lambda/D.
\]

Thus each homogeneous layer has

\[
u_i(r)=A_i\sin(k_i r)+B_i\cos(k_i r),
\qquad
\phi_i(r)=\frac{u_i(r)}{r}.
\]

Centre regularity removes the singular kernel cosine contribution. Interface concentration and flux continuity and the outer boundary form a global homogeneous linear system

\[
\mathbf M(\Lambda)\mathbf a=\mathbf 0,
\qquad
\det\mathbf M(\Lambda)=0.
\]

The roots determine the modal eigenvalues. For verified modal roots and coefficients, the formal solution is

\[
c_i(r,t)=c_{i,\mathrm{ss}}(r)+
\sum_{n=1}^{\infty}A_n\phi_{i,n}(r)e^{-\Lambda_n t}.
\]

This analytical track is retained as an independent reference. The five-layer modal completeness/truncation problem is not used to establish correctness of the production FV solver.

## 5. Conservative spherical finite volumes

Let cell \(j\) occupy \(r_{j-1/2}<r<r_{j+1/2}\). Its exact volume and face areas are

\[
V_j=\frac{4\pi}{3}
\left(r_{j+1/2}^3-r_{j-1/2}^3\right),
\qquad
A_{j\pm1/2}=4\pi r_{j\pm1/2}^2.
\tag{TRISO-FV-101}
\]

The spherical volume-centroid radius is

\[
r_j=
\frac{3}{4}
\frac{r_{j+1/2}^4-r_{j-1/2}^4}
{r_{j+1/2}^3-r_{j-1/2}^3}.
\tag{TRISO-FV-109A}
\]

Integrating the conservative PDE over the cell gives

\[
V_j\frac{dc_j}{dt}
=
A_{j-1/2}D\left.\frac{\partial c}{\partial r}\right|_{j-1/2}
-
A_{j+1/2}J_{r,j+1/2}
+S_jV_j,
\]

or, after defining conservative face conductances,

\[
\boxed{
V_j\frac{dc_j}{dt}
=
G_{j-1/2}(c_{j-1}-c_j)
+
G_{j+1/2}(c_{j+1}-c_j)
+S_jV_j
}.
\tag{TRISO-FV-155}
\]

### 5.1 Same-material and interface conductance

For a face \(f\) between adjacent cell centroids \(P,E\), let \(\delta_P=r_f-r_P\) and \(\delta_E=r_E-r_f\). Series diffusion resistance gives

\[
R_f=\frac{\delta_P}{D_P A_f}+
\frac{\delta_E}{D_E A_f},
\]

hence

\[
\boxed{
G_f=\frac{1}{R_f}
=
\frac{A_f}
{\delta_P/D_P+\delta_E/D_E}
}.
\tag{TRISO-FV-140}
\]

This form applies at discontinuous-\(D\) material interfaces and reduces to the ordinary two-point conductance when \(D_P=D_E\). Because the same \(G_f(c_E-c_P)\) appears with opposite signs in the two neighbouring balances, every internal face flux cancels exactly in the global inventory balance.

### 5.2 Centre

The innermost control volume has \(r_{1/2}=0\), so its west face area is zero and \(G_{1/2}=0\). No ghost cell is required.

### 5.3 Absorbing outer boundary

For the primary FV release case \(c(R,t)=0\). With last centroid \(r_N\),

\[
G_R=\frac{4\pi R^2D_5}{R-r_N}.
\]

The outer-cell balance is

\[
V_N\frac{dc_N}{dt}
=
G_{N-1/2}(c_{N-1}-c_N)-G_Rc_N+S_NV_N.
\]

The instantaneous release rate is

\[
\dot M_{\mathrm{rel}}=G_Rc_N.
\]

## 6. Backward-Euler system

For timestep \(\Delta t\),

\[
\frac{V_j}{\Delta t}(c_j^{n+1}-c_j^n)
=
G_W(c_{j-1}^{n+1}-c_j^{n+1})
+
G_E(c_{j+1}^{n+1}-c_j^{n+1})
+S_j^{n+1}V_j.
\]

Collecting coefficients,

\[
\boxed{
-G_Wc_{j-1}^{n+1}
+
\left(\frac{V_j}{\Delta t}+G_W+G_E\right)c_j^{n+1}
-G_Ec_{j+1}^{n+1}
=
\frac{V_j}{\Delta t}c_j^n+S_j^{n+1}V_j
}.
\]

This is tridiagonal. The matrix has positive diagonal and non-positive off-diagonal entries; for nonnegative old concentration and source, the Backward-Euler diffusion solve preserves the expected monotone/positive structure.

For verification only, Forward Euler has the sufficient coefficient-positivity restriction

\[
\boxed{
\Delta t\le
\min_j\frac{V_j}{G_{j-1/2}+G_{j+1/2}}
}.
\tag{TRISO-FV-300}
\]

Backward Euler is the production default and is not constrained by this explicit stability limit.

## 7. Inventory, release and conservation

The discrete inventory is

\[
M^n=\sum_jV_jc_j^n.
\]

For the no-source absorbing case, integrating the Backward-Euler outer flux consistently over a step gives

\[
M_{\mathrm{rel}}^{n+1}
=
M_{\mathrm{rel}}^n+
\Delta t\,G_Rc_N^{n+1}.
\]

The global discrete balance is

\[
M^{n+1}+M_{\mathrm{rel}}^{n+1}
=
M^0
\]

up to linear-solver and floating-point error. The release fraction and rate are

\[
F^{n}=\frac{M_{\mathrm{rel}}^n}{M^0},
\qquad
\dot F^{n}=\frac{G_Rc_N^n}{M^0}.
\]

These are computed from the same boundary flux used by the FV balance, so inventory loss and accumulated release provide independent conservation diagnostics.

## 8. Numerical verification contract

The five physical interfaces must coincide with FV faces. Spatial adequacy is determined by refinement, not by declaring a particular \(N\) exact. The production package executes homogeneous, controlled two-layer and five-layer synthetic cases; checks nonnegative concentrations, \(0\le F\le1\), inventory/release conservation, and separate mesh/time refinement.

The existing reviewed verification driver demonstrated approximately second-order spatial convergence on an aligned five-layer steady benchmark and approximately first-order Forward-Euler temporal self-convergence. The production Rust package adds Backward Euler and an arbitrary per-region interface-aligned mesh.

## 9. Scientific limitations

The supplied analytical Rust reference is available only through the repository's structural handoff summary; its original full uploaded function bodies are not committed here. Therefore the analytical module in the final package is a verified baseline implementation, not a claim of verbatim reconstruction.

The synthetic configurations are numerical verification cases, not literature-verified TRISO properties. Literature-dependent radii, diffusivities, nuclide, temperature and source data must be supplied through the separate template before physical prediction. Temperature/fluence-dependent diffusivity, arbitrary source schedules and production Robin release are deferred from the MVP unless explicitly enabled.
