# Finite-Volume Derivation of TRISO Diffusion and Release

## 0. Symbols and units

| Symbol | Meaning | Units |
|---|---|---|
| $r,\ t$ | radius, time | $\mathrm{m}$, $\mathrm{s}$ |
| $R_i$ | outer radius of region $i$ (Kernel, Buffer, IPyC, SiC, OPyC), $R_0=0$, $R_5=R$ | $\mathrm{m}$ |
| $D_i$ | diffusivity of region $i$ (constant for a fixed nuclide and temperature) | $\mathrm{m^2\,s^{-1}}$ |
| $c(r,t)$ | concentration of the tracked species | $\mathrm{mol\,m^{-3}}$ |
| $J_r$ | outward radial flux | $\mathrm{mol\,m^{-2}\,s^{-1}}$ |
| $S$ | volumetric source | $\mathrm{mol\,m^{-3}\,s^{-1}}$ |
| $c_0$ | initial kernel concentration | $\mathrm{mol\,m^{-3}}$ |
| $h,\ c_\infty$ | Robin transfer coefficient, external concentration (Case B only) | $\mathrm{m\,s^{-1}}$, $\mathrm{mol\,m^{-3}}$ |
| $\Lambda_n,\ \phi_n$ | $n$-th eigenvalue and eigenfunction | $\mathrm{s^{-1}}$, $\mathrm{mol\,m^{-3}}$ |
| $k_i=\sqrt{\Lambda/D_i}$ | wave number in region $i$ | $\mathrm{m^{-1}}$ |
| $u=r\phi,\ q=Du'$ | transformed eigenfunction and its flux variable | $\mathrm{mol\,m^{-2}}$, $\mathrm{mol\,m^{-1}\,s^{-1}}$ |
| $V_j,\ A_f$ | cell volume, face area | $\mathrm{m^3}$, $\mathrm{m^2}$ |
| $G_f$ | face conductance | $\mathrm{m^3\,s^{-1}}$ |
| $M,\ M_{\mathrm{rel}},\ F$ | inventory in the particle, cumulative release, release fraction $M_{\mathrm{rel}}/M^0$ | $\mathrm{mol}$, $\mathrm{mol}$, $1$ |

## 1. Scope and assumptions

1. Five concentric regions: Kernel, Buffer, IPyC, SiC and OPyC. Region $i$ occupies $R_{i-1}<r<R_i$.
2. Spherical symmetry. In each region $D_i$ is constant.
3. Interfaces are ideal: concentration and normal diffusive flux are continuous (no partition coefficient).
4. **Case A (release).** Frozen initial kernel inventory, no source for $t>0$, absorbing outer surface. The finite-volume (FV) solver and the exact series of Section 4 both solve Case A with identical inputs, so they are compared directly.
5. **Case B (analytical benchmark).** Continuous generation with a Robin outer boundary. Only its homogeneous eigenvalue condition is implemented (Section 4.6). Case A and Case B must not be compared with each other.

## 2. From conservation to the spherical equation

For any fixed control volume $V$ with outward unit normal $\mathbf n$:

**Equation TRISO-GOV-020**

```math
\frac{d}{dt}\int_V c\,dV=-\int_{\partial V}\mathbf J\cdot\mathbf n\,dA+\int_V S\,dV
```

The divergence theorem turns the surface term into a volume integral:

```math
\int_V\left(\frac{\partial c}{\partial t}+\nabla\cdot\mathbf J-S\right)dV=0
```

Because $V$ is arbitrary the integrand vanishes:

**Equation TRISO-GOV-030**

```math
\frac{\partial c}{\partial t}+\nabla\cdot\mathbf J=S
```

Fick's law:

**Equation TRISO-GOV-034**

```math
\mathbf J=-D\nabla c
```

Substitution gives:

**Equation TRISO-GOV-040**

```math
\frac{\partial c}{\partial t}=\nabla\cdot(D\nabla c)+S
```

With spherical symmetry $\mathbf J=J_r(r,t)\,\mathbf e_r$. A balance on the shell $r<r'<r+dr$ gives $\partial_t(4\pi r^2c\,dr)=-\left[4\pi(r+dr)^2J_r(r+dr)-4\pi r^2J_r(r)\right]+4\pi r^2S\,dr$, so $\nabla\cdot\mathbf J=r^{-2}\partial_r(r^2J_r)$ and, in region $i$,

**Equation TRISO-FV-100**

```math
\frac{\partial c_i}{\partial t}=\frac{1}{r^2}\frac{\partial}{\partial r}\left(r^2D_i\frac{\partial c_i}{\partial r}\right)+S_i
```

## 3. Conditions

Centre (regularity and symmetry):

```math
\left.\frac{\partial c_1}{\partial r}\right|_{r=0}=0
```

Interfaces $r=R_i$, $i=1,\dots,4$ (two conditions each):

```math
c_i(R_i,t)=c_{i+1}(R_i,t),\qquad D_i\left.\frac{\partial c_i}{\partial r}\right|_{R_i^-}=D_{i+1}\left.\frac{\partial c_{i+1}}{\partial r}\right|_{R_i^+}
```

Outer surface, Case A (absorbing) and Case B (Robin):

```math
c_5(R,t)=0\qquad\text{or}\qquad -D_5\left.\frac{\partial c_5}{\partial r}\right|_R=h\,[c_5(R,t)-c_\infty]
```

Initial data and source, Case A:

```math
c(r,0)=\begin{cases}c_0,&0\le r<R_1\\0,&R_1<r<R\end{cases}\qquad S_i=0\ (t>0)
```

## 4. Exact solution of Case A

### 4.1 Separation and transformation

Set $c=\phi(r)\,e^{-\Lambda t}$. Region by region (the product $r^2D\,\phi'$ is the conserved flux quantity):

```math
\frac{1}{r^2}\frac{d}{dr}\left(r^2D_i\frac{d\phi}{dr}\right)+\Lambda\phi=0
```

With $u=r\phi$ (Equation TRISO-ML-431) this becomes, in each region,

**Equation TRISO-ML-432**

```math
u''+k_i^2u=0,\qquad k_i^2=\frac{\Lambda}{D_i}
```

**Equation TRISO-ML-433**

```math
u_i(r)=A_i\sin(k_ir)+B_i\cos(k_ir),\qquad \phi_i=\frac{u_i}{r}
```

Regularity at the centre removes $B_1\cos(k_1r)/r$, so $B_1=0$ (TRISO-ML-435).

### 4.2 Conditions in the transformed variable

Concentration continuity $\phi_i(R_i)=\phi_{i+1}(R_i)$ gives, after multiplying by $R_i$,

**Equation TRISO-ML-439**

```math
u_i(R_i)=u_{i+1}(R_i)
```

Flux continuity uses $\phi'=(u'-u/r)/r$:

**Equation TRISO-ML-446**

```math
D_i\left[u_i'(R_i)-\frac{u_i(R_i)}{R_i}\right]=D_{i+1}\left[u_{i+1}'(R_i)-\frac{u_{i+1}(R_i)}{R_i}\right]
```

The outer condition for Case A is $u_5(R)=0$.

**Caution.** The quantity $D\,u'$ is not continuous across an interface when the diffusivities differ. Matching $D\,u'$ instead of Equation TRISO-ML-446 gives wrong eigenvalues and wrong release curves.

### 4.3 Characteristic function

Carry the state $(u,q)$ with $q=D\,u'$ from the centre outward. Inside a region of thickness $L$ (solution of Equation TRISO-ML-432):

**Equation TRISO-DEL-010**

```math
\begin{pmatrix}u\\ q\end{pmatrix}_{\mathrm{out}}=\begin{pmatrix}\cos k L & \dfrac{\sin kL}{Dk}\\[2mm] -Dk\sin kL & \cos kL\end{pmatrix}\begin{pmatrix}u\\ q\end{pmatrix}_{\mathrm{in}}
```

Across the interface at $R_i$, Equation TRISO-ML-439 keeps $u$ and Equation TRISO-ML-446 gives

**Equation TRISO-DEL-011**

```math
q_{i+1}(R_i)=q_i(R_i)+\left(D_{i+1}-D_i\right)\frac{u(R_i)}{R_i}
```

Start from $(u,q)=(0,1)$ at $r=0$ (regular solution), propagate through all five regions, and define

**Equation TRISO-DEL-012**

```math
g(\Lambda)=u(R;\Lambda),\qquad g(\Lambda_n)=0
```

The roots of $g$ are the eigenvalues. This is the same condition as $\det\mathbf M(\Lambda)=0$ (TRISO-ML-474) for the global coefficient system, evaluated by propagation instead of by a determinant.

**Completeness of the root list.** The number of eigenvalues not exceeding $\Lambda$ equals the number of zeros of the regular solution in $(0,R]$ (Sturm oscillation count). Isolating roots by bisection on this integer count cannot skip a root.

### 4.4 Orthogonality and release series

With weight $r^2$ for $\phi$, i.e. weight 1 for $u$, the interface and boundary terms cancel and (TRISO-ML-528, TRISO-ML-531)

```math
\int_0^R u_m u_n\,dr=0\qquad(m\ne n)
```

Expanding the initial data, $c=\sum_nA_n\phi_ne^{-\Lambda_nt}$ with $A_n=c_0\,I^{(1)}_n/N_n$, where

**Equation TRISO-DEL-020**

```math
I^{(1)}_n=\int_0^{R_1}r\,u_n\,dr,\qquad I_n=\int_0^{R}r\,u_n\,dr,\qquad N_n=\int_0^{R}u_n^2\,dr
```

The inventory is $M(t)=4\pi\sum_nA_nI_ne^{-\Lambda_nt}$ and $M^0=\tfrac{4\pi}{3}c_0R_1^3$. Hence

**Equation TRISO-DEL-021**

```math
F(t)=1-\sum_{n}w_n\,e^{-\Lambda_nt},\qquad w_n=\frac{3\,I^{(1)}_nI_n}{R_1^3\,N_n},\qquad\sum_nw_n=1
```

The integrals are evaluated region by region from the propagated states, using $u(r)=u\cos ks+(q/Dk)\sin ks$ with $s=r-R_{i-1}$.

**Homogeneous check.** For equal $D$, $u_n=\sin(n\pi r/R)$, $\Lambda_n=n^2\pi^2D/R^2$ and $N_n=R/2$. When the initial data fill the whole sphere ($R_1=R$) this gives $w_n=6/(n^2\pi^2)$, the classical result $F=1-\tfrac{6}{\pi^2}\sum n^{-2}e^{-n^2\pi^2Dt/R^2}$.

### 4.5 Status and numerical limits

- **Completeness and truncation.** Orthogonality and the interface and boundary cancellations are derived in the foundation document. Completeness of the eigenfunctions for this singular piecewise problem is not proved there, so the series is checked numerically against an independent implementation and through the weight sum $\sum_nw_n\to1$. Modes are kept up to $\Lambda_{\max}$ and the series is used only for $t\ge40/\Lambda_{\max}$, where omitted modes are suppressed by $e^{-40}$.
- **Conditioning.** When the flux $D(u'-u/r)$ at an interface is much smaller than $D\,u'$ and $D\,u/r$ (strongly contrasting adjacent diffusivities), the propagation loses digits by cancellation. The code reports the smallest value of $|D(u'-u/r)|/(|Du'|+|Du/r|)$ over modes and interfaces, and an accuracy floor of about $2.2\times10^{-16}$ divided by that ratio. Differences between FV and the series below that floor are not resolved.

### 4.6 Homogeneous Robin sphere (Case B reference)

For $\phi=\sin(zr/R)/r$ and $\mathrm{Bi}=hR/D$, the condition $-D\phi'(R)=h\phi(R)$ gives

```math
z\cot z=1-\mathrm{Bi}
```

## 5. Conservative spherical finite volumes

### 5.1 Geometry

Cell $j$ occupies $r_{j-1/2}<r<r_{j+1/2}$ with interfaces aligned to cell faces.

**Equation TRISO-FV-101**

```math
V_j=\frac{4\pi}{3}\left(r_{j+1/2}^3-r_{j-1/2}^3\right),\qquad A_{j\pm1/2}=4\pi r_{j\pm1/2}^2
```

The unknown is the exact cell average $C_j=V_j^{-1}\int c\,4\pi r^2dr$ (TRISO-FV-118) represented at the volume centroid

**Equation TRISO-FV-109A**

```math
r_j=\frac{3}{4}\,\frac{r_{j+1/2}^4-r_{j-1/2}^4}{r_{j+1/2}^3-r_{j-1/2}^3}
```

### 5.2 Exact cell balance

Multiply Equation TRISO-FV-100 by $4\pi r^2$ and integrate over the cell. The derivative term integrates exactly, $4\pi\left[r^2D\,\partial_rc\right]_{r_{j-1/2}}^{r_{j+1/2}}$. With $J_r=-D\,\partial_rc$ (TRISO-FV-114) this equals $A_{j-1/2}J_{j-1/2}-A_{j+1/2}J_{j+1/2}$, so

**Equation TRISO-FV-123**

```math
V_j\frac{dC_j}{dt}=A_{j-1/2}J_{j-1/2}-A_{j+1/2}J_{j+1/2}+S_jV_j
```

This is exact before any approximation of the face fluxes.

### 5.3 Face fluxes

Inside one material, with $\delta r_{PE}=r_E-r_P$ (TRISO-FV-124 to TRISO-FV-128):

```math
J_e\approx D\,\frac{C_P-C_E}{\delta r_{PE}}
```

At a face between materials, let $\delta_P=r_f-r_P$, $\delta_E=r_E-r_f$ and let $C_I$ be the (continuous) interface concentration. Assume $C$ varies linearly over each half cell. The same flux crosses both halves:

```math
J_f=D_P\frac{C_P-C_I}{\delta_P}=D_E\frac{C_I-C_E}{\delta_E}
```

Eliminating $C_I$ (TRISO-FV-133 to TRISO-FV-138) gives $C_P-C_E=J_f\left(\delta_P/D_P+\delta_E/D_E\right)$, hence the conductance

**Equation TRISO-FV-140**

```math
A_fJ_f=G_f\,(C_P-C_E),\qquad G_f=\frac{A_f}{\delta_P/D_P+\delta_E/D_E}
```

For $D_P=D_E$ this is the ordinary two-point form, and for equal half distances it contains the harmonic mean $2D_PD_E/(D_P+D_E)$. The linear half-cell assumption with the face area $A_f$ in both halves is the approximation made by the scheme. Comparison with the exact solution of Section 4 shows second-order convergence, including across material interfaces.

### 5.4 Semi-discrete equation and conservation

Substituting Equation TRISO-FV-140 into Equation TRISO-FV-123, with west and east conductances $G_W$ and $G_E$:

**Equation TRISO-FV-155**

```math
V_j\frac{dC_j}{dt}=G_W\,(C_{j-1}-C_j)+G_E\,(C_{j+1}-C_j)+S_jV_j
```

The flux $G_f(C_E-C_P)$ leaves one cell and enters its neighbour with opposite sign, so all internal fluxes cancel exactly in $\sum_jV_jC_j$.

### 5.5 Centre and outer boundary

The first cell has $r_{1/2}=0$, so its west face area and $G_W$ vanish. No ghost cell is needed.

For the absorbing surface $c(R,t)=0$ and the last centroid $r_N$,

**Equation TRISO-DEL-030**

```math
G_R=\frac{4\pi R^2D_5}{R-r_N},\qquad V_N\frac{dC_N}{dt}=G_W(C_{N-1}-C_N)-G_RC_N,\qquad \dot M_{\mathrm{rel}}=G_RC_N
```

### 5.6 Backward Euler

For time step $\Delta t$:

```math
\frac{V_j}{\Delta t}\left(C_j^{n+1}-C_j^n\right)=G_W\left(C_{j-1}^{n+1}-C_j^{n+1}\right)+G_E\left(C_{j+1}^{n+1}-C_j^{n+1}\right)+S_j^{n+1}V_j
```

**Equation TRISO-DEL-040**

```math
-G_WC_{j-1}^{n+1}+\left(\frac{V_j}{\Delta t}+G_W+G_E\right)C_j^{n+1}-G_EC_{j+1}^{n+1}=\frac{V_j}{\Delta t}C_j^n+S_j^{n+1}V_j
```

The matrix is tridiagonal with positive diagonal and non-positive off-diagonal entries and is strictly diagonally dominant, hence an M-matrix. For non-negative old data and source the new concentrations are non-negative.

**Subtraction-free elimination.** Let $v_j=V_j/\Delta t$ and $g_j^W=G_W$ of cell $j$ (so $g^W_{j+1}=g^E_j$). Define $e_1=v_1$ and $e_j=v_j+g^W_je_{j-1}/(e_{j-1}+g^W_j)$, pivots $d_j=e_j+g^E_j$, forward sweep $y_j=(\text{rhs}_j+g^W_jy_{j-1})/d_j$, and back substitution $C_j=y_j+(g^E_j/d_j)\,C_{j+1}$ with $C_N=y_N$. Every operation adds, multiplies or divides non-negative numbers. The textbook pivot $b_j-a_jc_{j-1}'$ subtracts nearly equal numbers when the Fourier number $D\Delta t/\Delta r^2$ is large and degrades the inventory balance to about $10^{-9}$. The form above keeps it below $10^{-12}$ (observed $\le3\times10^{-13}$).

For verification only, Forward Euler has the coefficient-positivity bound

**Equation TRISO-FV-300**

```math
\Delta t\le\min_j\frac{V_j}{G_W+G_E}
```

Backward Euler is the production scheme and has no such restriction.

### 5.7 Inventory, release, balance

```math
M^n=\sum_jV_jC_j^n,\qquad M_{\mathrm{rel}}^{n+1}=M_{\mathrm{rel}}^n+\Delta t\,G_RC_N^{n+1},\qquad F^n=\frac{M_{\mathrm{rel}}^n}{M^0},\qquad \dot F^n=\frac{G_RC_N^n}{M^0}
```

Summing Equation TRISO-DEL-040 over $j$ cancels all internal conductances and leaves

```math
M^{n+1}+M_{\mathrm{rel}}^{n+1}=M^0
```

up to linear-solver and rounding error. The release uses the same boundary flux as the balance, so the residual $M^n+M_{\mathrm{rel}}^n-M^0$ is an independent diagnostic.

## 6. Limitations

- Results obtained with the synthetic test inputs supplied with the code are numerical checks, not physical validation.
- Temperature and fluence dependence of $D$, source schedules and a production Robin release mode are not implemented.
- The exact series is trustworthy only above the accuracy floor that the code reports for each run. The floor grows roughly in proportion to the diffusivity contrast between adjacent layers.
- Completeness of the eigenfunction set for the singular piecewise problem is checked numerically, not proved.
