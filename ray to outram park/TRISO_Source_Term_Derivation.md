# First-Principles Derivation and Verification of Diffusive Source-Term Release from Multilayer TRISO Fuel

**Detailed derivation manuscript. The hard five-layer transform is independently verified with non-blocking findings. Controlled inverse-Laplace verification is authorized but has not yet been executed.**

This is the single continuous derivation path for the Ray TRISO project. It preserves the user's original notebook as a separate evidence stream and incorporates the corrections established during the foundation audit.

The supplied LaTeX notebook contains 33 substantive displayed equations. Every one is mapped in 13_canonical_equation_register.md.

## 1. Physical problem

A TRISO particle has five concentric regions: fuel kernel, buffer, IPyC, SiC, and OPyC. A fission-product species is created in the kernel, diffuses through the layers, may decay or undergo other modelled reactions, and may leave the particle.

Let $r$ be distance from the particle centre, $t$ be time, $c_i(r,t)$ be concentration in layer $i$, and $D_i$ be the layer diffusivity.

[ASSUMPTION] One representative spherical particle.
[ASSUMPTION] Spherical symmetry.
[ASSUMPTION] Each layer is homogeneous during one model evaluation.
[ASSUMPTION] Initial ideal interfaces have no interfacial storage or resistance.
[QUESTION FOR SUPERVISOR] The first canonical species, decay/trapping model, partition coefficients, and coolant concentration still need to be fixed.

## 1.1 Nomenclature

The notation is grouped here for reference; important symbols are also defined locally at first use.

| Group | Symbol | Definition | SI unit |
|---|---|---|---|
| Geometry | $r$ | radial coordinate from particle centre | m |
| Geometry | $R_i$ | outer radius of TRISO layer $i$ | m |
| Transport | $c_i$ | concentration of tracked species in layer $i$ | $\mathrm{mol\,m^{-3}}$ |
| Transport | $D_i$ | diffusion coefficient in layer $i$ | $\mathrm{m^2\,s^{-1}}$ |
| Transport | $\mathbf J$ | diffusive molar flux | $\mathrm{mol\,m^{-2}\,s^{-1}}$ |
| Sources | $S_i$ | net volumetric production term | $\mathrm{mol\,m^{-3}\,s^{-1}}$ |
| FV | $V_P$ | spherical control-volume volume | $\mathrm{m^3}$ |
| FV | $A_f$ | spherical face area | $\mathrm{m^2}$ |
| FV | $G_f$ | diffusive face conductance | $\mathrm{m^3\,s^{-1}}$ |
| Stochastic | $T$ | first-passage or release time | s |
| Stochastic | $\epsilon$ | interface capture distance | m |
| Stochastic | $\delta=\alpha\epsilon$ | reinsertion displacement | m |
| First passage | $G_a,G_b$ | inner/outer joint exit-time Laplace transforms | dimensionless |
| First passage | $H$ | centred-ball first-exit Laplace transform | dimensionless |
| Renewal | $\Phi_i(s)$ | release-time transform from state $i$ | dimensionless |
| Renewal | $\mathbf K(s)$ | transient renewal-transform matrix | dimensionless |
| Renewal | $\mathbf B(s)$ | direct-absorption transform vector | dimensionless |
| Numerical | $\kappa_2$ | spectral 2-norm condition number | dimensionless |

## 2. Conservation from a control volume

[EXACT] Begin with the amount of the conserved species inside an arbitrary fixed control volume V.

**Equation TRISO-GOV-020**

```math
N_V(t)=\int_V c(\mathbf{x},t)\,dV.
```

The units are

**Equation TRISO-GOV-021**

```math
[N_V]=\mathrm{mol}.
```

[EXACT] The rate of accumulation is

**Equation TRISO-GOV-022**

```math
\frac{dN_V}{dt}=\frac{d}{dt}\int_V c\,dV.
```

Let $\mathbf J$ be the diffusive molar flux vector. Its units are

**Equation TRISO-GOV-023**

```math
[\mathbf J]=\mathrm{mol\,m^{-2}\,s^{-1}}.
```

Let $S$ be the net volumetric production rate. Its units are

**Equation TRISO-GOV-024**

```math
[S]=\mathrm{mol\,m^{-3}\,s^{-1}}.
```

[EXACT] For outward unit normal $\mathbf n$, the outward amount crossing a boundary element in time $dt$ is proportional to $\mathbf J\cdot\mathbf n$. The outward rate is therefore

**Equation TRISO-GOV-025**

```math
\dot N_{\mathrm{out}}=\int_{\partial V}\mathbf J\cdot\mathbf n\,dA.
```

[EXACT] The production rate inside the control volume is

**Equation TRISO-GOV-026**

```math
\dot N_{\mathrm{gen}}=\int_V S\,dV.
```

[EXACT] Accumulation equals production minus outward flux:

**Equation TRISO-GOV-027**

```math
\frac{dN_V}{dt}=-\dot N_{\mathrm{out}}+\dot N_{\mathrm{gen}}.
```

Substitute the definitions of the two rates:

**Equation TRISO-GOV-028**

```math
\frac{d}{dt}\int_V c\,dV=-\int_{\partial V}\mathbf J\cdot\mathbf n\,dA+\int_V S\,dV.
```

[EXACT] Because $V$ is fixed in space, the time derivative passes through the volume integral:

**Equation TRISO-GOV-029**

```math
\frac{d}{dt}\int_V c\,dV=\int_V\frac{\partial c}{\partial t}\,dV.
```

[EXACT] Apply the divergence theorem to the surface term:

**Equation TRISO-GOV-030**

```math
\int_{\partial V}\mathbf J\cdot\mathbf n\,dA=\int_V\nabla\cdot\mathbf J\,dV.
```

Substitute this result:

**Equation TRISO-GOV-031**

```math
\int_V\frac{\partial c}{\partial t}\,dV=-\int_V\nabla\cdot\mathbf J\,dV+\int_VS\,dV.
```

Move the flux and source terms into one integrand:

**Equation TRISO-GOV-032**

```math
\int_V\left(\frac{\partial c}{\partial t}+\nabla\cdot\mathbf J-S\right)dV=0.
```

[EXACT] Since the control volume is arbitrary, the integrand must vanish almost everywhere:

**Equation TRISO-GOV-033**

```math
\boxed{\frac{\partial c}{\partial t}+\nabla\cdot\mathbf J=S.}
```


## 3. Constitutive law and general heterogeneous diffusion equation

[CONSTITUTIVE] Fickian diffusion relates flux to the concentration gradient:

**Equation TRISO-GOV-034**

```math
\boxed{\mathbf J=-D\nabla c.}
```

The gradient has units

**Equation TRISO-GOV-035**

```math
[\nabla c]=\mathrm{mol\,m^{-4}}.
```

Multiplying by $D$ gives

**Equation TRISO-GOV-036**

```math
[D\nabla c]=\mathrm{m^2\,s^{-1}}\times\mathrm{mol\,m^{-4}}.
```

Hence

**Equation TRISO-GOV-037**

```math
[D\nabla c]=\mathrm{mol\,m^{-2}\,s^{-1}},
```
which matches the flux units.

[EXACT] Substitute Fick's law into conservation:

**Equation TRISO-GOV-038**

```math
\frac{\partial c}{\partial t}+\nabla\cdot(-D\nabla c)=S.
```

[EXACT] Pull the minus sign through the divergence:

**Equation TRISO-GOV-039**

```math
\frac{\partial c}{\partial t}-\nabla\cdot(D\nabla c)=S.
```

[EXACT] Rearrange:

**Equation TRISO-GOV-040**

```math
\boxed{\frac{\partial c}{\partial t}=\nabla\cdot(D\nabla c)+S.}
```

[IMPORTANT] This is the general conservative form. The diffusivity must remain inside the divergence until a later layer-specific assumption establishes that it is constant with respect to the differentiated coordinate.


## 4. Spherical-coordinate derivation

[EXACT] In spherical coordinates, the gradient of a scalar field is

**Equation TRISO-SPH-020**

```math
\nabla c=\mathbf e_r\frac{\partial c}{\partial r}+\mathbf e_\theta\frac1r\frac{\partial c}{\partial\theta}+\mathbf e_\varphi\frac1{r\sin\theta}\frac{\partial c}{\partial\varphi}.
```

[EXACT] Write a general vector flux as $\mathbf J=J_r\mathbf e_r+J_\theta\mathbf e_\theta+J_\varphi\mathbf e_\varphi$.

[EXACT] Its spherical divergence is

**Equation TRISO-SPH-021**

```math
\nabla\cdot\mathbf J=\frac1{r^2}\frac{\partial}{\partial r}(r^2J_r)+\frac1{r\sin\theta}\frac{\partial}{\partial\theta}(\sin\theta J_\theta)+\frac1{r\sin\theta}\frac{\partial J_\varphi}{\partial\varphi}.
```

[ASSUMPTION] Spherical symmetry means the concentration is independent of both angular coordinates:

**Equation TRISO-SPH-022**

```math
c=c(r,t).
```

Therefore

**Equation TRISO-SPH-023**

```math
\frac{\partial c}{\partial\theta}=0.
```

and

**Equation TRISO-SPH-024**

```math
\frac{\partial c}{\partial\varphi}=0.
```

Substitute these zero angular derivatives into the gradient:

**Equation TRISO-SPH-025**

```math
\nabla c=\mathbf e_r\frac{\partial c}{\partial r}.
```

[CONSTITUTIVE] Fick's law therefore becomes radial:

**Equation TRISO-SPH-026**

```math
\boxed{\mathbf J=-D(r,t)\frac{\partial c}{\partial r}\mathbf e_r.}
```

Thus

**Equation TRISO-SPH-027**

```math
J_\theta=0.
```

and

**Equation TRISO-SPH-028**

```math
J_\varphi=0.
```

[EXACT] Insert the zero angular fluxes into the divergence:

**Equation TRISO-SPH-029**

```math
\nabla\cdot\mathbf J=\frac1{r^2}\frac{\partial}{\partial r}(r^2J_r).
```

The radial flux component is

**Equation TRISO-SPH-030**

```math
J_r=-D(r,t)\frac{\partial c}{\partial r}.
```

Substitution gives

**Equation TRISO-SPH-031**

```math
\nabla\cdot\mathbf J=\frac1{r^2}\frac{\partial}{\partial r}\left(-r^2D(r,t)\frac{\partial c}{\partial r}\right).
```

Insert this into conservation:

**Equation TRISO-SPH-032**

```math
\frac{\partial c}{\partial t}+\frac1{r^2}\frac{\partial}{\partial r}\left(-r^2D(r,t)\frac{\partial c}{\partial r}\right)=S(r,t).
```

[EXACT] Move the negative term to the right:

**Equation TRISO-SPH-033**

```math
\boxed{\frac{\partial c}{\partial t}=\frac1{r^2}\frac{\partial}{\partial r}\left(r^2D(r,t)\frac{\partial c}{\partial r}\right)+S(r,t).}
```


## 4.1 Specialisation to one homogeneous layer

[ASSUMPTION] In one material layer $i$, the benchmark assumes the diffusivity is constant with respect to radius and time during the analysis:

**Equation TRISO-SPH-034**

```math
D(r,t)=D_i.
```

[EXACT] Substitute $D_i$ into the conservative equation:

**Equation TRISO-SPH-035**

```math
\frac{\partial c_i}{\partial t}=\frac1{r^2}\frac{\partial}{\partial r}\left(r^2D_i\frac{\partial c_i}{\partial r}\right)+S_i.
```

[EXACT] Because $D_i$ is constant with respect to $r$, take it outside the derivative:

**Equation TRISO-SPH-036**

```math
\frac{\partial c_i}{\partial t}=\frac{D_i}{r^2}\frac{\partial}{\partial r}\left(r^2\frac{\partial c_i}{\partial r}\right)+S_i.
```

[EXACT] Apply the product rule:

**Equation TRISO-SPH-037**

```math
\frac{\partial}{\partial r}\left(r^2\frac{\partial c_i}{\partial r}\right)=\frac{\partial r^2}{\partial r}\frac{\partial c_i}{\partial r}+r^2\frac{\partial^2c_i}{\partial r^2}.
```

Differentiate $r^2$:

**Equation TRISO-SPH-038**

```math
\frac{\partial r^2}{\partial r}=2r.
```

Substitute:

**Equation TRISO-SPH-039**

```math
\frac{\partial}{\partial r}\left(r^2\frac{\partial c_i}{\partial r}\right)=2r\frac{\partial c_i}{\partial r}+r^2\frac{\partial^2c_i}{\partial r^2}.
```

Divide by $r^2$:

**Equation TRISO-SPH-040**

```math
\frac1{r^2}\frac{\partial}{\partial r}\left(r^2\frac{\partial c_i}{\partial r}\right)=\frac2r\frac{\partial c_i}{\partial r}+\frac{\partial^2c_i}{\partial r^2}.
```

Therefore:

**Equation TRISO-SPH-041**

```math
\boxed{\frac{\partial c_i}{\partial t}=D_i\left(\frac{\partial^2c_i}{\partial r^2}+\frac2r\frac{\partial c_i}{\partial r}\right)+S_i.}
```

[IMPORTANT] Equation TRISO-SPH-041 is a within-layer constant-diffusivity equation. It must not be differentiated through a discontinuous material interface.

## 5. Source and reaction terms

The conservation equation contains a net volumetric source term. We now separate the physical processes that may contribute to that term.

[EXACT] Define the net source in material layer i as

**Equation TRISO-GOV-100**

```math
S_i=S_{i,\mathrm{gen}}+S_{i,\mathrm{other}}+S_{i,\mathrm{release}}-S_{i,\mathrm{decay}}-S_{i,\mathrm{trap}}.
```

Each term has units

**Equation TRISO-GOV-101**

```math
[S_{i,\mathrm{gen}}]
=
[S_{i,\mathrm{other}}]
=
[S_{i,\mathrm{release}}]
=
[S_{i,\mathrm{decay}}]
=
[S_{i,\mathrm{trap}}]
=
\mathrm{mol\,m^{-3}\,s^{-1}}.
```

This equation is a bookkeeping definition. It does not yet choose a constitutive model for trapping, release, or generation.

### 5.1 Fission-product generation

Let $S_{i,\mathrm{gen}}$ denote the local rate at which the tracked species is created.

[ASSUMPTION] For the simplest TRISO source benchmark, generation is confined to the fuel kernel:

**Equation TRISO-GOV-102**

```math
S_{i,\mathrm{gen}}=
\begin{cases}
S_0,&0\le r<r_1,\\
0,&r_1<r<R.
\end{cases}
```

Here

**Equation TRISO-GOV-103**

```math
[S_0]=\mathrm{mol\,m^{-3}\,s^{-1}}.
```

A microscopic fission-based expression such as a fission rate multiplied by a product yield would require a selected species, fission cross section, neutron flux, and yield correlation. Those inputs are not fixed by the current project evidence.

[SOURCE NEEDED] Species-specific generation correlation and parameter values.

### 5.2 Radioactive decay

Suppose the tracked atoms disappear by radioactive decay independently with a constant decay probability per unit time.

Let $\lambda_d$ be the decay constant:

**Equation TRISO-GOV-104**

```math
[\lambda_d]=\mathrm{s^{-1}}.
```

Consider an amount $N$ of the tracked species.

During a short time interval $dt$, the expected fraction that decays is proportional to $\lambda_d dt$:

**Equation TRISO-GOV-105**

```math
dN_{\mathrm{decay}}=\lambda_d N\,dt.
```

Because decay removes atoms from the tracked species, the change in tracked inventory is negative:

**Equation TRISO-GOV-106**

```math
dN=-dN_{\mathrm{decay}}.
```

Substitute the decay amount:

**Equation TRISO-GOV-107**

```math
dN=-\lambda_dN\,dt.
```

Divide by $dt$:

**Equation TRISO-GOV-108**

```math
\frac{dN}{dt}=-\lambda_dN.
```

For a fixed volume element, $N=c\,dV$. Therefore

**Equation TRISO-GOV-109**

```math
\frac{d(c\,dV)}{dt}=-\lambda_dc\,dV.
```

For a fixed volume element, $dV$ is constant in time:

**Equation TRISO-GOV-110**

```math
\frac{\partial c}{\partial t}=-\lambda_dc.
```

Thus the local decay sink has the form

**Equation TRISO-GOV-111**

```math
\boxed{
S_{i,\mathrm{decay}}=\lambda_{d,i}c_i
}
```

and it enters the conservation equation with a minus sign.

Dimensional check:

**Equation TRISO-GOV-112**

```math
[\lambda_dc]
=
\mathrm{s^{-1}}\times\mathrm{mol\,m^{-3}}
=
\mathrm{mol\,m^{-3}\,s^{-1}}.
```

[CONSTITUTIVE] The first-order decay assumption is the mathematical statement that each tracked atom has the same constant decay hazard $\lambda_d$, independent of concentration.

[ASSUMPTION] The current base benchmark sets this decay contribution to zero:

**Equation TRISO-GOV-113**

```math
S_{i,\mathrm{decay}}=0.
```

### 5.3 Trapping and release from traps

Trapping is a transfer of the tracked species from the mobile population into a trapped population. Release is the reverse transfer.

Define

**Equation TRISO-GOV-114**

```math
S_{i,\mathrm{trap}}
```

as the positive rate at which mobile species are removed into traps, and

**Equation TRISO-GOV-115**

```math
S_{i,\mathrm{release}}
```

as the positive rate at which trapped species are returned to the mobile population.

Their units are

**Equation TRISO-GOV-116**

```math
[S_{i,\mathrm{trap}}]
=
[S_{i,\mathrm{release}}]
=
\mathrm{mol\,m^{-3}\,s^{-1}}.
```

The mobile-species equation therefore contains

**Equation TRISO-GOV-117**

```math
S_{i,\mathrm{release}}-S_{i,\mathrm{trap}}.
```

No first-order, saturation, occupancy, or irradiation-dependent trapping law is assumed here.

[QUESTION FOR SUPERVISOR] Select the physical trapping/release constitutive model, if trapping is required in the final species model.

### 5.4 General reaction-inclusive equation

Substitute the separated source terms into the conservation equation:

**Equation TRISO-GOV-118**

```math
\frac{\partial c_i}{\partial t}
=
\frac{1}{r^2}
\frac{\partial}{\partial r}
\left(
r^2D_i\frac{\partial c_i}{\partial r}
\right)
+
S_{i,\mathrm{gen}}
+
S_{i,\mathrm{other}}
+
S_{i,\mathrm{release}}
-
S_{i,\mathrm{decay}}
-
S_{i,\mathrm{trap}}.
```

This is the general layer-wise source/reaction form under spherical symmetry.

[QUESTION FOR SUPERVISOR] Confirm whether the final physical model should contain decay and/or trapping in addition to diffusion.

### 5.5 Two source problems must remain distinct

There are two different physical initial-value problems in this project.

**Problem A: initially empty particle with continuing generation**

The initial inventory is zero:

**Equation TRISO-IC-100**

```math
c_i(r,0)=0.
```

The kernel generation remains active for $t>0$:

**Equation TRISO-IC-101**

```math
S_{1,\mathrm{gen}}=S_0.
```

This is the source-driven problem used by the homogeneous Part-I steady analytical benchmark.

**Problem B: initially loaded kernel with no continuing generation**

The kernel initially contains mobile species:

**Equation TRISO-IC-102**

```math
c_1(r,0)=c_0
\qquad
0\le r<r_1,
```

while the coatings initially contain none:

**Equation TRISO-IC-103**

```math
c_i(r,0)=0
\qquad
r_1<r<R.
```

After $t=0$, the benchmark source is zero:

**Equation TRISO-IC-104**

```math
S_{i,\mathrm{gen}}=0.
```

This is the initial-inventory release problem used by the production WOS verification contract.

These problems have different source terms and different physical histories. They must not be merged into one equation by notation alone.

## 6. Initial, centre, interface and outer conditions

### 6.1 Centre condition from spherical symmetry

At the exact centre there is no preferred radial direction.

[ASSUMPTION] Extend the radial concentration profile evenly through the mathematical origin for the purpose of the local limit:

**Equation TRISO-BC-100**

```math
c(-r,t)=c(r,t).
```

Differentiate this relation with respect to $r$:

**Equation TRISO-BC-101**

```math
-c_r(-r,t)=c_r(r,t).
```

Set $r=0$:

**Equation TRISO-BC-102**

```math
-c_r(0,t)=c_r(0,t).
```

Therefore

**Equation TRISO-BC-103**

```math
\boxed{c_r(0,t)=0.}
```

This is the mathematical expression of spherical symmetry at the centre.

It also agrees with the physical interpretation: a nonzero radial derivative at the centre would select one direction as different from the opposite direction.

### 6.2 Apparent singularity at the origin

The spherical operator contains

**Equation TRISO-BC-104**

```math
\frac{2}{r}c_r.
```

At $r=0$, this expression is of the form $0/0$ for a smooth symmetric field, so it must not be evaluated by direct substitution.

Because $c_r(0,t)=0$, apply L'Hôpital's rule:

**Equation TRISO-BC-105**

```math
\lim_{r\to0}\frac{c_r(r,t)}{r}
=
\lim_{r\to0}\frac{c_{rr}(r,t)}{1}.
```

Therefore

**Equation TRISO-BC-106**

```math
\lim_{r\to0}\frac{c_r(r,t)}{r}
=
c_{rr}(0,t).
```

Multiply by 2:

**Equation TRISO-BC-107**

```math
\lim_{r\to0}\frac{2}{r}c_r(r,t)
=
2c_{rr}(0,t).
```

The full spherical diffusion operator therefore has the centre limit

**Equation TRISO-BC-108**

```math
\lim_{r\to0}
\left(
c_{rr}+\frac2r c_r
\right)
=
c_{rr}(0,t)+2c_{rr}(0,t).
```

Collect the two identical curvature contributions:

**Equation TRISO-BC-109**

```math
\boxed{
\lim_{r\to0}
\left(
c_{rr}+\frac2r c_r
\right)
=
3c_{rr}(0,t).
}
```

This is the continuum origin of the factor 3 that later becomes the factor 6 in the central FTCS stencil.

[ASSUMPTION] The limit requires sufficient smoothness of the radial field near the origin.

### 6.3 Material-interface conservation

Consider an infinitesimally thin spherical control volume surrounding interface $r=r_k$.

Let its inner radius be $r_k-\varepsilon$ and its outer radius be $r_k+\varepsilon$.

The volume is

**Equation TRISO-INT-100**

```math
V_\varepsilon
=
\frac{4\pi}{3}
\left[
(r_k+\varepsilon)^3-(r_k-\varepsilon)^3
\right].
```

As $\varepsilon\to0$,

**Equation TRISO-INT-101**

```math
V_\varepsilon\to0.
```

The inward diffusive amount rate crossing the inner surface is

**Equation TRISO-INT-102**

```math
4\pi(r_k-\varepsilon)^2J_{r,k}^{-}.
```

The outward diffusive amount rate crossing the outer surface is

**Equation TRISO-INT-103**

```math
4\pi(r_k+\varepsilon)^2J_{r,k}^{+}.
```

Let $\Gamma_k$ be any explicitly modelled interfacial inventory per unit area, and let $g_k$ be any explicitly modelled interfacial production rate per unit area.

Their units are

**Equation TRISO-INT-104**

```math
[\Gamma_k]=\mathrm{mol\,m^{-2}},
\qquad
[g_k]=\mathrm{mol\,m^{-2}\,s^{-1}}.
```

The interface balance is

**Equation TRISO-INT-105**

```math
\frac{d}{dt}
\left(
4\pi r_k^2\Gamma_k
\right)
=
4\pi(r_k-\varepsilon)^2J_{r,k}^{-}
-
4\pi(r_k+\varepsilon)^2J_{r,k}^{+}
+
4\pi r_k^2g_k
+
o(1).
```

Divide by $4\pi r_k^2$:

**Equation TRISO-INT-106**

```math
\frac{d\Gamma_k}{dt}
=
\left(
\frac{r_k-\varepsilon}{r_k}
\right)^2
J_{r,k}^{-}
-
\left(
\frac{r_k+\varepsilon}{r_k}
\right)^2
J_{r,k}^{+}
+
g_k
+
o(1).
```

Take the zero-thickness limit:

**Equation TRISO-INT-107**

```math
\frac{d\Gamma_k}{dt}
=
J_{r,k}^{-}
-
J_{r,k}^{+}
+
g_k.
```

Therefore the general interface jump condition is

**Equation TRISO-INT-108**

```math
\boxed{
J_{r,k}^{-}-J_{r,k}^{+}
=
\frac{d\Gamma_k}{dt}-g_k.
}
```

For an ideal interface with no interfacial storage and no interfacial generation,

**Equation TRISO-INT-109**

```math
\Gamma_k=0,
\qquad
g_k=0.
```

Hence

**Equation TRISO-INT-110**

```math
J_{r,k}^{-}=J_{r,k}^{+}.
```

Thus

**Equation TRISO-INT-111**

```math
\boxed{
J_{r,k}^{-}=J_{r,k}^{+}.
}
```

Now substitute Fick's law on the inner side:

**Equation TRISO-INT-112**

```math
J_{r,k}^{-}
=
-D_k
\frac{\partial c_k}{\partial r}
\bigg|_{r_k^-}.
```

Substitute Fick's law on the outer side:

**Equation TRISO-INT-113**

```math
J_{r,k}^{+}
=
-D_{k+1}
\frac{\partial c_{k+1}}{\partial r}
\bigg|_{r_k^+}.
```

Equate them:

**Equation TRISO-INT-114**

```math
\boxed{
-D_k
\frac{\partial c_k}{\partial r}
\bigg|_{r_k^-}
=
-D_{k+1}
\frac{\partial c_{k+1}}{\partial r}
\bigg|_{r_k^+}.
}
```

This condition came from conservation. It did not require concentration continuity.

### 6.4 Concentration continuity is a separate interface assumption

Flux continuity answers the question:

> Is species amount conserved across the zero-thickness interface?

It does not answer:

> What equilibrium relation connects the two concentrations at the interface?

For an ideal perfectly equilibrated interface, one may impose concentration continuity:

**Equation TRISO-INT-115**

```math
\boxed{
c_k(r_k,t)=c_{k+1}(r_k,t).
}
```

[ASSUMPTION] This is an ideal-interface constitutive/equilibrium assumption.

A species-specific partition coefficient $K_k$ instead gives a different relation:

**Equation TRISO-INT-116**

```math
\boxed{
c_{k+1}(r_k,t)=K_kc_k(r_k,t).
}
```

The value and definition of $K_k$ depend on the species and the two materials.

[SOURCE NEEDED] Species-specific partition/solubility data if $K_k\ne1$ is required physically.

### 6.5 Interfacial resistance is a third, distinct model

Partitioning and interfacial resistance are not the same statement.

A finite interfacial mass-transfer coefficient $h_{\mathrm{int}}$ can instead be used in a constitutive resistance law such as

**Equation TRISO-INT-117**

```math
J_{r,k}
=
h_{\mathrm{int}}
\left(
c_k-\frac{c_{k+1}}{K_k}
\right).
```

The units are

**Equation TRISO-INT-118**

```math
[h_{\mathrm{int}}]=\mathrm{m\,s^{-1}}.
```

This relation introduces a finite concentration jump for finite resistance.

[SOURCE NEEDED] The physical interfacial-resistance law and coefficient, if such resistance is required.

[ASSUMPTION] The frozen numerical benchmark uses $K_k=1$ and no explicit interfacial resistance, so (TRISO-INT-115) and (TRISO-INT-117) are not simultaneously imposed.

### 6.6 Outer boundary conditions

At the outer surface $r=R$, define the outward radial flux as

**Equation TRISO-BC-110**

```math
J_R=J_r(R,t).
```

By Fick's law,

**Equation TRISO-BC-111**

```math
J_R
=
-D_5
\frac{\partial c_5}{\partial r}
\bigg|_{R}.
```

#### Dirichlet: absorbing or prescribed surface concentration

The simplest absorbing boundary is

**Equation TRISO-BC-112**

```math
\boxed{
c_5(R,t)=0.
}
```

More generally, a prescribed surface concentration $c_b(t)$ is

**Equation TRISO-BC-113**

```math
c_5(R,t)=c_b(t).
```

The absorbing case is $c_b=0$.

[ASSUMPTION] The production WOS verification benchmark uses the absorbing case.

#### Neumann: prescribed outward flux

A prescribed outward flux is written

**Equation TRISO-BC-114**

```math
\boxed{
J_R=J_b(t).
}
```

Substitute the diffusive flux:

**Equation TRISO-BC-115**

```math
-D_5
\frac{\partial c_5}{\partial r}
\bigg|_R
=
J_b(t).
```

The units are

**Equation TRISO-BC-116**

```math
[J_b]=\mathrm{mol\,m^{-2}\,s^{-1}}.
```

#### Robin: finite external mass transfer

Let the external coolant concentration be $c_\infty(t)$.

[CONSTITUTIVE] A linear external mass-transfer law is

**Equation TRISO-BC-117**

```math
J_R
=
h
\left[
c_5(R,t)-c_\infty(t)
\right].
```

The units of $h$ are

**Equation TRISO-BC-118**

```math
[h]=\mathrm{m\,s^{-1}}.
```

Substitute the diffusive surface flux:

**Equation TRISO-BC-119**

```math
-D_5
\frac{\partial c_5}{\partial r}
\bigg|_R
=
h
\left[
c_5(R,t)-c_\infty(t)
\right].
```

Both sides have units

**Equation TRISO-BC-120**

```math
\mathrm{mol\,m^{-2}\,s^{-1}}.
```

For zero bulk concentration,

**Equation TRISO-BC-121**

```math
c_\infty=0,
```

so

**Equation TRISO-BC-122**

```math
-D_5c_5'(R,t)=hc_5(R,t).
```

For very large $h$, a finite flux requires

**Equation TRISO-BC-123**

```math
c_5(R,t)-c_\infty(t)\to0.
```

Thus the Robin condition approaches the Dirichlet condition

**Equation TRISO-BC-124**

```math
c_5(R,t)=c_\infty(t).
```

For $h\to0$,

**Equation TRISO-BC-125**

```math
J_R\to0,
```

which approaches the zero-flux Neumann condition.

The three outer-boundary models are therefore mathematically distinct:

```math
\text{Dirichlet: concentration prescribed},
```

```math
\text{Neumann: flux prescribed},
```

```math
\text{Robin: flux responds to concentration difference}.
```

[ASSUMPTION] The Part-I analytical benchmark uses Robin with $c_\infty=0$.

[ASSUMPTION] The production WOS verification benchmark uses absorbing Dirichlet (c_5(R,t)=0).

### 6.7 Initial-condition summary

The initial condition must be selected together with the source model.

Problem A:

**Equation TRISO-IC-105**

```math
c_i(r,0)=0,
\qquad
S_{1,\mathrm{gen}}=S_0.
```

Problem B:

**Equation TRISO-IC-106**

```math
c_1(r,0)=c_0,
\qquad
c_{2..5}(r,0)=0,
\qquad
S_{i,\mathrm{gen}}=0.
```

The first homogeneous analytical Part-I benchmark belongs to Problem A.

The production WOS release benchmark belongs to Problem B.

## 7. Part I homogeneous analytical benchmark

The following solution belongs to **Problem A**: an initially empty homogeneous sphere with a continuing uniform source and a finite-transfer Robin boundary.

[ASSUMPTION] Replace the five-layer particle temporarily by one homogeneous sphere of radius $R$ and constant diffusivity $D$.

The governing equation is

**Equation TRISO-ANA-100**

```math
\frac{\partial c}{\partial t}
=
D
\left(
\frac{\partial^2c}{\partial r^2}
+
\frac2r
\frac{\partial c}{\partial r}
\right)
+
S_0.
```

At steady state the concentration no longer changes with time:

**Equation TRISO-ANA-101**

```math
\frac{\partial w}{\partial t}=0.
```

Therefore

**Equation TRISO-ANA-102**

```math
0
=
D
\left(
\frac{d^2w}{dr^2}
+
\frac2r\frac{dw}{dr}
\right)
+
S_0.
```

Move the source term to the other side:

**Equation TRISO-ANA-103**

```math
D
\left(
w''
+
\frac2r w'
\right)
=
-S_0.
```

Divide by $D$:

**Equation TRISO-ANA-104**

```math
w''
+
\frac2r w'
=
-\frac{S_0}{D}.
```

Multiply by $r^2$:

**Equation TRISO-ANA-105**

```math
r^2w''
+
2rw'
=
-\frac{S_0}{D}r^2.
```

Recognise the product derivative on the left. Verify it explicitly using the product rule:

**Equation TRISO-ANA-106**

```math
\frac{d}{dr}(r^2w')
=
\frac{d r^2}{dr}w'
+
r^2w''.
```

Differentiate $r^2$:

**Equation TRISO-ANA-107**

```math
\frac{d r^2}{dr}=2r.
```

Therefore

**Equation TRISO-ANA-108**

```math
\frac{d}{dr}(r^2w')
=
2rw'+r^2w''.
```

Hence the steady equation becomes

**Equation TRISO-ANA-109**

```math
\frac{d}{dr}(r^2w')
=
-\frac{S_0}{D}r^2.
```

Integrate both sides with respect to $r$:

**Equation TRISO-ANA-110**

```math
\int
\frac{d}{dr}(r^2w')\,dr
=
-\frac{S_0}{D}
\int r^2\,dr.
```

The left-hand integral is

**Equation TRISO-ANA-111**

```math
r^2w'.
```

The right-hand integral is

**Equation TRISO-ANA-112**

```math
-\frac{S_0}{D}\frac{r^3}{3}.
```

Introduce the integration constant $A$:

**Equation TRISO-ANA-113**

```math
r^2w'
=
-\frac{S_0r^3}{3D}
+
A.
```

At the centre, regularity requires $w'(0)$ to remain finite.

If $A\ne0$, then division by $r^2$ gives a term proportional to $1/r^2$, which diverges.

Therefore

**Equation TRISO-ANA-114**

```math
A=0.
```

Substitute $A=0$:

**Equation TRISO-ANA-115**

```math
r^2w'
=
-\frac{S_0r^3}{3D}.
```

For $r>0$, divide by $r^2$:

**Equation TRISO-ANA-116**

```math
w'
=
-\frac{S_0r}{3D}.
```

Integrate again:

**Equation TRISO-ANA-117**

```math
\int dw
=
-\frac{S_0}{3D}\int r\,dr.
```

The left-hand integral is

**Equation TRISO-ANA-118**

```math
w.
```

The right-hand integral is

**Equation TRISO-ANA-119**

```math
-\frac{S_0}{3D}\frac{r^2}{2}.
```

Introduce the second integration constant $B$:

**Equation TRISO-ANA-120**

```math
w
=
B
-
\frac{S_0r^2}{6D}.
```

Now apply the Robin boundary condition at $r=R$:

**Equation TRISO-ANA-121**

```math
-Dw'(R)=hw(R)
```

because $c_\infty=0$ for this benchmark.

Evaluate the derivative at $R$:

**Equation TRISO-ANA-122**

```math
w'(R)
=
-\frac{S_0R}{3D}.
```

Multiply by (-D):

**Equation TRISO-ANA-123**

```math
-Dw'(R)
=
\frac{S_0R}{3}.
```

Evaluate the concentration at $R$:

**Equation TRISO-ANA-124**

```math
w(R)
=
B
-
\frac{S_0R^2}{6D}.
```

Substitute both expressions into the Robin condition:

**Equation TRISO-ANA-125**

```math
\frac{S_0R}{3}
=
h
\left(
B
-
\frac{S_0R^2}{6D}
\right).
```

Divide by $h$:

**Equation TRISO-ANA-126**

```math
\frac{S_0R}{3h}
=
B
-
\frac{S_0R^2}{6D}.
```

Add the quadratic term to both sides:

**Equation TRISO-ANA-127**

```math
B
=
\frac{S_0R}{3h}
+
\frac{S_0R^2}{6D}.
```

Substitute $B$ into the profile:

**Equation TRISO-ANA-128**

```math
w(r)
=
\frac{S_0R}{3h}
+
\frac{S_0R^2}{6D}
-
\frac{S_0r^2}{6D}.
```

Collect the two quadratic terms:

**Equation TRISO-ANA-129**

```math
\boxed{
w(r)
=
\frac{S_0R}{3h}
+
\frac{S_0}{6D}
\left(
R^2-r^2
\right).
}
```

### 7.1 Dimensional check

The first term has units

**Equation TRISO-ANA-130**

```math
\left[
\frac{S_0R}{h}
\right]
=
\frac{
\mathrm{mol\,m^{-3}\,s^{-1}}
\times
\mathrm m
}{
\mathrm{m\,s^{-1}}
}
=
\mathrm{mol\,m^{-3}}.
```

The second term has units

**Equation TRISO-ANA-131**

```math
\left[
\frac{S_0R^2}{D}
\right]
=
\frac{
\mathrm{mol\,m^{-3}\,s^{-1}}
\times
\mathrm{m^2}
}{
\mathrm{m^2\,s^{-1}}
}
=
\mathrm{mol\,m^{-3}}.
```

Both terms therefore have concentration units.

### 7.2 Independent global generation/release balance

The total generation rate in the homogeneous sphere is

**Equation TRISO-ANA-132**

```math
\dot N_{\mathrm{gen}}
=
\int_0^R
S_0\,4\pi r^2\,dr.
```

Pull out constants:

**Equation TRISO-ANA-133**

```math
\dot N_{\mathrm{gen}}
=
4\pi S_0
\int_0^Rr^2\,dr.
```

Evaluate the integral:

**Equation TRISO-ANA-134**

```math
\int_0^Rr^2\,dr
=
\frac{R^3}{3}.
```

Therefore

**Equation TRISO-ANA-135**

```math
\boxed{
\dot N_{\mathrm{gen}}
=
\frac{4\pi R^3S_0}{3}.
}
```

At steady state, this must equal the outward surface release rate:

**Equation TRISO-ANA-136**

```math
\dot N_{\mathrm{out}}
=
4\pi R^2
\left[
h w(R)
\right].
```

Set generation equal to release:

**Equation TRISO-ANA-137**

```math
\frac{4\pi R^3S_0}{3}
=
4\pi R^2h w(R).
```

Cancel $4\pi R^2$:

**Equation TRISO-ANA-138**

```math
\frac{S_0R}{3}
=
h w(R).
```

Divide by $h$:

**Equation TRISO-ANA-139**

```math
w(R)=\frac{S_0R}{3h}.
```

Evaluate the analytical profile at $r=R$:

**Equation TRISO-ANA-140**

```math
w(R)
=
\frac{S_0R}{3h}
+
\frac{S_0}{6D}(R^2-R^2).
```

The second term is zero:

**Equation TRISO-ANA-141**

```math
w(R)
=
\frac{S_0R}{3h}.
```

The independently derived balance and the analytical solution therefore agree.

[VERIFIED] The homogeneous steady benchmark is internally consistent under its stated assumptions.

## 7.3 Scope of this benchmark

This analytical solution does **not** represent the production WOS initial-kernel release problem.

It represents Problem A:

- initially empty particle;
- continuing uniform source;
- homogeneous diffusivity;
- Robin outer boundary.

The production WOS verification problem is Problem B and uses an initial kernel inventory with an absorbing outer boundary.

Keeping these benchmarks separate prevents source and boundary semantics from being mixed.

## 8. Homogeneous transient deviation problem

The following transient benchmark belongs to Problem A: an initially empty homogeneous sphere with a continuing uniform source and a Robin outer boundary.

The steady solution $w(r)$ has already been obtained. We now remove the steady part so that the remaining transient problem has no source term.

### 8.1 Define the transient deviation

Define

**Equation TRISO-ANA-200**

```math
v(r,t)=c(r,t)-w(r).
```

Rearrange this definition:

**Equation TRISO-ANA-201**

```math
c(r,t)=v(r,t)+w(r).
```

Because $w$ is a steady solution, it does not depend on time:

**Equation TRISO-ANA-202**

```math
\frac{\partial w}{\partial t}=0.
```

Differentiate $c=v+w$ with respect to time:

**Equation TRISO-ANA-203**

```math
\frac{\partial c}{\partial t}
=
\frac{\partial v}{\partial t}
+
\frac{\partial w}{\partial t}.
```

Substitute (TRISO-ANA-202):

**Equation TRISO-ANA-204**

```math
\boxed{
\frac{\partial c}{\partial t}
=
\frac{\partial v}{\partial t}.
}
```

Differentiate $c=v+w$ with respect to radius:

**Equation TRISO-ANA-205**

```math
\frac{\partial c}{\partial r}
=
\frac{\partial v}{\partial r}
+
\frac{dw}{dr}.
```

Differentiate once more:

**Equation TRISO-ANA-206**

```math
\frac{\partial^2 c}{\partial r^2}
=
\frac{\partial^2v}{\partial r^2}
+
\frac{d^2w}{dr^2}.
```

Start from the full homogeneous source-driven PDE:

**Equation TRISO-ANA-207**

```math
\frac{\partial c}{\partial t}
=
D
\left(
\frac{\partial^2c}{\partial r^2}
+
\frac2r\frac{\partial c}{\partial r}
\right)
+
S_0.
```

Substitute the time derivative from (TRISO-ANA-204):

**Equation TRISO-ANA-208**

```math
\frac{\partial v}{\partial t}
=
D
\left(
\frac{\partial^2c}{\partial r^2}
+
\frac2r\frac{\partial c}{\partial r}
\right)
+
S_0.
```

Substitute the second spatial derivative from (TRISO-ANA-206):

**Equation TRISO-ANA-209**

```math
\frac{\partial v}{\partial t}
=
D
\left(
\frac{\partial^2v}{\partial r^2}
+
\frac{d^2w}{dr^2}
+
\frac2r\frac{\partial c}{\partial r}
\right)
+
S_0.
```

Substitute the first spatial derivative from (TRISO-ANA-205):

**Equation TRISO-ANA-210**

```math
\frac{\partial v}{\partial t}
=
D
\left(
\frac{\partial^2v}{\partial r^2}
+
\frac{d^2w}{dr^2}
+
\frac2r
\left[
\frac{\partial v}{\partial r}
+
\frac{dw}{dr}
\right]
\right)
+
S_0.
```

Distribute the factor $D$:

**Equation TRISO-ANA-211**

```math
\frac{\partial v}{\partial t}
=
D\frac{\partial^2v}{\partial r^2}
+
D\frac{d^2w}{dr^2}
+
\frac{2D}{r}\frac{\partial v}{\partial r}
+
\frac{2D}{r}\frac{dw}{dr}
+
S_0.
```

Rearrange the terms into transient and steady groups:

**Equation TRISO-ANA-212**

```math
\frac{\partial v}{\partial t}
=
D
\left(
\frac{\partial^2v}{\partial r^2}
+
\frac2r\frac{\partial v}{\partial r}
\right)
+
\left[
D
\left(
\frac{d^2w}{dr^2}
+
\frac2r\frac{dw}{dr}
\right)
+
S_0
\right].
```

The steady solution satisfies

**Equation TRISO-ANA-213**

```math
D
\left(
\frac{d^2w}{dr^2}
+
\frac2r\frac{dw}{dr}
\right)
+
S_0
=
0.
```

Substitute (TRISO-ANA-213) into (TRISO-ANA-212):

**Equation TRISO-ANA-214**

```math
\boxed{
\frac{\partial v}{\partial t}
=
D
\left(
\frac{\partial^2v}{\partial r^2}
+
\frac2r\frac{\partial v}{\partial r}
\right).
}
```

The source has disappeared because the steady part $w$ already accounts for the long-time source balance.

### 8.2 Transform the centre condition

The original centre condition is

**Equation TRISO-ANA-215**

```math
\frac{\partial c}{\partial r}(0,t)=0.
```

Substitute (TRISO-ANA-205):

**Equation TRISO-ANA-216**

```math
\frac{\partial v}{\partial r}(0,t)+w'(0)=0.
```

The steady solution has

**Equation TRISO-ANA-217**

```math
w'(r)=-\frac{S_0r}{3D}.
```

Evaluate it at the centre:

**Equation TRISO-ANA-218**

```math
w'(0)=0.
```

Therefore

**Equation TRISO-ANA-219**

```math
\boxed{
\frac{\partial v}{\partial r}(0,t)=0.
}
```

### 8.3 Transform the outer Robin condition

The original Robin condition is

**Equation TRISO-ANA-220**

```math
-Dc_r(R,t)=hw(R,t).
```

Substitute $c=v+w$:

**Equation TRISO-ANA-221**

```math
-D
\left[
v_r(R,t)+w'(R)
\right]
=
h
\left[
v(R,t)+w(R)
\right].
```

Rearrange the transient and steady terms:

**Equation TRISO-ANA-222**

```math
-Dv_r(R,t)-hw(R)
=
hv(R,t)+Dw'(R).
```

The steady solution satisfies

**Equation TRISO-ANA-223**

```math
-Dw'(R)=hw(R).
```

Therefore

**Equation TRISO-ANA-224**

```math
-Dv_r(R,t)=hv(R,t).
```

Hence the transient Robin condition is

**Equation TRISO-ANA-225**

```math
\boxed{
-Dv_r(R,t)=hv(R,t).
}
```

### 8.4 Transform the initial condition

The original initial condition for Problem A is

**Equation TRISO-ANA-226**

```math
c(r,0)=0.
```

Apply the definition $v=c-w$:

**Equation TRISO-ANA-227**

```math
v(r,0)=c(r,0)-w(r).
```

Substitute $c(r,0)=0$:

**Equation TRISO-ANA-228**

```math
\boxed{
v(r,0)=-w(r).
}
```

Thus the complete transient deviation problem is

```math
v_t
=
D
\left(
v_{rr}+\frac2r v_r
\right),
```

with

```math
v_r(0,t)=0,
```

```math
-Dv_r(R,t)=hv(R,t),
```

and

```math
v(r,0)=-w(r).
```

## 9. Separation of variables

### 9.1 Assume a separated solution

Seek a non-zero transient mode in the form

**Equation TRISO-ANA-229**

```math
v(r,t)=\phi(r)T(t).
```

Differentiate with respect to time:

**Equation TRISO-ANA-230**

```math
v_t=\phi(r)T'(t).
```

Differentiate with respect to radius:

**Equation TRISO-ANA-231**

```math
v_r=\phi'(r)T(t).
```

Differentiate once more:

**Equation TRISO-ANA-232**

```math
v_{rr}=\phi''(r)T(t).
```

Substitute these three expressions into the transient PDE:

**Equation TRISO-ANA-233**

```math
\phi T'
=
D
\left[
\phi''T
+
\frac2r\phi'T
\right].
```

Factor out $T$ on the right:

**Equation TRISO-ANA-234**

```math
\phi T'
=
DT
\left(
\phi''+\frac2r\phi'
\right).
```

Divide by $D\phi T$, assuming the separated factors are non-zero at the point considered:

**Equation TRISO-ANA-235**

```math
\frac{T'}{DT}
=
\frac{\phi''+2\phi'/r}{\phi}.
```

The left side depends only on $t$, while the right side depends only on $r$.

For one separated mode to satisfy the equation for every $r$ and $t$, both sides must equal the same constant.

Choose the separation constant as $-k^2$:

**Equation TRISO-ANA-236**

```math
\frac{T'}{DT}=-k^2.
```

Then the time equation is

**Equation TRISO-ANA-237**

```math
T'=-Dk^2T.
```

The radial equation is

**Equation TRISO-ANA-238**

```math
\boxed{
\phi''
+
\frac2r\phi'
+
k^2\phi
=
0.
}
```

### 9.2 Dimensions of the separation constant

The left side of (TRISO-ANA-236) has units

**Equation TRISO-ANA-239**

```math
\left[
\frac{T'}{DT}
\right]
=
\frac{\mathrm{s^{-1}}}{\mathrm{m^2\,s^{-1}}}
=
\mathrm{m^{-2}}.
```

Therefore

**Equation TRISO-ANA-240**

```math
[k^2]=\mathrm{m^{-2}}.
```

Hence

**Equation TRISO-ANA-241**

```math
[k]=\mathrm{m^{-1}}.
```

Define the temporal decay rate

**Equation TRISO-ANA-242**

```math
\boxed{
\Lambda=Dk^2.
}
```

Then

**Equation TRISO-ANA-243**

```math
[\Lambda]
=
\mathrm{m^2\,s^{-1}}
\times
\mathrm{m^{-2}}
=
\mathrm{s^{-1}}.
```

Thus $k$ is a spatial wave number, while $\Lambda$ is a temporal decay rate.

### 9.3 Solve the temporal equation

Starting from

**Equation TRISO-ANA-244**

```math
T'=-\Lambda T,
```

divide by $T$:

**Equation TRISO-ANA-245**

```math
\frac{T'}{T}=-\Lambda.
```

Write the derivative as a differential:

**Equation TRISO-ANA-246**

```math
\frac{dT}{T}=-\Lambda\,dt.
```

Integrate:

**Equation TRISO-ANA-247**

```math
\int\frac{dT}{T}
=
-\Lambda\int dt.
```

Therefore

**Equation TRISO-ANA-248**

```math
\ln|T|
=
-\Lambda t+C.
```

Exponentiate:

**Equation TRISO-ANA-249**

```math
|T|=e^{C}e^{-\Lambda t}.
```

Absorb the constant into an arbitrary amplitude $C_T$:

**Equation TRISO-ANA-250**

```math
T(t)=C_Te^{-\Lambda t}.
```

The constant $C_T$ can be absorbed into the spatial amplitude, so take

**Equation TRISO-ANA-251**

```math
\boxed{
T(t)=e^{-\Lambda t}.
}
```

Therefore each separated mode has the form

**Equation TRISO-ANA-252**

```math
v(r,t)=\phi(r)e^{-\Lambda t}.
```

## 10. Radial eigenproblem, Robin condition, and modal expansion

### 10.1 Transform the radial eigenproblem with $u=r\phi$

Start from

**Equation TRISO-ANA-253**

```math
\phi''
+
\frac2r\phi'
+
k^2\phi
=
0.
```

Introduce

**Equation TRISO-ANA-254**

```math
u(r)=r\phi(r).
```

Solve the definition for $\phi$:

**Equation TRISO-ANA-255**

```math
\phi(r)=\frac{u(r)}{r}.
```

Differentiate using the quotient rule:

**Equation TRISO-ANA-256**

```math
\phi'
=
\frac{r u'-u}{r^2}.
```

Differentiate again. Write the numerator as $n=ru'-u$:

**Equation TRISO-ANA-257**

```math
n'=u'+ru''-u'.
```

Therefore

**Equation TRISO-ANA-258**

```math
n'=ru''.
```

Apply the quotient rule to $n/r^2$:

**Equation TRISO-ANA-259**

```math
\phi''
=
\frac{n'r^2-n(2r)}{r^4}.
```

Substitute $n'=ru''$ and $n=ru'-u$:

**Equation TRISO-ANA-260**

```math
\phi''
=
\frac{r^3u''-2r(ru'-u)}{r^4}.
```

Expand the numerator:

**Equation TRISO-ANA-261**

```math
\phi''
=
\frac{r^3u''-2r^2u'+2ru}{r^4}.
```

Divide each term by $r^4$:

**Equation TRISO-ANA-262**

```math
\phi''
=
\frac{u''}{r}
-
\frac{2u'}{r^2}
+
\frac{2u}{r^3}.
```

Now substitute (TRISO-ANA-256), (TRISO-ANA-255), and (TRISO-ANA-262) into (TRISO-ANA-253):

**Equation TRISO-ANA-263**

```math
\left(
\frac{u''}{r}
-
\frac{2u'}{r^2}
+
\frac{2u}{r^3}
\right)
+
\frac2r
\left(
\frac{ru'-u}{r^2}
\right)
+
k^2\frac{u}{r}
=
0.
```

Expand the second term:

**Equation TRISO-ANA-264**

```math
\frac{u''}{r}
-
\frac{2u'}{r^2}
+
\frac{2u}{r^3}
+
\frac{2u'}{r^2}
-
\frac{2u}{r^3}
+
k^2\frac{u}{r}
=
0.
```

Cancel the (u') terms explicitly:

**Equation TRISO-ANA-265**

```math
\frac{u''}{r}
+
k^2\frac{u}{r}
=
0.
```

Multiply by $r$:

**Equation TRISO-ANA-266**

```math
\boxed{
u''+k^2u=0.
}
```

### 10.2 Solve the transformed radial equation

The characteristic equation is

**Equation TRISO-ANA-267**

```math
m^2+k^2=0.
```

Its roots are

**Equation TRISO-ANA-268**

```math
m=\pm ik.
```

Therefore the real-valued solution is

**Equation TRISO-ANA-269**

```math
\boxed{
u(r)=A\sin(kr)+B\cos(kr).
}
```

Substitute into $\phi=u/r$:

**Equation TRISO-ANA-270**

```math
\phi(r)
=
\frac{A\sin(kr)+B\cos(kr)}{r}.
```

### 10.3 Centre regularity

Examine the cosine contribution as (r\to0).

Use the known limit

**Equation TRISO-ANA-271**

```math
\lim_{r\to0}\cos(kr)=1.
```

Therefore

**Equation TRISO-ANA-272**

```math
\lim_{r\to0}\frac{B\cos(kr)}{r}
=
\lim_{r\to0}\frac{B}{r}.
```

For $B\ne0$, this diverges.

A physical concentration perturbation must remain finite at the particle centre.

Therefore

**Equation TRISO-ANA-273**

```math
B=0.
```

The eigenfunction becomes

**Equation TRISO-ANA-274**

```math
\phi(r)=A\frac{\sin(kr)}{r}.
```

For the sine term, use

**Equation TRISO-ANA-275**

```math
\sin(kr)=kr+O(r^3)
\qquad
(r\to0).
```

Divide by $r$:

**Equation TRISO-ANA-276**

```math
\frac{\sin(kr)}{r}
=
k+O(r^2).
```

Therefore

**Equation TRISO-ANA-277**

```math
\boxed{
\lim_{r\to0}\phi(r)=Ak.
}
```

The apparent (1/r) singularity is removable for the sine branch.

It is often convenient to absorb $k$ into the modal amplitude. Define

**Equation TRISO-ANA-278**

```math
C=A k.
```

Then the same mode may be written as

**Equation TRISO-ANA-279**

```math
\phi(r)=C\frac{\sin(kr)}{kr}.
```

The normalization is arbitrary; only the relative spatial shape matters for the eigenvalue problem.

### 10.4 Derive the Robin eigencondition

Start from the normalized form

**Equation TRISO-ANA-280**

```math
\phi(r)=C\frac{\sin(kr)}{kr}.
```

The derivative is easier to obtain by treating (C/k) as a constant:

**Equation TRISO-ANA-281**

```math
\phi(r)=\frac{C}{k}\frac{\sin(kr)}{r}.
```

Differentiate $\sin(kr)/r$ using the quotient rule:

**Equation TRISO-ANA-282**

```math
\frac{d}{dr}
\left(
\frac{\sin(kr)}{r}
\right)
=
\frac{
kr\cos(kr)-\sin(kr)
}{
r^2
}.
```

Therefore

**Equation TRISO-ANA-283**

```math
\boxed{
\phi'(r)
=
\frac{C}{k}
\frac{
kr\cos(kr)-\sin(kr)
}{
r^2
}.
}
```

At the outer boundary,

**Equation TRISO-ANA-284**

```math
-D\phi'(R)=h\phi(R).
```

Substitute $\phi'(R)$:

**Equation TRISO-ANA-285**

```math
-D
\frac{C}{k}
\frac{
kR\cos(kR)-\sin(kR)
}{
R^2
}
=
h\phi(R).
```

Substitute $\phi(R)=C\sin(kR)/(kR)$:

**Equation TRISO-ANA-286**

```math
-D
\frac{C}{k}
\frac{
kR\cos(kR)-\sin(kR)
}{
R^2
}
=
h
\frac{C}{kR}
\sin(kR).
```

Multiply both sides by $kR^2/C$, assuming $C\ne0$:

**Equation TRISO-ANA-287**

```math
-D
\left[
kR\cos(kR)-\sin(kR)
\right]
=
hR\sin(kR).
```

Multiply by (-1):

**Equation TRISO-ANA-288**

```math
D
\left[
\sin(kR)-kR\cos(kR)
\right]
=
hR\sin(kR).
```

Define the dimensionless eigenvariable

**Equation TRISO-ANA-289**

```math
\mu=kR.
```

Its dimensions are

**Equation TRISO-ANA-290**

```math
[\mu]=[k][R]=\mathrm{m^{-1}}\times\mathrm m=1.
```

Define the Biot number

**Equation TRISO-ANA-291**

```math
\mathrm{Bi}=\frac{hR}{D}.
```

Its dimensions are

**Equation TRISO-ANA-292**

```math
[\mathrm{Bi}]
=
\frac{
\mathrm{m\,s^{-1}}\times\mathrm m
}{
\mathrm{m^2\,s^{-1}}
}
=
1.
```

Substitute $\mu=kR$ and $hR/D=\mathrm{Bi}$ into (TRISO-ANA-288):

**Equation TRISO-ANA-293**

```math
\boxed{
\sin\mu-\mu\cos\mu
=
\mathrm{Bi}\sin\mu.
}
```

Rearrange:

**Equation TRISO-ANA-294**

```math
(1-\mathrm{Bi})\sin\mu
=
\mu\cos\mu.
```

For $\sin\mu\ne0$, divide by $\sin\mu$:

**Equation TRISO-ANA-295**

```math
1-\mathrm{Bi}
=
\mu\frac{\cos\mu}{\sin\mu}.
```

Use $\cot\mu=\cos\mu/\sin\mu$:

**Equation TRISO-ANA-296**

```math
\boxed{
\mu\cot\mu=1-\mathrm{Bi}.
}
```

### 10.5 Check whether division by $\sin\mu$ loses roots

The undivided equation (TRISO-ANA-293) must be used for this check.

Suppose

**Equation TRISO-ANA-297**

```math
\sin\mu=0.
```

Then $\mu=n\pi$ for integer $n$.

Substitute into (TRISO-ANA-293):

**Equation TRISO-ANA-298**

```math
0-n\pi\cos(n\pi)=0.
```

Because

**Equation TRISO-ANA-299**

```math
\cos(n\pi)=(-1)^n,
```

this becomes

**Equation TRISO-ANA-300**

```math
-n\pi(-1)^n=0.
```

For positive $n$, this is not zero.

Therefore no positive eigenvalue is lost when dividing by $\sin\mu$.

The only simultaneous zero is $\mu=0$, which does not satisfy the positive transient-mode condition for the Robin problem with $h>0$.

### 10.6 Eigenvalue definitions

Let $\mu_n$ denote the positive roots of (TRISO-ANA-293).

Then

**Equation TRISO-ANA-301**

```math
k_nR=\mu_n.
```

Therefore

**Equation TRISO-ANA-302**

```math
\boxed{
k_n=\frac{\mu_n}{R}.
}
```

Since

**Equation TRISO-ANA-303**

```math
\Lambda_n=Dk_n^2,
```

substitute (TRISO-ANA-302):

**Equation TRISO-ANA-304**

```math
\Lambda_n
=
D
\left(
\frac{\mu_n}{R}
\right)^2.
```

Thus

**Equation TRISO-ANA-305**

```math
\boxed{
\Lambda_n
=
D\frac{\mu_n^2}{R^2}.
}
```

The dimensions are

**Equation TRISO-ANA-306**

```math
[\Lambda_n]
=
\mathrm{m^2\,s^{-1}}
\times
\mathrm{m^{-2}}
=
\mathrm{s^{-1}}.
```

The separated transient mode is therefore

**Equation TRISO-ANA-307**

```math
v_n(r,t)=\phi_n(r)e^{-\Lambda_nt}.
```

### 10.7 Sturm–Liouville form

Start from the radial eigenproblem:

**Equation TRISO-SL-200**

```math
\phi_n''
+
\frac2r\phi_n'
+
k_n^2\phi_n
=
0.
```

Multiply by $r^2$:

**Equation TRISO-SL-201**

```math
r^2\phi_n''
+
2r\phi_n'
+
k_n^2r^2\phi_n
=
0.
```

The first two terms are a product derivative because

**Equation TRISO-SL-202**

```math
\frac{d}{dr}(r^2\phi_n')
=
2r\phi_n'
+
r^2\phi_n''.
```

Therefore

**Equation TRISO-SL-203**

```math
\frac{d}{dr}(r^2\phi_n')
+
k_n^2r^2\phi_n
=
0.
```

Move the eigenvalue term to the other side:

**Equation TRISO-SL-204**

```math
-\frac{d}{dr}(r^2\phi_n')
=
k_n^2r^2\phi_n.
```

This is the self-adjoint Sturm–Liouville form

```math
-\frac{d}{dr}
\left(
p(r)\frac{d\phi_n}{dr}
\right)
+
q(r)\phi_n
=
\lambda_n w(r)\phi_n
```

with the identifications

**Equation TRISO-SL-205**

```math
p(r)=r^2,
```

**Equation TRISO-SL-206**

```math
q(r)=0,
```

**Equation TRISO-SL-207**

```math
w(r)=r^2,
```

and

**Equation TRISO-SL-208**

```math
\lambda_n=k_n^2.
```

The eigenvalue in this Sturm–Liouville problem is therefore $k_n^2$, with units $\mathrm{m^{-2}}$, not the temporal decay rate $\Lambda_n$.

The interval is $0<r<R$.

The centre condition is regularity of $\phi_n$, equivalent for these modes to a finite $\phi_n(0)$ and zero radial derivative at the centre.

The outer boundary is the homogeneous Robin condition

**Equation TRISO-SL-209**

```math
-D\phi_n'(R)=h\phi_n(R).
```

[THEOREM / STANDARD FORM] Sturm–Liouville theory provides the framework for eigenvalues and eigenfunctions of self-adjoint second-order problems. See the NIST Digital Library of Mathematical Functions, §1.13(viii), which identifies Sturm–Liouville eigenvalues/eigenfunctions and the Liouville form. [NIST_DLMF].

Because the centre endpoint has $p(0)=0$, it is more precise to call this a radial **singular** Sturm–Liouville endpoint rather than an ordinary regular endpoint. The orthogonality used below can nevertheless be derived directly for the present eigenfunctions, so no stronger theorem is needed.

### 10.8 Derive orthogonality directly

Take two distinct eigenfunctions $\phi_m$ and $\phi_n$ with eigenvalues $k_m^2$ and $k_n^2$:

**Equation TRISO-SL-210**

```math
-\frac{d}{dr}
(r^2\phi_m')
=
k_m^2r^2\phi_m,
```

and

**Equation TRISO-SL-211**

```math
-\frac{d}{dr}
(r^2\phi_n')
=
k_n^2r^2\phi_n.
```

Multiply the first equation by $\phi_n$:

**Equation TRISO-SL-212**

```math
-\phi_n\frac{d}{dr}(r^2\phi_m')
=
k_m^2r^2\phi_m\phi_n.
```

Multiply the second equation by $\phi_m$:

**Equation TRISO-SL-213**

```math
-\phi_m\frac{d}{dr}(r^2\phi_n')
=
k_n^2r^2\phi_m\phi_n.
```

Subtract the second equation from the first:

**Equation TRISO-SL-214**

```math
-\phi_n\frac{d}{dr}(r^2\phi_m')
+
\phi_m\frac{d}{dr}(r^2\phi_n')
=
(k_m^2-k_n^2)r^2\phi_m\phi_n.
```

Recognise the left side as a derivative:

**Equation TRISO-SL-215**

```math
\frac{d}{dr}
\left[
r^2
(
\phi_m\phi_n'
-
\phi_n\phi_m'
)
\right]
=
(k_n^2-k_m^2)r^2\phi_m\phi_n.
```

Integrate from (0) to $R$:

**Equation TRISO-SL-216**

```math
\int_0^R
\frac{d}{dr}
\left[
r^2
(
\phi_m\phi_n'
-
\phi_n\phi_m'
)
\right]dr
=
(k_n^2-k_m^2)
\int_0^R
r^2\phi_m\phi_n\,dr.
```

Evaluate the left-hand integral:

**Equation TRISO-SL-217**

```math
\left[
r^2
(
\phi_m\phi_n'
-
\phi_n\phi_m'
)
\right]_0^R
=
(k_n^2-k_m^2)
\int_0^R
r^2\phi_m\phi_n\,dr.
```

At $r=R$, both eigenfunctions satisfy the same Robin condition:

**Equation TRISO-SL-218**

```math
\phi_m'(R)=-\frac{h}{D}\phi_m(R),
```

and

**Equation TRISO-SL-219**

```math
\phi_n'(R)=-\frac{h}{D}\phi_n(R).
```

Therefore the outer boundary term is zero:

**Equation TRISO-SL-220**

```math
R^2
[
\phi_m(R)\phi_n'(R)
-
\phi_n(R)\phi_m'(R)
]
=0.
```

At the centre, the regular eigenfunctions are finite and their derivatives remain bounded, while $r^2\to0$.

Hence

**Equation TRISO-SL-221**

```math
\lim_{r\to0}
r^2
[
\phi_m\phi_n'
-
\phi_n\phi_m'
]
=0.
```

Therefore the complete boundary term is zero:

**Equation TRISO-SL-222**

```math
(k_n^2-k_m^2)
\int_0^R
r^2\phi_m\phi_n\,dr
=
0.
```

For distinct eigenvalues,

**Equation TRISO-SL-223**

```math
k_n^2\ne k_m^2,
```

so

**Equation TRISO-SL-224**

```math
\boxed{
\int_0^R
r^2\phi_m(r)\phi_n(r)\,dr
=
0,
\qquad m\ne n.
}
```

The weight is therefore

**Equation TRISO-SL-225**

```math
\boxed{
w(r)=r^2.
}
```

The same $r^2$ weight also follows directly from spherical volume $dV=4\pi r^2dr$.

### 10.9 Modal coefficient projection

At $t=0$, (TRISO-ANA-228) gives

**Equation TRISO-SL-226**

```math
v(r,0)=-w(r).
```

Represent the initial transient as an eigenfunction series:

**Equation TRISO-SL-227**

```math
-w(r)
=
\sum_{n=1}^{\infty}
A_n\phi_n(r).
```

Multiply both sides by $r^2\phi_m(r)$:

**Equation TRISO-SL-228**

```math
-r^2w(r)\phi_m(r)
=
\sum_{n=1}^{\infty}
A_n r^2\phi_n(r)\phi_m(r).
```

Integrate from (0) to $R$:

**Equation TRISO-SL-229**

```math
-\int_0^R
r^2w(r)\phi_m(r)\,dr
=
\sum_{n=1}^{\infty}
A_n
\int_0^R
r^2\phi_n(r)\phi_m(r)\,dr.
```

For $n\ne m$, orthogonality makes the corresponding integrals zero:

**Equation TRISO-SL-230**

```math
\int_0^R
r^2\phi_n\phi_m\,dr
=
0.
```

The remaining $n=m$ term is

**Equation TRISO-SL-231**

```math
-\int_0^R
r^2w(r)\phi_m(r)\,dr
=
A_m
\int_0^R
r^2\phi_m(r)^2\,dr.
```

Divide by the non-zero mode norm:

**Equation TRISO-SL-232**

```math
\boxed{
A_m
=
-
\frac{
\int_0^R
r^2w(r)\phi_m(r)\,dr
}{
\int_0^R
r^2\phi_m(r)^2\,dr
}.
}
```

Rename $m$ to $n$:

**Equation TRISO-SL-233**

```math
\boxed{
A_n
=
-
\frac{
\int_0^R
r^2w(r)\phi_n(r)\,dr
}{
\int_0^R
r^2\phi_n(r)^2\,dr
}.
}
```

Each mode evolves with $e^{-\Lambda_nt}$, so

**Equation TRISO-SL-234**

```math
v(r,t)
=
\sum_{n=1}^{\infty}
A_n\phi_n(r)e^{-\Lambda_nt}.
```

Since $c=v+w$,

**Equation TRISO-SL-235**

```math
\boxed{
c(r,t)
=
w(r)
+
\sum_{n=1}^{\infty}
A_n\phi_n(r)e^{-\Lambda_nt}.
}
```

### 10.10 Checks on the transient series

Each mode has decay factor

**Equation TRISO-SL-236**

```math
e^{-\Lambda_nt}.
```

At $t=0$,

**Equation TRISO-SL-237**

```math
e^{-\Lambda_n\cdot0}=1.
```

Therefore

**Equation TRISO-SL-238**

```math
c(r,0)
=
w(r)
+
\sum_{n=1}^{\infty}A_n\phi_n(r).
```

Using the defining expansion (TRISO-SL-227),

**Equation TRISO-SL-239**

```math
\sum_{n=1}^{\infty}A_n\phi_n(r)
=
-w(r).
```

Hence, formally,

**Equation TRISO-SL-240**

```math
\boxed{
c(r,0)=0.
}
```

As $t\to\infty$, every mode with $\Lambda_n>0$ satisfies

**Equation TRISO-SL-241**

```math
e^{-\Lambda_nt}\to0.
```

Therefore, provided the modal expansion has the required convergence,

**Equation TRISO-SL-242**

```math
\boxed{
c(r,t)\to w(r).
}
```

The convergence of the infinite series itself has not been numerically established here. The statements above are the formal consequences of the eigen-expansion framework.

## 11. Five-layer steady analytical formulation

### 11.1 Geometry, domains, and layer quantities

Define the concentric radii

**Equation TRISO-ML-300**

```math
r_0=0<r_1<r_2<r_3<r_4<r_5=R.
```

The five material regions are

**Equation TRISO-ML-301**

```math
\Omega_1=(r_0,r_1)
\quad\text{fuel kernel},
```

**Equation TRISO-ML-302**

```math
\Omega_2=(r_1,r_2)
\quad\text{buffer},
```

**Equation TRISO-ML-303**

```math
\Omega_3=(r_2,r_3)
\quad\text{IPyC},
```

**Equation TRISO-ML-304**

```math
\Omega_4=(r_3,r_4)
\quad\text{SiC},
```

and

**Equation TRISO-ML-305**

```math
\Omega_5=(r_4,r_5)
\quad\text{OPyC}.
```

In material layer $i$, define

**Equation TRISO-ML-306**

```math
c_i(r,t),
\qquad
r_{i-1}<r<r_i.
```

The concentration units are

**Equation TRISO-ML-307**

```math
[c_i]=\mathrm{mol\,m^{-3}}.
```

Let $D_i$ be the diffusivity in layer $i$:

**Equation TRISO-ML-308**

```math
[D_i]=\mathrm{m^2\,s^{-1}}.
```

Let $S_i$ be the net volumetric source in layer $i$:

**Equation TRISO-ML-309**

```math
[S_i]=\mathrm{mol\,m^{-3}\,s^{-1}}.
```

The coordinates $r,t$, outer radius $R$, transfer coefficient $h$, and external concentration $c_\infty$ are global quantities. The fields $c_i$, diffusivities $D_i$, and sources $S_i$ are material-layer quantities.

For the source-driven five-layer benchmark,

**Equation TRISO-ML-310**

```math
S_1=S_0,
```

while

**Equation TRISO-ML-311**

```math
S_i=0,
\qquad
i=2,3,4,5.
```

[ASSUMPTION] Each $D_i>0$ is constant within its material layer for this analytical benchmark.

[ASSUMPTION] Interfaces have zero storage, zero interfacial source, $K_i=1$, and no explicit interfacial resistance.

### 11.2 Kernel steady solution

The steady conservative equation in the kernel is

**Equation TRISO-ML-312**

```math
0=
\frac1{r^2}
\frac{d}{dr}
\left(
r^2D_1\frac{dc_1}{dr}
\right)
+S_0.
```

Move the source term to the right:

**Equation TRISO-ML-313**

```math
\frac1{r^2}
\frac{d}{dr}
\left(
r^2D_1\frac{dc_1}{dr}
\right)
=-S_0.
```

Multiply by $r^2$:

**Equation TRISO-ML-314**

```math
\frac{d}{dr}
\left(
r^2D_1\frac{dc_1}{dr}
\right)
=-S_0r^2.
```

Because $D_1$ is constant in the kernel,

**Equation TRISO-ML-315**

```math
D_1
\frac{d}{dr}
\left(
r^2\frac{dc_1}{dr}
\right)
=-S_0r^2.
```

Divide by $D_1$:

**Equation TRISO-ML-316**

```math
\frac{d}{dr}
\left(
r^2\frac{dc_1}{dr}
\right)
=-\frac{S_0}{D_1}r^2.
```

Integrate:

**Equation TRISO-ML-317**

```math
r^2\frac{dc_1}{dr}
=
-\frac{S_0r^3}{3D_1}
+C_1.
```

Divide by $r^2$, for $r>0$:

**Equation TRISO-ML-318**

```math
\frac{dc_1}{dr}
=
-\frac{S_0r}{3D_1}
+\frac{C_1}{r^2}.
```

Centre regularity requires $dc_1/dr$ to remain finite as $r\to0$. The term $C_1/r^2$ diverges unless

**Equation TRISO-ML-319**

```math
C_1=0.
```

Therefore

**Equation TRISO-ML-320**

```math
\frac{dc_1}{dr}
=
-\frac{S_0r}{3D_1}.
```

Integrate again:

**Equation TRISO-ML-321**

```math
c_1(r)
=
-\frac{S_0r^2}{6D_1}
+A_1.
```

Hence the regular kernel profile is

**Equation TRISO-ML-322**

```math
\boxed{
c_1(r)
=
A_1-\frac{S_0r^2}{6D_1}.
}
```

### 11.3 Source-free coating solutions

For $i=2,3,4,5$,

**Equation TRISO-ML-323**

```math
S_i=0.
```

The steady equation is

**Equation TRISO-ML-324**

```math
0=
\frac1{r^2}
\frac{d}{dr}
\left(
r^2D_i\frac{dc_i}{dr}
\right).
```

Multiply by $r^2$:

**Equation TRISO-ML-325**

```math
\frac{d}{dr}
\left(
r^2D_i\frac{dc_i}{dr}
\right)=0.
```

Use constant $D_i$:

**Equation TRISO-ML-326**

```math
D_i
\frac{d}{dr}
\left(
r^2\frac{dc_i}{dr}
\right)=0.
```

Divide by $D_i>0$:

**Equation TRISO-ML-327**

```math
\frac{d}{dr}
\left(
r^2\frac{dc_i}{dr}
\right)=0.
```

Integrate:

**Equation TRISO-ML-328**

```math
r^2\frac{dc_i}{dr}=C_i.
```

Divide by $r^2$:

**Equation TRISO-ML-329**

```math
\frac{dc_i}{dr}=\frac{C_i}{r^2}.
```

Integrate:

**Equation TRISO-ML-330**

```math
c_i(r)=C_i\int r^{-2}dr+A_i.
```

Since

**Equation TRISO-ML-331**

```math
\int r^{-2}dr=-\frac1r,
```

we obtain

**Equation TRISO-ML-332**

```math
c_i(r)=A_i-\frac{C_i}{r}.
```

Define $B_i=-C_i$. Then

**Equation TRISO-ML-333**

```math
\boxed{
c_i(r)=A_i+\frac{B_i}{r},
\qquad i=2,3,4,5.
}
```

The coating layers do not include $r=0$, so their $1/r$ terms are finite within their own domains and are not removed by centre regularity.

### 11.4 Total kernel generation and common steady flux

The total kernel generation rate is

**Equation TRISO-ML-334**

```math
\dot N_{\mathrm{gen}}
=
4\pi
\int_0^{r_1}
S_0r^2\,dr.
```

Pull the constants outside:

**Equation TRISO-ML-335**

```math
\dot N_{\mathrm{gen}}
=
4\pi S_0
\int_0^{r_1}r^2\,dr.
```

Evaluate the integral:

**Equation TRISO-ML-336**

```math
\int_0^{r_1}r^2\,dr
=
\left[\frac{r^3}{3}\right]_0^{r_1}.
```

Therefore

**Equation TRISO-ML-337**

```math
\int_0^{r_1}r^2\,dr
=
\frac{r_1^3}{3}.
```

Hence

**Equation TRISO-ML-338**

```math
\boxed{
\dot N_{\mathrm{gen}}
=
\frac{4\pi S_0r_1^3}{3}.
}
```

At steady state, with no coating source, reaction, or storage, the same total amount rate crosses every sphere outside the kernel:

**Equation TRISO-ML-339**

```math
4\pi r^2J_r(r)
=
\dot N_{\mathrm{gen}},
\qquad r>r_1.
```

Substitute (TRISO-ML-338):

**Equation TRISO-ML-340**

```math
4\pi r^2J_r(r)
=
\frac{4\pi S_0r_1^3}{3}.
```

Cancel $4\pi$:

**Equation TRISO-ML-341**

```math
r^2J_r(r)=\frac{S_0r_1^3}{3}.
```

Divide by $r^2$:

**Equation TRISO-ML-342**

```math
\boxed{
J_r(r)=\frac{S_0r_1^3}{3r^2}.
}
```

The positive sign is outward.

### 11.5 Recover shell gradients from Fick's law

In shell $i$,

**Equation TRISO-ML-343**

```math
J_r=-D_i\frac{dc_i}{dr}.
```

Substitute (TRISO-ML-342):

**Equation TRISO-ML-344**

```math
-D_i\frac{dc_i}{dr}
=
\frac{S_0r_1^3}{3r^2}.
```

Divide by $-D_i$:

**Equation TRISO-ML-345**

```math
\boxed{
\frac{dc_i}{dr}
=
-\frac{S_0r_1^3}{3D_ir^2}.
}
```

Integrate from $r$ to the outer radius $r_i$ of that shell:

**Equation TRISO-ML-346**

```math
\int_{c_i(r)}^{c_i(r_i)}dc_i
=
-\frac{S_0r_1^3}{3D_i}
\int_r^{r_i}\rho^{-2}d\rho.
```

The radial integral is

**Equation TRISO-ML-347**

```math
\int_r^{r_i}\rho^{-2}d\rho
=
\frac1r-\frac1{r_i}.
```

Therefore

**Equation TRISO-ML-348**

```math
c_i(r_i)-c_i(r)
=
-\frac{S_0r_1^3}{3D_i}
\left(
\frac1r-\frac1{r_i}
\right).
```

Multiply by $-1$:

**Equation TRISO-ML-349**

```math
\boxed{
c_i(r)-c_i(r_i)
=
\frac{S_0r_1^3}{3D_i}
\left(
\frac1r-\frac1{r_i}
\right).
}
```

Differentiating $A_i+B_i/r$ gives

**Equation TRISO-ML-350**

```math
\frac{dc_i}{dr}=-\frac{B_i}{r^2}.
```

Compare with (TRISO-ML-345):

**Equation TRISO-ML-351**

```math
-\frac{B_i}{r^2}
=
-\frac{S_0r_1^3}{3D_ir^2}.
```

Hence

**Equation TRISO-ML-352**

```math
\boxed{
B_i=\frac{S_0r_1^3}{3D_i}.
}
```

This independently agrees with the direct shell ODE solution.

### 11.6 Interface matching

At $r=r_1$, flux continuity is

**Equation TRISO-ML-353**

```math
-D_1c_1'(r_1)
=
-D_2c_2'(r_1),
```

and ideal concentration continuity is

**Equation TRISO-ML-354**

```math
c_1(r_1)=c_2(r_1).
```

At $r=r_2$,

**Equation TRISO-ML-355**

```math
-D_2c_2'(r_2)
=
-D_3c_3'(r_2),
```

and

**Equation TRISO-ML-356**

```math
c_2(r_2)=c_3(r_2).
```

At $r=r_3$,

**Equation TRISO-ML-357**

```math
-D_3c_3'(r_3)
=
-D_4c_4'(r_3),
```

and

**Equation TRISO-ML-358**

```math
c_3(r_3)=c_4(r_3).
```

At $r=r_4$,

**Equation TRISO-ML-359**

```math
-D_4c_4'(r_4)
=
-D_5c_5'(r_4),
```

and

**Equation TRISO-ML-360**

```math
c_4(r_4)=c_5(r_4).
```

The flux equations are already satisfied by the common steady amount rate. The concentration equations relate the additive constants.

Across shell $i$,

**Equation TRISO-ML-361**

```math
c_i(r_{i-1})-c_i(r_i)
=
\frac{S_0r_1^3}{3D_i}
\left(
\frac1{r_{i-1}}-\frac1{r_i}
\right),
\qquad i=2,3,4,5.
```

Thus each inner interface concentration is obtained from the next outer interface concentration by adding that shell's concentration drop.

### 11.7 Outer Robin condition and inward propagation

At $R=r_5$,

**Equation TRISO-ML-362**

```math
-D_5c_5'(R)
=
h[c_5(R)-c_\infty].
```

For the benchmark,

**Equation TRISO-ML-363**

```math
c_\infty=0.
```

Therefore

**Equation TRISO-ML-364**

```math
-D_5c_5'(R)=hc_5(R).
```

The common flux gives

**Equation TRISO-ML-365**

```math
-D_5c_5'(R)
=
\frac{S_0r_1^3}{3R^2}.
```

Equate the two expressions:

**Equation TRISO-ML-366**

```math
hc_5(R)
=
\frac{S_0r_1^3}{3R^2}.
```

Divide by $h$:

**Equation TRISO-ML-367**

```math
\boxed{
c_5(R)
=
\frac{S_0r_1^3}{3hR^2}.
}
```

Move inward through OPyC:

**Equation TRISO-ML-368**

```math
c_5(r_4)
=
c_5(R)
+
\frac{S_0r_1^3}{3D_5}
\left(
\frac1{r_4}-\frac1R
\right).
```

Apply continuity:

**Equation TRISO-ML-369**

```math
c_4(r_4)=c_5(r_4).
```

Move inward through SiC:

**Equation TRISO-ML-370**

```math
c_4(r_3)
=
c_4(r_4)
+
\frac{S_0r_1^3}{3D_4}
\left(
\frac1{r_3}-\frac1{r_4}
\right).
```

Apply continuity:

**Equation TRISO-ML-371**

```math
c_3(r_3)=c_4(r_3).
```

Move inward through IPyC:

**Equation TRISO-ML-372**

```math
c_3(r_2)
=
c_3(r_3)
+
\frac{S_0r_1^3}{3D_3}
\left(
\frac1{r_2}-\frac1{r_3}
\right).
```

Apply continuity:

**Equation TRISO-ML-373**

```math
c_2(r_2)=c_3(r_2).
```

Move inward through the buffer:

**Equation TRISO-ML-374**

```math
c_2(r_1)
=
c_2(r_2)
+
\frac{S_0r_1^3}{3D_2}
\left(
\frac1{r_1}-\frac1{r_2}
\right).
```

Apply continuity:

**Equation TRISO-ML-375**

```math
c_1(r_1)=c_2(r_1).
```

From the kernel profile,

**Equation TRISO-ML-376**

```math
c_1(r)-c_1(r_1)
=
\frac{S_0}{6D_1}(r_1^2-r^2).
```

Hence

**Equation TRISO-ML-377**

```math
\boxed{
c_1(r)
=
c_2(r_1)
+
\frac{S_0}{6D_1}(r_1^2-r^2).
}
```

Equations (TRISO-ML-367) through (TRISO-ML-377) give the complete steady five-layer solution recursively.

### 11.8 Spherical resistance formulation

Define the common steady amount rate outside the kernel:

**Equation TRISO-ML-378**

```math
\dot N=4\pi r^2J_r.
```

In shell $i$,

**Equation TRISO-ML-379**

```math
\dot N
=
-4\pi r^2D_i\frac{dc_i}{dr}.
```

Rearrange:

**Equation TRISO-ML-380**

```math
dc_i
=
-\frac{\dot N}{4\pi D_i}\frac{dr}{r^2}.
```

Integrate from $r_{i-1}$ to $r_i$:

**Equation TRISO-ML-381**

```math
c_i(r_i)-c_i(r_{i-1})
=
-\frac{\dot N}{4\pi D_i}
\int_{r_{i-1}}^{r_i}r^{-2}dr.
```

Evaluate the radial integral:

**Equation TRISO-ML-382**

```math
\int_{r_{i-1}}^{r_i}r^{-2}dr
=
\frac1{r_{i-1}}-\frac1{r_i}.
```

Therefore

**Equation TRISO-ML-383**

```math
c_i(r_{i-1})-c_i(r_i)
=
\dot N
\frac1{4\pi D_i}
\left(
\frac1{r_{i-1}}-\frac1{r_i}
\right).
```

Define

**Equation TRISO-ML-384**

```math
\boxed{
\mathcal R_i
=
\frac1{4\pi D_i}
\left(
\frac1{r_{i-1}}-\frac1{r_i}
\right),
\qquad i=2,3,4,5.
}
```

Then

**Equation TRISO-ML-385**

```math
c_i(r_{i-1})-c_i(r_i)=\dot N\mathcal R_i.
```

Its units are

**Equation TRISO-ML-386**

```math
[\mathcal R_i]
=
\mathrm{s\,m^{-3}}.
```

For external transfer,

**Equation TRISO-ML-387**

```math
\dot N
=
4\pi R^2h[c_5(R)-c_\infty].
```

Rearrange:

**Equation TRISO-ML-388**

```math
c_5(R)-c_\infty
=
\dot N
\frac1{4\pi R^2h}.
```

Define

**Equation TRISO-ML-389**

```math
\boxed{
\mathcal R_h
=
\frac1{4\pi R^2h}.
}
```

Its units are also

**Equation TRISO-ML-390**

```math
[\mathcal R_h]=\mathrm{s\,m^{-3}}.
```

Because the same $\dot N$ passes through every coating and the external film, the concentration drops add:

**Equation TRISO-ML-391**

```math
c_2(r_1)-c_\infty
=
\dot N
\left(
\mathcal R_2+\mathcal R_3+\mathcal R_4+\mathcal R_5+\mathcal R_h
\right).
```

Thus

**Equation TRISO-ML-392**

```math
\boxed{
c_2(r_1)
=
c_\infty
+
\dot N
\left(
\mathcal R_2+\mathcal R_3+\mathcal R_4+\mathcal R_5+\mathcal R_h
\right).
}
```

For the benchmark $c_\infty=0$ and

**Equation TRISO-ML-393**

```math
\dot N=\frac{4\pi S_0r_1^3}{3}.
```

Substituting (TRISO-ML-384), (TRISO-ML-389), and (TRISO-ML-393) into (TRISO-ML-392) reproduces exactly the inward-recursion concentration at $r_1$.

The kernel itself is source-containing, so it is not represented by the same source-free shell resistance. Its centre-to-interface concentration rise is instead

**Equation TRISO-ML-394**

```math
c_1(0)-c_1(r_1)
=
\frac{S_0r_1^2}{6D_1}.
```

This distinction prevents a source-containing kernel from being incorrectly treated as an ordinary source-free series resistance.

## 12. Five-layer transient analytical formulation

### 12.1 Define the steady reference and transient deviation

Let

**Equation TRISO-ML-400**

```math
c_{i,\mathrm{ss}}(r)
```

denote the steady five-layer solution derived in Section 11.

Define the transient deviation in each layer:

**Equation TRISO-ML-401**

```math
\boxed{
v_i(r,t)
=
c_i(r,t)-c_{i,\mathrm{ss}}(r).
}
```

Rearrange:

**Equation TRISO-ML-402**

```math
c_i(r,t)
=
v_i(r,t)+c_{i,\mathrm{ss}}(r).
```

Because the steady reference is time independent,

**Equation TRISO-ML-403**

```math
\frac{\partial c_{i,\mathrm{ss}}}{\partial t}=0.
```

Therefore

**Equation TRISO-ML-404**

```math
\frac{\partial c_i}{\partial t}
=
\frac{\partial v_i}{\partial t}.
```

Similarly,

**Equation TRISO-ML-405**

```math
\frac{\partial c_i}{\partial r}
=
\frac{\partial v_i}{\partial r}
+
\frac{dc_{i,\mathrm{ss}}}{dr},
```

and

**Equation TRISO-ML-406**

```math
\frac{\partial^2c_i}{\partial r^2}
=
\frac{\partial^2v_i}{\partial r^2}
+
\frac{d^2c_{i,\mathrm{ss}}}{dr^2}.
```

The full layer equation is

**Equation TRISO-ML-407**

```math
\frac{\partial c_i}{\partial t}
=
D_i
\left(
\frac{\partial^2c_i}{\partial r^2}
+
\frac2r\frac{\partial c_i}{\partial r}
\right)
+
S_i.
```

Substitute (TRISO-ML-404) through (TRISO-ML-406):

**Equation TRISO-ML-408**

```math
\frac{\partial v_i}{\partial t}
=
D_i
\left(
v_{i,rr}
+
c_{i,\mathrm{ss}}''
+
\frac2r v_{i,r}
+
\frac2r c_{i,\mathrm{ss}}'
\right)
+
S_i.
```

Group transient and steady terms:

**Equation TRISO-ML-409**

```math
\frac{\partial v_i}{\partial t}
=
D_i
\left(
v_{i,rr}+\frac2r v_{i,r}
\right)
+
\left[
D_i
\left(
c_{i,\mathrm{ss}}''
+\frac2r c_{i,\mathrm{ss}}'
\right)
+
S_i
\right].
```

The steady solution satisfies

**Equation TRISO-ML-410**

```math
D_i
\left(
c_{i,\mathrm{ss}}''
+\frac2r c_{i,\mathrm{ss}}'
\right)
+
S_i
=
0.
```

Therefore

**Equation TRISO-ML-411**

```math
\boxed{
\frac{\partial v_i}{\partial t}
=
D_i
\left(
\frac{\partial^2v_i}{\partial r^2}
+
\frac2r\frac{\partial v_i}{\partial r}
\right).
}
```

### 12.2 Homogeneous transient conditions

At the centre, both $c_1$ and $c_{1,\mathrm{ss}}$ satisfy zero radial derivative. Therefore

**Equation TRISO-ML-412**

```math
\boxed{
v_{1,r}(0,t)=0.
}
```

At interface $r=r_i$, both the full and steady solutions satisfy concentration continuity. Subtracting the steady relation from the full relation gives

**Equation TRISO-ML-413**

```math
\boxed{
v_i(r_i,t)=v_{i+1}(r_i,t).
}
```

Both the full and steady solutions also satisfy flux continuity. Subtraction gives

**Equation TRISO-ML-414**

```math
\boxed{
-D_iv_i'(r_i,t)
=
-D_{i+1}v_{i+1}'(r_i,t).
}
```

At $R$, the full Robin condition is

**Equation TRISO-ML-415**

```math
-D_5c_5'(R,t)=h[c_5(R,t)-c_\infty].
```

The steady solution satisfies

**Equation TRISO-ML-416**

```math
-D_5c_{5,\mathrm{ss}}'(R)
=
h[c_{5,\mathrm{ss}}(R)-c_\infty].
```

Subtract (TRISO-ML-416) from (TRISO-ML-415):

**Equation TRISO-ML-417**

```math
\boxed{
-D_5v_5'(R,t)=hv_5(R,t).
}
```

For the initially empty source-driven problem,

**Equation TRISO-ML-418**

```math
c_i(r,0)=0.
```

Hence

**Equation TRISO-ML-419**

```math
\boxed{
v_i(r,0)
=
-c_{i,\mathrm{ss}}(r).
}
```

### 12.3 Separation in each layer

For one global transient mode, assume

**Equation TRISO-ML-420**

```math
v_i(r,t)
=
\phi_i(r)e^{-\Lambda t}.
```

The same temporal factor must apply in every layer because the interface conditions couple the layer amplitudes at the same physical time. A single global eigenmode cannot use independent exponential time factors on the two sides of one interface and still satisfy the interface equations for all $t$, except in a degenerate zero-amplitude case.

Differentiate with respect to time:

**Equation TRISO-ML-421**

```math
\frac{\partial v_i}{\partial t}
=
-\Lambda\phi_i e^{-\Lambda t}.
```

Differentiate with respect to radius:

**Equation TRISO-ML-422**

```math
\frac{\partial v_i}{\partial r}
=
\phi_i'e^{-\Lambda t}.
```

Differentiate again:

**Equation TRISO-ML-423**

```math
\frac{\partial^2v_i}{\partial r^2}
=
\phi_i''e^{-\Lambda t}.
```

Substitute into (TRISO-ML-411):

**Equation TRISO-ML-424**

```math
-\Lambda\phi_i e^{-\Lambda t}
=
D_i
\left(
\phi_i''
+\frac2r\phi_i'
\right)
e^{-\Lambda t}.
```

Cancel the non-zero exponential factor:

**Equation TRISO-ML-425**

```math
-\Lambda\phi_i
=
D_i
\left(
\phi_i''
+\frac2r\phi_i'
\right).
```

Divide by $D_i$:

**Equation TRISO-ML-426**

```math
\phi_i''
+\frac2r\phi_i'
+
\frac{\Lambda}{D_i}\phi_i
=
0.
```

Define

**Equation TRISO-ML-427**

```math
\boxed{
k_i^2=\frac{\Lambda}{D_i}.
}
```

Then

**Equation TRISO-ML-428**

```math
[k_i^2]
=
\frac{\mathrm{s^{-1}}}{\mathrm{m^2\,s^{-1}}}
=
\mathrm{m^{-2}},
```

so

**Equation TRISO-ML-429**

```math
[k_i]=\mathrm{m^{-1}}.
```

Each layer generally has a different $k_i$, because each layer has a different $D_i$, even though all layers in one global mode share the same $\Lambda$.

The radial equation is

**Equation TRISO-ML-430**

```math
\boxed{
\phi_i''
+\frac2r\phi_i'
+k_i^2\phi_i
=
0.
}
```

### 12.4 Apply the proven transformation $u_i=r\phi_i$

Section 10 proved that the transformation

**Equation TRISO-ML-431**

```math
u_i=r\phi_i
```

maps (TRISO-ML-430) to

**Equation TRISO-ML-432**

```math
\boxed{
u_i''+k_i^2u_i=0.
}
```

Therefore

**Equation TRISO-ML-433**

```math
\boxed{
u_i(r)
=
A_i\sin(k_ir)+B_i\cos(k_ir).
}
```

In the kernel,

**Equation TRISO-ML-434**

```math
\phi_1(r)=\frac{u_1(r)}{r}.
```

The cosine contribution $B_1\cos(k_1r)/r$ diverges as $r\to0$, exactly as proved in Section 10.

Therefore

**Equation TRISO-ML-435**

```math
\boxed{
B_1=0.
}
```

Thus

**Equation TRISO-ML-436**

```math
u_1(r)=A_1\sin(k_1r).
```

No coating layer contains the origin, so $B_i$ is not forced to zero for $i=2,3,4,5$.

### 12.5 Transform concentration continuity

At interface $r=r_i$,

**Equation TRISO-ML-437**

```math
\phi_i(r_i)=\phi_{i+1}(r_i).
```

Use $\phi_i=u_i/r$:

**Equation TRISO-ML-438**

```math
\frac{u_i(r_i)}{r_i}
=
\frac{u_{i+1}(r_i)}{r_i}.
```

Multiply by the common non-zero radius $r_i$:

**Equation TRISO-ML-439**

```math
\boxed{
u_i(r_i)=u_{i+1}(r_i).
}
```

### 12.6 Transform flux continuity

Start from

**Equation TRISO-ML-440**

```math
\phi_i(r)=\frac{u_i(r)}{r}.
```

Differentiate using the quotient rule:

**Equation TRISO-ML-441**

```math
\phi_i'(r)
=
\frac{ru_i'(r)-u_i(r)}{r^2}.
```

Separate the two terms:

**Equation TRISO-ML-442**

```math
\boxed{
\phi_i'(r)
=
\frac{u_i'(r)}{r}
-
\frac{u_i(r)}{r^2}.
}
```

Flux continuity at $r=r_i$ is

**Equation TRISO-ML-443**

```math
-D_i\phi_i'(r_i)
=
-D_{i+1}\phi_{i+1}'(r_i).
```

Cancel the common minus sign:

**Equation TRISO-ML-444**

```math
D_i\phi_i'(r_i)
=
D_{i+1}\phi_{i+1}'(r_i).
```

Substitute (TRISO-ML-442) on both sides:

**Equation TRISO-ML-445**

```math
D_i
\left[
\frac{u_i'(r_i)}{r_i}
-
\frac{u_i(r_i)}{r_i^2}
\right]
=
D_{i+1}
\left[
\frac{u_{i+1}'(r_i)}{r_i}
-
\frac{u_{i+1}(r_i)}{r_i^2}
\right].
```

Multiply by the common factor $r_i$:

**Equation TRISO-ML-446**

```math
\boxed{
D_i
\left[
u_i'(r_i)-\frac{u_i(r_i)}{r_i}
\right]
=
D_{i+1}
\left[
u_{i+1}'(r_i)-\frac{u_{i+1}(r_i)}{r_i}
\right].
}
```

This is the transformed ideal flux-continuity condition.

### 12.7 Transform the outer Robin condition

The transient Robin condition is

**Equation TRISO-ML-447**

```math
-D_5\phi_5'(R)=h\phi_5(R).
```

Use

**Equation TRISO-ML-448**

```math
\phi_5'(R)
=
\frac{u_5'(R)}{R}
-
\frac{u_5(R)}{R^2},
```

and

**Equation TRISO-ML-449**

```math
\phi_5(R)=\frac{u_5(R)}{R}.
```

Substitute both:

**Equation TRISO-ML-450**

```math
-D_5
\left[
\frac{u_5'(R)}{R}
-
\frac{u_5(R)}{R^2}
\right]
=
h\frac{u_5(R)}{R}.
```

Multiply by $R$:

**Equation TRISO-ML-451**

```math
\boxed{
-D_5
\left[
u_5'(R)-\frac{u_5(R)}{R}
\right]
=
hu_5(R).
}
```

Equivalently,

**Equation TRISO-ML-452**

```math
D_5u_5'(R)
+
\left(
h-\frac{D_5}{R}
\right)u_5(R)
=
0.
```

### 12.8 Count the unknown coefficients

Before centre regularity, five layers would provide ten coefficients:

**Equation TRISO-ML-453**

```math
(A_1,B_1,A_2,B_2,A_3,B_3,A_4,B_4,A_5,B_5).
```

Centre regularity fixes

**Equation TRISO-ML-454**

```math
B_1=0.
```

Therefore nine independent coefficients remain.

Define

**Equation TRISO-ML-455**

```math
\boxed{
\mathbf a
=
(A_1,A_2,B_2,A_3,B_3,A_4,B_4,A_5,B_5)^T.
}
```

Thus

**Equation TRISO-ML-456**

```math
\mathbf a\in\mathbb R^9
```

for real $\Lambda>0$.

### 12.9 Define interface shorthand

For compact matrix notation, define at interface $r=r_j$

**Equation TRISO-ML-457**

```math
s_{ij}=\sin(k_ir_j),
\qquad
c_{ij}=\cos(k_ir_j).
```

For a sine basis term,

**Equation TRISO-ML-458**

```math
u_i=A_i\sin(k_ir),
```

and

**Equation TRISO-ML-459**

```math
u_i'=A_ik_i\cos(k_ir).
```

Therefore the transformed flux factor for the sine basis at $r_j$ is

**Equation TRISO-ML-460**

```math
F^{(s)}_{ij}
=
D_i
\left(
k_ic_{ij}-\frac{s_{ij}}{r_j}
\right).
```

For a cosine basis term,

**Equation TRISO-ML-461**

```math
u_i=B_i\cos(k_ir),
```

and

**Equation TRISO-ML-462**

```math
u_i'=-B_ik_i\sin(k_ir).
```

Therefore its transformed flux factor is

**Equation TRISO-ML-463**

```math
F^{(c)}_{ij}
=
D_i
\left(
-k_is_{ij}-\frac{c_{ij}}{r_j}
\right).
```

These quantities depend on $\Lambda$ through $k_i=\sqrt{\Lambda/D_i}$.

### 12.10 Assemble the global homogeneous coefficient system

There are two equations at each of the four internal interfaces:

- one concentration-continuity equation;
- one flux-continuity equation.

This gives

**Equation TRISO-ML-464**

```math
4\times2=8
```

interface equations.

The outer Robin boundary supplies one more equation:

**Equation TRISO-ML-465**

```math
8+1=9.
```

These nine equations determine the nine coefficients up to an arbitrary overall modal normalization when $\Lambda$ is an eigenvalue.

Write

**Equation TRISO-ML-466**

```math
\boxed{
\mathbf M(\Lambda)\mathbf a=\mathbf0.
}
```

The matrix dimensions are

**Equation TRISO-ML-467**

```math
\boxed{
\mathbf M(\Lambda)\in\mathbb R^{9\times9}.
}
```

Using the coefficient order in (TRISO-ML-455), rows 1–2 correspond to $r_1$, rows 3–4 to $r_2$, rows 5–6 to $r_3$, rows 7–8 to $r_4$, and row 9 to the outer Robin condition.

The explicit matrix is

**Equation TRISO-ML-468**

```math
\mathbf M(\Lambda)=
\begin{pmatrix}
s_{11} & -s_{21} & -c_{21} & 0 & 0 & 0 & 0 & 0 & 0\\
F^{(s)}_{11} & -F^{(s)}_{21} & -F^{(c)}_{21} & 0 & 0 & 0 & 0 & 0 & 0\\
0 & s_{22} & c_{22} & -s_{32} & -c_{32} & 0 & 0 & 0 & 0\\
0 & F^{(s)}_{22} & F^{(c)}_{22} & -F^{(s)}_{32} & -F^{(c)}_{32} & 0 & 0 & 0 & 0\\
0 & 0 & 0 & s_{33} & c_{33} & -s_{43} & -c_{43} & 0 & 0\\
0 & 0 & 0 & F^{(s)}_{33} & F^{(c)}_{33} & -F^{(s)}_{43} & -F^{(c)}_{43} & 0 & 0\\
0 & 0 & 0 & 0 & 0 & s_{44} & c_{44} & -s_{54} & -c_{54}\\
0 & 0 & 0 & 0 & 0 & F^{(s)}_{44} & F^{(c)}_{44} & -F^{(s)}_{54} & -F^{(c)}_{54}\\
0 & 0 & 0 & 0 & 0 & 0 & 0 & G_s & G_c
\end{pmatrix}.
```

The outer-row coefficients follow from (TRISO-ML-452).

For the sine basis,

**Equation TRISO-ML-469**

```math
G_s
=
D_5k_5\cos(k_5R)
+
\left(
h-\frac{D_5}{R}
\right)
\sin(k_5R).
```

For the cosine basis,

**Equation TRISO-ML-470**

```math
G_c
=
-D_5k_5\sin(k_5R)
+
\left(
h-\frac{D_5}{R}
\right)
\cos(k_5R).
```

Centre regularity does not appear as a matrix row because it has already been used to eliminate $B_1$ from the unknown vector.

### 12.11 Global eigenvalue condition

For a generic value of $\Lambda$, the homogeneous system

**Equation TRISO-ML-471**

```math
\mathbf M(\Lambda)\mathbf a=\mathbf0
```

has only the trivial solution

**Equation TRISO-ML-472**

```math
\mathbf a=\mathbf0
```

when $\mathbf M$ is nonsingular.

A non-zero global eigenmode requires a non-trivial coefficient vector:

**Equation TRISO-ML-473**

```math
\mathbf a\ne\mathbf0.
```

A square homogeneous linear system has a non-trivial solution only if its matrix is singular.

Therefore

**Equation TRISO-ML-474**

```math
\boxed{
\det\mathbf M(\Lambda)=0.
}
```

Define

**Equation TRISO-ML-475**

```math
\boxed{
F(\Lambda)=\det\mathbf M(\Lambda).
}
```

The global modal decay rates are the positive roots

**Equation TRISO-ML-476**

```math
F(\Lambda_n)=0.
```

For each root $\Lambda_n$, the layer wave numbers are

**Equation TRISO-ML-477**

```math
\boxed{
k_{i,n}
=
\sqrt{\frac{\Lambda_n}{D_i}}.
}
```

Thus one global decay rate $\Lambda_n$ generates five material-dependent spatial wave numbers.

### 12.12 Global conservative self-adjoint structure

Return to the separated transient equation before the substitution $u_i=r\phi_i$.

Equation (TRISO-ML-425) is

**Equation TRISO-ML-478**

```math
-\Lambda\phi_i
=
D_i
\left(
\phi_i''
+
\frac2r\phi_i'
\right).
```

Multiply by $r^2$:

**Equation TRISO-ML-479**

```math
-\Lambda r^2\phi_i
=
D_i
\left(
r^2\phi_i''
+
2r\phi_i'
\right).
```

Because $D_i$ is constant inside layer $i$,

**Equation TRISO-ML-480**

```math
D_i
\left(
r^2\phi_i''
+
2r\phi_i'
\right)
=
\frac{d}{dr}
\left(
r^2D_i\phi_i'
\right).
```

Therefore

**Equation TRISO-ML-481**

```math
-\Lambda r^2\phi_i
=
\frac{d}{dr}
\left(
r^2D_i\phi_i'
\right).
```

Multiply by $-1$:

**Equation TRISO-ML-482**

```math
\boxed{
-\frac{d}{dr}
\left(
r^2D_i\phi_i'
\right)
=
\Lambda r^2\phi_i.
}
```

This is the conservative eigen-equation in layer $i$.

Define the piecewise diffusivity

**Equation TRISO-ML-483**

```math
D(r)=D_i,
\qquad
r_{i-1}<r<r_i.
```

Then the piecewise Sturm–Liouville coefficient is

**Equation TRISO-ML-484**

```math
\boxed{
p(r)=r^2D(r).
}
```

There is no zeroth-order potential term:

**Equation TRISO-ML-485**

```math
\boxed{
q(r)=0.
}
```

Comparing

**Equation TRISO-ML-486**

```math
-\frac{d}{dr}
\left(
p(r)\phi'
\right)
+
q(r)\phi
=
\Lambda w(r)\phi
```

with (TRISO-ML-482) shows that the weight is

**Equation TRISO-ML-487**

```math
\boxed{
w(r)=r^2.
}
```

Thus the weight $r^2$ follows from the physical conservative eigen-equation; it is not imported by analogy with the homogeneous sphere.

### 12.13 Layerwise Lagrange identity for two global modes

Let global mode $m$ have eigenvalue $\Lambda_m$ and layer functions $\phi_i^{(m)}$.

In layer $i$,

**Equation TRISO-ML-488**

```math
-\frac{d}{dr}
\left(
r^2D_i\frac{d\phi_i^{(m)}}{dr}
\right)
=
\Lambda_m r^2\phi_i^{(m)}.
```

Let global mode $n$ have eigenvalue $\Lambda_n$:

**Equation TRISO-ML-489**

```math
-\frac{d}{dr}
\left(
r^2D_i\frac{d\phi_i^{(n)}}{dr}
\right)
=
\Lambda_n r^2\phi_i^{(n)}.
```

Multiply the $m$-equation by $\phi_i^{(n)}$:

**Equation TRISO-ML-490**

```math
-\phi_i^{(n)}
\frac{d}{dr}
\left(
r^2D_i\phi_i^{(m)\prime}
\right)
=
\Lambda_m r^2
\phi_i^{(m)}
\phi_i^{(n)}.
```

Multiply the $n$-equation by $\phi_i^{(m)}$:

**Equation TRISO-ML-491**

```math
-\phi_i^{(m)}
\frac{d}{dr}
\left(
r^2D_i\phi_i^{(n)\prime}
\right)
=
\Lambda_n r^2
\phi_i^{(m)}
\phi_i^{(n)}.
```

Subtract (TRISO-ML-491) from (TRISO-ML-490):

**Equation TRISO-ML-492**

```math
-\phi_i^{(n)}
\frac{d}{dr}
\left(
r^2D_i\phi_i^{(m)\prime}
\right)
+
\phi_i^{(m)}
\frac{d}{dr}
\left(
r^2D_i\phi_i^{(n)\prime}
\right)
=
(\Lambda_m-\Lambda_n)
r^2\phi_i^{(m)}\phi_i^{(n)}.
```

Define

**Equation TRISO-ML-493**

```math
P_i(r)=r^2D_i.
```

Use the product rule:

```math
\frac{d}{dr}
\left[
P_i
\left(
\phi_i^{(m)}\phi_i^{(n)\prime}
-
\phi_i^{(n)}\phi_i^{(m)\prime}
\right)
\right]
```

**Equation TRISO-ML-494**

```math
=
\phi_i^{(m)}
\frac{d}{dr}
\left(
P_i\phi_i^{(n)\prime}
\right)
-
\phi_i^{(n)}
\frac{d}{dr}
\left(
P_i\phi_i^{(m)\prime}
\right).
```

Therefore (TRISO-ML-492) becomes

**Equation TRISO-ML-495**

```math
\frac{d}{dr}
\left[
r^2D_i
\left(
\phi_i^{(m)}\phi_i^{(n)\prime}
-
\phi_i^{(n)}\phi_i^{(m)\prime}
\right)
\right]
=
(\Lambda_m-\Lambda_n)
r^2\phi_i^{(m)}\phi_i^{(n)}.
```

Integrate over layer $i$:

```math
\int_{r_{i-1}}^{r_i}
\frac{d}{dr}
\left[
r^2D_i
\left(
\phi_i^{(m)}\phi_i^{(n)\prime}
-
\phi_i^{(n)}\phi_i^{(m)\prime}
\right)
\right]dr
```

**Equation TRISO-ML-496**

```math
=
(\Lambda_m-\Lambda_n)
\int_{r_{i-1}}^{r_i}
r^2\phi_i^{(m)}\phi_i^{(n)}\,dr.
```

Evaluate the derivative integral:

```math
\left[
r^2D_i
\left(
\phi_i^{(m)}\phi_i^{(n)\prime}
-
\phi_i^{(n)}\phi_i^{(m)\prime}
\right)
\right]_{r_{i-1}}^{r_i}
```

**Equation TRISO-ML-497**

```math
=
(\Lambda_m-\Lambda_n)
\int_{r_{i-1}}^{r_i}
r^2\phi_i^{(m)}\phi_i^{(n)}\,dr.
```

### 12.14 Sum over all five layers

Define the layer boundary expression

**Equation TRISO-ML-498**

```math
\mathcal B_i(r)
=
r^2D_i
\left(
\phi_i^{(m)}\phi_i^{(n)\prime}
-
\phi_i^{(n)}\phi_i^{(m)\prime}
\right).
```

Equation (TRISO-ML-497) is

**Equation TRISO-ML-499**

```math
\mathcal B_i(r_i)
-
\mathcal B_i(r_{i-1})
=
(\Lambda_m-\Lambda_n)
\int_{r_{i-1}}^{r_i}
r^2\phi_i^{(m)}\phi_i^{(n)}\,dr.
```

Sum from $i=1$ to $5$:

```math
\sum_{i=1}^{5}
\left[
\mathcal B_i(r_i)
-
\mathcal B_i(r_{i-1})
\right]
```

**Equation TRISO-ML-500**

```math
=
(\Lambda_m-\Lambda_n)
\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2\phi_i^{(m)}\phi_i^{(n)}\,dr.
```

Write the left side explicitly:

```math
\mathcal B_1(r_1)-\mathcal B_1(0)
+
\mathcal B_2(r_2)-\mathcal B_2(r_1)
```

**Equation TRISO-ML-501**

```math
+
\mathcal B_3(r_3)-\mathcal B_3(r_2)
+
\mathcal B_4(r_4)-\mathcal B_4(r_3)
+
\mathcal B_5(R)-\mathcal B_5(r_4).
```

Group the four internal-interface contributions:

```math
-\mathcal B_1(0)
+
\left[
\mathcal B_1(r_1)-\mathcal B_2(r_1)
\right]
+
\left[
\mathcal B_2(r_2)-\mathcal B_3(r_2)
\right]
```

**Equation TRISO-ML-502**

```math
+
\left[
\mathcal B_3(r_3)-\mathcal B_4(r_3)
\right]
+
\left[
\mathcal B_4(r_4)-\mathcal B_5(r_4)
\right]
+
\mathcal B_5(R).
```

### 12.15 Explicit cancellation at one internal interface

Consider interface $r=r_j$ between layers $j$ and $j+1$.

The contribution from the left layer is

**Equation TRISO-ML-503**

```math
\mathcal B_j(r_j)
=
r_j^2D_j
\left[
\phi_j^{(m)}\phi_j^{(n)\prime}
-
\phi_j^{(n)}\phi_j^{(m)\prime}
\right]_{r_j}.
```

The contribution from the right layer enters with a minus sign:

**Equation TRISO-ML-504**

```math
-\mathcal B_{j+1}(r_j)
=
-r_j^2D_{j+1}
\left[
\phi_{j+1}^{(m)}\phi_{j+1}^{(n)\prime}
-
\phi_{j+1}^{(n)}\phi_{j+1}^{(m)\prime}
\right]_{r_j}.
```

For ideal concentration continuity,

**Equation TRISO-ML-505**

```math
\phi_j^{(m)}(r_j)
=
\phi_{j+1}^{(m)}(r_j),
```

and

**Equation TRISO-ML-506**

```math
\phi_j^{(n)}(r_j)
=
\phi_{j+1}^{(n)}(r_j).
```

For ideal flux continuity,

**Equation TRISO-ML-507**

```math
D_j\phi_j^{(m)\prime}(r_j)
=
D_{j+1}\phi_{j+1}^{(m)\prime}(r_j),
```

and

**Equation TRISO-ML-508**

```math
D_j\phi_j^{(n)\prime}(r_j)
=
D_{j+1}\phi_{j+1}^{(n)\prime}(r_j).
```

Use (TRISO-ML-505) and (TRISO-ML-508) in the first product of (TRISO-ML-503):

**Equation TRISO-ML-509**

```math
D_j
\phi_j^{(m)}
\phi_j^{(n)\prime}
=
\phi_{j+1}^{(m)}
D_{j+1}\phi_{j+1}^{(n)\prime}.
```

Use (TRISO-ML-506) and (TRISO-ML-507) in the second product:

**Equation TRISO-ML-510**

```math
D_j
\phi_j^{(n)}
\phi_j^{(m)\prime}
=
\phi_{j+1}^{(n)}
D_{j+1}\phi_{j+1}^{(m)\prime}.
```

Therefore

**Equation TRISO-ML-511**

```math
\mathcal B_j(r_j)
=
r_j^2D_{j+1}
\left[
\phi_{j+1}^{(m)}
\phi_{j+1}^{(n)\prime}
-
\phi_{j+1}^{(n)}
\phi_{j+1}^{(m)\prime}
\right].
```

The right side of (TRISO-ML-511) is exactly

**Equation TRISO-ML-512**

```math
\mathcal B_{j+1}(r_j).
```

Hence

**Equation TRISO-ML-513**

```math
\boxed{
\mathcal B_j(r_j)-\mathcal B_{j+1}(r_j)=0.
}
```

The same argument applies independently at $r_1,r_2,r_3,r_4$, including when adjacent diffusivities are unequal.

Thus all four internal-interface terms in (TRISO-ML-502) cancel pairwise.

### 12.16 Centre boundary contribution

At the centre,

**Equation TRISO-ML-514**

```math
\mathcal B_1(0)
=
\lim_{r\to0}
r^2D_1
\left(
\phi_1^{(m)}\phi_1^{(n)\prime}
-
\phi_1^{(n)}\phi_1^{(m)\prime}
\right).
```

Regularity gives finite centre values for both eigenfunctions:

**Equation TRISO-ML-515**

```math
|\phi_1^{(m)}(0)|<\infty,
\qquad
|\phi_1^{(n)}(0)|<\infty.
```

Spherical symmetry gives

**Equation TRISO-ML-516**

```math
\phi_1^{(m)\prime}(0)=0,
\qquad
\phi_1^{(n)\prime}(0)=0.
```

For regular eigenfunctions, the bracketed quantity remains bounded as $r\to0$.

Since

**Equation TRISO-ML-517**

```math
r^2D_1\to0
\qquad
(r\to0),
```

we obtain

**Equation TRISO-ML-518**

```math
\boxed{
\mathcal B_1(0)=0.
}
```

### 12.17 Outer Robin boundary contribution

At $r=R$,

**Equation TRISO-ML-519**

```math
\mathcal B_5(R)
=
R^2D_5
\left[
\phi_5^{(m)}(R)\phi_5^{(n)\prime}(R)
-
\phi_5^{(n)}(R)\phi_5^{(m)\prime}(R)
\right].
```

Both modes satisfy the same homogeneous Robin condition:

**Equation TRISO-ML-520**

```math
-D_5\phi_5^{(m)\prime}(R)
=
h\phi_5^{(m)}(R),
```

and

**Equation TRISO-ML-521**

```math
-D_5\phi_5^{(n)\prime}(R)
=
h\phi_5^{(n)}(R).
```

Solve the first condition for the derivative:

**Equation TRISO-ML-522**

```math
D_5\phi_5^{(m)\prime}(R)
=
-h\phi_5^{(m)}(R).
```

Similarly,

**Equation TRISO-ML-523**

```math
D_5\phi_5^{(n)\prime}(R)
=
-h\phi_5^{(n)}(R).
```

Substitute into (TRISO-ML-519):

**Equation TRISO-ML-524**

```math
\mathcal B_5(R)
=
R^2
\left[
-h\phi_5^{(m)}(R)\phi_5^{(n)}(R)
+
h\phi_5^{(n)}(R)\phi_5^{(m)}(R)
\right].
```

The two products are identical and have opposite signs:

**Equation TRISO-ML-525**

```math
\boxed{
\mathcal B_5(R)=0.
}
```

### 12.18 Global multilayer orthogonality

All terms on the left side of (TRISO-ML-500) now vanish:

- centre term by (TRISO-ML-518);
- four interface pairs by (TRISO-ML-513);
- outer Robin term by (TRISO-ML-525).

Therefore

**Equation TRISO-ML-526**

```math
0
=
(\Lambda_m-\Lambda_n)
\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2
\phi_i^{(m)}(r)
\phi_i^{(n)}(r)
\,dr.
```

For distinct eigenvalues,

**Equation TRISO-ML-527**

```math
\Lambda_m\ne\Lambda_n.
```

Divide by $\Lambda_m-\Lambda_n$:

**Equation TRISO-ML-528**

```math
\boxed{
\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2
\phi_i^{(m)}(r)
\phi_i^{(n)}(r)
\,dr
=
0,
\qquad
m\ne n.
}
```

Thus the correct global weight is

**Equation TRISO-ML-529**

```math
\boxed{
w(r)=r^2.
}
```

Define the global piecewise eigenfunction

**Equation TRISO-ML-530**

```math
\Phi_n(r)
=
\phi_i^{(n)}(r),
\qquad
r_{i-1}<r<r_i.
```

Then define the weighted inner product

**Equation TRISO-ML-531**

```math
\boxed{
\langle f,g\rangle_w
=
\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2f_i(r)g_i(r)\,dr.
}
```

Global orthogonality is

**Equation TRISO-ML-532**

```math
\boxed{
\langle\Phi_m,\Phi_n\rangle_w=0,
\qquad
m\ne n.
}
```

### 12.19 Modal norm and normalization

Define the norm of mode $n$:

**Equation TRISO-ML-533**

```math
\boxed{
N_n
=
\langle\Phi_n,\Phi_n\rangle_w
=
\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2
\left[
\phi_i^{(n)}(r)
\right]^2
dr.
}
```

If $\phi_i$ carries concentration units, then

**Equation TRISO-ML-534**

```math
[N_n]
=
\mathrm{m^3}
[\phi]^2.
```

If instead the eigenfunctions are chosen dimensionless, then

**Equation TRISO-ML-535**

```math
[N_n]=\mathrm{m^3}.
```

The eigenvalue problem determines each coefficient vector only up to an arbitrary non-zero scale.

If

**Equation TRISO-ML-536**

```math
\Phi_n\to\alpha_n\Phi_n,
```

then

**Equation TRISO-ML-537**

```math
N_n\to\alpha_n^2N_n.
```

The corresponding modal amplitude transforms inversely:

**Equation TRISO-ML-538**

```math
A_n\to\frac{A_n}{\alpha_n}.
```

Therefore the physical product

**Equation TRISO-ML-539**

```math
A_n\Phi_n
```

is unchanged.

One convenient convention is unit weighted norm:

**Equation TRISO-ML-540**

```math
N_n=1.
```

No such normalization is required for the projection formula below.

### 12.20 Project the initial transient state

For the source-driven benchmark,

**Equation TRISO-ML-541**

```math
v_i(r,0)
=
-c_{i,\mathrm{ss}}(r).
```

Assume the global modal representation

**Equation TRISO-ML-542**

```math
v_i(r,t)
=
\sum_{n=1}^{\infty}
A_n
\phi_i^{(n)}(r)
e^{-\Lambda_nt}.
```

At $t=0$,

**Equation TRISO-ML-543**

```math
e^{-\Lambda_n0}=1.
```

Therefore

**Equation TRISO-ML-544**

```math
-c_{i,\mathrm{ss}}(r)
=
\sum_{n=1}^{\infty}
A_n\phi_i^{(n)}(r),
\qquad
r_{i-1}<r<r_i.
```

Multiply the equation in layer $i$ by

**Equation TRISO-ML-545**

```math
r^2\phi_i^{(m)}(r).
```

This gives

**Equation TRISO-ML-546**

```math
-r^2
c_{i,\mathrm{ss}}(r)
\phi_i^{(m)}(r)
=
\sum_{n=1}^{\infty}
A_n
r^2
\phi_i^{(n)}(r)
\phi_i^{(m)}(r).
```

Integrate over layer $i$:

**Equation TRISO-ML-547**

```math
-\int_{r_{i-1}}^{r_i}
r^2
c_{i,\mathrm{ss}}
\phi_i^{(m)}
\,dr
=
\sum_{n=1}^{\infty}
A_n
\int_{r_{i-1}}^{r_i}
r^2
\phi_i^{(n)}
\phi_i^{(m)}
\,dr.
```

Sum all five layers:

```math
-\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2
c_{i,\mathrm{ss}}
\phi_i^{(m)}
\,dr
```

**Equation TRISO-ML-548**

```math
=
\sum_{n=1}^{\infty}
A_n
\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2
\phi_i^{(n)}
\phi_i^{(m)}
\,dr.
```

For every $n\ne m$, global orthogonality gives

**Equation TRISO-ML-549**

```math
\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2
\phi_i^{(n)}
\phi_i^{(m)}
\,dr
=
0.
```

Therefore only the $n=m$ term remains:

**Equation TRISO-ML-550**

```math
-\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2
c_{i,\mathrm{ss}}
\phi_i^{(m)}
\,dr
=
A_m
\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2
\left[
\phi_i^{(m)}
\right]^2
dr.
```

The denominator is $N_m$:

**Equation TRISO-ML-551**

```math
-\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2
c_{i,\mathrm{ss}}
\phi_i^{(m)}
\,dr
=
A_mN_m.
```

Divide by $N_m>0$:

**Equation TRISO-ML-552**

```math
\boxed{
A_m
=
-
\frac{
\displaystyle
\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2
c_{i,\mathrm{ss}}(r)
\phi_i^{(m)}(r)
\,dr
}{
\displaystyle
\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2
\left[
\phi_i^{(m)}(r)
\right]^2
dr
}.
}
```

This is the multilayer modal coefficient formula, conditional on the assumed completeness of the global eigenfunction family for representing the initial transient state.

### 12.21 Complete formal five-layer transient solution

Because

**Equation TRISO-ML-553**

```math
c_i=v_i+c_{i,\mathrm{ss}},
```

substitute the modal expansion (TRISO-ML-542):

**Equation TRISO-ML-554**

```math
\boxed{
c_i(r,t)
=
c_{i,\mathrm{ss}}(r)
+
\sum_{n=1}^{\infty}
A_n
\phi_i^{(n)}(r)
e^{-\Lambda_nt},
\qquad
r_{i-1}<r<r_i.
}
```

#### Initial-time check

At $t=0$,

**Equation TRISO-ML-555**

```math
c_i(r,0)
=
c_{i,\mathrm{ss}}(r)
+
\sum_{n=1}^{\infty}
A_n\phi_i^{(n)}(r).
```

If the eigenfunction expansion represents the initial transient,

**Equation TRISO-ML-556**

```math
\sum_{n=1}^{\infty}
A_n\phi_i^{(n)}(r)
=
-c_{i,\mathrm{ss}}(r).
```

Therefore

**Equation TRISO-ML-557**

```math
c_i(r,0)=0.
```

This recovery is formal and depends on the completeness/convergence statement discussed below.

#### Long-time check

For every positive decay rate,

**Equation TRISO-ML-558**

```math
\Lambda_n>0,
```

so

**Equation TRISO-ML-559**

```math
e^{-\Lambda_nt}\to0
\qquad
(t\to\infty).
```

Formally,

**Equation TRISO-ML-560**

```math
\boxed{
c_i(r,t)\to c_{i,\mathrm{ss}}(r)
\qquad
(t\to\infty).
}
```

#### Interface check

Every global eigenmode separately satisfies

**Equation TRISO-ML-561**

```math
\phi_i^{(n)}(r_i)
=
\phi_{i+1}^{(n)}(r_i),
```

and

**Equation TRISO-ML-562**

```math
D_i\phi_i^{(n)\prime}(r_i)
=
D_{i+1}\phi_{i+1}^{(n)\prime}(r_i).
```

A linear combination of such modes therefore preserves both homogeneous interface conditions.

Adding the steady solution restores the corresponding full interface conditions.

#### Centre check

Every kernel eigenfunction has $B_1=0$, so it is regular at $r=0$.

Therefore each transient mode is finite at the centre and satisfies the centre symmetry condition.

#### Outer-boundary check

Every mode satisfies

**Equation TRISO-ML-563**

```math
-D_5\phi_5^{(n)\prime}(R)
=
h\phi_5^{(n)}(R).
```

Multiplication by the scalar factor $A_ne^{-\Lambda_nt}$ preserves this relation.

Therefore the complete transient sum satisfies the homogeneous Robin condition whenever termwise boundary evaluation is justified.

Adding the steady solution restores the full Robin boundary condition.

### 12.22 Completeness and convergence status

The orthogonality result (TRISO-ML-528) was derived directly from the conservative differential equations, interface conditions, centre regularity, and outer Robin condition.

It does not require a completeness theorem.

[VERIFIED] Distinct global eigenmodes are orthogonal in the weighted inner product with $w(r)=r^2$.

The projection algebra leading to (TRISO-ML-552) is also valid once an expansion in the eigenfunctions is admitted.

[DERIVED CONDITIONALLY] The modal projection formula follows from orthogonality under the assumption that the initial transient lies in the closure of the global eigenfunction span.

Completeness is a separate spectral statement.

Standard regular Sturm–Liouville completeness theorems provide completeness in an appropriate weighted $L^2$ space for regular self-adjoint problems on finite intervals. The present TRISO problem is more delicate because:

1. $p(r)=r^2D(r)$ vanishes at $r=0$, so the centre is a singular endpoint;
2. $D(r)$ is piecewise constant and discontinuous at four internal interfaces;
3. the operator domain includes transmission conditions enforcing continuity of concentration and flux.

The current derivation has explicitly demonstrated the self-adjoint boundary/interface cancellation needed for symmetry of the operator.

However, this project has not yet supplied a theorem specifically covering the singular endpoint together with these piecewise transmission conditions and proving completeness of the resulting eigenfunctions.

[SOURCE / THEOREM NEEDED] A rigorous completeness theorem for this self-adjoint singular Sturm–Liouville/transmission problem, or an equivalent self-adjoint compact-resolvent operator formulation.

Accordingly, the current statuses are:

- orthogonality: **VERIFIED by direct derivation**;
- modal projection formula: **DERIVED CONDITIONALLY**;
- completeness: **THEOREM-DEPENDENT / NOT YET PROVED IN THIS PROJECT**;
- numerical convergence of truncated modal sums: **UNVERIFIED**.

The standard Sturm–Liouville framework and regular-problem completeness results provide mathematical context, but they are not being silently promoted to a proof for this singular piecewise problem.

### 12.23 Homogeneous-diffusivity reduction check

Set

**Equation TRISO-ML-564**

```math
D_1=D_2=D_3=D_4=D_5=D.
```

Then

**Equation TRISO-ML-565**

```math
p(r)=r^2D
```

throughout the sphere.

Flux continuity becomes

**Equation TRISO-ML-566**

```math
D\phi_i'(r_i)=D\phi_{i+1}'(r_i).
```

Cancel $D>0$:

**Equation TRISO-ML-567**

```math
\phi_i'(r_i)=\phi_{i+1}'(r_i).
```

Together with concentration continuity,

**Equation TRISO-ML-568**

```math
\phi_i(r_i)=\phi_{i+1}(r_i),
```

the artificial internal material boundaries become transparent to the homogeneous eigenfunction.

The global inner product becomes

**Equation TRISO-ML-569**

```math
\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2\phi_m\phi_n\,dr.
```

Because the five intervals partition $(0,R)$,

**Equation TRISO-ML-570**

```math
\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2\phi_m\phi_n\,dr
=
\int_0^R
r^2\phi_m\phi_n\,dr.
```

Therefore the multilayer orthogonality relation reduces to

**Equation TRISO-ML-571**

```math
\boxed{
\int_0^R
r^2\phi_m(r)\phi_n(r)\,dr
=
0,
\qquad
m\ne n,
}
```

which is exactly the homogeneous spherical weight derived in Section 10.

### 12.24 Unequal-diffusivity interface check

The interface cancellation did not require

**Equation TRISO-ML-572**

```math
D_j=D_{j+1}.
```

Instead it used the physical transmission condition

**Equation TRISO-ML-573**

```math
D_j\phi_j'
=
D_{j+1}\phi_{j+1}'.
```

Therefore unequal adjacent diffusivities remain fully compatible with the global orthogonality proof.

The diffusivity discontinuity is carried by $p(r)=r^2D(r)$, while the weight remains $r^2$.

### 12.25 Updated analytical status

[VERIFIED] Five-layer steady analytical solution under the frozen ideal benchmark assumptions.

[VERIFIED] Explicit global eigenvalue system and determinant condition.

[VERIFIED] Piecewise conservative self-adjoint/Lagrange structure.

[VERIFIED] Pairwise cancellation of all four ideal-interface boundary terms.

[VERIFIED] Centre and Robin boundary cancellation.

[VERIFIED] Global orthogonality with weight $r^2$.

[DERIVED CONDITIONALLY] Global modal coefficient projection.

[FORMAL / CONDITIONALLY VERIFIED] Infinite five-layer transient modal representation.

[THEOREM-DEPENDENT] Completeness of the global eigenfunction family.

[UNVERIFIED] Numerical convergence rate and truncation error of the modal series.

## 13. Original FTCS deterministic benchmark discretisation

This section derives the original Ray notebook's FTCS method as a transparent deterministic benchmark.

It is **not** the supervisor repository's production Walk-on-Spheres method.

The present subsection is restricted to an interior node lying entirely inside one homogeneous material region, so that the local diffusivity is constant and no material interface lies inside the stencil.

### 13.1 Starting continuous equation

Inside one homogeneous layer, begin from

**Equation TRISO-DIS-100**

```math
\frac{\partial c}{\partial t}
=
D
\left(
\frac{\partial^2c}{\partial r^2}
+
\frac2r\frac{\partial c}{\partial r}
\right)
+
S.
```

[ASSUMPTION] $D$ is constant over the local stencil.

[ASSUMPTION] The mesh is uniform for this Part-I benchmark.

### 13.2 Define the spatial mesh

Divide the interval

**Equation TRISO-DIS-101**

```math
0\le r\le R
```

into $N$ equal intervals.

Define

**Equation TRISO-DIS-102**

```math
\Delta r=\frac{R}{N}.
```

The node locations are

**Equation TRISO-DIS-103**

```math
r_i=i\Delta r,
\qquad
i=0,1,\ldots,N.
```

Therefore

**Equation TRISO-DIS-104**

```math
r_0=0,
```

and

**Equation TRISO-DIS-105**

```math
r_N=R.
```

### 13.3 Define the temporal mesh

Let the time step be

**Equation TRISO-DIS-106**

```math
\Delta t>0.
```

Define

**Equation TRISO-DIS-107**

```math
t_j=j\Delta t,
\qquad
j=0,1,2,\ldots.
```

Let

**Equation TRISO-DIS-108**

```math
C_i^j
```

denote the numerical approximation to

**Equation TRISO-DIS-109**

```math
c(r_i,t_j).
```

Thus

**Equation TRISO-DIS-110**

```math
C_i^j\approx c(r_i,t_j).
```

The units remain

**Equation TRISO-DIS-111**

```math
[C_i^j]=\mathrm{mol\,m^{-3}}.
```

### 13.4 Forward approximation of the time derivative

At fixed radius $r_i$, Taylor-expand the exact solution from $t_j$ to $t_j+\Delta t$:

**Equation TRISO-DIS-112**

```math
c(r_i,t_j+\Delta t)
=
c(r_i,t_j)
+
\Delta t
\frac{\partial c}{\partial t}(r_i,t_j)
+
O(\Delta t^2).
```

Subtract $c(r_i,t_j)$ from both sides:

**Equation TRISO-DIS-113**

```math
c(r_i,t_j+\Delta t)-c(r_i,t_j)
=
\Delta t
\frac{\partial c}{\partial t}(r_i,t_j)
+
O(\Delta t^2).
```

Divide by $\Delta t$:

**Equation TRISO-DIS-114**

```math
\frac{
c(r_i,t_j+\Delta t)-c(r_i,t_j)
}{
\Delta t
}
=
\frac{\partial c}{\partial t}(r_i,t_j)
+
O(\Delta t).
```

Rearrange:

**Equation TRISO-DIS-115**

```math
\frac{\partial c}{\partial t}(r_i,t_j)
=
\frac{
c(r_i,t_j+\Delta t)-c(r_i,t_j)
}{
\Delta t
}
+
O(\Delta t).
```

Replace the exact nodal values by the numerical unknowns:

**Equation TRISO-DIS-116**

```math
\boxed{
\frac{\partial c}{\partial t}(r_i,t_j)
\approx
\frac{C_i^{j+1}-C_i^j}{\Delta t}.
}
```

The forward-time approximation is first-order accurate in time.

### 13.5 Centred approximation of the first radial derivative

At fixed $t_j$, Taylor-expand about $r_i$ toward $r_i+\Delta r$:

**Equation TRISO-DIS-117**

```math
c(r_i+\Delta r,t_j)
=
c_i
+
\Delta r\,c_{r,i}
+
\frac{\Delta r^2}{2}c_{rr,i}
+
\frac{\Delta r^3}{6}c_{rrr,i}
+
O(\Delta r^4).
```

Taylor-expand toward $r_i-\Delta r$:

**Equation TRISO-DIS-118**

```math
c(r_i-\Delta r,t_j)
=
c_i
-
\Delta r\,c_{r,i}
+
\frac{\Delta r^2}{2}c_{rr,i}
-
\frac{\Delta r^3}{6}c_{rrr,i}
+
O(\Delta r^4).
```

Subtract the backward expansion from the forward expansion:

**Equation TRISO-DIS-119**

```math
c(r_i+\Delta r,t_j)
-
c(r_i-\Delta r,t_j)
=
2\Delta r\,c_{r,i}
+
\frac{\Delta r^3}{3}c_{rrr,i}
+
O(\Delta r^5).
```

Divide by $2\Delta r$:

**Equation TRISO-DIS-120**

```math
\frac{
c(r_i+\Delta r,t_j)-c(r_i-\Delta r,t_j)
}{
2\Delta r
}
=
c_{r,i}
+
O(\Delta r^2).
```

Therefore

**Equation TRISO-DIS-121**

```math
\boxed{
\frac{\partial c}{\partial r}(r_i,t_j)
\approx
\frac{
C_{i+1}^j-C_{i-1}^j
}{
2\Delta r
}.
}
```

### 13.6 Centred approximation of the second radial derivative

Add the two Taylor expansions (TRISO-DIS-117) and (TRISO-DIS-118):

**Equation TRISO-DIS-122**

```math
c(r_i+\Delta r,t_j)
+
c(r_i-\Delta r,t_j)
=
2c_i
+
\Delta r^2c_{rr,i}
+
O(\Delta r^4).
```

Subtract $2c_i$:

**Equation TRISO-DIS-123**

```math
c(r_i+\Delta r,t_j)
-
2c_i
+
c(r_i-\Delta r,t_j)
=
\Delta r^2c_{rr,i}
+
O(\Delta r^4).
```

Divide by $\Delta r^2$:

**Equation TRISO-DIS-124**

```math
\frac{
c(r_i+\Delta r,t_j)-2c_i+c(r_i-\Delta r,t_j)
}{
\Delta r^2
}
=
c_{rr,i}
+
O(\Delta r^2).
```

Therefore

**Equation TRISO-DIS-125**

```math
\boxed{
\frac{\partial^2c}{\partial r^2}(r_i,t_j)
\approx
\frac{
C_{i-1}^j-2C_i^j+C_{i+1}^j
}{
\Delta r^2
}.
}
```

### 13.7 Substitute the discrete derivatives into the PDE

Evaluate the continuous equation at $(r_i,t_j)$:

**Equation TRISO-DIS-126**

```math
c_{t,i}^j
=
D
\left(
c_{rr,i}^j
+
\frac2{r_i}c_{r,i}^j
\right)
+
S_i^j.
```

Substitute the forward-time approximation:

**Equation TRISO-DIS-127**

```math
\frac{
C_i^{j+1}-C_i^j
}{
\Delta t
}
=
D
\left(
c_{rr,i}^j
+
\frac2{r_i}c_{r,i}^j
\right)
+
S_i^j.
```

Substitute the centred second derivative:

**Equation TRISO-DIS-128**

```math
\frac{
C_i^{j+1}-C_i^j
}{
\Delta t
}
=
D
\left[
\frac{
C_{i-1}^j-2C_i^j+C_{i+1}^j
}{
\Delta r^2
}
+
\frac2{r_i}c_{r,i}^j
\right]
+
S_i^j.
```

Substitute the centred first derivative:

**Equation TRISO-DIS-129**

```math
\frac{
C_i^{j+1}-C_i^j
}{
\Delta t
}
=
D
\left[
\frac{
C_{i-1}^j-2C_i^j+C_{i+1}^j
}{
\Delta r^2
}
+
\frac2{r_i}
\frac{
C_{i+1}^j-C_{i-1}^j
}{
2\Delta r
}
\right]
+
S_i^j.
```

Cancel the factor $2$ in the radial first-derivative term:

**Equation TRISO-DIS-130**

```math
\frac{
C_i^{j+1}-C_i^j
}{
\Delta t
}
=
D
\left[
\frac{
C_{i-1}^j-2C_i^j+C_{i+1}^j
}{
\Delta r^2
}
+
\frac{
C_{i+1}^j-C_{i-1}^j
}{
r_i\Delta r
}
\right]
+
S_i^j.
```

Use the uniform-mesh identity

**Equation TRISO-DIS-131**

```math
r_i=i\Delta r.
```

Then

**Equation TRISO-DIS-132**

```math
r_i\Delta r
=
i\Delta r^2.
```

Therefore

**Equation TRISO-DIS-133**

```math
\frac{
C_i^{j+1}-C_i^j
}{
\Delta t
}
=
\frac{D}{\Delta r^2}
\left[
C_{i-1}^j
-
2C_i^j
+
C_{i+1}^j
+
\frac1i
\left(
C_{i+1}^j-C_{i-1}^j
\right)
\right]
+
S_i^j.
```

### 13.8 Collect neighbour coefficients

Expand the $1/i$ term:

**Equation TRISO-DIS-134**

```math
\frac{
C_i^{j+1}-C_i^j
}{
\Delta t
}
=
\frac{D}{\Delta r^2}
\left[
C_{i-1}^j
-
2C_i^j
+
C_{i+1}^j
+
\frac1iC_{i+1}^j
-
\frac1iC_{i-1}^j
\right]
+
S_i^j.
```

Collect the $C_{i-1}^j$ terms:

**Equation TRISO-DIS-135**

```math
C_{i-1}^j
-
\frac1iC_{i-1}^j
=
\left(
1-\frac1i
\right)
C_{i-1}^j.
```

Collect the $C_{i+1}^j$ terms:

**Equation TRISO-DIS-136**

```math
C_{i+1}^j
+
\frac1iC_{i+1}^j
=
\left(
1+\frac1i
\right)
C_{i+1}^j.
```

Thus

**Equation TRISO-DIS-137**

```math
\frac{
C_i^{j+1}-C_i^j
}{
\Delta t
}
=
\frac{D}{\Delta r^2}
\left[
\left(
1-\frac1i
\right)C_{i-1}^j
-
2C_i^j
+
\left(
1+\frac1i
\right)C_{i+1}^j
\right]
+
S_i^j.
```

Multiply by $\Delta t$:

**Equation TRISO-DIS-138**

```math
C_i^{j+1}-C_i^j
=
\frac{D\Delta t}{\Delta r^2}
\left[
\left(
1-\frac1i
\right)C_{i-1}^j
-
2C_i^j
+
\left(
1+\frac1i
\right)C_{i+1}^j
\right]
+
S_i^j\Delta t.
```

Add $C_i^j$ to both sides:

**Equation TRISO-DIS-139**

```math
C_i^{j+1}
=
C_i^j
+
\frac{D\Delta t}{\Delta r^2}
\left[
\left(
1-\frac1i
\right)C_{i-1}^j
-
2C_i^j
+
\left(
1+\frac1i
\right)C_{i+1}^j
\right]
+
S_i^j\Delta t.
```

### 13.9 Define the Fourier number

Define

**Equation TRISO-DIS-140**

```math
\boxed{
\mathrm{Fo}
=
\frac{D\Delta t}{\Delta r^2}.
}
```

Its units are

**Equation TRISO-DIS-141**

```math
[\mathrm{Fo}]
=
\frac{
\mathrm{m^2\,s^{-1}}\mathrm{s}
}{
\mathrm{m^2}
}
=
1.
```

Therefore $\mathrm{Fo}$ is dimensionless.

Substitute the definition into (TRISO-DIS-139):

**Equation TRISO-DIS-142**

```math
C_i^{j+1}
=
C_i^j
+
\mathrm{Fo}
\left[
\left(
1-\frac1i
\right)C_{i-1}^j
-
2C_i^j
+
\left(
1+\frac1i
\right)C_{i+1}^j
\right]
+
S_i^j\Delta t.
```

Distribute $\mathrm{Fo}$:

**Equation TRISO-DIS-143**

```math
C_i^{j+1}
=
C_i^j
+
\mathrm{Fo}
\left(
1-\frac1i
\right)C_{i-1}^j
-
2\mathrm{Fo}C_i^j
+
\mathrm{Fo}
\left(
1+\frac1i
\right)C_{i+1}^j
+
S_i^j\Delta t.
```

Collect the central coefficient:

**Equation TRISO-DIS-144**

```math
C_i^j-2\mathrm{Fo}C_i^j
=
(1-2\mathrm{Fo})C_i^j.
```

Hence the original Ray interior FTCS update is

**Equation TRISO-DIS-145**

```math
\boxed{
C_i^{j+1}
=
\mathrm{Fo}
\left(
1-\frac1i
\right)C_{i-1}^j
+
(1-2\mathrm{Fo})C_i^j
+
\mathrm{Fo}
\left(
1+\frac1i
\right)C_{i+1}^j
+
S_i^j\Delta t.
}
```

This is algebraically equivalent to the ordering used in the original notebook.

### 13.10 Dimensional check of the source increment

The source contribution is

**Equation TRISO-DIS-146**

```math
S_i^j\Delta t.
```

Its units are

**Equation TRISO-DIS-147**

```math
[S_i^j\Delta t]
=
\mathrm{mol\,m^{-3}\,s^{-1}}
\times
\mathrm{s}.
```

Therefore

**Equation TRISO-DIS-148**

```math
[S_i^j\Delta t]
=
\mathrm{mol\,m^{-3}},
```

which matches the concentration units of every other term in (TRISO-DIS-145).

### 13.11 Local consistency order

The forward-time derivative has truncation error

**Equation TRISO-DIS-149**

```math
O(\Delta t).
```

The centred first derivative has truncation error

**Equation TRISO-DIS-150**

```math
O(\Delta r^2).
```

The centred second derivative has truncation error

**Equation TRISO-DIS-151**

```math
O(\Delta r^2).
```

Therefore, away from $r=0$, material interfaces, and the outer boundary, the local differential approximation is formally

**Equation TRISO-DIS-152**

```math
\boxed{
O(\Delta t)+O(\Delta r^2).
}
```

This is a consistency statement only. It is not by itself a proof of stability or convergence.

### 13.12 Scope and limitations of the interior stencil

Equation (TRISO-DIS-145) assumes:

1. $i\ge1$, so the $1/i$ factor is defined;
2. the stencil lies inside one homogeneous constant-$D$ region;
3. the spatial mesh is uniform;
4. the time step is uniform;
5. the source value $S_i^j$ is known explicitly at time level $j$.

It must **not** be applied unchanged:

- at $r=0$;
- across a discontinuous material interface;
- at the outer boundary.

Those cases require separate derivations.

The original notebook's FTCS method is retained as a transparent deterministic benchmark. It does not redefine the production WOS method and it does not yet establish the preferred discretisation for the full five-layer discontinuous-$D$ problem.

## 14. Centre discretisation

The interior stencil in Section 13 cannot be evaluated at $i=0$ because it contains the factor $1/i$.

The centre must therefore be derived from the regular spherical limit.

### 14.1 Continuous centre operator

Inside the homogeneous kernel, the radial diffusion operator is

**Equation TRISO-DIS-200**

```math
\mathcal L[c]
=
\frac{\partial^2c}{\partial r^2}
+
\frac2r\frac{\partial c}{\partial r}.
```

For a sufficiently smooth spherically symmetric field,

**Equation TRISO-DIS-201**

```math
\frac{\partial c}{\partial r}(0,t)=0.
```

Consider the apparently singular term

**Equation TRISO-DIS-202**

```math
\lim_{r\to0}
\frac2r
\frac{\partial c}{\partial r}.
```

Because the numerator tends to zero,

```math
\lim_{r\to0}
\frac{\partial c/\partial r}{r}
```

has the indeterminate form $0/0$.

Apply L'Hôpital's rule with respect to $r$:

**Equation TRISO-DIS-203**

```math
\lim_{r\to0}
\frac{\partial c/\partial r}{r}
=
\lim_{r\to0}
\frac{\partial^2c/\partial r^2}{1}.
```

Therefore

**Equation TRISO-DIS-204**

```math
\lim_{r\to0}
\frac1r
\frac{\partial c}{\partial r}
=
\frac{\partial^2c}{\partial r^2}(0,t).
```

Multiply by $2$:

**Equation TRISO-DIS-205**

```math
\lim_{r\to0}
\frac2r
\frac{\partial c}{\partial r}
=
2
\frac{\partial^2c}{\partial r^2}(0,t).
```

Hence

**Equation TRISO-DIS-206**

```math
\mathcal L[c](0,t)
=
\frac{\partial^2c}{\partial r^2}(0,t)
+
2\frac{\partial^2c}{\partial r^2}(0,t).
```

Collect the terms:

**Equation TRISO-DIS-207**

```math
\boxed{
\mathcal L[c](0,t)
=
3
\frac{\partial^2c}{\partial r^2}(0,t).
}
```

This is the exact smooth-origin limit of the spherical radial operator.

### 14.2 Introduce a symmetric ghost point

The uniform mesh has

**Equation TRISO-DIS-208**

```math
r_0=0,
\qquad
r_1=\Delta r.
```

For the purpose of constructing a centred derivative at the origin, introduce a mathematical ghost point

**Equation TRISO-DIS-209**

```math
r_{-1}=-\Delta r.
```

Spherical symmetry corresponds to an even extension of the radial concentration:

**Equation TRISO-DIS-210**

```math
c(-r,t)=c(r,t).
```

Evaluate this at $r=\Delta r$:

**Equation TRISO-DIS-211**

```math
c(-\Delta r,t)=c(\Delta r,t).
```

In nodal notation,

**Equation TRISO-DIS-212**

```math
\boxed{
C_{-1}^j=C_1^j.
}
```

The ghost point is a mathematical device. It does not represent a physical negative-radius material region.

### 14.3 Centre second derivative

Use the centred second-derivative formula at $i=0$:

**Equation TRISO-DIS-213**

```math
\frac{\partial^2c}{\partial r^2}(0,t_j)
\approx
\frac{
C_{-1}^j-2C_0^j+C_1^j
}{
\Delta r^2
}.
```

Substitute the symmetry relation $C_{-1}^j=C_1^j$:

**Equation TRISO-DIS-214**

```math
\frac{\partial^2c}{\partial r^2}(0,t_j)
\approx
\frac{
C_1^j-2C_0^j+C_1^j
}{
\Delta r^2
}.
```

Add the two $C_1^j$ terms:

**Equation TRISO-DIS-215**

```math
\boxed{
\frac{\partial^2c}{\partial r^2}(0,t_j)
\approx
\frac{
2(C_1^j-C_0^j)
}{
\Delta r^2
}.
}
```

### 14.4 Discrete spherical operator at the centre

From the exact centre limit,

**Equation TRISO-DIS-216**

```math
\mathcal L[c](0,t)
=
3c_{rr}(0,t).
```

Substitute the discrete second derivative (TRISO-DIS-215):

**Equation TRISO-DIS-217**

```math
\mathcal L[c](0,t_j)
\approx
3
\frac{
2(C_1^j-C_0^j)
}{
\Delta r^2
}.
```

Multiply the factors $3$ and $2$:

**Equation TRISO-DIS-218**

```math
\boxed{
\mathcal L[c](0,t_j)
\approx
\frac{
6(C_1^j-C_0^j)
}{
\Delta r^2
}.
}
```

This is the origin of the factor $6$ in the Ray notebook's centre update.

### 14.5 Apply the centre PDE

In the homogeneous kernel, the PDE at the centre is interpreted through the regular limit:

**Equation TRISO-DIS-219**

```math
\frac{\partial c}{\partial t}(0,t)
=
D_1\mathcal L[c](0,t)
+
S_0.
```

Use the forward-time approximation:

**Equation TRISO-DIS-220**

```math
\frac{
C_0^{j+1}-C_0^j
}{
\Delta t
}
=
D_1\mathcal L[c](0,t_j)
+
S_0.
```

Substitute (TRISO-DIS-218):

**Equation TRISO-DIS-221**

```math
\frac{
C_0^{j+1}-C_0^j
}{
\Delta t
}
=
D_1
\frac{
6(C_1^j-C_0^j)
}{
\Delta r^2
}
+
S_0.
```

Multiply by $\Delta t$:

**Equation TRISO-DIS-222**

```math
C_0^{j+1}-C_0^j
=
\frac{
6D_1\Delta t
}{
\Delta r^2
}
(C_1^j-C_0^j)
+
S_0\Delta t.
```

Define the kernel Fourier number

**Equation TRISO-DIS-223**

```math
\boxed{
\mathrm{Fo}_1
=
\frac{D_1\Delta t}{\Delta r^2}.
}
```

Substitute it:

**Equation TRISO-DIS-224**

```math
C_0^{j+1}-C_0^j
=
6\mathrm{Fo}_1
(C_1^j-C_0^j)
+
S_0\Delta t.
```

Add $C_0^j$ to both sides:

**Equation TRISO-DIS-225**

```math
\boxed{
C_0^{j+1}
=
C_0^j
+
6\mathrm{Fo}_1
(C_1^j-C_0^j)
+
S_0\Delta t.
}
```

Expand the difference:

**Equation TRISO-DIS-226**

```math
C_0^{j+1}
=
C_0^j
+
6\mathrm{Fo}_1C_1^j
-
6\mathrm{Fo}_1C_0^j
+
S_0\Delta t.
```

Collect the centre coefficient:

**Equation TRISO-DIS-227**

```math
\boxed{
C_0^{j+1}
=
(1-6\mathrm{Fo}_1)C_0^j
+
6\mathrm{Fo}_1C_1^j
+
S_0\Delta t.
}
```

### 14.6 Consistency of the centre approximation

For a smooth even radial field, expand about $r=0$:

**Equation TRISO-DIS-228**

```math
c(\Delta r,t)
=
c(0,t)
+
\frac{\Delta r^2}{2}c_{rr}(0,t)
+
\frac{\Delta r^4}{24}c_{rrrr}(0,t)
+
O(\Delta r^6).
```

Subtract $c(0,t)$:

**Equation TRISO-DIS-229**

```math
c(\Delta r,t)-c(0,t)
=
\frac{\Delta r^2}{2}c_{rr}(0,t)
+
\frac{\Delta r^4}{24}c_{rrrr}(0,t)
+
O(\Delta r^6).
```

Multiply by $2/\Delta r^2$:

**Equation TRISO-DIS-230**

```math
\frac{
2[c(\Delta r,t)-c(0,t)]
}{
\Delta r^2
}
=
c_{rr}(0,t)
+
\frac{\Delta r^2}{12}c_{rrrr}(0,t)
+
O(\Delta r^4).
```

Therefore the centre second derivative in (TRISO-DIS-215) is second-order accurate in space:

**Equation TRISO-DIS-231**

```math
c_{rr}(0,t)
=
\frac{
2(C_1-C_0)
}{
\Delta r^2
}
+
O(\Delta r^2).
```

Multiplying by the exact factor $3$ does not change the spatial order:

**Equation TRISO-DIS-232**

```math
\mathcal L[c](0,t)
=
\frac{
6(C_1-C_0)
}{
\Delta r^2
}
+
O(\Delta r^2).
```

Combined with forward Euler time stepping, the centre equation is locally

**Equation TRISO-DIS-233**

```math
\boxed{
O(\Delta t)+O(\Delta r^2).
}
```

### 14.7 Scope of the centre formula

The centre formula requires:

- spherical symmetry;
- sufficient smoothness at $r=0$;
- the centre to lie inside one homogeneous kernel material;
- constant $D_1$ over the centre stencil.

It does not determine the treatment of material interfaces or the outer surface.

The coefficient $1-6\mathrm{Fo}_1$ will later enter the FTCS monotonicity/stability discussion, but no stability conclusion is drawn here.

## 15. Material-interface discretisation for discontinuous diffusivity

The homogeneous interior FTCS stencil from Section 13 must not be centred across a material interface because the diffusivity is discontinuous there.

This section derives an interface-aligned discrete constraint directly from the two physical interface conditions already established in the continuous model:

1. ideal concentration continuity;
2. diffusive flux continuity.

The derivation is for the frozen ideal benchmark with $K=1$ and no explicit interfacial resistance.

### 15.1 Place a mesh node at the material interface

Let a material interface occur at

**Equation TRISO-DIS-300**

```math
r=r_I.
```

Choose the mesh so that one numerical node lies exactly at the interface:

**Equation TRISO-DIS-301**

```math
r_I=r_{i}.
```

Let the material immediately inside the interface have diffusivity

**Equation TRISO-DIS-302**

```math
D^-,
```

and the material immediately outside have diffusivity

**Equation TRISO-DIS-303**

```math
D^+.
```

For a uniform mesh,

**Equation TRISO-DIS-304**

```math
r_{i-1}=r_I-\Delta r,
```

and

**Equation TRISO-DIS-305**

```math
r_{i+1}=r_I+\Delta r.
```

The interface concentration is represented by one nodal unknown:

**Equation TRISO-DIS-306**

```math
C_I^j.
```

### 15.2 Discrete concentration continuity

For the ideal $K=1$ interface, the continuous condition is

**Equation TRISO-DIS-307**

```math
c^-(r_I,t)=c^+(r_I,t).
```

Represent both limiting concentrations by the same interface unknown:

**Equation TRISO-DIS-308**

```math
c^-(r_I,t_j)\approx C_I^j,
```

and

**Equation TRISO-DIS-309**

```math
c^+(r_I,t_j)\approx C_I^j.
```

Thus concentration continuity is built directly into the interface-node representation.

No averaging of the two material concentrations is required because there is only one ideal-interface concentration degree of freedom.

### 15.3 Continuous flux continuity

The exact ideal-interface condition is

**Equation TRISO-DIS-310**

```math
-D^-
\left.
\frac{\partial c^-}{\partial r}
\right|_{r_I^-}
=
-D^+
\left.
\frac{\partial c^+}{\partial r}
\right|_{r_I^+}.
```

The minus signs occur because the outward radial diffusive flux is

**Equation TRISO-DIS-311**

```math
J_r=-D\frac{\partial c}{\partial r}.
```

### 15.4 Approximate the inner-side gradient

On the inner material side, use the interface node and its inner neighbour.

The radial distance is

**Equation TRISO-DIS-312**

```math
r_I-r_{i-1}=\Delta r.
```

A first-order one-sided approximation is

**Equation TRISO-DIS-313**

```math
\left.
\frac{\partial c^-}{\partial r}
\right|_{r_I^-}
\approx
\frac{
C_I^j-C_{i-1}^j
}{
\Delta r
}.
```

Therefore the inner-side outward flux is approximated by

**Equation TRISO-DIS-314**

```math
J_I^-
\approx
-D^-
\frac{
C_I^j-C_{i-1}^j
}{
\Delta r
}.
```

### 15.5 Approximate the outer-side gradient

On the outer material side,

**Equation TRISO-DIS-315**

```math
r_{i+1}-r_I=\Delta r.
```

The one-sided gradient is

**Equation TRISO-DIS-316**

```math
\left.
\frac{\partial c^+}{\partial r}
\right|_{r_I^+}
\approx
\frac{
C_{i+1}^j-C_I^j
}{
\Delta r
}.
```

Therefore

**Equation TRISO-DIS-317**

```math
J_I^+
\approx
-D^+
\frac{
C_{i+1}^j-C_I^j
}{
\Delta r
}.
```

### 15.6 Enforce discrete flux continuity

Set the two approximated fluxes equal:

**Equation TRISO-DIS-318**

```math
-D^-
\frac{
C_I^j-C_{i-1}^j
}{
\Delta r
}
=
-D^+
\frac{
C_{i+1}^j-C_I^j
}{
\Delta r
}.
```

Cancel the common factor $-1/\Delta r$:

**Equation TRISO-DIS-319**

```math
D^-
\left(
C_I^j-C_{i-1}^j
\right)
=
D^+
\left(
C_{i+1}^j-C_I^j
\right).
```

Expand both sides:

**Equation TRISO-DIS-320**

```math
D^-C_I^j-D^-C_{i-1}^j
=
D^+C_{i+1}^j-D^+C_I^j.
```

Add $D^+C_I^j$ to both sides:

**Equation TRISO-DIS-321**

```math
(D^-+D^+)C_I^j-D^-C_{i-1}^j
=
D^+C_{i+1}^j.
```

Add $D^-C_{i-1}^j$ to both sides:

**Equation TRISO-DIS-322**

```math
(D^-+D^+)C_I^j
=
D^-C_{i-1}^j
+
D^+C_{i+1}^j.
```

Divide by $D^-+D^+>0$:

**Equation TRISO-DIS-323**

```math
\boxed{
C_I^j
=
\frac{
D^-C_{i-1}^j
+
D^+C_{i+1}^j
}{
D^-+D^+
}.
}
```

This is an algebraic interface constraint, not a homogeneous-material FTCS update.

### 15.7 Unequal grid spacing

The same derivation can be retained if the interface is not equally spaced from its neighbouring nodes.

Define

**Equation TRISO-DIS-324**

```math
\Delta r^-=r_I-r_{i-1},
```

and

**Equation TRISO-DIS-325**

```math
\Delta r^+=r_{i+1}-r_I.
```

Then the two flux approximations are

**Equation TRISO-DIS-326**

```math
J_I^-
\approx
-D^-
\frac{
C_I-C_{i-1}
}{
\Delta r^-
},
```

and

**Equation TRISO-DIS-327**

```math
J_I^+
\approx
-D^+
\frac{
C_{i+1}-C_I
}{
\Delta r^+
}.
```

Flux continuity gives

**Equation TRISO-DIS-328**

```math
\frac{D^-}{\Delta r^-}
(C_I-C_{i-1})
=
\frac{D^+}{\Delta r^+}
(C_{i+1}-C_I).
```

Expand:

**Equation TRISO-DIS-329**

```math
\frac{D^-}{\Delta r^-}C_I
-
\frac{D^-}{\Delta r^-}C_{i-1}
=
\frac{D^+}{\Delta r^+}C_{i+1}
-
\frac{D^+}{\Delta r^+}C_I.
```

Collect the interface unknown:

**Equation TRISO-DIS-330**

```math
\left(
\frac{D^-}{\Delta r^-}
+
\frac{D^+}{\Delta r^+}
\right)
C_I
=
\frac{D^-}{\Delta r^-}C_{i-1}
+
\frac{D^+}{\Delta r^+}C_{i+1}.
```

Therefore

**Equation TRISO-DIS-331**

```math
\boxed{
C_I
=
\frac{
\dfrac{D^-}{\Delta r^-}C_{i-1}
+
\dfrac{D^+}{\Delta r^+}C_{i+1}
}{
\dfrac{D^-}{\Delta r^-}
+
\dfrac{D^+}{\Delta r^+}
}.
}
```

Equation (TRISO-DIS-323) is recovered when

**Equation TRISO-DIS-332**

```math
\Delta r^-=\Delta r^+=\Delta r.
```

### 15.8 Equivalent two-node conductance and harmonic diffusivity

Sometimes the interface concentration is eliminated so that the flux is written directly between the two neighbouring material nodes.

Start from the inner-side flux relation:

**Equation TRISO-DIS-333**

```math
J_I
=
-D^-
\frac{
C_I-C_{i-1}
}{
\Delta r^-
}.
```

Rearrange for the inner concentration drop:

**Equation TRISO-DIS-334**

```math
C_{i-1}-C_I
=
J_I
\frac{\Delta r^-}{D^-}.
```

From the outer-side relation,

**Equation TRISO-DIS-335**

```math
J_I
=
-D^+
\frac{
C_{i+1}-C_I
}{
\Delta r^+
}.
```

Rearrange:

**Equation TRISO-DIS-336**

```math
C_I-C_{i+1}
=
J_I
\frac{\Delta r^+}{D^+}.
```

Add the two concentration drops:

**Equation TRISO-DIS-337**

```math
C_{i-1}-C_{i+1}
=
J_I
\left(
\frac{\Delta r^-}{D^-}
+
\frac{\Delta r^+}{D^+}
\right).
```

Solve for the flux:

**Equation TRISO-DIS-338**

```math
\boxed{
J_I
=
\frac{
C_{i-1}-C_{i+1}
}{
\dfrac{\Delta r^-}{D^-}
+
\dfrac{\Delta r^+}{D^+}
}.
}
```

The denominator is the sum of the two local diffusion resistances per unit area.

Define the total node-to-node distance

**Equation TRISO-DIS-339**

```math
\Delta r_{\mathrm{tot}}
=
\Delta r^-+\Delta r^+.
```

Define an effective diffusivity $D_{\mathrm{eff}}$ by

**Equation TRISO-DIS-340**

```math
J_I
=
D_{\mathrm{eff}}
\frac{
C_{i-1}-C_{i+1}
}{
\Delta r_{\mathrm{tot}}
}.
```

Equate (TRISO-DIS-338) and (TRISO-DIS-340):

**Equation TRISO-DIS-341**

```math
\frac{D_{\mathrm{eff}}}{\Delta r_{\mathrm{tot}}}
=
\frac1{
\dfrac{\Delta r^-}{D^-}
+
\dfrac{\Delta r^+}{D^+}
}.
```

Multiply by $\Delta r_{\mathrm{tot}}$:

**Equation TRISO-DIS-342**

```math
\boxed{
D_{\mathrm{eff}}
=
\frac{
\Delta r^-+\Delta r^+
}{
\dfrac{\Delta r^-}{D^-}
+
\dfrac{\Delta r^+}{D^+}
}.
}
```

For equal half-distances,

**Equation TRISO-DIS-343**

```math
\Delta r^-=\Delta r^+,
```

the effective diffusivity becomes

**Equation TRISO-DIS-344**

```math
D_{\mathrm{eff}}
=
\frac{2}{
\dfrac1{D^-}
+
\dfrac1{D^+}
}.
```

Thus

**Equation TRISO-DIS-345**

```math
\boxed{
D_{\mathrm{eff}}
=
\frac{
2D^-D^+
}{
D^-+D^+
}.
}
```

This is the harmonic mean of the adjacent diffusivities.

It appears because diffusion resistances add in series; it is not an arbitrary averaging rule.

### 15.9 Limiting checks

If

**Equation TRISO-DIS-346**

```math
D^-=D^+=D,
```

then (TRISO-DIS-323) becomes

**Equation TRISO-DIS-347**

```math
C_I
=
\frac{
DC_{i-1}+DC_{i+1}
}{
2D
}.
```

Cancel $D$:

**Equation TRISO-DIS-348**

```math
\boxed{
C_I
=
\frac{
C_{i-1}+C_{i+1}
}{2}.
}
```

This is the expected linear interpolation for equal diffusivity and equal spacing.

If

**Equation TRISO-DIS-349**

```math
D^+\ll D^-,
```

then the low-diffusivity outer material contributes the dominant diffusion resistance in (TRISO-DIS-338).

The interface concentration correspondingly approaches the concentration on the high-diffusivity side only according to the resistance-weighted relation; it is not obtained from an arithmetic diffusivity average.

### 15.10 Accuracy and scope

The one-sided interface gradients in (TRISO-DIS-313) and (TRISO-DIS-316) are first-order approximations to the limiting interface derivatives when used with only one neighbouring node on each side.

Therefore the algebraic interface relation derived here is conservative with respect to the approximated flux, but this particular gradient construction is not automatically second-order accurate at the interface.

A higher-order interface treatment would require additional same-material nodes or a finite-volume formulation with carefully defined face fluxes.

[IMPORTANT] The present result establishes the correct **discrete transmission logic**:

- one ideal-interface concentration;
- no differentiation of $D$ through its jump;
- equal discrete flux on both sides;
- resistance-weighted, rather than arithmetic, diffusivity coupling.

It does not yet establish the globally preferred five-layer spatial discretisation.

## 16. Outer Robin boundary: ghost-point elimination and surface update

The outer surface is

**Equation TRISO-DIS-400**

```math
r_N=R.
```

For the homogeneous Part-I FTCS benchmark, the outer material has constant diffusivity $D$ and the external bulk concentration is

**Equation TRISO-DIS-401**

```math
c_\infty=0.
```

The continuous Robin condition is therefore

**Equation TRISO-DIS-402**

```math
-D
\frac{\partial c}{\partial r}(R,t)
=
h c(R,t).
```

The left side is the outward diffusive flux from the particle. The right side is the outward external mass-transfer flux.

### 16.1 Introduce the outer ghost point

The last physical node is

**Equation TRISO-DIS-403**

```math
r_N=R.
```

The adjacent physical interior node is

**Equation TRISO-DIS-404**

```math
r_{N-1}=R-\Delta r.
```

Introduce a mathematical ghost point outside the particle:

**Equation TRISO-DIS-405**

```math
r_{N+1}=R+\Delta r.
```

Its numerical value is denoted

**Equation TRISO-DIS-406**

```math
C_{N+1}^j.
```

The ghost value is not an external physical concentration. It is an algebraic device used to retain a centred derivative at $r=R$.

### 16.2 Centred approximation of the surface gradient

At time $t_j$, approximate the radial derivative by

**Equation TRISO-DIS-407**

```math
\frac{\partial c}{\partial r}(R,t_j)
\approx
\frac{
C_{N+1}^j-C_{N-1}^j
}{
2\Delta r
}.
```

Substitute this approximation into the Robin condition:

**Equation TRISO-DIS-408**

```math
-D
\frac{
C_{N+1}^j-C_{N-1}^j
}{
2\Delta r
}
=
hC_N^j.
```

Multiply both sides by $2\Delta r$:

**Equation TRISO-DIS-409**

```math
-D
\left(
C_{N+1}^j-C_{N-1}^j
\right)
=
2h\Delta r\,C_N^j.
```

Divide by $-D$:

**Equation TRISO-DIS-410**

```math
C_{N+1}^j-C_{N-1}^j
=
-\frac{2h\Delta r}{D}C_N^j.
```

Add $C_{N-1}^j$ to both sides:

**Equation TRISO-DIS-411**

```math
C_{N+1}^j
=
C_{N-1}^j
-
\frac{2h\Delta r}{D}C_N^j.
```

Define the dimensionless mesh transfer parameter

**Equation TRISO-DIS-412**

```math
\boxed{
\kappa
=
\frac{h\Delta r}{D}.
}
```

Its units are

**Equation TRISO-DIS-413**

```math
[\kappa]
=
\frac{
\mathrm{m\,s^{-1}}\mathrm m
}{
\mathrm{m^2\,s^{-1}}
}
=
1.
```

Therefore the ghost relation is

**Equation TRISO-DIS-414**

```math
\boxed{
C_{N+1}^j
=
C_{N-1}^j
-
2\kappa C_N^j.
}
```

### 16.3 Surface approximation of the second radial derivative

Use the centred second derivative at node $N$:

**Equation TRISO-DIS-415**

```math
\frac{\partial^2c}{\partial r^2}(R,t_j)
\approx
\frac{
C_{N-1}^j
-
2C_N^j
+
C_{N+1}^j
}{
\Delta r^2
}.
```

Substitute the ghost relation (TRISO-DIS-414):

**Equation TRISO-DIS-416**

```math
\frac{\partial^2c}{\partial r^2}(R,t_j)
\approx
\frac{
C_{N-1}^j
-
2C_N^j
+
C_{N-1}^j
-
2\kappa C_N^j
}{
\Delta r^2
}.
```

Collect the two interior-neighbour terms:

**Equation TRISO-DIS-417**

```math
C_{N-1}^j+C_{N-1}^j
=
2C_{N-1}^j.
```

Collect the two surface terms:

**Equation TRISO-DIS-418**

```math
-2C_N^j-2\kappa C_N^j
=
-2(1+\kappa)C_N^j.
```

Therefore

**Equation TRISO-DIS-419**

```math
\boxed{
\frac{\partial^2c}{\partial r^2}(R,t_j)
\approx
\frac{
2C_{N-1}^j
-
2(1+\kappa)C_N^j
}{
\Delta r^2
}.
}
```

### 16.4 Surface approximation of the first radial derivative

The centred derivative is

**Equation TRISO-DIS-420**

```math
\frac{\partial c}{\partial r}(R,t_j)
\approx
\frac{
C_{N+1}^j-C_{N-1}^j
}{
2\Delta r
}.
```

Substitute (TRISO-DIS-414):

**Equation TRISO-DIS-421**

```math
\frac{\partial c}{\partial r}(R,t_j)
\approx
\frac{
C_{N-1}^j
-
2\kappa C_N^j
-
C_{N-1}^j
}{
2\Delta r
}.
```

Cancel the two $C_{N-1}^j$ terms:

**Equation TRISO-DIS-422**

```math
\frac{\partial c}{\partial r}(R,t_j)
\approx
-\frac{
2\kappa C_N^j
}{
2\Delta r
}.
```

Cancel the factor $2$:

**Equation TRISO-DIS-423**

```math
\boxed{
\frac{\partial c}{\partial r}(R,t_j)
\approx
-\frac{\kappa}{\Delta r}C_N^j.
}
```

Using $\kappa=h\Delta r/D$,

**Equation TRISO-DIS-424**

```math
-\frac{\kappa}{\Delta r}C_N^j
=
-\frac hD C_N^j.
```

Thus the eliminated ghost relation reproduces the discrete Robin gradient exactly within the chosen centred boundary approximation.

### 16.5 Discrete spherical operator at the outer surface

The spherical operator is

**Equation TRISO-DIS-425**

```math
\mathcal L[c](R,t)
=
c_{rr}(R,t)
+
\frac2R c_r(R,t).
```

Substitute the discrete second derivative (TRISO-DIS-419):

**Equation TRISO-DIS-426**

```math
\mathcal L[c](R,t_j)
\approx
\frac{
2C_{N-1}^j
-
2(1+\kappa)C_N^j
}{
\Delta r^2
}
+
\frac2R c_r(R,t_j).
```

Substitute the discrete first derivative (TRISO-DIS-423):

**Equation TRISO-DIS-427**

```math
\mathcal L[c](R,t_j)
\approx
\frac{
2C_{N-1}^j
-
2(1+\kappa)C_N^j
}{
\Delta r^2
}
-
\frac{2\kappa}{R\Delta r}C_N^j.
```

Use

**Equation TRISO-DIS-428**

```math
R=N\Delta r.
```

Therefore

**Equation TRISO-DIS-429**

```math
R\Delta r
=
N\Delta r^2.
```

Hence

**Equation TRISO-DIS-430**

```math
\frac{2\kappa}{R\Delta r}
=
\frac{2\kappa}{N\Delta r^2}.
```

Substitute:

**Equation TRISO-DIS-431**

```math
\mathcal L[c](R,t_j)
\approx
\frac{
2C_{N-1}^j
-
2(1+\kappa)C_N^j
}{
\Delta r^2
}
-
\frac{
2\kappa C_N^j
}{
N\Delta r^2
}.
```

Put the terms over the common denominator:

**Equation TRISO-DIS-432**

```math
\mathcal L[c](R,t_j)
\approx
\frac2{\Delta r^2}
\left[
C_{N-1}^j
-
(1+\kappa)C_N^j
-
\frac{\kappa}{N}C_N^j
\right].
```

Collect the surface coefficient:

**Equation TRISO-DIS-433**

```math
(1+\kappa)
+
\frac{\kappa}{N}
=
1+\kappa
\left(
1+\frac1N
\right).
```

Therefore

**Equation TRISO-DIS-434**

```math
\boxed{
\mathcal L[c](R,t_j)
\approx
\frac2{\Delta r^2}
\left[
C_{N-1}^j
-
\left(
1+\kappa\left(1+\frac1N\right)
\right)
C_N^j
\right].
}
```

### 16.6 Apply the surface PDE

For the homogeneous benchmark,

**Equation TRISO-DIS-435**

```math
\frac{\partial c}{\partial t}(R,t)
=
D\mathcal L[c](R,t)
+
S_R.
```

For the original homogeneous source benchmark,

**Equation TRISO-DIS-436**

```math
S_R=S_0.
```

For the physical five-layer kernel-confined source problem, the OPyC source would instead be zero. This distinction must be preserved when the boundary formula is reused outside the Part-I benchmark.

Use forward Euler:

**Equation TRISO-DIS-437**

```math
\frac{
C_N^{j+1}-C_N^j
}{
\Delta t
}
=
D\mathcal L[c](R,t_j)
+
S_R^j.
```

Substitute (TRISO-DIS-434):

**Equation TRISO-DIS-438**

```math
\frac{
C_N^{j+1}-C_N^j
}{
\Delta t
}
=
\frac{2D}{\Delta r^2}
\left[
C_{N-1}^j
-
\left(
1+\kappa\left(1+\frac1N\right)
\right)
C_N^j
\right]
+
S_R^j.
```

Multiply by $\Delta t$:

**Equation TRISO-DIS-439**

```math
C_N^{j+1}-C_N^j
=
\frac{2D\Delta t}{\Delta r^2}
\left[
C_{N-1}^j
-
\left(
1+\kappa\left(1+\frac1N\right)
\right)
C_N^j
\right]
+
S_R^j\Delta t.
```

Use

**Equation TRISO-DIS-440**

```math
\mathrm{Fo}
=
\frac{D\Delta t}{\Delta r^2}.
```

Then

**Equation TRISO-DIS-441**

```math
C_N^{j+1}-C_N^j
=
2\mathrm{Fo}
\left[
C_{N-1}^j
-
\left(
1+\kappa\left(1+\frac1N\right)
\right)
C_N^j
\right]
+
S_R^j\Delta t.
```

Add $C_N^j$ to both sides:

**Equation TRISO-DIS-442**

```math
C_N^{j+1}
=
C_N^j
+
2\mathrm{Fo}C_{N-1}^j
-
2\mathrm{Fo}
\left(
1+\kappa\left(1+\frac1N\right)
\right)
C_N^j
+
S_R^j\Delta t.
```

Collect the surface coefficient:

**Equation TRISO-DIS-443**

```math
\boxed{
C_N^{j+1}
=
2\mathrm{Fo}C_{N-1}^j
+
\left[
1
-
2\mathrm{Fo}
\left(
1+\kappa\left(1+\frac1N\right)
\right)
\right]
C_N^j
+
S_R^j\Delta t.
}
```

This recovers the surface update written in the original Ray derivation.

### 16.7 Dimensional checks

The Fourier number is dimensionless:

**Equation TRISO-DIS-444**

```math
[\mathrm{Fo}]=1.
```

The mesh transfer parameter is dimensionless:

**Equation TRISO-DIS-445**

```math
[\kappa]=1.
```

Therefore every coefficient multiplying a concentration in (TRISO-DIS-443) is dimensionless.

The source increment has units

**Equation TRISO-DIS-446**

```math
[S_R\Delta t]
=
\mathrm{mol\,m^{-3}}.
```

Thus every term in the update has concentration units.

### 16.8 Limiting checks

If

**Equation TRISO-DIS-447**

```math
h=0,
```

then

**Equation TRISO-DIS-448**

```math
\kappa=0.
```

The ghost relation becomes

**Equation TRISO-DIS-449**

```math
C_{N+1}^j=C_{N-1}^j.
```

This is the expected symmetric zero-gradient ghost condition for a zero-flux Neumann boundary.

The surface update becomes

**Equation TRISO-DIS-450**

```math
C_N^{j+1}
=
2\mathrm{Fo}C_{N-1}^j
+
(1-2\mathrm{Fo})C_N^j
+
S_R^j\Delta t.
```

For finite $h>0$, increasing $h$ increases $\kappa$, which strengthens the outward-transfer contribution in the surface coefficient.

The formal limit $h\to\infty$ is more delicate for this explicit ghost formulation because

**Equation TRISO-DIS-451**

```math
\kappa=\frac{h\Delta r}{D}\to\infty
```

at fixed $\Delta r$.

The continuum Robin condition approaches the absorbing Dirichlet condition $c(R,t)=0$, but the explicit ghost update becomes increasingly stiff rather than automatically turning into a numerically well-conditioned Dirichlet update.

Therefore an absorbing Dirichlet boundary should be imposed directly when that is the intended numerical model, rather than obtained by taking $\kappa\to\infty$ in (TRISO-DIS-443).

### 16.9 Complete truncation error of the Robin ghost surface closure

The centred Robin derivative by itself is second-order accurate, but the complete surface PDE closure also inserts the ghost value into a second-derivative stencil.

The complete boundary operator must therefore be expanded directly.

Define

**Equation TRISO-DIS-452**

```math
\beta=\frac{h}{D}.
```

The exact Robin condition gives

**Equation TRISO-DIS-453**

```math
c_r(R)=-\beta c(R).
```

The ghost construction is

**Equation TRISO-DIS-454**

```math
c_g(R+\Delta r)
=
c(R-\Delta r)
-
2\beta\Delta r\,c(R).
```

Taylor-expand the exact interior value:

**Equation TRISO-DIS-455**

```math
c(R-\Delta r)
=
c(R)
-
\Delta r\,c_r(R)
+
\frac{\Delta r^2}{2}c_{rr}(R)
-
\frac{\Delta r^3}{6}c_{rrr}(R)
+
O(\Delta r^4).
```

Substitute the Robin derivative $c_r(R)=-\beta c(R)$:

**Equation TRISO-DIS-456**

```math
c(R-\Delta r)
=
c(R)
+
\beta\Delta r\,c(R)
+
\frac{\Delta r^2}{2}c_{rr}(R)
-
\frac{\Delta r^3}{6}c_{rrr}(R)
+
O(\Delta r^4).
```

Substitute this expansion into the ghost construction:

**Equation TRISO-DIS-457**

```math
c_g(R+\Delta r)
=
c(R)
-
\beta\Delta r\,c(R)
+
\frac{\Delta r^2}{2}c_{rr}(R)
-
\frac{\Delta r^3}{6}c_{rrr}(R)
+
O(\Delta r^4).
```

The exact smooth continuation would be

**Equation TRISO-DIS-458**

```math
c(R+\Delta r)
=
c(R)
+
\Delta r\,c_r(R)
+
\frac{\Delta r^2}{2}c_{rr}(R)
+
\frac{\Delta r^3}{6}c_{rrr}(R)
+
O(\Delta r^4).
```

Apply $c_r(R)=-\beta c(R)$:

**Equation TRISO-DIS-459**

```math
c(R+\Delta r)
=
c(R)
-
\beta\Delta r\,c(R)
+
\frac{\Delta r^2}{2}c_{rr}(R)
+
\frac{\Delta r^3}{6}c_{rrr}(R)
+
O(\Delta r^4).
```

Subtract the exact continuation from the ghost continuation:

**Equation TRISO-DIS-460**

```math
c_g(R+\Delta r)-c(R+\Delta r)
=
-\frac{\Delta r^3}{3}c_{rrr}(R)
+
O(\Delta r^4).
```

Thus the ghost value error is

**Equation TRISO-DIS-461**

```math
\boxed{
c_g-c_{\mathrm{exact}}
=
O(\Delta r^3).
}
```

The ghost value enters the centred second derivative divided by $\Delta r^2$:

**Equation TRISO-DIS-462**

```math
c_{rr}^{\,g}(R)
=
\frac{
c(R-\Delta r)-2c(R)+c_g(R+\Delta r)
}{
\Delta r^2
}.
```

Write the ghost value as

**Equation TRISO-DIS-463**

```math
c_g(R+\Delta r)
=
c(R+\Delta r)
+
\varepsilon_g,
```

where

**Equation TRISO-DIS-464**

```math
\varepsilon_g
=
-\frac{\Delta r^3}{3}c_{rrr}(R)
+
O(\Delta r^4).
```

Substitute:

**Equation TRISO-DIS-465**

```math
c_{rr}^{\,g}(R)
=
\frac{
c(R-\Delta r)-2c(R)+c(R+\Delta r)
}{
\Delta r^2
}
+
\frac{\varepsilon_g}{\Delta r^2}.
```

The ordinary centred second derivative contributes

**Equation TRISO-DIS-466**

```math
c_{rr}(R)+O(\Delta r^2).
```

The ghost-error contribution is

**Equation TRISO-DIS-467**

```math
\frac{\varepsilon_g}{\Delta r^2}
=
-\frac{\Delta r}{3}c_{rrr}(R)
+
O(\Delta r^2).
```

Therefore

**Equation TRISO-DIS-468**

```math
\boxed{
c_{rr}^{\,g}(R)
=
c_{rr}(R)
-
\frac{\Delta r}{3}c_{rrr}(R)
+
O(\Delta r^2).
}
```

The first-derivative Robin closure remains second-order, but the second-derivative part is generically first-order at the boundary.

Hence the complete spherical surface operator has generic local spatial truncation

**Equation TRISO-DIS-469**

```math
\boxed{
\mathcal L_h[c](R)
=
\mathcal L[c](R)
+
O(\Delta r).
}
```

With forward Euler time stepping, the boundary local consistency is therefore generically

**Equation TRISO-DIS-470**

```math
\boxed{
O(\Delta t)+O(\Delta r),
}
```

unless additional cancellation or superconvergence is demonstrated.

[CORRECTION] The previous claim of $O(\Delta t)$+$O(\Delta r^2)$ for the complete Robin surface update was overstated. Only the centred first-derivative approximation was second-order.

[OPEN REVIEW FINDING R2-D01] The analytical overclaim is corrected, but the independent reviewer requires an actual grid-refinement study before this finding is closed. No global FTCS convergence order is claimed here.

### 16.10 Scope

Equation (TRISO-DIS-443) belongs to the homogeneous Part-I FTCS benchmark.

For the physical five-layer problem, the same derivational structure may be applied to the OPyC layer by replacing $D$ with $D_5$ and using the physically selected outer source and boundary parameters.

The formula must not be confused with the production WOS absorbing-boundary treatment.

No stability conclusion is drawn in this section.

## 17. FTCS monotonicity and stability analysis

The original Ray notebook inferred a stability restriction from non-negative update coefficients.

That argument is useful, but the mathematical statement must be made precisely.

Non-negative coefficients with an appropriate row sum establish a monotonicity/maximum-principle style property for the homogeneous update. They are not automatically a necessary-and-sufficient characterization of every possible matrix stability notion.

This section first derives the coefficient restrictions and then states exactly what they prove.

### 17.1 Remove forcing for stability analysis

The complete explicit update contains source terms.

To study propagation of perturbations, compare two numerical solutions subject to the same prescribed source.

Let their difference be

**Equation TRISO-DIS-500**

```math
E_i^j
=
C_i^j-\widetilde C_i^j.
```

Because both solutions have the same additive source, subtraction cancels that source.

Thus the error/perturbation equation is homogeneous.

Stability of the linear time-marching operator can therefore be analysed from the source-free amplification step

**Equation TRISO-DIS-501**

```math
\mathbf E^{j+1}
=
\mathbf A\mathbf E^j.
```

### 17.2 Interior-row coefficients

For an interior homogeneous node, Section 13 gives

**Equation TRISO-DIS-502**

```math
E_i^{j+1}
=
\mathrm{Fo}
\left(
1-\frac1i
\right)
E_{i-1}^j
+
(1-2\mathrm{Fo})E_i^j
+
\mathrm{Fo}
\left(
1+\frac1i
\right)
E_{i+1}^j.
```

Define

**Equation TRISO-DIS-503**

```math
a_i
=
\mathrm{Fo}
\left(
1-\frac1i
\right),
```

**Equation TRISO-DIS-504**

```math
b_i
=
1-2\mathrm{Fo},
```

and

**Equation TRISO-DIS-505**

```math
d_i
=
\mathrm{Fo}
\left(
1+\frac1i
\right).
```

For $i\ge1$,

**Equation TRISO-DIS-506**

```math
1-\frac1i\ge0.
```

Since $\mathrm{Fo}\ge0$,

**Equation TRISO-DIS-507**

```math
a_i\ge0.
```

Similarly,

**Equation TRISO-DIS-508**

```math
1+\frac1i>0,
```

so

**Equation TRISO-DIS-509**

```math
d_i\ge0.
```

The central coefficient is non-negative when

**Equation TRISO-DIS-510**

```math
1-2\mathrm{Fo}\ge0.
```

Rearrange:

**Equation TRISO-DIS-511**

```math
2\mathrm{Fo}\le1.
```

Therefore

**Equation TRISO-DIS-512**

```math
\boxed{
\mathrm{Fo}\le\frac12.
}
```

Now add the three interior coefficients:

**Equation TRISO-DIS-513**

```math
a_i+b_i+d_i
=
\mathrm{Fo}\left(1-\frac1i\right)
+
1-2\mathrm{Fo}
+
\mathrm{Fo}\left(1+\frac1i\right).
```

Expand:

**Equation TRISO-DIS-514**

```math
a_i+b_i+d_i
=
\mathrm{Fo}
-\frac{\mathrm{Fo}}i
+
1
-
2\mathrm{Fo}
+
\mathrm{Fo}
+
\frac{\mathrm{Fo}}i.
```

Cancel the radial terms:

**Equation TRISO-DIS-515**

```math
-\frac{\mathrm{Fo}}i
+
\frac{\mathrm{Fo}}i
=
0.
```

Cancel the diffusion contributions:

**Equation TRISO-DIS-516**

```math
\mathrm{Fo}-2\mathrm{Fo}+\mathrm{Fo}=0.
```

Hence

**Equation TRISO-DIS-517**

```math
\boxed{
a_i+b_i+d_i=1.
}
```

When (TRISO-DIS-512) holds, an interior update is therefore a convex combination of the previous-time neighbouring values.

### 17.3 Centre-row coefficients

Section 14 gives the homogeneous centre error update

**Equation TRISO-DIS-518**

```math
E_0^{j+1}
=
(1-6\mathrm{Fo})E_0^j
+
6\mathrm{Fo}E_1^j.
```

The neighbour coefficient satisfies

**Equation TRISO-DIS-519**

```math
6\mathrm{Fo}\ge0.
```

The centre coefficient is non-negative when

**Equation TRISO-DIS-520**

```math
1-6\mathrm{Fo}\ge0.
```

Therefore

**Equation TRISO-DIS-521**

```math
6\mathrm{Fo}\le1.
```

Hence

**Equation TRISO-DIS-522**

```math
\boxed{
\mathrm{Fo}\le\frac16.
}
```

The centre-row sum is

**Equation TRISO-DIS-523**

```math
(1-6\mathrm{Fo})+6\mathrm{Fo}.
```

Therefore

**Equation TRISO-DIS-524**

```math
\boxed{
(1-6\mathrm{Fo})+6\mathrm{Fo}=1.
}
```

Under (TRISO-DIS-522), the centre row is also a convex combination.

### 17.4 Robin surface-row coefficients

Section 16 gives the homogeneous surface error update

**Equation TRISO-DIS-525**

```math
E_N^{j+1}
=
2\mathrm{Fo}E_{N-1}^j
+
\left[
1
-
2\mathrm{Fo}
\left(
1+\kappa\left(1+\frac1N\right)
\right)
\right]
E_N^j.
```

The interior-neighbour coefficient is

**Equation TRISO-DIS-526**

```math
2\mathrm{Fo}\ge0.
```

The surface coefficient is non-negative when

**Equation TRISO-DIS-527**

```math
1
-
2\mathrm{Fo}
\left(
1+\kappa\left(1+\frac1N\right)
\right)
\ge0.
```

Move the second term to the other side:

**Equation TRISO-DIS-528**

```math
1
\ge
2\mathrm{Fo}
\left(
1+\kappa\left(1+\frac1N\right)
\right).
```

For $h\ge0$, $D>0$, and $\Delta r>0$,

**Equation TRISO-DIS-529**

```math
\kappa\ge0.
```

Therefore the denominator below is positive.

Divide:

**Equation TRISO-DIS-530**

```math
\boxed{
\mathrm{Fo}
\le
\frac{
1
}{
2\left[
1+\kappa\left(1+\frac1N\right)
\right]
}.
}
```

Now calculate the surface-row sum:

**Equation TRISO-DIS-531**

```math
2\mathrm{Fo}
+
1
-
2\mathrm{Fo}
\left(
1+\kappa\left(1+\frac1N\right)
\right).
```

Expand the last term:

**Equation TRISO-DIS-532**

```math
2\mathrm{Fo}
+
1
-
2\mathrm{Fo}
-
2\mathrm{Fo}\kappa
\left(
1+\frac1N
\right).
```

Cancel $2\mathrm{Fo}-2\mathrm{Fo}$:

**Equation TRISO-DIS-533**

```math
\boxed{
\text{surface row sum}
=
1
-
2\mathrm{Fo}\kappa
\left(
1+\frac1N
\right).
}
```

For $\kappa\ge0$,

**Equation TRISO-DIS-534**

```math
\text{surface row sum}\le1.
```

Under the non-negativity restriction (TRISO-DIS-530),

**Equation TRISO-DIS-535**

```math
\text{surface row sum}\ge0.
```

Thus the Robin row is sub-convex: part of the previous concentration can leave through the external boundary.

### 17.5 Combined coefficient-non-negativity condition

The three restrictions are

**Equation TRISO-DIS-536**

```math
\mathrm{Fo}\le\frac12
```

for ordinary interior nodes,

**Equation TRISO-DIS-537**

```math
\mathrm{Fo}\le\frac16
```

for the centre,

and

**Equation TRISO-DIS-538**

```math
\mathrm{Fo}
\le
\frac{
1
}{
2\left[
1+\kappa\left(1+\frac1N\right)
\right]
}
```

for the Robin surface.

Because

**Equation TRISO-DIS-539**

```math
\frac16<\frac12,
```

the interior bound is never the controlling restriction once the centre node is included.

Therefore a sufficient coefficient-non-negativity condition for this homogeneous benchmark is

**Equation TRISO-DIS-540**

```math
\boxed{
\mathrm{Fo}
\le
\min
\left\{
\frac16,
\,
\frac{
1
}{
2\left[
1+\kappa\left(1+\frac1N\right)
\right]
}
\right\}.
}
```

This is the corrected form of the original notebook restriction.

### 17.6 Amplification matrix

Collect the nodal errors into

**Equation TRISO-DIS-541**

```math
\mathbf E^j
=
(E_0^j,E_1^j,\ldots,E_N^j)^T.
```

The homogeneous update is

**Equation TRISO-DIS-542**

```math
\boxed{
\mathbf E^{j+1}
=
\mathbf A\mathbf E^j.
}
```

The matrix has size

**Equation TRISO-DIS-543**

```math
\mathbf A\in\mathbb R^{(N+1)\times(N+1)}.
```

The centre row contains

**Equation TRISO-DIS-544**

```math
A_{0,0}=1-6\mathrm{Fo},
```

and

**Equation TRISO-DIS-545**

```math
A_{0,1}=6\mathrm{Fo}.
```

For an ordinary interior row $i$,

**Equation TRISO-DIS-546**

```math
A_{i,i-1}
=
\mathrm{Fo}
\left(
1-\frac1i
\right),
```

**Equation TRISO-DIS-547**

```math
A_{i,i}
=
1-2\mathrm{Fo},
```

and

**Equation TRISO-DIS-548**

```math
A_{i,i+1}
=
\mathrm{Fo}
\left(
1+\frac1i
\right).
```

The Robin surface row contains

**Equation TRISO-DIS-549**

```math
A_{N,N-1}=2\mathrm{Fo},
```

and

**Equation TRISO-DIS-550**

```math
A_{N,N}
=
1
-
2\mathrm{Fo}
\left(
1+\kappa\left(1+\frac1N\right)
\right).
```

All other entries are zero for this homogeneous tridiagonal benchmark.

### 17.7 $\ell_\infty$ stability under the monotonicity restriction

The induced infinity norm of a matrix is

**Equation TRISO-DIS-551**

```math
\|\mathbf A\|_\infty
=
\max_i
\sum_j
|A_{ij}|.
```

Under (TRISO-DIS-540), every non-zero entry of $\mathbf A$ is non-negative.

Therefore

**Equation TRISO-DIS-552**

```math
|A_{ij}|=A_{ij}.
```

For the centre row, the row sum is exactly

**Equation TRISO-DIS-553**

```math
1.
```

For each ordinary interior row, the row sum is exactly

**Equation TRISO-DIS-554**

```math
1.
```

For the Robin row, the row sum is at most

**Equation TRISO-DIS-555**

```math
1.
```

Therefore

**Equation TRISO-DIS-556**

```math
\boxed{
\|\mathbf A\|_\infty\le1.
}
```

Apply the matrix norm inequality:

**Equation TRISO-DIS-557**

```math
\|\mathbf E^{j+1}\|_\infty
=
\|\mathbf A\mathbf E^j\|_\infty
\le
\|\mathbf A\|_\infty
\|\mathbf E^j\|_\infty.
```

Use (TRISO-DIS-556):

**Equation TRISO-DIS-558**

```math
\|\mathbf E^{j+1}\|_\infty
\le
\|\mathbf E^j\|_\infty.
```

Repeat the inequality over $j$ steps:

**Equation TRISO-DIS-559**

```math
\boxed{
\|\mathbf E^j\|_\infty
\le
\|\mathbf E^0\|_\infty.
}
```

Thus the coefficient restriction (TRISO-DIS-540) is not merely a heuristic: for this assembled homogeneous benchmark it is a sufficient condition for non-amplification in the discrete $\ell_\infty$ norm.

### 17.8 Positivity and discrete maximum-principle interpretation

Suppose

**Equation TRISO-DIS-560**

```math
E_i^j\ge0
```

for every node.

Under (TRISO-DIS-540), every amplification coefficient is non-negative.

Therefore every component of

**Equation TRISO-DIS-561**

```math
\mathbf E^{j+1}
=
\mathbf A\mathbf E^j
```

is also non-negative.

Hence the homogeneous update preserves non-negativity.

For an interior or centre row whose coefficients sum to one, the new value lies between the minimum and maximum of the contributing old values.

At the Robin boundary, the row sum is less than or equal to one because concentration can leave the domain.

This is the precise monotonicity/maximum-principle content of the notebook's coefficient argument.

### 17.9 Spectral-radius consequence

For every square matrix,

**Equation TRISO-DIS-562**

```math
\rho(\mathbf A)
\le
\|\mathbf A\|
```

for any induced matrix norm.

Using the infinity norm,

**Equation TRISO-DIS-563**

```math
\rho(\mathbf A)
\le
\|\mathbf A\|_\infty.
```

Under (TRISO-DIS-540),

**Equation TRISO-DIS-564**

```math
\|\mathbf A\|_\infty\le1.
```

Therefore

**Equation TRISO-DIS-565**

```math
\boxed{
\rho(\mathbf A)\le1.
}
```

So the monotonicity restriction also provides a sufficient spectral-radius bound for this homogeneous assembled amplification matrix.

This does **not** prove that (TRISO-DIS-540) is necessary for spectral stability.

There may be parameter values with some negative coefficients for which

**Equation TRISO-DIS-566**

```math
\rho(\mathbf A)\le1.
```

Determining the exact necessary-and-sufficient spectral stability region would require analysis of the eigenvalues of the specific amplification matrix.

### 17.10 Relation to the standard explicit-Euler eigenvalue condition

Write a semi-discrete diffusion system abstractly as

**Equation TRISO-DIS-567**

```math
\frac{d\mathbf C}{dt}
=
\mathbf L\mathbf C.
```

Forward Euler gives

**Equation TRISO-DIS-568**

```math
\mathbf C^{j+1}
=
\left(
\mathbf I+\Delta t\,\mathbf L
\right)
\mathbf C^j.
```

Therefore

**Equation TRISO-DIS-569**

```math
\mathbf A
=
\mathbf I+\Delta t\,\mathbf L.
```

If $\lambda_\ell$ is an eigenvalue of $\mathbf L$, then the corresponding amplification eigenvalue is

**Equation TRISO-DIS-570**

```math
g_\ell
=
1+\Delta t\,\lambda_\ell.
```

For a real non-positive diffusion eigenvalue,

**Equation TRISO-DIS-571**

```math
\lambda_\ell\le0,
```

the scalar forward-Euler stability requirement is

**Equation TRISO-DIS-572**

```math
|1+\Delta t\,\lambda_\ell|\le1.
```

For real $\lambda_\ell\le0$, this is equivalent to

**Equation TRISO-DIS-573**

```math
-1
\le
1+\Delta t\,\lambda_\ell
\le
1.
```

Subtract $1$:

**Equation TRISO-DIS-574**

```math
-2
\le
\Delta t\,\lambda_\ell
\le
0.
```

Because $\lambda_\ell<0$ for a decaying mode, the lower inequality gives

**Equation TRISO-DIS-575**

```math
\Delta t
\le
\frac{2}{|\lambda_\ell|}.
```

For all modes,

**Equation TRISO-DIS-576**

```math
\boxed{
\Delta t
\le
\frac{2}{
\max_\ell|\lambda_\ell|
}
}
```

would be the exact scalar forward-Euler restriction if the relevant semi-discrete operator has a real non-positive spectrum and is diagonalizable in the norm under consideration.

The present section does not compute $\max|\lambda_\ell|$ for the spherical matrix.

Therefore (TRISO-DIS-576) is a framework for a sharper spectral analysis, not a completed numerical bound for this benchmark.

### 17.11 Important limitation: material interfaces

The amplification matrix in Sections 17.6–17.10 corresponds to the homogeneous benchmark using:

- the centre row from Section 14;
- homogeneous interior rows from Section 13;
- the Robin surface row from Section 16.

The full five-layer problem contains discontinuous diffusivities and the interface transmission treatment from Section 15.

Its assembled operator is therefore different.

The bound

**Equation TRISO-DIS-577**

```math
\mathrm{Fo}
\le
\min
\left\{
\frac16,
\frac1{
2[1+\kappa(1+1/N)]
}
\right\}
```

must **not** be presented as a proved stability bound for an arbitrary five-layer discretisation.

A five-layer stability condition depends on the final chosen conservative spatial discretisation, the layer-specific diffusivities, mesh spacings, and interface treatment.

### 17.12 Correct status of the original notebook claim

The original notebook's coefficient argument is therefore classified as follows.

[VERIFIED] Interior coefficient non-negativity requires

**Equation TRISO-DIS-578**

```math
\mathrm{Fo}\le\frac12.
```

[VERIFIED] Centre coefficient non-negativity requires

**Equation TRISO-DIS-579**

```math
\mathrm{Fo}\le\frac16.
```

[VERIFIED] Robin-surface coefficient non-negativity requires

**Equation TRISO-DIS-580**

```math
\mathrm{Fo}
\le
\frac1{
2[1+\kappa(1+1/N)]
}.
```

[VERIFIED] Their minimum is a sufficient monotonicity condition for the homogeneous benchmark.

[VERIFIED] Under that same condition, the assembled homogeneous amplification matrix satisfies

**Equation TRISO-DIS-581**

```math
\|\mathbf A\|_\infty\le1
```

and consequently

**Equation TRISO-DIS-582**

```math
\rho(\mathbf A)\le1.
```

[NOT PROVED] The condition is necessary for spectral stability.

[NOT APPLICABLE WITHOUT RE-DERIVATION] The same bound is the exact stability condition for the final discontinuous-$D$, five-layer discretisation.

## 18. Conservative finite-volume discretisation of the five-layer PDE

The original FTCS derivation is a useful benchmark, but the actual five-layer PDE contains discontinuous material diffusivities.

A conservative finite-volume formulation is therefore derived directly from

**Equation TRISO-FV-100**

```math
\frac{\partial c}{\partial t}
=
\frac1{r^2}
\frac{\partial}{\partial r}
\left(
r^2D\frac{\partial c}{\partial r}
\right)
+
S.
```

This choice is a mathematical discretisation of the verified continuum model. It does not replace or redefine the production WOS algorithm.

### 18.1 Spherical control-volume geometry

Let cell $P$ occupy

**Equation TRISO-FV-101**

```math
r_{P-\frac12}
<
r
<
r_{P+\frac12}.
```

Its west face is

**Equation TRISO-FV-102**

```math
r_w=r_{P-\frac12},
```

and its east face is

**Equation TRISO-FV-103**

```math
r_e=r_{P+\frac12}.
```

The corresponding spherical face areas are

**Equation TRISO-FV-104**

```math
\boxed{
A_w=4\pi r_w^2
}
```

and

**Equation TRISO-FV-105**

```math
\boxed{
A_e=4\pi r_e^2.
}
```

The exact cell volume is

**Equation TRISO-FV-106**

```math
V_P
=
\int_{r_w}^{r_e}4\pi r^2\,dr.
```

Evaluate the integral:

**Equation TRISO-FV-107**

```math
V_P
=
4\pi
\left[
\frac{r^3}{3}
\right]_{r_w}^{r_e}.
```

Therefore

**Equation TRISO-FV-108**

```math
\boxed{
V_P
=
\frac{4\pi}{3}
\left(
r_e^3-r_w^3
\right).
}
```

### 18.1.1 Frozen finite-volume unknown and representative coordinate

The canonical finite-volume unknown remains the **exact spherical cell average** defined later in (TRISO-FV-118). It is not redefined as a point value.

For geometry and two-point flux reconstruction, assign each cell a representative radial coordinate equal to its spherical volume centroid:

**Equation TRISO-FV-109A**

```math
\boxed{
r_P
=
\frac{
\displaystyle
\int_{r_w}^{r_e}
r\,4\pi r^2\,dr
}{
\displaystyle
\int_{r_w}^{r_e}
4\pi r^2\,dr
}.
}
```

Evaluate the numerator:

**Equation TRISO-FV-109B**

```math
\int_{r_w}^{r_e}
4\pi r^3\,dr
=
\pi
\left(
r_e^4-r_w^4
\right).
```

Use the exact volume (TRISO-FV-108):

**Equation TRISO-FV-109C**

```math
V_P
=
\frac{4\pi}{3}
\left(
r_e^3-r_w^3
\right).
```

Therefore

**Equation TRISO-FV-109D**

```math
\boxed{
r_P
=
\frac34
\frac{
r_e^4-r_w^4
}{
r_e^3-r_w^3
}.
}
```

The canonical representation is therefore:

- $C_P$: exact spherical volume average over cell $P$;
- $r_P$: spherical volume-centroid coordinate used as the representative location of that average in two-point reconstruction;
- $r_{P+1/2}$: physical face coordinate;
- $r_{P+1}-r_P$: distance between representative cell coordinates;
- $\delta r_P=r_{P+1/2}-r_P$: distance from cell $P$'s representative coordinate to its east face;
- $\delta r_{P+1}=r_{P+1}-r_{P+1/2}$: distance from the shared face to the neighbouring representative coordinate.

At a material interface, the physical interface is aligned with a face $r_{P+1/2}$.

At the outer boundary,

**Equation TRISO-FV-109E**

```math
\delta r_R=R-r_{M-1}.
```

[IMPORTANT] The face-gradient formulas treat the exact cell averages as reconstructed values located at their volume centroids. That reconstruction is an approximation whose spatial order must be established in the subsequent accuracy study; it is not part of the exact control-volume balance.

### 18.2 Integrate conservation over one spherical cell

Multiply (TRISO-FV-100) by the spherical volume element

**Equation TRISO-FV-109**

```math
4\pi r^2\,dr.
```

This gives

**Equation TRISO-FV-110**

```math
4\pi r^2
\frac{\partial c}{\partial t}\,dr
=
4\pi
\frac{\partial}{\partial r}
\left(
r^2D\frac{\partial c}{\partial r}
\right)dr
+
4\pi r^2S\,dr.
```

Integrate from $r_w$ to $r_e$:

**Equation TRISO-FV-111**

```math
\int_{r_w}^{r_e}
4\pi r^2
\frac{\partial c}{\partial t}\,dr
=
\int_{r_w}^{r_e}
4\pi
\frac{\partial}{\partial r}
\left(
r^2D\frac{\partial c}{\partial r}
\right)dr
+
\int_{r_w}^{r_e}
4\pi r^2S\,dr.
```

Evaluate the derivative integral:

**Equation TRISO-FV-112**

```math
\int_{r_w}^{r_e}
4\pi
\frac{\partial}{\partial r}
\left(
r^2D\frac{\partial c}{\partial r}
\right)dr
=
4\pi
\left[
r^2D\frac{\partial c}{\partial r}
\right]_{r_w}^{r_e}.
```

Expand the boundary evaluation:

**Equation TRISO-FV-113**

```math
4\pi
\left[
r^2D\frac{\partial c}{\partial r}
\right]_{r_w}^{r_e}
=
A_eD_e
\left.\frac{\partial c}{\partial r}\right|_e
-
A_wD_w
\left.\frac{\partial c}{\partial r}\right|_w.
```

Define the outward radial Fickian flux

**Equation TRISO-FV-114**

```math
J_r=-D\frac{\partial c}{\partial r}.
```

Then

**Equation TRISO-FV-115**

```math
D\frac{\partial c}{\partial r}=-J_r.
```

Substitute into (TRISO-FV-113):

**Equation TRISO-FV-116**

```math
A_eD_e c_r|_e-A_wD_wc_r|_w
=
-A_eJ_e+A_wJ_w.
```

Thus the integrated conservation equation is

**Equation TRISO-FV-117**

```math
\int_{r_w}^{r_e}
4\pi r^2
\frac{\partial c}{\partial t}\,dr
=
A_wJ_w-A_eJ_e
+
\int_{r_w}^{r_e}4\pi r^2S\,dr.
```

This is the discrete starting point: accumulation equals inward face flow minus outward face flow plus generation.

### 18.3 Cell-average unknown

Define the volume-averaged concentration

**Equation TRISO-FV-118**

```math
\boxed{
C_P(t)
=
\frac1{V_P}
\int_{r_w}^{r_e}
c(r,t)\,4\pi r^2\,dr.
}
```

Multiply by $V_P$:

**Equation TRISO-FV-119**

```math
V_PC_P
=
\int_{r_w}^{r_e}
c\,4\pi r^2\,dr.
```

For a fixed mesh, $V_P$ is constant in time.

Differentiate:

**Equation TRISO-FV-120**

```math
V_P\frac{dC_P}{dt}
=
\int_{r_w}^{r_e}
4\pi r^2
\frac{\partial c}{\partial t}\,dr.
```

Define the volume-averaged source

**Equation TRISO-FV-121**

```math
\boxed{
S_P
=
\frac1{V_P}
\int_{r_w}^{r_e}
S(r,t)\,4\pi r^2\,dr.
}
```

Therefore

**Equation TRISO-FV-122**

```math
S_PV_P
=
\int_{r_w}^{r_e}
S(r,t)\,4\pi r^2\,dr.
```

Substitute (TRISO-FV-120) and (TRISO-FV-122) into (TRISO-FV-117):

**Equation TRISO-FV-123**

```math
\boxed{
V_P\frac{dC_P}{dt}
=
A_wJ_w-A_eJ_e+S_PV_P.
}
```

Equation (TRISO-FV-123) is an exact control-volume balance before face-flux approximation.

### 18.4 Interior face flux inside one material

Let cells $P$ and $E$ share east face $e$.

Assume the face lies inside one material with constant diffusivity $D_e$.

Let the cell-centre distance be

**Equation TRISO-FV-124**

```math
\delta r_{PE}=r_E-r_P.
```

Approximate the face gradient by

**Equation TRISO-FV-125**

```math
\left.
\frac{\partial c}{\partial r}
\right|_e
\approx
\frac{C_E-C_P}{\delta r_{PE}}.
```

Fick's law gives

**Equation TRISO-FV-126**

```math
J_e
\approx
-D_e
\frac{C_E-C_P}{\delta r_{PE}}.
```

Reverse the numerator:

**Equation TRISO-FV-127**

```math
\boxed{
J_e
\approx
D_e
\frac{C_P-C_E}{\delta r_{PE}}.
}
```

Similarly, for west neighbour $W$,

**Equation TRISO-FV-128**

```math
\boxed{
J_w
\approx
D_w
\frac{C_W-C_P}{\delta r_{WP}}.
}
```

The sign convention is consistent with $J_r>0$ meaning outward radial transport.

### 18.5 Face crossing a material interface

Now let face $e$ coincide with an interface between two materials.

Let the distance from centre $P$ to the interface be

**Equation TRISO-FV-129**

```math
\delta r_P.
```

Let the distance from the interface to centre $E$ be

**Equation TRISO-FV-130**

```math
\delta r_E.
```

Let the diffusivities be

**Equation TRISO-FV-131**

```math
D_P
```

and

**Equation TRISO-FV-132**

```math
D_E.
```

Let the ideal interface concentration be $C_I$.

The flux from $P$ to the interface is

**Equation TRISO-FV-133**

```math
J_e
=
D_P
\frac{C_P-C_I}{\delta r_P}.
```

The flux from the interface to $E$ is

**Equation TRISO-FV-134**

```math
J_e
=
D_E
\frac{C_I-C_E}{\delta r_E}.
```

Solve the first equation for the concentration drop:

**Equation TRISO-FV-135**

```math
C_P-C_I
=
J_e\frac{\delta r_P}{D_P}.
```

Solve the second equation for its drop:

**Equation TRISO-FV-136**

```math
C_I-C_E
=
J_e\frac{\delta r_E}{D_E}.
```

Add the two equations:

**Equation TRISO-FV-137**

```math
C_P-C_E
=
J_e
\left(
\frac{\delta r_P}{D_P}
+
\frac{\delta r_E}{D_E}
\right).
```

Solve for $J_e$:

**Equation TRISO-FV-138**

```math
\boxed{
J_e
=
\frac{
C_P-C_E
}{
\dfrac{\delta r_P}{D_P}
+
\dfrac{\delta r_E}{D_E}
}.
}
```

Define the face conductance per unit area

**Equation TRISO-FV-139**

```math
\boxed{
g_e
=
\left(
\frac{\delta r_P}{D_P}
+
\frac{\delta r_E}{D_E}
\right)^{-1}.
}
```

Then

**Equation TRISO-FV-140**

```math
\boxed{
J_e=g_e(C_P-C_E).
}
```

The interface law is therefore generated by adding diffusion resistances, exactly as in the continuous steady resistance derivation.

### 18.6 Effective face diffusivity

Define the centre-to-centre distance

**Equation TRISO-FV-141**

```math
\delta r_{PE}
=
\delta r_P+\delta r_E.
```

Define $D_e^{\mathrm{eff}}$ by

**Equation TRISO-FV-142**

```math
J_e
=
D_e^{\mathrm{eff}}
\frac{C_P-C_E}{\delta r_{PE}}.
```

Compare with (TRISO-FV-138):

**Equation TRISO-FV-143**

```math
\frac{D_e^{\mathrm{eff}}}{\delta r_{PE}}
=
\left(
\frac{\delta r_P}{D_P}
+
\frac{\delta r_E}{D_E}
\right)^{-1}.
```

Multiply by $\delta r_{PE}$:

**Equation TRISO-FV-144**

```math
\boxed{
D_e^{\mathrm{eff}}
=
\frac{
\delta r_P+\delta r_E
}{
\dfrac{\delta r_P}{D_P}
+
\dfrac{\delta r_E}{D_E}
}.
}
```

For equal half-cell distances,

**Equation TRISO-FV-145**

```math
\delta r_P=\delta r_E,
```

this reduces to

**Equation TRISO-FV-146**

```math
\boxed{
D_e^{\mathrm{eff}}
=
\frac{2D_PD_E}{D_P+D_E}.
}
```

Thus the harmonic mean arises naturally from the conservative face-flux derivation.

### 18.7 Semi-discrete conservative equation

Define the west conductance

**Equation TRISO-FV-147**

```math
G_w
=
\frac{A_wD_w^{\mathrm{eff}}}{\delta r_{WP}},
```

and the east conductance

**Equation TRISO-FV-148**

```math
G_e
=
\frac{A_eD_e^{\mathrm{eff}}}{\delta r_{PE}}.
```

Their units are

**Equation TRISO-FV-149**

```math
[G_w]=[G_e]=\mathrm{m^3\,s^{-1}}.
```

Using (TRISO-FV-127) and (TRISO-FV-128),

**Equation TRISO-FV-150**

```math
A_wJ_w
=
G_w(C_W-C_P),
```

and

**Equation TRISO-FV-151**

```math
A_eJ_e
=
G_e(C_P-C_E).
```

Substitute these into (TRISO-FV-123):

**Equation TRISO-FV-152**

```math
V_P\frac{dC_P}{dt}
=
G_w(C_W-C_P)
-
G_e(C_P-C_E)
+
S_PV_P.
```

Expand:

**Equation TRISO-FV-153**

```math
V_P\frac{dC_P}{dt}
=
G_wC_W
-
G_wC_P
-
G_eC_P
+
G_eC_E
+
S_PV_P.
```

Collect the central concentration:

**Equation TRISO-FV-154**

```math
\boxed{
V_P\frac{dC_P}{dt}
=
G_wC_W
-
(G_w+G_e)C_P
+
G_eC_E
+
S_PV_P.
}
```

Divide by $V_P$:

**Equation TRISO-FV-155**

```math
\boxed{
\frac{dC_P}{dt}
=
\frac{G_w}{V_P}C_W
-
\frac{G_w+G_e}{V_P}C_P
+
\frac{G_e}{V_P}C_E
+
S_P.
}
```

This is the conservative semi-discrete equation for an ordinary spherical control volume.

### 18.8 Explicit Euler time discretisation

Apply forward Euler:

**Equation TRISO-FV-156**

```math
\frac{
C_P^{j+1}-C_P^j
}{
\Delta t
}
=
\frac{G_w}{V_P}C_W^j
-
\frac{G_w+G_e}{V_P}C_P^j
+
\frac{G_e}{V_P}C_E^j
+
S_P^j.
```

Multiply by $\Delta t$:

**Equation TRISO-FV-157**

```math
C_P^{j+1}-C_P^j
=
\frac{\Delta t\,G_w}{V_P}C_W^j
-
\frac{\Delta t(G_w+G_e)}{V_P}C_P^j
+
\frac{\Delta t\,G_e}{V_P}C_E^j
+
S_P^j\Delta t.
```

Add $C_P^j$:

**Equation TRISO-FV-158**

```math
\boxed{
C_P^{j+1}
=
\frac{\Delta t\,G_w}{V_P}C_W^j
+
\left[
1-
\frac{\Delta t(G_w+G_e)}{V_P}
\right]C_P^j
+
\frac{\Delta t\,G_e}{V_P}C_E^j
+
S_P^j\Delta t.
}
```

This is an explicit conservative finite-volume update.

### 18.9 Exact discrete conservation over multiple cells

Sum (TRISO-FV-123) over all control volumes $P=1,\ldots,M$:

**Equation TRISO-FV-159**

```math
\sum_{P=1}^{M}
V_P\frac{dC_P}{dt}
=
\sum_{P=1}^{M}
(A_wJ_w-A_eJ_e)
+
\sum_{P=1}^{M}S_PV_P.
```

At a shared internal face, the east flux of one cell is the west flux of the next cell.

For example,

**Equation TRISO-FV-160**

```math
-A_{e,P}J_{e,P}
+
A_{w,P+1}J_{w,P+1}
=
0
```

because

**Equation TRISO-FV-161**

```math
A_{e,P}=A_{w,P+1}
```

and the same single face flux is used:

**Equation TRISO-FV-162**

```math
J_{e,P}=J_{w,P+1}.
```

Therefore every internal-face contribution cancels pairwise.

Only the physical domain boundaries remain:

**Equation TRISO-FV-163**

```math
\boxed{
\frac{d}{dt}
\left(
\sum_PV_PC_P
\right)
=
\text{boundary inflow}
-
\text{boundary outflow}
+
\sum_PS_PV_P.
}
```

This is the principal conservation advantage of the finite-volume construction.

### 18.10 Centre control volume

For the central cell,

**Equation TRISO-FV-164**

```math
r_w=0.
```

Therefore its west-face area is

**Equation TRISO-FV-165**

```math
A_w=4\pi(0)^2.
```

Hence

**Equation TRISO-FV-166**

```math
\boxed{
A_w=0.
}
```

The centre requires no artificial inward flux condition in the control-volume balance because the spherical face at $r=0$ has zero area.

The central balance becomes

**Equation TRISO-FV-167**

```math
\boxed{
V_0\frac{dC_0}{dt}
=
-A_eJ_e
+
S_0V_0.
}
```

Using the outward face-flux convention,

**Equation TRISO-FV-168**

```math
J_e
=
G_e^{(A=1)}(C_0-C_1),
```

where $G_e^{(A=1)}$ denotes the conductance per unit area.

Equivalently, using total conductance $G_e$,

**Equation TRISO-FV-169**

```math
\boxed{
V_0\frac{dC_0}{dt}
=
G_e(C_1-C_0)
+
S_0V_0.
}
```

Thus centre regularity is built geometrically into the zero-area inner face.

### 18.11 Kernel-confined source

For a cell entirely inside the kernel,

**Equation TRISO-FV-170**

```math
S_P=S_0
```

for the constant-source benchmark.

For a cell entirely outside the kernel,

**Equation TRISO-FV-171**

```math
S_P=0.
```

If a control-volume face is aligned with the kernel boundary $r_1$, no cell straddles the source discontinuity.

Then the discrete total generation is

**Equation TRISO-FV-172**

```math
\dot N_{\mathrm{gen}}^{\,h}
=
\sum_{P\in\mathrm{kernel}}
S_0V_P.
```

Because the kernel control volumes exactly partition $0<r<r_1$,

**Equation TRISO-FV-173**

```math
\sum_{P\in\mathrm{kernel}}V_P
=
\frac{4\pi r_1^3}{3}.
```

Therefore

**Equation TRISO-FV-174**

```math
\boxed{
\dot N_{\mathrm{gen}}^{\,h}
=
\frac{4\pi S_0r_1^3}{3}.
}
```

The aligned finite-volume source inventory exactly reproduces the continuous total generation for constant $S_0$.

### 18.12 Cell-centred Robin boundary closure

At the physical outer surface,

**Equation TRISO-FV-175**

```math
r=R.
```

Let the centre of the outermost OPyC control volume be at

**Equation TRISO-FV-176**

```math
r_P<R.
```

Define the centre-to-surface distance

**Equation TRISO-FV-177**

```math
\boxed{
\delta r_R=R-r_P.
}
```

Let the outer OPyC diffusivity be

**Equation TRISO-FV-178**

```math
D_5.
```

Let the physical surface concentration be

**Equation TRISO-FV-179**

```math
C_R.
```

The outer-cell unknown is the cell-centred or cell-average concentration

**Equation TRISO-FV-180**

```math
C_P.
```

These two concentrations are not silently identified.

#### 18.12.1 Half-cell diffusion relation

Approximate the OPyC concentration gradient between the outer cell centre and the physical surface by

**Equation TRISO-FV-181**

```math
\left.
\frac{\partial c}{\partial r}
\right|_{P\rightarrow R}
\approx
\frac{C_R-C_P}{\delta r_R}.
```

Fick's law gives the outward radial flux

**Equation TRISO-FV-182**

```math
J_R
=
-D_5
\frac{C_R-C_P}{\delta r_R}.
```

Reverse the concentration difference:

**Equation TRISO-FV-183**

```math
\boxed{
J_R
=
D_5
\frac{C_P-C_R}{\delta r_R}.
}
```

Solve for the half-cell concentration drop:

**Equation TRISO-FV-184**

```math
C_P-C_R
=
J_R\frac{\delta r_R}{D_5}.
```

The quantity

**Equation TRISO-FV-185**

```math
\frac{\delta r_R}{D_5}
```

is the diffusion resistance per unit area of the outer half-cell.

#### 18.12.2 External film relation

The physical Robin law is

**Equation TRISO-FV-186**

```math
\boxed{
J_R
=
h(C_R-c_\infty).
}
```

Solve for the film concentration drop:

**Equation TRISO-FV-187**

```math
C_R-c_\infty
=
\frac{J_R}{h}.
```

The external-film resistance per unit area is therefore

**Equation TRISO-FV-188**

```math
\frac1h.
```

#### 18.12.3 Add the two series concentration drops

Write the total cell-centre-to-bulk concentration difference as

**Equation TRISO-FV-189**

```math
C_P-c_\infty
=
(C_P-C_R)
+
(C_R-c_\infty).
```

Substitute the half-cell drop (TRISO-FV-184):

**Equation TRISO-FV-190**

```math
C_P-c_\infty
=
J_R\frac{\delta r_R}{D_5}
+
(C_R-c_\infty).
```

Substitute the film drop (TRISO-FV-187):

**Equation TRISO-FV-191**

```math
C_P-c_\infty
=
J_R\frac{\delta r_R}{D_5}
+
\frac{J_R}{h}.
```

Factor out $J_R$:

**Equation TRISO-FV-192**

```math
C_P-c_\infty
=
J_R
\left(
\frac{\delta r_R}{D_5}
+
\frac1h
\right).
```

Solve for the outward boundary flux:

**Equation TRISO-FV-193**

```math
\boxed{
J_R
=
\frac{
C_P-c_\infty
}{
\dfrac{\delta r_R}{D_5}
+
\dfrac1h
}.
}
```

Thus the half-cell diffusion resistance and external-film resistance add in series.

#### 18.12.4 Effective boundary transfer coefficient

Define the effective cell-centre-to-bulk transfer coefficient

**Equation TRISO-FV-194**

```math
\boxed{
h_{\mathrm{eff}}
=
\left(
\frac{\delta r_R}{D_5}
+
\frac1h
\right)^{-1}.
}
```

Then

**Equation TRISO-FV-195**

```math
\boxed{
J_R
=
h_{\mathrm{eff}}
(C_P-c_\infty).
}
```

Multiply numerator and denominator of (TRISO-FV-194) by $hD_5$:

**Equation TRISO-FV-196**

```math
h_{\mathrm{eff}}
=
\frac{hD_5}{
h\delta r_R+D_5
}.
```

Therefore

**Equation TRISO-FV-197**

```math
\boxed{
h_{\mathrm{eff}}
=
\frac{hD_5}{
D_5+h\delta r_R
}.
}
```

The units are

**Equation TRISO-FV-198**

```math
[h_{\mathrm{eff}}]
=
\mathrm{m\,s^{-1}}.
```

#### 18.12.5 Recover the physical surface concentration

The surface concentration can also be obtained explicitly.

From (TRISO-FV-187),

**Equation TRISO-FV-199**

```math
C_R
=
c_\infty+\frac{J_R}{h}.
```

Substitute (TRISO-FV-193):

**Equation TRISO-FV-200**

```math
C_R
=
c_\infty
+
\frac1h
\frac{
C_P-c_\infty
}{
\dfrac{\delta r_R}{D_5}
+
\dfrac1h
}.
```

Multiply the second term's denominator by $h$:

**Equation TRISO-FV-201**

```math
C_R
=
c_\infty
+
\frac{
C_P-c_\infty
}{
1+\dfrac{h\delta r_R}{D_5}
}.
```

Define the half-cell boundary Biot number

**Equation TRISO-FV-202**

```math
\boxed{
\mathrm{Bi}_R
=
\frac{h\delta r_R}{D_5}.
}
```

Then

**Equation TRISO-FV-203**

```math
\boxed{
C_R
=
c_\infty
+
\frac{
C_P-c_\infty
}{
1+\mathrm{Bi}_R
}.
}
```

This expression keeps the cell-centre and physical surface concentrations distinct.

#### 18.12.6 Limiting checks

If

**Equation TRISO-FV-204**

```math
h\to0,
```

then

**Equation TRISO-FV-205**

```math
\frac1h\to\infty.
```

Therefore

**Equation TRISO-FV-206**

```math
h_{\mathrm{eff}}\to0.
```

Hence

**Equation TRISO-FV-207**

```math
J_R\to0.
```

This recovers the insulating Neumann limit.

If

**Equation TRISO-FV-208**

```math
\delta r_R\to0,
```

then the half-cell resistance vanishes:

**Equation TRISO-FV-209**

```math
\frac{\delta r_R}{D_5}\to0.
```

Therefore

**Equation TRISO-FV-210**

```math
h_{\mathrm{eff}}\to h.
```

Thus the cell-centred closure approaches the physical Robin law as the outer cell centre approaches the surface.

If

**Equation TRISO-FV-211**

```math
h\to\infty,
```

then the film resistance vanishes:

**Equation TRISO-FV-212**

```math
\frac1h\to0.
```

Therefore

**Equation TRISO-FV-213**

```math
\boxed{
h_{\mathrm{eff}}
\to
\frac{D_5}{\delta r_R}.
}
```

The resulting flux is

**Equation TRISO-FV-214**

```math
J_R
\to
\frac{D_5}{\delta r_R}
(C_P-c_\infty).
```

This is the expected half-cell diffusion flux to a prescribed Dirichlet surface concentration $C_R=c_\infty$.

Unlike the ghost-point formula, this cell-centred resistance closure remains finite in the $h\to\infty$ limit.

#### 18.12.7 Outer-face amount conductance

The physical outer area is

**Equation TRISO-FV-215**

```math
\boxed{
A_R=4\pi R^2.
}
```

Define the total outer-boundary conductance

**Equation TRISO-FV-216**

```math
\boxed{
G_R=A_Rh_{\mathrm{eff}}.
}
```

Its units are

**Equation TRISO-FV-217**

```math
[G_R]
=
\mathrm{m^2}
\times
\mathrm{m\,s^{-1}}
=
\mathrm{m^3\,s^{-1}}.
```

The outward amount rate is

**Equation TRISO-FV-218**

```math
A_RJ_R
=
G_R(C_P-c_\infty).
```

#### 18.12.8 Final outer-cell semi-discrete balance

For the outermost control volume, the exact balance is

**Equation TRISO-FV-219**

```math
V_P\frac{dC_P}{dt}
=
A_wJ_w
-
A_RJ_R
+
S_PV_P.
```

The west-face amount rate is

**Equation TRISO-FV-220**

```math
A_wJ_w
=
G_w(C_W-C_P).
```

The outer amount rate is

**Equation TRISO-FV-221**

```math
A_RJ_R
=
G_R(C_P-c_\infty).
```

Substitute both:

**Equation TRISO-FV-222**

```math
V_P\frac{dC_P}{dt}
=
G_w(C_W-C_P)
-
G_R(C_P-c_\infty)
+
S_PV_P.
```

Expand the west term:

**Equation TRISO-FV-223**

```math
V_P\frac{dC_P}{dt}
=
G_wC_W
-
G_wC_P
-
G_R(C_P-c_\infty)
+
S_PV_P.
```

Expand the boundary term:

**Equation TRISO-FV-224**

```math
V_P\frac{dC_P}{dt}
=
G_wC_W
-
G_wC_P
-
G_RC_P
+
G_Rc_\infty
+
S_PV_P.
```

Collect the cell-centre concentration:

**Equation TRISO-FV-225**

```math
\boxed{
V_P\frac{dC_P}{dt}
=
G_wC_W
-
(G_w+G_R)C_P
+
G_Rc_\infty
+
S_PV_P.
}
```

Divide by $V_P$:

**Equation TRISO-FV-226**

```math
\boxed{
\frac{dC_P}{dt}
=
\frac{G_w}{V_P}C_W
-
\frac{G_w+G_R}{V_P}C_P
+
\frac{G_R}{V_P}c_\infty
+
S_P.
}
```

For the benchmark

**Equation TRISO-FV-227**

```math
c_\infty=0,
```

this reduces to

**Equation TRISO-FV-228**

```math
\boxed{
\frac{dC_P}{dt}
=
\frac{G_w}{V_P}C_W
-
\frac{G_w+G_R}{V_P}C_P
+
S_P.
}
```

For the physical five-layer kernel-confined source model, the outer OPyC cell has

**Equation TRISO-FV-229**

```math
S_P=0.
```

#### 18.12.9 Explicit Euler outer-cell update

Apply forward Euler to (TRISO-FV-226):

**Equation TRISO-FV-230**

```math
\frac{
C_P^{j+1}-C_P^j
}{
\Delta t
}
=
\frac{G_w}{V_P}C_W^j
-
\frac{G_w+G_R}{V_P}C_P^j
+
\frac{G_R}{V_P}c_\infty^j
+
S_P^j.
```

Multiply by $\Delta t$:

**Equation TRISO-FV-231**

```math
C_P^{j+1}-C_P^j
=
\frac{\Delta tG_w}{V_P}C_W^j
-
\frac{\Delta t(G_w+G_R)}{V_P}C_P^j
+
\frac{\Delta tG_R}{V_P}c_\infty^j
+
S_P^j\Delta t.
```

Add $C_P^j$:

**Equation TRISO-FV-232**

```math
\boxed{
C_P^{j+1}
=
\frac{\Delta tG_w}{V_P}C_W^j
+
\left[
1-\frac{\Delta t(G_w+G_R)}{V_P}
\right]C_P^j
+
\frac{\Delta tG_R}{V_P}c_\infty^j
+
S_P^j\Delta t.
}
```

This closes the cell-centred Robin boundary without identifying the cell-centre concentration with the physical surface concentration.

### 18.13 Explicit-Euler positivity condition for an ordinary cell

From (TRISO-FV-158), the neighbour coefficients are

**Equation TRISO-FV-600**

```math
\frac{\Delta t\,G_w}{V_P}\ge0
```

and

**Equation TRISO-FV-601**

```math
\frac{\Delta t\,G_e}{V_P}\ge0.
```

The central coefficient is non-negative when

**Equation TRISO-FV-602**

```math
1-
\frac{\Delta t(G_w+G_e)}{V_P}
\ge0.
```

Rearrange:

**Equation TRISO-FV-603**

```math
\Delta t(G_w+G_e)
\le
V_P.
```

Therefore

**Equation TRISO-FV-604**

```math
\boxed{
\Delta t
\le
\frac{V_P}{G_w+G_e}.
}
```

For every ordinary cell, a sufficient global coefficient-positivity restriction is

**Equation TRISO-FV-605**

```math
\boxed{
\Delta t
\le
\min_P
\frac{V_P}{G_w+G_e}.
}
```

This is the finite-volume analogue of the earlier homogeneous FTCS monotonicity restriction.

The outer Robin closure is now available in Section 18.12. The final global positivity bound must include its boundary conductance.

### 18.14 Global five-layer semi-discrete matrix

Let the finite-volume mesh contain $M$ spherical cells.

Index the cell-average concentrations by

**Equation TRISO-FV-233**

```math
\mathbf C(t)
=
(C_0,C_1,\ldots,C_{M-1})^T.
```

Define the diagonal volume matrix

**Equation TRISO-FV-234**

```math
\boxed{
\mathbf V
=
\operatorname{diag}
(V_0,V_1,\ldots,V_{M-1}).
}
```

Every cell volume is positive:

**Equation TRISO-FV-235**

```math
V_P>0.
```

Let $G_{P+\frac12}$ denote the total conductance of the face shared by cells $P$ and $P+1$.

At an ordinary same-material face,

**Equation TRISO-FV-236**

```math
G_{P+\frac12}
=
\frac{
A_{P+\frac12}D
}{
r_{P+1}-r_P
}.
```

At a material-interface face,

**Equation TRISO-FV-237**

```math
\boxed{
G_{P+\frac12}
=
A_{P+\frac12}
\left(
\frac{\delta r_P}{D_P}
+
\frac{\delta r_{P+1}}{D_{P+1}}
\right)^{-1}.
}
```

Thus the matrix assembly does not require a separate interface unknown.

### 18.15 Central-cell row

The central-cell balance is

**Equation TRISO-FV-238**

```math
V_0\frac{dC_0}{dt}
=
G_{\frac12}(C_1-C_0)
+
S_0V_0.
```

Expand:

**Equation TRISO-FV-239**

```math
V_0\frac{dC_0}{dt}
=
-G_{\frac12}C_0
+
G_{\frac12}C_1
+
S_0V_0.
```

Therefore the first row of the transport matrix contains

**Equation TRISO-FV-240**

```math
K_{0,0}
=
-G_{\frac12},
```

and

**Equation TRISO-FV-241**

```math
K_{0,1}
=
G_{\frac12}.
```

### 18.16 Ordinary-cell row

For cell $P$, with

**Equation TRISO-FV-242**

```math
1\le P\le M-2,
```

the conservative balance is

**Equation TRISO-FV-243**

```math
V_P\frac{dC_P}{dt}
=
G_{P-\frac12}(C_{P-1}-C_P)
+
G_{P+\frac12}(C_{P+1}-C_P)
+
S_PV_P.
```

Expand the west-face term:

**Equation TRISO-FV-244**

```math
G_{P-\frac12}(C_{P-1}-C_P)
=
G_{P-\frac12}C_{P-1}
-
G_{P-\frac12}C_P.
```

Expand the east-face term:

**Equation TRISO-FV-245**

```math
G_{P+\frac12}(C_{P+1}-C_P)
=
G_{P+\frac12}C_{P+1}
-
G_{P+\frac12}C_P.
```

Substitute:

**Equation TRISO-FV-246**

```math
V_P\frac{dC_P}{dt}
=
G_{P-\frac12}C_{P-1}
-
G_{P-\frac12}C_P
+
G_{P+\frac12}C_{P+1}
-
G_{P+\frac12}C_P
+
S_PV_P.
```

Collect the central coefficient:

**Equation TRISO-FV-247**

```math
\boxed{
V_P\frac{dC_P}{dt}
=
G_{P-\frac12}C_{P-1}
-
\left(
G_{P-\frac12}+G_{P+\frac12}
\right)C_P
+
G_{P+\frac12}C_{P+1}
+
S_PV_P.
}
```

Therefore

**Equation TRISO-FV-248**

```math
K_{P,P-1}
=
G_{P-\frac12},
```

**Equation TRISO-FV-249**

```math
K_{P,P}
=
-\left(
G_{P-\frac12}+G_{P+\frac12}
\right),
```

and

**Equation TRISO-FV-250**

```math
K_{P,P+1}
=
G_{P+\frac12}.
```

These formulas remain valid when either face is a material interface because the corresponding $G$ already contains the resistance-weighted discontinuous-$D$ coupling.

### 18.17 Outermost-cell row

Let the outermost cell index be

**Equation TRISO-FV-251**

```math
P=M-1.
```

Section 18.12 gives

**Equation TRISO-FV-252**

```math
V_{M-1}\frac{dC_{M-1}}{dt}
=
G_{M-\frac32}C_{M-2}
-
\left(
G_{M-\frac32}+G_R
\right)C_{M-1}
+
G_Rc_\infty
+
S_{M-1}V_{M-1}.
```

Therefore

**Equation TRISO-FV-253**

```math
K_{M-1,M-2}
=
G_{M-\frac32},
```

and

**Equation TRISO-FV-254**

```math
K_{M-1,M-1}
=
-\left(
G_{M-\frac32}+G_R
\right).
```

The external concentration enters as a forcing term rather than as another particle unknown.

### 18.18 Assemble the transport matrix

Define the source vector

**Equation TRISO-FV-255**

```math
\mathbf S
=
(S_0,S_1,\ldots,S_{M-1})^T.
```

Define the external-boundary forcing vector

**Equation TRISO-FV-256**

```math
\mathbf b_\infty
=
(0,0,\ldots,0,G_Rc_\infty)^T.
```

The complete semi-discrete system is

**Equation TRISO-FV-257**

```math
\boxed{
\mathbf V
\frac{d\mathbf C}{dt}
=
\mathbf K\mathbf C
+
\mathbf V\mathbf S
+
\mathbf b_\infty.
}
```

The matrix $\mathbf K$ is tridiagonal:

**Equation TRISO-FV-258**

```math
\mathbf K
=
\begin{pmatrix}
-G_{\frac12}
&
G_{\frac12}
&
0
&
\cdots
&
0
\\
G_{\frac12}
&
-(G_{\frac12}+G_{\frac32})
&
G_{\frac32}
&
\ddots
&
\vdots
\\
0
&
G_{\frac32}
&
-(G_{\frac32}+G_{\frac52})
&
\ddots
&
0
\\
\vdots
&
\ddots
&
\ddots
&
\ddots
&
G_{M-\frac32}
\\
0
&
\cdots
&
0
&
G_{M-\frac32}
&
-(G_{M-\frac32}+G_R)
\end{pmatrix}.
```

Because each shared-face conductance appears identically in the two neighbouring rows,

**Equation TRISO-FV-259**

```math
\boxed{
\mathbf K=\mathbf K^T.
}
```

The time-evolution operator in concentration coordinates is

**Equation TRISO-FV-260**

```math
\boxed{
\mathbf L
=
\mathbf V^{-1}\mathbf K.
}
```

The matrix $\mathbf L$ is generally not symmetric because cell volumes differ with radius.

### 18.19 Weighted self-adjoint structure of the semi-discrete operator

Define the discrete volume-weighted inner product

**Equation TRISO-FV-261**

```math
\boxed{
\langle\mathbf x,\mathbf y\rangle_V
=
\mathbf x^T\mathbf V\mathbf y.
}
```

Evaluate

**Equation TRISO-FV-262**

```math
\langle\mathbf x,\mathbf L\mathbf y\rangle_V
=
\mathbf x^T\mathbf V\mathbf L\mathbf y.
```

Use $\mathbf L=\mathbf V^{-1}\mathbf K$:

**Equation TRISO-FV-263**

```math
\mathbf V\mathbf L
=
\mathbf K.
```

Therefore

**Equation TRISO-FV-264**

```math
\langle\mathbf x,\mathbf L\mathbf y\rangle_V
=
\mathbf x^T\mathbf K\mathbf y.
```

Because $\mathbf K=\mathbf K^T$,

**Equation TRISO-FV-265**

```math
\mathbf x^T\mathbf K\mathbf y
=
\mathbf y^T\mathbf K\mathbf x.
```

Reverse the preceding steps:

**Equation TRISO-FV-266**

```math
\mathbf y^T\mathbf K\mathbf x
=
\langle\mathbf L\mathbf x,\mathbf y\rangle_V.
```

Hence

**Equation TRISO-FV-267**

```math
\boxed{
\langle\mathbf x,\mathbf L\mathbf y\rangle_V
=
\langle\mathbf L\mathbf x,\mathbf y\rangle_V.
}
```

Thus the conservative semi-discrete diffusion operator is self-adjoint in the volume-weighted discrete inner product.

This is the discrete analogue of the continuum $r^2$-weighted self-adjoint structure.

### 18.20 Negative-semidefinite diffusion form

For any vector $\mathbf x$,

**Equation TRISO-FV-268**

```math
\mathbf x^T\mathbf K\mathbf x
```

can be grouped face-by-face.

An internal face between $P$ and $P+1$ contributes

**Equation TRISO-FV-269**

```math
-G_{P+\frac12}x_P^2
+
2G_{P+\frac12}x_Px_{P+1}
-
G_{P+\frac12}x_{P+1}^2.
```

Factor $-G_{P+\frac12}$:

**Equation TRISO-FV-270**

```math
-G_{P+\frac12}
\left(
x_P^2
-
2x_Px_{P+1}
+
x_{P+1}^2
\right).
```

Recognise the square:

**Equation TRISO-FV-271**

```math
x_P^2
-
2x_Px_{P+1}
+
x_{P+1}^2
=
(x_{P+1}-x_P)^2.
```

Therefore each internal face contributes

**Equation TRISO-FV-272**

```math
-G_{P+\frac12}
(x_{P+1}-x_P)^2.
```

The Robin boundary contributes

**Equation TRISO-FV-273**

```math
-G_Rx_{M-1}^2.
```

Thus

**Equation TRISO-FV-274**

```math
\boxed{
\mathbf x^T\mathbf K\mathbf x
=
-
\sum_{P=0}^{M-2}
G_{P+\frac12}
(x_{P+1}-x_P)^2
-
G_Rx_{M-1}^2.
}
```

Since all conductances are non-negative,

**Equation TRISO-FV-275**

```math
\boxed{
\mathbf x^T\mathbf K\mathbf x\le0.
}
```

For $G_R>0$, equality requires both

**Equation TRISO-FV-276**

```math
x_{P+1}=x_P
```

for every internal face and

**Equation TRISO-FV-277**

```math
x_{M-1}=0.
```

Therefore

**Equation TRISO-FV-278**

```math
\mathbf x=\mathbf0.
```

Hence, for a finite-transfer or absorbing outer boundary with $G_R>0$,

**Equation TRISO-FV-279**

```math
\boxed{
\mathbf K
\text{ is negative definite.}
}
```

If $G_R=0$, the constant vector is the expected zero mode of a closed no-flux particle.

### 18.21 Global discrete inventory balance

Define the discrete total particle inventory

**Equation TRISO-FV-280**

```math
\boxed{
N_h(t)
=
\mathbf 1^T\mathbf V\mathbf C
=
\sum_{P=0}^{M-1}
V_PC_P.
}
```

Differentiate:

**Equation TRISO-FV-281**

```math
\frac{dN_h}{dt}
=
\mathbf1^T
\mathbf V
\frac{d\mathbf C}{dt}.
```

Use the semi-discrete system:

**Equation TRISO-FV-282**

```math
\frac{dN_h}{dt}
=
\mathbf1^T\mathbf K\mathbf C
+
\mathbf1^T\mathbf V\mathbf S
+
\mathbf1^T\mathbf b_\infty.
```

All internal conductance contributions cancel in the row sum.

The only non-zero transport contribution is the Robin boundary:

**Equation TRISO-FV-283**

```math
\mathbf1^T\mathbf K\mathbf C
=
-G_RC_{M-1}.
```

The boundary forcing is

**Equation TRISO-FV-284**

```math
\mathbf1^T\mathbf b_\infty
=
G_Rc_\infty.
```

Therefore

**Equation TRISO-FV-285**

```math
\boxed{
\frac{dN_h}{dt}
=
\sum_{P=0}^{M-1}S_PV_P
-
G_R(C_{M-1}-c_\infty).
}
```

This is exactly the discrete statement

**Equation TRISO-FV-286**

```math
\text{accumulation}
=
\text{generation}
-
\text{outward release}.
```

For the kernel-confined constant source,

**Equation TRISO-FV-287**

```math
\sum_PS_PV_P
=
\frac{4\pi S_0r_1^3}{3}
```

when the source/material interface is face-aligned.

Thus

**Equation TRISO-FV-288**

```math
\boxed{
\frac{dN_h}{dt}
=
\frac{4\pi S_0r_1^3}{3}
-
G_R(C_{M-1}-c_\infty).
}
```

### 18.22 Explicit-Euler matrix update

Apply forward Euler to (TRISO-FV-257):

**Equation TRISO-FV-289**

```math
\mathbf V
\frac{
\mathbf C^{j+1}-\mathbf C^j
}{
\Delta t
}
=
\mathbf K\mathbf C^j
+
\mathbf V\mathbf S^j
+
\mathbf b_\infty^j.
```

Multiply by $\mathbf V^{-1}$:

**Equation TRISO-FV-290**

```math
\frac{
\mathbf C^{j+1}-\mathbf C^j
}{
\Delta t
}
=
\mathbf V^{-1}\mathbf K\mathbf C^j
+
\mathbf S^j
+
\mathbf V^{-1}\mathbf b_\infty^j.
```

Multiply by $\Delta t$:

**Equation TRISO-FV-291**

```math
\mathbf C^{j+1}-\mathbf C^j
=
\Delta t\,\mathbf V^{-1}\mathbf K\mathbf C^j
+
\Delta t\,\mathbf S^j
+
\Delta t\,\mathbf V^{-1}\mathbf b_\infty^j.
```

Add $\mathbf C^j$:

**Equation TRISO-FV-292**

```math
\boxed{
\mathbf C^{j+1}
=
\mathbf A_{\mathrm{FV}}\mathbf C^j
+
\Delta t\,\mathbf S^j
+
\Delta t\,\mathbf V^{-1}\mathbf b_\infty^j,
}
```

where

**Equation TRISO-FV-293**

```math
\boxed{
\mathbf A_{\mathrm{FV}}
=
\mathbf I
+
\Delta t\,\mathbf V^{-1}\mathbf K.
}
```

### 18.23 Completed coefficient-positivity bound

For the central cell, the explicit update has central coefficient

**Equation TRISO-FV-294**

```math
1-
\frac{\Delta tG_{\frac12}}{V_0}.
```

It is non-negative when

**Equation TRISO-FV-295**

```math
\boxed{
\Delta t
\le
\frac{V_0}{G_{\frac12}}.
}
```

For an ordinary cell $P$,

**Equation TRISO-FV-296**

```math
1-
\frac{
\Delta t
\left(
G_{P-\frac12}+G_{P+\frac12}
\right)
}{
V_P
}
\ge0.
```

Therefore

**Equation TRISO-FV-297**

```math
\boxed{
\Delta t
\le
\frac{
V_P
}{
G_{P-\frac12}+G_{P+\frac12}
}.
}
```

For the outer cell,

**Equation TRISO-FV-298**

```math
1-
\frac{
\Delta t
\left(
G_{M-\frac32}+G_R
\right)
}{
V_{M-1}
}
\ge0.
```

Therefore

**Equation TRISO-FV-299**

```math
\boxed{
\Delta t
\le
\frac{
V_{M-1}
}{
G_{M-\frac32}+G_R
}.
}
```

Combine all cells:

**Equation TRISO-FV-300**

```math
\boxed{
\Delta t
\le
\min
\left\{
\frac{V_0}{G_{\frac12}},
\;
\min_{1\le P\le M-2}
\frac{
V_P
}{
G_{P-\frac12}+G_{P+\frac12}
},
\;
\frac{
V_{M-1}
}{
G_{M-\frac32}+G_R
}
\right\}.
}
```

Under this condition, every off-diagonal amplification coefficient is non-negative and every diagonal amplification coefficient is non-negative.

### 18.24 Row sums and monotonicity

For the centre row, the two homogeneous coefficients sum to

**Equation TRISO-FV-301**

```math
1.
```

For every ordinary interior cell, the three homogeneous coefficients sum to

**Equation TRISO-FV-302**

```math
1.
```

For the outer row, the particle-state coefficients sum to

**Equation TRISO-FV-303**

```math
1-
\frac{\Delta tG_R}{V_{M-1}}.
```

For $G_R\ge0$,

**Equation TRISO-FV-304**

```math
1-
\frac{\Delta tG_R}{V_{M-1}}
\le1.
```

Under (TRISO-FV-300), the row sum is also non-negative.

Therefore the homogeneous particle amplification matrix is substochastic.

Hence

**Equation TRISO-FV-305**

```math
\boxed{
\|\mathbf A_{\mathrm{FV}}\|_\infty\le1.
}
```

Consequently,

**Equation TRISO-FV-306**

```math
\boxed{
\rho(\mathbf A_{\mathrm{FV}})\le1.
}
```

Thus (TRISO-FV-300) is a sufficient explicit-Euler monotonicity and $\ell_\infty$-stability condition for the completed finite-volume system.

It is not asserted to be a necessary spectral-stability condition.

### 18.25 Semi-discrete spectral sign

The generalized eigenproblem is

**Equation TRISO-FV-307**

```math
\mathbf K\mathbf x
=
\lambda
\mathbf V\mathbf x.
```

Premultiply by $\mathbf x^T$:

**Equation TRISO-FV-308**

```math
\mathbf x^T\mathbf K\mathbf x
=
\lambda
\mathbf x^T\mathbf V\mathbf x.
```

For non-zero $\mathbf x$,

**Equation TRISO-FV-309**

```math
\mathbf x^T\mathbf V\mathbf x>0.
```

From (TRISO-FV-275),

**Equation TRISO-FV-310**

```math
\mathbf x^T\mathbf K\mathbf x\le0.
```

Therefore

**Equation TRISO-FV-311**

```math
\boxed{
\lambda\le0.
}
```

For $G_R>0$, $\mathbf K$ is negative definite, so

**Equation TRISO-FV-312**

```math
\boxed{
\lambda<0
}
```

for every non-zero mode.

This establishes the expected diffusive sign of the semi-discrete spectrum without numerically enumerating eigenvalues.

### 18.26 Remaining accuracy and convergence questions

The complete five-layer finite-volume algebra is now assembled.

The following statements are established:

- exact control-volume conservation;
- resistance-weighted interface fluxes;
- a closed cell-centred Robin boundary;
- a symmetric conductance matrix $\mathbf K$;
- volume-weighted self-adjointness;
- non-positive semi-discrete spectrum;
- a sufficient explicit-Euler monotonicity/$\ell_\infty$-stability bound.

The remaining mathematical questions are narrower:

[UNVERIFIED] Global spatial order of accuracy when $D(r)$ is discontinuous.

[UNVERIFIED] Global temporal/spatial convergence rate of the fully discrete scheme.

[UNVERIFIED] Numerical convergence study against the analytical benchmarks.

These require a dedicated consistency/convergence pass rather than further coefficient assembly.

### 18.27 Status of the five-layer deterministic discretisation

[VERIFIED] The spherical control-volume geometry and exact integrated conservation balance.

[VERIFIED] Conservative internal face coupling.

[VERIFIED] Harmonic/resistance-weighted treatment of discontinuous diffusivity.

[VERIFIED] Pairwise cancellation of internal face fluxes.

[VERIFIED] Geometric centre treatment through the zero-area inner face.

[VERIFIED] Exact kernel generation inventory when material/source interfaces align with control-volume faces.

[VERIFIED] Cell-centred Robin surface closure through half-cell diffusion plus external-film resistance.

[VERIFIED] Final assembled five-layer coefficient matrix including the outer boundary.

[UNVERIFIED] Accuracy order of the complete multilayer finite-volume scheme.

[CONDITIONALLY VERIFIED] Sufficient explicit-Euler monotonicity/stability bound; convergence rate remains unverified.

The finite-volume derivation is therefore the current canonical deterministic route for the discontinuous-$D$ five-layer model, while the original FTCS scheme remains a transparent homogeneous benchmark.

## 18A. Finite-volume consistency: smooth same-material cells

This section begins the accuracy/convergence stage authorised by the independent discrete-mathematics audit.

It treats only smooth cells whose two faces lie inside one material with constant diffusivity $D$. Material-interface and outer-boundary consistency are deferred to subsequent subsections.

### 18A.1 Exact cell average versus representative point value

The canonical unknown is the exact spherical volume average

**Equation TRISO-ACC-100**

```math
C_P
=
\frac1{V_P}
\int_{r_w}^{r_e}
c(r)\,4\pi r^2\,dr.
```

The representative coordinate $r_P$ is the spherical volume centroid:

**Equation TRISO-ACC-101**

```math
r_P
=
\frac1{V_P}
\int_{r_w}^{r_e}
r\,4\pi r^2\,dr.
```

Subtract $r_P$ inside the weighted first moment:

**Equation TRISO-ACC-102**

```math
\int_{r_w}^{r_e}
(r-r_P)\,4\pi r^2\,dr
=
\int_{r_w}^{r_e}
r\,4\pi r^2\,dr
-
r_P
\int_{r_w}^{r_e}
4\pi r^2\,dr.
```

Use the centroid definition in the first term:

**Equation TRISO-ACC-103**

```math
\int_{r_w}^{r_e}
r\,4\pi r^2\,dr
=
r_PV_P.
```

Use the volume definition in the second term:

**Equation TRISO-ACC-104**

```math
\int_{r_w}^{r_e}
4\pi r^2\,dr
=
V_P.
```

Therefore

**Equation TRISO-ACC-105**

```math
\boxed{
\int_{r_w}^{r_e}
(r-r_P)\,4\pi r^2\,dr
=
0.
}
```

Taylor-expand a smooth concentration about $r_P$:

**Equation TRISO-ACC-106**

```math
c(r)
=
c(r_P)
+
c_r(r_P)(r-r_P)
+
\frac12c_{rr}(r_P)(r-r_P)^2
+
O(h_P^3),
```

where

**Equation TRISO-ACC-107**

```math
h_P=r_e-r_w.
```

Insert (TRISO-ACC-106) into the exact average:

**Equation TRISO-ACC-108**

```math
C_P
=
\frac1{V_P}
\int_{r_w}^{r_e}
\left[
c(r_P)
+
c_r(r_P)(r-r_P)
+
\frac12c_{rr}(r_P)(r-r_P)^2
+
O(h_P^3)
\right]
4\pi r^2\,dr.
```

Separate the constant term:

**Equation TRISO-ACC-109**

```math
\frac{c(r_P)}{V_P}
\int_{r_w}^{r_e}4\pi r^2\,dr
=
c(r_P).
```

The first-order term vanishes by (TRISO-ACC-105):

**Equation TRISO-ACC-110**

```math
\frac{c_r(r_P)}{V_P}
\int_{r_w}^{r_e}
(r-r_P)4\pi r^2\,dr
=
0.
```

Define the weighted second central moment

**Equation TRISO-ACC-111**

```math
\mu_{2,P}
=
\frac1{V_P}
\int_{r_w}^{r_e}
(r-r_P)^2\,4\pi r^2\,dr.
```

For a shape-regular refining radial mesh,

**Equation TRISO-ACC-112**

```math
\mu_{2,P}=O(h_P^2).
```

Therefore

**Equation TRISO-ACC-113**

```math
\boxed{
C_P
=
c(r_P)
+
\frac12c_{rr}(r_P)\mu_{2,P}
+
O(h_P^3).
}
```

In particular,

**Equation TRISO-ACC-114**

```math
\boxed{
C_P-c(r_P)=O(h_P^2).
}
```

Thus locating the exact cell average at the spherical volume centroid is a second-order point-representation approximation for a smooth field. It is not exact equality.

### 18A.2 Exact integrated balance remains independent of reconstruction

The control-volume identity

**Equation TRISO-ACC-115**

```math
V_P\frac{dC_P}{dt}
=
A_wJ_w-A_eJ_e+S_PV_P
```

was obtained by exact integration.

No point-value approximation was used to obtain (TRISO-ACC-115).

Therefore the spatial consistency question is isolated to the approximation of the face fluxes.

### 18A.3 Two-point gradient on a smooth same-material face

Consider a face $f=r_{P+\frac12}$ between cells $P$ and $E=P+1$.

Define

**Equation TRISO-ACC-116**

```math
d_P=r_f-r_P,
```

and

**Equation TRISO-ACC-117**

```math
d_E=r_E-r_f.
```

Hence

**Equation TRISO-ACC-118**

```math
r_E-r_P=d_P+d_E.
```

Taylor-expand the exact point value at $r_P$ about the face:

**Equation TRISO-ACC-119**

```math
c(r_P)
=
c_f
-
d_Pc_f'
+
\frac{d_P^2}{2}c_f''
-
\frac{d_P^3}{6}c_f'''
+
O(h^4).
```

Taylor-expand the exact point value at $r_E$:

**Equation TRISO-ACC-120**

```math
c(r_E)
=
c_f
+
d_Ec_f'
+
\frac{d_E^2}{2}c_f''
+
\frac{d_E^3}{6}c_f'''
+
O(h^4).
```

Subtract (TRISO-ACC-119) from (TRISO-ACC-120):

**Equation TRISO-ACC-121**

```math
c(r_E)-c(r_P)
=
(d_P+d_E)c_f'
+
\frac{d_E^2-d_P^2}{2}c_f''
+
\frac{d_E^3+d_P^3}{6}c_f'''
+
O(h^4).
```

Divide by $d_P+d_E$:

**Equation TRISO-ACC-122**

```math
\frac{c(r_E)-c(r_P)}{r_E-r_P}
=
c_f'
+
\frac{d_E-d_P}{2}c_f''
+
\frac{d_E^3+d_P^3}{6(d_P+d_E)}c_f'''
+
O(h^3).
```

For a locally symmetric representative geometry,

**Equation TRISO-ACC-123**

```math
d_E-d_P=O(h^2),
```

while

**Equation TRISO-ACC-124**

```math
d_P=O(h),
\qquad
d_E=O(h).
```

Therefore

**Equation TRISO-ACC-125**

```math
\frac{d_E^3+d_P^3}{d_P+d_E}
=
O(h^2).
```

Hence the point-value two-point gradient is

**Equation TRISO-ACC-126**

```math
\boxed{
\frac{c(r_E)-c(r_P)}{r_E-r_P}
=
c_r(r_f)
+
O(h^2)
}
```

under the local-symmetry condition (TRISO-ACC-123).

### 18A.4 Effect of using exact cell averages

Write the exact cell averages as

**Equation TRISO-ACC-127**

```math
C_P=c(r_P)+\eta_P,
```

and

**Equation TRISO-ACC-128**

```math
C_E=c(r_E)+\eta_E,
```

where

**Equation TRISO-ACC-129**

```math
\eta_P=O(h^2),
\qquad
\eta_E=O(h^2).
```

The numerical gradient is

**Equation TRISO-ACC-130**

```math
\frac{C_E-C_P}{r_E-r_P}.
```

Substitute (TRISO-ACC-127) and (TRISO-ACC-128):

**Equation TRISO-ACC-131**

```math
\frac{C_E-C_P}{r_E-r_P}
=
\frac{c(r_E)-c(r_P)}{r_E-r_P}
+
\frac{\eta_E-\eta_P}{r_E-r_P}.
```

For a smoothly varying family of shape-regular cells, the cell-average representation error varies smoothly between adjacent cells, so

**Equation TRISO-ACC-132**

```math
\eta_E-\eta_P=O(h^3).
```

Since

**Equation TRISO-ACC-133**

```math
r_E-r_P=O(h),
```

we obtain

**Equation TRISO-ACC-134**

```math
\frac{\eta_E-\eta_P}{r_E-r_P}
=
O(h^2).
```

Combine (TRISO-ACC-126) and (TRISO-ACC-134):

**Equation TRISO-ACC-135**

```math
\boxed{
\frac{C_E-C_P}{r_E-r_P}
=
c_r(r_f)
+
O(h^2).
}
```

This result is conditional on smooth solution data, shape-regular refinement, and the local geometric relation (TRISO-ACC-123).

### 18A.5 Same-material face-flux consistency

Inside one material,

**Equation TRISO-ACC-136**

```math
J_f=-D\,c_r(r_f).
```

The two-point numerical flux is

**Equation TRISO-ACC-137**

```math
J_f^h
=
-D
\frac{C_E-C_P}{r_E-r_P}.
```

Use (TRISO-ACC-135):

**Equation TRISO-ACC-138**

```math
J_f^h
=
-D
\left[
c_r(r_f)+O(h^2)
\right].
```

Because $D$ is constant and finite,

**Equation TRISO-ACC-139**

```math
\boxed{
J_f^h
=
J_f
+
O(h^2).
}
```

Thus an ordinary smooth same-material face flux is second-order consistent under the stated mesh assumptions.

### 18A.6 Complete smooth-cell divergence consistency

The exact diffusion contribution to the cell-average evolution is

**Equation TRISO-ACC-140**

```math
\mathcal D_P
=
\frac{
A_wJ_w-A_eJ_e
}{
V_P
}.
```

The numerical diffusion contribution is

**Equation TRISO-ACC-141**

```math
\mathcal D_P^h
=
\frac{
A_wJ_w^h-A_eJ_e^h
}{
V_P
}.
```

Define the face-flux errors

**Equation TRISO-ACC-142**

```math
\varepsilon_w
=
J_w^h-J_w,
```

and

**Equation TRISO-ACC-143**

```math
\varepsilon_e
=
J_e^h-J_e.
```

Subtract the exact cell diffusion term from the numerical one:

**Equation TRISO-ACC-144**

```math
\mathcal D_P^h-\mathcal D_P
=
\frac{
A_w(J_w^h-J_w)
-
A_e(J_e^h-J_e)
}{
V_P
}.
```

Use the error definitions:

**Equation TRISO-ACC-145**

```math
\boxed{
\mathcal D_P^h-\mathcal D_P
=
\frac{
A_w\varepsilon_w-A_e\varepsilon_e
}{
V_P}.
}
```

The face analysis alone gives

**Equation TRISO-ACC-146**

```math
\varepsilon_w=O(h^2),
\qquad
\varepsilon_e=O(h^2).
```

Since

**Equation TRISO-ACC-147**

```math
V_P=O(h)
```

for a refining shell away from pathological mesh degeneration, the estimate (TRISO-ACC-146) by itself would permit only

**Equation TRISO-ACC-148**

```math
\mathcal D_P^h-\mathcal D_P=O(h).
```

Therefore second-order face consistency alone is insufficient to establish second-order cell-divergence consistency.

A cancellation property of the leading face errors is required.

#### 18A.6.1 Smooth leading face-error field

For a smooth solution and a smoothly varying, locally symmetric mesh family, assume the face-flux truncation error admits the local expansion

**Equation TRISO-ACC-149**

```math
\boxed{
\varepsilon_f
=
h^2E(r_f)
+
O(h^3),
}
```

where $E(r)$ is smooth within the material.

This is stronger than the statement $\varepsilon_f=O(h^2)$.

At the west face,

**Equation TRISO-ACC-150**

```math
\varepsilon_w
=
h^2E(r_w)
+
O(h^3).
```

At the east face,

**Equation TRISO-ACC-151**

```math
\varepsilon_e
=
h^2E(r_e)
+
O(h^3).
```

Substitute into the numerator of (TRISO-ACC-145):

**Equation TRISO-ACC-152**

```math
A_w\varepsilon_w-A_e\varepsilon_e
=
h^2
\left[
A_wE(r_w)-A_eE(r_e)
\right]
+
O(h^4),
```

where the remainder scaling assumes a shape-regular cell with bounded spherical face areas.

Define

**Equation TRISO-ACC-153**

```math
F(r)=A(r)E(r),
```

with

**Equation TRISO-ACC-154**

```math
A(r)=4\pi r^2.
```

Then

**Equation TRISO-ACC-155**

```math
A_wE(r_w)-A_eE(r_e)
=
F(r_w)-F(r_e).
```

Taylor-expand $F(r_e)$ about $r_w$:

**Equation TRISO-ACC-156**

```math
F(r_e)
=
F(r_w)
+
(r_e-r_w)F'(r_w)
+
O(h^2).
```

Since

**Equation TRISO-ACC-157**

```math
r_e-r_w=h_P=O(h),
```

subtracting gives

**Equation TRISO-ACC-158**

```math
F(r_w)-F(r_e)
=
-h_PF'(r_w)
+
O(h^2).
```

Therefore

**Equation TRISO-ACC-159**

```math
F(r_w)-F(r_e)=O(h).
```

Return to (TRISO-ACC-152):

**Equation TRISO-ACC-160**

```math
A_w\varepsilon_w-A_e\varepsilon_e
=
h^2O(h)
+
O(h^4).
```

Hence

**Equation TRISO-ACC-161**

```math
\boxed{
A_w\varepsilon_w-A_e\varepsilon_e
=
O(h^3).
}
```

Divide by

**Equation TRISO-ACC-162**

```math
V_P=O(h).
```

Then

**Equation TRISO-ACC-163**

```math
\boxed{
\mathcal D_P^h-\mathcal D_P
=
O(h^2).
}
```

Thus the complete smooth same-material finite-volume diffusion operator is second-order consistent **provided** the leading face-flux error varies smoothly from face to face as in (TRISO-ACC-149).

#### 18A.6.2 Relation to the exact spherical differential operator

The exact integrated diffusion term is

**Equation TRISO-ACC-164**

```math
\mathcal D_P
=
\frac1{V_P}
\int_{r_w}^{r_e}
4\pi
\frac{\partial}{\partial r}
\left(
r^2D c_r
\right)dr.
```

For a smooth integrand, the exact cell average of the differential operator differs from its value at the volume centroid by the same centroid-moment mechanism used in Section 18A.1.

Therefore

**Equation TRISO-ACC-165**

```math
\mathcal D_P
=
\left[
\frac1{r^2}
\frac{\partial}{\partial r}
\left(
r^2D c_r
\right)
\right]_{r=r_P}
+
O(h^2).
```

Combine (TRISO-ACC-163) and (TRISO-ACC-165):

**Equation TRISO-ACC-166**

```math
\boxed{
\mathcal D_P^h
=
\left[
\frac1{r^2}
\frac{\partial}{\partial r}
\left(
r^2D c_r
\right)
\right]_{r=r_P}
+
O(h^2).
}
```

This result applies only to smooth same-material cells under the stated mesh/error-regularity assumptions.

#### 18A.6.3 Why the cancellation assumption matters

If the leading face-error coefficient is not smooth across the cell, then

**Equation TRISO-ACC-167**

```math
A_w\varepsilon_w-A_e\varepsilon_e
```

need not be $O(h^3)$.

It may remain only

**Equation TRISO-ACC-168**

```math
O(h^2).
```

Division by $V_P=O(h)$ would then give only

**Equation TRISO-ACC-169**

```math
O(h)
```

cell-divergence consistency.

This is precisely why the smooth-cell result cannot be transferred automatically across a discontinuous material interface.

### 18A.7 Discontinuous-diffusivity interface-face consistency

Consider a physical material interface at

**Equation TRISO-ACC-170**

```math
r=r_I.
```

Let cell $P$ lie immediately to the left of the interface and cell $E$ immediately to the right.

Define

**Equation TRISO-ACC-171**

```math
d_P=r_I-r_P,
```

and

**Equation TRISO-ACC-172**

```math
d_E=r_E-r_I.
```

Let the diffusivities be

**Equation TRISO-ACC-173**

```math
D^- \quad\text{for }r<r_I,
```

and

**Equation TRISO-ACC-174**

```math
D^+ \quad\text{for }r>r_I.
```

For the frozen ideal interface,

**Equation TRISO-ACC-175**

```math
c^-(r_I)=c^+(r_I)=c_I.
```

The exact outward flux is continuous:

**Equation TRISO-ACC-176**

```math
J_I
=
-D^-c_r^-(r_I)
=
-D^+c_r^+(r_I).
```

### 18A.7.1 Expand the left-side concentration

Taylor-expand the exact point value at $r_P=r_I-d_P$ about the interface from the left:

**Equation TRISO-ACC-177**

```math
c(r_P)
=
c_I
-
d_Pc_r^-(r_I)
+
\frac{d_P^2}{2}c_{rr}^-(r_I)
-
\frac{d_P^3}{6}c_{rrr}^-(r_I)
+
O(h^4).
```

Use

**Equation TRISO-ACC-178**

```math
c_r^-(r_I)
=
-\frac{J_I}{D^-}.
```

Substitute:

**Equation TRISO-ACC-179**

```math
c(r_P)
=
c_I
+
\frac{d_P}{D^-}J_I
+
\frac{d_P^2}{2}c_{rr}^-(r_I)
-
\frac{d_P^3}{6}c_{rrr}^-(r_I)
+
O(h^4).
```

Rearrange the interface-to-left concentration difference:

**Equation TRISO-ACC-180**

```math
c(r_P)-c_I
=
\frac{d_P}{D^-}J_I
+
\frac{d_P^2}{2}c_{rr}^-(r_I)
-
\frac{d_P^3}{6}c_{rrr}^-(r_I)
+
O(h^4).
```

### 18A.7.2 Expand the right-side concentration

Taylor-expand from the right:

**Equation TRISO-ACC-181**

```math
c(r_E)
=
c_I
+
d_Ec_r^+(r_I)
+
\frac{d_E^2}{2}c_{rr}^+(r_I)
+
\frac{d_E^3}{6}c_{rrr}^+(r_I)
+
O(h^4).
```

Use

**Equation TRISO-ACC-182**

```math
c_r^+(r_I)
=
-\frac{J_I}{D^+}.
```

Therefore

**Equation TRISO-ACC-183**

```math
c(r_E)
=
c_I
-
\frac{d_E}{D^+}J_I
+
\frac{d_E^2}{2}c_{rr}^+(r_I)
+
\frac{d_E^3}{6}c_{rrr}^+(r_I)
+
O(h^4).
```

Rearrange:

**Equation TRISO-ACC-184**

```math
c_I-c(r_E)
=
\frac{d_E}{D^+}J_I
-
\frac{d_E^2}{2}c_{rr}^+(r_I)
-
\frac{d_E^3}{6}c_{rrr}^+(r_I)
+
O(h^4).
```

### 18A.7.3 Add the two exact concentration drops

Add (TRISO-ACC-180) and (TRISO-ACC-184):

```math
c(r_P)-c(r_E)
=
J_I
\left(
\frac{d_P}{D^-}
+
\frac{d_E}{D^+}
\right)
```

**Equation TRISO-ACC-185**

```math
+
\frac{d_P^2}{2}c_{rr}^-(r_I)
-
\frac{d_E^2}{2}c_{rr}^+(r_I)
-
\frac{d_P^3}{6}c_{rrr}^-(r_I)
-
\frac{d_E^3}{6}c_{rrr}^+(r_I)
+
O(h^4).
```

Define the two-half-cell resistance

**Equation TRISO-ACC-186**

```math
\boxed{
R_I
=
\frac{d_P}{D^-}
+
\frac{d_E}{D^+}.
}
```

For a shape-regular mesh,

**Equation TRISO-ACC-187**

```math
R_I=O(h).
```

Define the second-order remainder

**Equation TRISO-ACC-188**

```math
Q_I
=
\frac{d_P^2}{2}c_{rr}^-(r_I)
-
\frac{d_E^2}{2}c_{rr}^+(r_I).
```

Then

**Equation TRISO-ACC-189**

```math
Q_I=O(h^2).
```

Equation (TRISO-ACC-185) becomes

**Equation TRISO-ACC-190**

```math
c(r_P)-c(r_E)
=
J_IR_I
+
Q_I
+
O(h^3).
```

Solve for the exact flux:

**Equation TRISO-ACC-191**

```math
J_I
=
\frac{
c(r_P)-c(r_E)-Q_I+O(h^3)
}{
R_I
}.
```

Separate the resistance formula:

**Equation TRISO-ACC-192**

```math
J_I
=
\frac{
c(r_P)-c(r_E)
}{
R_I
}
-
\frac{Q_I}{R_I}
+
O(h^2).
```

Because

**Equation TRISO-ACC-193**

```math
Q_I=O(h^2)
```

and

**Equation TRISO-ACC-194**

```math
R_I=O(h),
```

we have

**Equation TRISO-ACC-195**

```math
\frac{Q_I}{R_I}=O(h).
```

Therefore the point-value resistance flux

**Equation TRISO-ACC-196**

```math
J_I^{h,\mathrm{pt}}
=
\frac{
c(r_P)-c(r_E)
}{
R_I
}
```

satisfies, generically,

**Equation TRISO-ACC-197**

```math
\boxed{
J_I^{h,\mathrm{pt}}
=
J_I
+
O(h).
}
```

Thus the basic two-point harmonic/resistance interface flux is generically first-order accurate at a discontinuity.

### 18A.7.4 Special cancellation condition

The leading $O(h)$ flux error vanishes if

**Equation TRISO-ACC-198**

```math
Q_I=O(h^3).
```

At leading order this requires

**Equation TRISO-ACC-199**

```math
d_P^2c_{rr}^-(r_I)
-
d_E^2c_{rr}^+(r_I)
=
O(h^3).
```

For equal half-distances,

**Equation TRISO-ACC-200**

```math
d_P=d_E,
```

a sufficient leading-order cancellation condition is

**Equation TRISO-ACC-201**

```math
c_{rr}^-(r_I)
=
c_{rr}^+(r_I).
```

Such equality is not generally implied by concentration continuity and flux continuity when $D^-\ne D^+$.

Therefore second-order interface flux accuracy must not be assumed merely because the harmonic resistance is physically conservative.

### 18A.7.5 Effect of exact cell averages

The numerical scheme uses exact cell averages rather than exact point values.

Write

**Equation TRISO-ACC-202**

```math
C_P=c(r_P)+\eta_P,
```

and

**Equation TRISO-ACC-203**

```math
C_E=c(r_E)+\eta_E.
```

For smooth one-sided fields within each material,

**Equation TRISO-ACC-204**

```math
\eta_P=O(h^2),
\qquad
\eta_E=O(h^2).
```

The numerical interface flux is

**Equation TRISO-ACC-205**

```math
J_I^h
=
\frac{
C_P-C_E
}{
R_I
}.
```

Substitute the average representations:

**Equation TRISO-ACC-206**

```math
J_I^h
=
\frac{
c(r_P)-c(r_E)
}{
R_I
}
+
\frac{
\eta_P-\eta_E
}{
R_I
}.
```

Across a material discontinuity, the leading $O(h^2)$ average-representation coefficients on the two sides need not match smoothly.

Therefore, generically,

**Equation TRISO-ACC-207**

```math
\eta_P-\eta_E=O(h^2).
```

Since

**Equation TRISO-ACC-208**

```math
R_I=O(h),
```

the cell-average correction contributes

**Equation TRISO-ACC-209**

```math
\frac{\eta_P-\eta_E}{R_I}
=
O(h).
```

Combine this with (TRISO-ACC-197):

**Equation TRISO-ACC-210**

```math
\boxed{
J_I^h
=
J_I
+
O(h)
}
```

generically for the canonical cell-average, two-point resistance interface flux.

### 18A.7.6 Conservation remains exact despite first-order local accuracy

The same numerical interface flux $J_I^h$ is used by both adjacent control volumes.

The left cell contains the outward interface amount rate

**Equation TRISO-ACC-211**

```math
-A_IJ_I^h.
```

The right cell contains the corresponding inward amount rate

**Equation TRISO-ACC-212**

```math
+A_IJ_I^h.
```

Add them:

**Equation TRISO-ACC-213**

```math
-A_IJ_I^h+A_IJ_I^h=0.
```

Therefore

**Equation TRISO-ACC-214**

```math
\boxed{
\text{interface conservation is exact at the discrete level}
}
```

even though the local interface flux is generically only first-order accurate.

Conservation and formal order are separate properties.

### 18A.7.7 Consequence for cells adjacent to the interface

Let the interface face-flux error be

**Equation TRISO-ACC-215**

```math
\varepsilon_I=O(h).
```

The interface amount-rate error is

**Equation TRISO-ACC-216**

```math
A_I\varepsilon_I.
```

For an interface away from the origin,

**Equation TRISO-ACC-217**

```math
A_I=O(1)
```

under radial refinement of a fixed physical geometry.

The adjacent cell volume satisfies

**Equation TRISO-ACC-218**

```math
V_P=O(h).
```

Therefore the contribution of the interface flux error to the adjacent cell-average time derivative can scale as

**Equation TRISO-ACC-219**

```math
\frac{
A_I\varepsilon_I
}{
V_P
}
=
O(1).
```

This does **not** by itself prove that the global solution fails to converge.

It shows that a pointwise local truncation-error argument at the interface-adjacent cell is insufficient for establishing a global order.

A global stability-plus-consistency argument in an appropriate integrated norm, or direct grid-refinement evidence, is required.

### 18A.7.8 Interface consistency status

[VERIFIED] The harmonic/resistance interface flux exactly enforces one common discrete flux and therefore exact discrete conservation.

[VERIFIED] For piecewise smooth solutions satisfying ideal concentration and flux continuity, the basic two-point point-value interface flux is generically $O(h)$ accurate.

[VERIFIED] Using exact cell averages at volume centroids does not generically improve that interface order; the canonical interface flux remains $O(h)$ unless additional cancellation occurs.

[NOT ESTABLISHED] A second-order interface flux for unequal diffusivities.

[NOT ESTABLISHED] A pointwise vanishing truncation error in the cells directly adjacent to a discontinuous interface.

[NOT ESTABLISHED] The global spatial convergence order of the conservative scheme.

### 18A.8 Cell-centred Robin boundary consistency

Consider the outermost OPyC cell with representative coordinate

**Equation TRISO-ACC-220**

```math
r_P=R-d,
```

where

**Equation TRISO-ACC-221**

```math
d=\delta r_R=O(h).
```

Let

**Equation TRISO-ACC-222**

```math
c_R=c(R)
```

denote the exact physical surface concentration.

The exact Robin condition is

**Equation TRISO-ACC-223**

```math
J_R=h(c_R-c_\infty).
```

The same exact outward flux also satisfies

**Equation TRISO-ACC-224**

```math
J_R=-D_5c_r(R).
```

The numerical cell-centred closure is

**Equation TRISO-ACC-225**

```math
J_R^h
=
\frac{
C_P-c_\infty
}{
\dfrac d{D_5}+\dfrac1h
}.
```

The objective is to compare (TRISO-ACC-225) with the exact $J_R$.

#### 18A.8.1 Exact point-value expansion from the surface to the cell representative point

Taylor-expand the exact OPyC solution from $R$ inward to $r_P=R-d$:

**Equation TRISO-ACC-226**

```math
c(r_P)
=
c_R
-
dc_r(R)
+
\frac{d^2}{2}c_{rr}(R)
-
\frac{d^3}{6}c_{rrr}(R)
+
O(h^4).
```

Use the exact flux relation

**Equation TRISO-ACC-227**

```math
c_r(R)
=
-\frac{J_R}{D_5}.
```

Substitute:

**Equation TRISO-ACC-228**

```math
c(r_P)
=
c_R
+
\frac d{D_5}J_R
+
\frac{d^2}{2}c_{rr}(R)
-
\frac{d^3}{6}c_{rrr}(R)
+
O(h^4).
```

The Robin law gives

**Equation TRISO-ACC-229**

```math
c_R-c_\infty
=
\frac{J_R}{h}.
```

Subtract $c_\infty$ from (TRISO-ACC-228):

**Equation TRISO-ACC-230**

```math
c(r_P)-c_\infty
=
(c_R-c_\infty)
+
\frac d{D_5}J_R
+
\frac{d^2}{2}c_{rr}(R)
-
\frac{d^3}{6}c_{rrr}(R)
+
O(h^4).
```

Substitute (TRISO-ACC-229):

**Equation TRISO-ACC-231**

```math
c(r_P)-c_\infty
=
J_R
\left(
\frac1h+\frac d{D_5}
\right)
+
\frac{d^2}{2}c_{rr}(R)
-
\frac{d^3}{6}c_{rrr}(R)
+
O(h^4).
```

Define the exact cell-centre-to-bulk resistance

**Equation TRISO-ACC-232**

```math
R_B
=
\frac1h+\frac d{D_5}.
```

For fixed finite $h>0$,

**Equation TRISO-ACC-233**

```math
R_B=O(1)
```

as $h\to0$ in the mesh-refinement sense $d\to0$; here $h$ is the physical transfer coefficient and is held fixed.

Equation (TRISO-ACC-231) becomes

**Equation TRISO-ACC-234**

```math
c(r_P)-c_\infty
=
J_RR_B
+
\frac{d^2}{2}c_{rr}(R)
+
O(h^3).
```

Solve for $J_R$:

**Equation TRISO-ACC-235**

```math
J_R
=
\frac{
c(r_P)-c_\infty
}{
R_B
}
-
\frac{
d^2c_{rr}(R)
}{
2R_B
}
+
O(h^3).
```

Therefore the point-value series-resistance flux

**Equation TRISO-ACC-236**

```math
J_R^{h,\mathrm{pt}}
=
\frac{
c(r_P)-c_\infty
}{
R_B
}
```

satisfies

**Equation TRISO-ACC-237**

```math
\boxed{
J_R^{h,\mathrm{pt}}
=
J_R+O(h^2)
}
```

for fixed finite $h>0$.

#### 18A.8.2 Effect of the exact cell average

The numerical closure uses the exact cell average $C_P$, not $c(r_P)$.

Write

**Equation TRISO-ACC-238**

```math
C_P=c(r_P)+\eta_P.
```

From the centroid analysis,

**Equation TRISO-ACC-239**

```math
\eta_P=O(h^2).
```

Substitute into the numerical boundary flux:

**Equation TRISO-ACC-240**

```math
J_R^h
=
\frac{
c(r_P)+\eta_P-c_\infty
}{
R_B
}.
```

Separate the point-value part:

**Equation TRISO-ACC-241**

```math
J_R^h
=
\frac{
c(r_P)-c_\infty
}{
R_B
}
+
\frac{\eta_P}{R_B}.
```

For fixed finite $h>0$,

**Equation TRISO-ACC-242**

```math
R_B=O(1).
```

Therefore

**Equation TRISO-ACC-243**

```math
\frac{\eta_P}{R_B}=O(h^2).
```

Combine with (TRISO-ACC-237):

**Equation TRISO-ACC-244**

```math
\boxed{
J_R^h
=
J_R+O(h^2)
}
```

for the finite-transfer Robin boundary under smooth OPyC data and fixed physical $h$.

### 18A.8.3 Boundary amount-rate consistency

The exact outer area is

**Equation TRISO-ACC-245**

```math
A_R=4\pi R^2.
```

The exact outward amount rate is

**Equation TRISO-ACC-246**

```math
\dot N_R=A_RJ_R.
```

The numerical amount rate is

**Equation TRISO-ACC-247**

```math
\dot N_R^h=A_RJ_R^h.
```

Subtract:

**Equation TRISO-ACC-248**

```math
\dot N_R^h-\dot N_R
=
A_R(J_R^h-J_R).
```

Because $A_R$ is fixed under mesh refinement and (TRISO-ACC-244) gives $J_R^h-J_R=O(h^2)$,

**Equation TRISO-ACC-249**

```math
\boxed{
\dot N_R^h-\dot N_R
=
O(h^2).
}
```

Thus the total Robin release rate is second-order consistent for fixed finite $h$.

### 18A.8.4 Outer-cell local residual scaling

The outer-cell volume satisfies

**Equation TRISO-ACC-250**

```math
V_P=O(h).
```

If the boundary amount-rate error is

**Equation TRISO-ACC-251**

```math
O(h^2),
```

then its contribution to the outer cell-average time-derivative residual can scale as

**Equation TRISO-ACC-252**

```math
\frac{O(h^2)}{O(h)}
=
O(h).
```

Therefore the outermost cell can have only first-order pointwise local truncation consistency even though the physical boundary release rate itself is second-order accurate.

This is analogous to the distinction already identified at material interfaces: a lower pointwise residual in $O(1)$ special cells does not by itself determine the global solution convergence order.

### 18A.8.5 Neumann and Dirichlet limiting regimes

For the insulating limit

**Equation TRISO-ACC-253**

```math
h=0,
```

the exact boundary condition is

**Equation TRISO-ACC-254**

```math
J_R=0.
```

The resistance formula is interpreted by its limit

**Equation TRISO-ACC-255**

```math
\frac1h\to\infty,
```

which gives

**Equation TRISO-ACC-256**

```math
J_R^h\to0.
```

Thus the no-flux boundary is represented exactly as a limiting boundary law.

The absorbing Dirichlet limit is different.

If

**Equation TRISO-ACC-257**

```math
h\to\infty,
```

then

**Equation TRISO-ACC-258**

```math
R_B
=
\frac d{D_5}.
```

Now

**Equation TRISO-ACC-259**

```math
R_B=O(h)
```

where here $h$ denotes the mesh-size order symbol, not the transfer coefficient.

To avoid this notational collision, denote the mesh scale by $\mathfrak h$.

Then

**Equation TRISO-ACC-260**

```math
d=O(\mathfrak h),
```

and

**Equation TRISO-ACC-261**

```math
R_B=O(\mathfrak h).
```

The $O(\mathfrak h^2)$ cell-average representation error divided by $R_B=O(\mathfrak h)$ can contribute

**Equation TRISO-ACC-262**

```math
O(\mathfrak h)
```

to the Dirichlet-limit boundary flux.

Therefore the finite-$h$ second-order flux result (TRISO-ACC-244) must **not** be transferred automatically to the absorbing Dirichlet limit.

For a true absorbing boundary, a separate Dirichlet-boundary consistency analysis is required.

### 18A.8.6 Robin boundary consistency status

[VERIFIED] For fixed finite physical transfer coefficient $0<h<\infty$, the cell-centred series-resistance Robin flux is $O(\mathfrak h^2)$ consistent under smooth OPyC data and the centroid representation assumptions.

[VERIFIED] The total outer release amount rate is also $O(\mathfrak h^2)$ consistent.

[VERIFIED] The outer-cell pointwise time-derivative residual may be only $O(\mathfrak h)$ because the boundary amount-rate error is divided by a cell volume $O(\mathfrak h)$.

[VERIFIED] The no-flux Neumann limit is recovered.

[NOT ESTABLISHED] Second-order boundary-flux accuracy in the absorbing Dirichlet limit $h\to\infty$.

[NOT ESTABLISHED] Global spatial convergence order.

### 18A.9 Global stability and conditional convergence framework

The local consistency results are not uniform over the particle:

- smooth same-material cells have conditional $O(\mathfrak h^2)$ divergence consistency;
- a fixed number of cells adjacent to the four material interfaces can have $O(1)$ pointwise residuals under the present two-point transmission flux;
- the outer Robin cell can have an $O(\mathfrak h)$ pointwise residual for fixed finite physical $h$.

A global convergence argument must therefore use a norm that respects cell volumes rather than taking the maximum pointwise residual as the only consistency measure.

#### 18A.9.1 Volume-weighted discrete norm

Define the discrete volume-weighted norm

**Equation TRISO-ACC-263**

```math
\boxed{
\|\mathbf x\|_V^2
=
\mathbf x^T\mathbf V\mathbf x
=
\sum_{P=0}^{M-1}
V_Px_P^2.
}
```

This is the natural discrete analogue of the spherical $L^2$ norm because

**Equation TRISO-ACC-264**

```math
V_P
=
\int_{\Omega_P}dV.
```

For a shape-regular radial refinement of a fixed particle,

**Equation TRISO-ACC-265**

```math
V_P=O(\mathfrak h)
```

for cells away from the origin.

The central cell is smaller:

**Equation TRISO-ACC-266**

```math
V_0=O(\mathfrak h^3),
```

because its radius is itself $O(\mathfrak h)$.

### 18A.9.2 Semi-discrete error equation

Let

**Equation TRISO-ACC-267**

```math
\overline{\mathbf c}(t)
```

denote the vector of exact spherical cell averages of the continuum solution on the numerical mesh.

Define the semi-discrete residual $\boldsymbol\tau_h(t)$ by inserting these exact cell averages into the numerical operator:

**Equation TRISO-ACC-268**

```math
\boxed{
\mathbf V
\frac{d\overline{\mathbf c}}{dt}
=
\mathbf K\overline{\mathbf c}
+
\mathbf V\mathbf S
+
\mathbf b_\infty
+
\mathbf V\boldsymbol\tau_h.
}
```

The numerical semi-discrete solution satisfies

**Equation TRISO-ACC-269**

```math
\mathbf V
\frac{d\mathbf C}{dt}
=
\mathbf K\mathbf C
+
\mathbf V\mathbf S
+
\mathbf b_\infty.
```

Define the error

**Equation TRISO-ACC-270**

```math
\boxed{
\mathbf e
=
\mathbf C-\overline{\mathbf c}.
}
```

Subtract (TRISO-ACC-268) from (TRISO-ACC-269):

**Equation TRISO-ACC-271**

```math
\mathbf V
\frac{d\mathbf e}{dt}
=
\mathbf K\mathbf e
-
\mathbf V\boldsymbol\tau_h.
```

### 18A.9.3 Energy identity

Premultiply by $\mathbf e^T$:

**Equation TRISO-ACC-272**

```math
\mathbf e^T\mathbf V
\frac{d\mathbf e}{dt}
=
\mathbf e^T\mathbf K\mathbf e
-
\mathbf e^T\mathbf V\boldsymbol\tau_h.
```

Because $\mathbf V$ is time independent,

**Equation TRISO-ACC-273**

```math
\mathbf e^T\mathbf V
\frac{d\mathbf e}{dt}
=
\frac12
\frac{d}{dt}
\left(
\mathbf e^T\mathbf V\mathbf e
\right).
```

Use the norm definition:

**Equation TRISO-ACC-274**

```math
\boxed{
\frac12
\frac{d}{dt}
\|\mathbf e\|_V^2
=
\mathbf e^T\mathbf K\mathbf e
-
\langle\mathbf e,\boldsymbol\tau_h\rangle_V.
}
```

From the previously proved conductance identity,

**Equation TRISO-ACC-275**

```math
\mathbf e^T\mathbf K\mathbf e
\le0.
```

Therefore

**Equation TRISO-ACC-276**

```math
\frac12
\frac{d}{dt}
\|\mathbf e\|_V^2
\le
-
\langle\mathbf e,\boldsymbol\tau_h\rangle_V.
```

Take absolute value of the forcing term:

**Equation TRISO-ACC-277**

```math
-\langle\mathbf e,\boldsymbol\tau_h\rangle_V
\le
\left|
\langle\mathbf e,\boldsymbol\tau_h\rangle_V
\right|.
```

Apply Cauchy-Schwarz in the $V$-inner product:

**Equation TRISO-ACC-278**

```math
\left|
\langle\mathbf e,\boldsymbol\tau_h\rangle_V
\right|
\le
\|\mathbf e\|_V
\|\boldsymbol\tau_h\|_V.
```

Hence

**Equation TRISO-ACC-279**

```math
\frac12
\frac{d}{dt}
\|\mathbf e\|_V^2
\le
\|\mathbf e\|_V
\|\boldsymbol\tau_h\|_V.
```

For $\|\mathbf e\|_V>0$,

**Equation TRISO-ACC-280**

```math
\frac{d}{dt}
\|\mathbf e\|_V^2
=
2\|\mathbf e\|_V
\frac{d}{dt}\|\mathbf e\|_V.
```

Substitute into (TRISO-ACC-279):

**Equation TRISO-ACC-281**

```math
\|\mathbf e\|_V
\frac{d}{dt}\|\mathbf e\|_V
\le
\|\mathbf e\|_V
\|\boldsymbol\tau_h\|_V.
```

Cancel $\|\mathbf e\|_V$:

**Equation TRISO-ACC-282**

```math
\boxed{
\frac{d}{dt}\|\mathbf e\|_V
\le
\|\boldsymbol\tau_h\|_V.
}
```

The same inequality follows by continuity through instants at which the error norm is zero.

Integrate from $0$ to $t$:

**Equation TRISO-ACC-283**

```math
\|\mathbf e(t)\|_V-\|\mathbf e(0)\|_V
\le
\int_0^t
\|\boldsymbol\tau_h(s)\|_V\,ds.
```

Therefore

**Equation TRISO-ACC-284**

```math
\boxed{
\|\mathbf e(t)\|_V
\le
\|\mathbf e(0)\|_V
+
\int_0^t
\|\boldsymbol\tau_h(s)\|_V\,ds.
}
```

This proves semi-discrete energy stability and shows that convergence follows if the volume-weighted residual norm tends to zero and the initial discrete representation converges.

### 18A.9.4 Residual scaling from the established local results

Assume the number of material interfaces remains fixed at four as the mesh is refined.

For $O(\mathfrak h^{-1})$ ordinary smooth cells,

**Equation TRISO-ACC-285**

```math
\tau_P=O(\mathfrak h^2).
```

Each such cell contributes to the squared $V$-norm

**Equation TRISO-ACC-286**

```math
V_P\tau_P^2
=
O(\mathfrak h)
O(\mathfrak h^4)
=
O(\mathfrak h^5).
```

Summing $O(\mathfrak h^{-1})$ smooth cells gives

**Equation TRISO-ACC-287**

```math
O(\mathfrak h^{-1})
O(\mathfrak h^5)
=
O(\mathfrak h^4).
```

Therefore the smooth-region contribution to the residual norm is

**Equation TRISO-ACC-288**

```math
O(\mathfrak h^2).
```

Now consider the fixed number of interface-adjacent cells.

The local analysis permits

**Equation TRISO-ACC-289**

```math
\tau_P=O(1).
```

Each such cell has

**Equation TRISO-ACC-290**

```math
V_P=O(\mathfrak h).
```

Therefore each contributes

**Equation TRISO-ACC-291**

```math
V_P\tau_P^2
=
O(\mathfrak h).
```

A fixed number of such cells still contributes

**Equation TRISO-ACC-292**

```math
O(\mathfrak h)
```

to the squared residual norm.

Hence the interface-region contribution can be only

**Equation TRISO-ACC-293**

```math
\boxed{
O(\mathfrak h^{1/2})
}
```

in the $V$-norm under the currently proved local bounds.

For the single outer Robin cell,

**Equation TRISO-ACC-294**

```math
\tau_{M-1}=O(\mathfrak h).
```

Its volume is

**Equation TRISO-ACC-295**

```math
V_{M-1}=O(\mathfrak h).
```

Therefore its squared contribution is

**Equation TRISO-ACC-296**

```math
V_{M-1}\tau_{M-1}^2
=
O(\mathfrak h^3).
```

and its contribution to the $V$-norm is

**Equation TRISO-ACC-297**

```math
O(\mathfrak h^{3/2}).
```

The conservative interface region therefore dominates the currently provable residual estimate:

**Equation TRISO-ACC-298**

```math
\boxed{
\|\boldsymbol\tau_h\|_V
=
O(\mathfrak h^{1/2})
}
```

under the local bounds established so far.

### 18A.9.5 Conditional semi-discrete convergence bound

If the exact initial condition is represented by exact cell averages, then

**Equation TRISO-ACC-299**

```math
\mathbf e(0)=\mathbf0.
```

Use (TRISO-ACC-284):

**Equation TRISO-ACC-300**

```math
\|\mathbf e(t)\|_V
\le
\int_0^t
\|\boldsymbol\tau_h(s)\|_V\,ds.
```

If the residual bound is uniform for $0\le s\le T$,

**Equation TRISO-ACC-301**

```math
\|\boldsymbol\tau_h(s)\|_V
\le
C_T\mathfrak h^{1/2},
```

then

**Equation TRISO-ACC-302**

```math
\|\mathbf e(t)\|_V
\le
\int_0^t
C_T\mathfrak h^{1/2}\,ds.
```

Evaluate the integral:

**Equation TRISO-ACC-303**

```math
\boxed{
\|\mathbf e(t)\|_V
\le
tC_T\mathfrak h^{1/2},
\qquad
0\le t\le T.
}
```

Thus the present energy argument supports at least a **conditional $O(\mathfrak h^{1/2})$ upper-bound convergence rate in the volume-weighted norm**, given the established local residual estimates and sufficient regularity.

This is a conservative bound, not a prediction of the observed numerical order.

Diffusive smoothing, transmission structure, cancellation, or a sharper negative-norm/interface analysis may yield a higher actual rate.

### 18A.9.6 Fully discrete temporal error

The explicit Euler method has local temporal truncation error

**Equation TRISO-ACC-304**

```math
O(\Delta t^2)
```

per step.

Over a fixed time interval, under stability, its global temporal order is

**Equation TRISO-ACC-305**

```math
O(\Delta t).
```

Therefore the fully discrete error should be separated conceptually as

**Equation TRISO-ACC-306**

```math
\boxed{
\text{error}
=
\text{spatial error}
+
O(\Delta t),
}
```

rather than inferring spatial order from a refinement study in which $\Delta t$ is not reduced sufficiently.

The previously derived explicit positivity/stability condition requires

**Equation TRISO-ACC-307**

```math
\Delta t
\le
\Delta t_{\max}(h).
```

For diffusion on a regular mesh,

**Equation TRISO-ACC-308**

```math
\Delta t_{\max}=O(\mathfrak h^2)
```

away from pathological coefficient scaling.

Choosing

**Equation TRISO-ACC-309**

```math
\Delta t\propto\mathfrak h^2
```

therefore both respects the expected explicit-diffusion stability scaling and makes the first-order temporal error

**Equation TRISO-ACC-310**

```math
O(\Delta t)=O(\mathfrak h^2).
```

Under that refinement path, temporal error should not dominate a spatial rate lower than second order.

### 18A.9.7 What the proof does not establish

The energy estimate proves stability of the semi-discrete error equation in the $V$-norm.

It does not establish that the $O(\mathfrak h^{1/2})$ bound is sharp.

It does not establish an $L^\infty$ convergence rate.

It does not prove second-order global convergence.

It does not replace numerical grid refinement.

The four material interfaces are a measure-shrinking set under refinement, but their stronger local residuals require either a sharper transmission-problem estimate or empirical convergence evidence before a better global rate is claimed.

### 18A.10 Accuracy/convergence study specification

The subsequent numerical study should distinguish three questions.

#### Spatial refinement

Use a sequence of interface-aligned meshes with refinement ratio

**Equation TRISO-ACC-311**

```math
q=\frac{h_k}{h_{k+1}}>1.
```

Keep the physical geometry, diffusivities, source, $h$, and final observation time fixed.

Choose time steps satisfying

**Equation TRISO-ACC-312**

```math
\Delta t_k
=
C h_k^2
```

with $C$ below the explicit stability bound on every mesh.

Compare numerical solutions using at least:

1. the volume-weighted concentration norm;
2. total particle inventory;
3. outer release rate;
4. selected interface-adjacent concentrations.

When an exact analytical benchmark is available, define

**Equation TRISO-ACC-313**

```math
E_k
=
\|\mathbf C_{h_k}-\overline{\mathbf c}_{\mathrm{exact},h_k}\|_V.
```

The observed order between successive meshes is

**Equation TRISO-ACC-314**

```math
\boxed{
p_{\mathrm{obs}}
=
\frac{
\log(E_k/E_{k+1})
}{
\log q
}.
}
```

No target value of $p_{\mathrm{obs}}$ is assumed in advance for the discontinuous-$D$ five-layer problem.

#### Temporal refinement

On a sufficiently fine fixed spatial mesh, refine

**Equation TRISO-ACC-315**

```math
\Delta t_k
\to
\frac{\Delta t_k}{q_t}.
```

The expected explicit-Euler temporal order is one:

**Equation TRISO-ACC-316**

```math
p_t\approx1
```

once spatial error is subdominant.

#### Conservation refinement

For every run, evaluate the discrete inventory residual

**Equation TRISO-ACC-317**

```math
\mathcal R_{\mathrm{cons}}
=
\frac{dN_h}{dt}
-
\sum_PS_PV_P
+
G_R(C_{M-1}-c_\infty).
```

For the algebraically conservative semi-discrete formulation this should vanish up to time-discretisation and floating-point effects when evaluated consistently.

The study must report the norm definition, mesh geometry, interface alignment, time-step scaling, and whether the comparison uses exact cell averages or point samples.

### 18A.11 Executed refinement evidence

The canonical refinement specification has now been executed with the standalone Rust driver `verification/fv_convergence.rs` through GitHub Actions run `36654437525`.

The persisted raw results are `verification/fv_convergence_results.txt`.

The benchmark uses five aligned unit-thickness layers,

**Equation TRISO-ACC-318**

```math
(r_1,r_2,r_3,r_4,R)=(1,2,3,4,5).
```

with

**Equation TRISO-ACC-319**

```math
(D_1,D_2,D_3,D_4,D_5)=(1,0.5,2,0.25,1.5),
```

and

**Equation TRISO-ACC-320**

```math
S_0=1,\qquad h=0.8,\qquad c_\infty=0.
```

These are normalized verification parameters, not claimed physical TRISO material data.

#### Spatial refinement result

The volume-weighted steady errors were

**Equation TRISO-ACC-321**

```math
E_{25}=1.832317223290\times10^{-2},
```

**Equation TRISO-ACC-322**

```math
E_{50}=4.621922061067\times10^{-3},
```

**Equation TRISO-ACC-323**

```math
E_{100}=1.158126274643\times10^{-3},
```

**Equation TRISO-ACC-324**

```math
E_{200}=2.896983800097\times10^{-4},
```

**Equation TRISO-ACC-325**

```math
E_{400}=7.243504635747\times10^{-5}.
```

The corresponding observed orders were

**Equation TRISO-ACC-326**

```math
1.987104,\quad1.996700,\quad1.999169,\quad1.999792.
```

Therefore this aligned five-layer steady benchmark exhibits asymptotic behavior consistent with

**Equation TRISO-ACC-327**

```math
\boxed{E_h=O(\mathfrak h^2).}
```

This observed second-order behavior is substantially sharper than the conservative analytical $O(h^{1/2})$ bound in TRISO-ACC-303. It demonstrates that the local interface-adjacent residual estimate is not predictive of the observed global steady error for this benchmark. It does not prove second-order convergence for every discontinuous-D problem.

#### Temporal refinement result

On the fixed N=200 spatial mesh, successive-step Richardson differences gave observed temporal orders

**Equation TRISO-ACC-328**

```math
0.999700,\quad0.999780,\quad1.000066,\quad0.999962.
```

Therefore the executed result is consistent with

**Equation TRISO-ACC-329**

```math
\boxed{p_t=1.}
```

#### Conservation result

The exact benchmark generation rate is

**Equation TRISO-ACC-330**

```math
\frac{4\pi}{3}=4.188790204786\ldots
```

Across the steady spatial sequence, the computed release rate remained equal to this value to approximately $10^{-12}$ or better.

The steady conservation residual ranged from approximately

**Equation TRISO-ACC-331**

```math
2.7\times10^{-15}
```

to

**Equation TRISO-ACC-332**

```math
2.9\times10^{-12}.
```

During the temporal study, the maximum discrete inventory residual remained below

**Equation TRISO-ACC-333**

```math
\boxed{4\times10^{-10}.}
```

The increase in the reported absolute residual as the time step becomes very small is consistent with floating-point cancellation in the finite-difference evaluation of the inventory time derivative; no systematic conservation drift is observed in this study.

#### Scope of the executed evidence

[EXECUTED / VERIFIED] The canonical aligned five-layer FV benchmark converges approximately second order in the steady volume-weighted spatial norm over the tested meshes.

[EXECUTED / VERIFIED] Forward Euler converges first order in time on the tested fixed spatial mesh.

[EXECUTED / VERIFIED] The conservative FV balance closes to floating-point accuracy in the tested steady and transient runs.

[NOT ESTABLISHED] The same spatial order for non-interface-aligned meshes.

[NOT ESTABLISHED] The same spatial order for arbitrary diffusivity ratios, geometries, or source distributions.

[NOT ESTABLISHED] The absorbing Dirichlet boundary convergence order.

[NOT APPLICABLE] These deterministic FV results do not validate the production WOS algorithm.
### 18A.12 What has and has not been proved

[VERIFIED] The exact spherical cell average differs from the point value at the spherical volume centroid by $O(h^2)$ for a smooth field.

[CONDITIONALLY VERIFIED] The two-point same-material face gradient is $O(h^2)$ consistent when the refining mesh is shape regular, adjacent cell-average representation errors vary smoothly, and the face is locally centred between representative coordinates to $O(h^2)$.

[CONDITIONALLY VERIFIED] Under those same assumptions, the same-material diffusive face flux is $O(h^2)$ consistent.

[CONDITIONALLY VERIFIED] The complete smooth same-material cell divergence is $O(h^2)$ consistent when the leading $O(h^2)$ face-flux error has a smooth coefficient across neighbouring faces.

[VERIFIED] The canonical resistance-weighted interface flux is conservative and generically $O(h)$ accurate for piecewise-smooth unequal-D transmission data; second-order interface accuracy is not established.

[VERIFIED] For fixed finite physical h, the cell-centred Robin FV boundary flux and total release rate are $O(mesh^2)$ consistent; the outer-cell pointwise residual may remain $O(mesh)$.

[CONDITIONALLY ESTABLISHED] The energy argument gives a conservative $O(mesh^1/2)$ bound from local residual estimates. [EXECUTED] The aligned five-layer steady benchmark instead exhibits asymptotic $O(mesh^2)$ volume-weighted convergence over N=25–400; this observed rate is benchmark-specific rather than a universal theorem.

The prescribed aligned five-layer spatial/temporal/conservation refinement study has now executed successfully. The next verification decision is whether to broaden the parameter/interface-alignment study or submit this deterministic accuracy/convergence milestone for independent audit; no WOS conclusion follows from these FV results.

## 19. Implementation provenance

[IMPORTANT] The supervisor production repository remains read-only. The production WOS source provenance used by this research is recorded here for reproducibility; all research verification code and manuscript changes remain in Ray's repository.

- Production geometry: `constructive_solid_geometry/mod.rs` — `TrisoCell::new`, `TrisoCell::new_crp6_geometry`, `TrisoCell::get_triso_region`, `TrisoCell::try_get_diffusion_coefficient`.
- Production WOS: `first_passage/walk_on_spheres.rs` — `WoSWalker`, `step_multilayer`, `walk_until_released`, `nearest_interface_distance`, `shell_bounds`, `sample_uniform_in_ball`.
- Production interface decision: `first_passage/interface.rs` — `does_transmit`.
- Production homogeneous sphere FPT: `first_passage/sphere_fpt.rs` — first-passage distribution and lookup/interpolation.
- Historical production verification: `verification_and_validation/crp6_case1_kernel_release_vs_crank.md`.
- Historical interface-overshoot analysis: `docs/buffer_clt_failure_analysis.md`.

The research-side equation-to-code function map is maintained in `ray to outram park/TRACEABILITY.md`.

## 20. Foundation gate

All 33 original displayed equations are now accounted for in the central register.

The mathematical middle is now continuous from physical problem through conservation, Fickian transport, spherical reduction, source/decay convention, conditions, Part-I steady and transient analysis, Part-II steady analytical solution, Part-II transient eigenvalue framework, and Part-I FTCS.

Remaining foundation gaps are explicitly retained:

- physical source/decay/trapping closure;
- physical partition/interfacial-resistance choice;
- five-layer transient modal coefficient convergence;
- complete FTCS spectral stability proof;
- complete equation-to-code verification;
- WOS-to-continuum transient verification.

The independent continuous-mathematics audit has passed the project for discretisation, and the independent deterministic discrete-mathematics audit has passed the canonical finite-volume model for accuracy/convergence study. Review finding R2-D01 remains open specifically for the retained FTCS Robin benchmark until grid-refinement evidence is produced; it does not block the canonical FV accuracy/convergence track.

# 21. Method 2 stochastic diffusion and first-passage formulation

## 21.1 Diffusion process and backward generator

[EXACT] In one homogeneous material with constant diffusivity $D$,

**Equation TRISO-FPT-001**

```math
\frac{\partial c}{\partial t}=D\nabla^2c.
```

[DEFINITION] Let $X_t$ be the diffusion process generated by the same operator.

Its infinitesimal generator is

**Equation TRISO-FPT-002**

```math
\mathcal L f=D\nabla^2f.
```

[DEFINITION] Let $T$ be the first exit time from a prescribed region.

Define

**Equation TRISO-FPT-003**

```math
u(\mathbf x,s)
=
\mathbb E_{\mathbf x}\left[e^{-sT}g(X_T)\right].
```

The backward first-exit equation is

**Equation TRISO-FPT-004**

```math
\mathcal Lu=su.
```

Substituting Eq. (TRISO-FPT-002) into Eq. (TRISO-FPT-004) gives:

**Equation TRISO-FPT-005**

```math
D\nabla^2u=su.
```

[ASSUMPTION] Under spherical symmetry, $u=u(r,s)$.

The radial Laplacian is

**Equation TRISO-FPT-006**

```math
\nabla^2u
=
\frac{1}{r^2}\frac{d}{dr}\left(r^2\frac{du}{dr}\right).
```

Substituting the radial Laplacian into Eq. (TRISO-FPT-005) gives:

**Equation TRISO-FPT-007**

```math
D\frac{1}{r^2}\frac{d}{dr}(r^2u')=su.
```

Apply the product rule:

**Equation TRISO-FPT-008**

```math
\frac{d}{dr}(r^2u')=2ru'+r^2u''.
```

Divide by $r^2$:

**Equation TRISO-FPT-009**

```math
\frac{1}{r^2}\frac{d}{dr}(r^2u')
=
u''+\frac{2}{r}u'.
```

Hence

**Equation TRISO-FPT-010**

```math
\boxed{
D\left(u''+\frac{2}{r}u'\right)=su.
}
```

This is the backward equation associated with the same diffusion operator as the forward continuum model.

## 21.2 Uniform-volume initial position

[ASSUMPTION] Production histories are initially uniform in kernel volume.

Let $U\sim\mathcal U(0,1)$.

The enclosed-volume fraction at radius $r$ is

**Equation TRISO-WOS-001**

```math
U
=
\frac{(4\pi/3)r^3}{(4\pi/3)R_1^3}.
```

Cancel $4\pi/3$:

**Equation TRISO-WOS-002**

```math
U=\frac{r^3}{R_1^3}.
```

Multiply by $R_1^3$:

**Equation TRISO-WOS-003**

```math
r^3=R_1^3U.
```

Take the real cube root:

**Equation TRISO-WOS-004**

```math
\boxed{r=R_1U^{1/3}.}
```

## 21.3 Production interface law

[INFERRED FROM CODE] For the frozen $K=1$ interface rule,

**Equation TRISO-WOS-005**

```math
\boxed{
p_{i\rightarrow j}
=
\frac{D_j}{D_i+D_j}.
}
```

The reflection probability is

**Equation TRISO-WOS-006**

```math
p_{i\rightarrow i}
=
1-p_{i\rightarrow j}.
```

Substituting Eq. (TRISO-WOS-005) into the reflection identity gives:

**Equation TRISO-WOS-007**

```math
p_{i\rightarrow i}
=
1-\frac{D_j}{D_i+D_j}.
```

Write the unit term over the same denominator:

**Equation TRISO-WOS-008**

```math
p_{i\rightarrow i}
=
\frac{D_i+D_j}{D_i+D_j}
-
\frac{D_j}{D_i+D_j}.
```

Subtract the numerators:

**Equation TRISO-WOS-009**

```math
\boxed{
p_{i\rightarrow i}
=
\frac{D_i}{D_i+D_j}.
}
```

[DEFINITION] The finite capture distance is $\epsilon$.

[DEFINITION] The reinsertion distance is

**Equation TRISO-WOS-010**

```math
\boxed{\delta=\alpha\epsilon.}
```

[INFERRED FROM CODE] The interface decision contributes zero physical time.

[IMPORTANT] These finite-capture semantics define Process A and remain distinct from the exact-interface process below.

## 21.4 Centred-ball first-passage transform

[EXACT] Consider a homogeneous ball of radius $b$ with absorbing boundary at $r=b$.

Define

**Equation TRISO-FPT-011**

```math
H(r,s)=\mathbb E_r[e^{-sT_b}].
```

The backward radial equation is

**Equation TRISO-FPT-012**

```math
D\left(H''+\frac{2}{r}H'\right)=sH.
```

Introduce

**Equation TRISO-FPT-013**

```math
v(r)=rH(r,s).
```

The same derivative cancellation used for the shell gives

**Equation TRISO-FPT-014**

```math
v''-\lambda^2v=0,
\qquad
\lambda=\sqrt{\frac{s}{D}}.
```

The general solution is

**Equation TRISO-FPT-015**

```math
v(r)=A\sinh(\lambda r)+B\cosh(\lambda r).
```

Regularity of $H=v/r$ at $r=0$ requires

**Equation TRISO-FPT-016**

```math
v(0)=0.
```

Substitute $r=0$ into TRISO-FPT-015:

**Equation TRISO-FPT-017**

```math
0=A\sinh0+B\cosh0.
```

Therefore

**Equation TRISO-FPT-018**

```math
B=0.
```

At the absorbing sphere,

**Equation TRISO-FPT-019**

```math
H(b,s)=1.
```

Hence

**Equation TRISO-FPT-019A**

```math
v(b)=b.
```

Using $v(b)=A\sinh(\lambda b)$,

**Equation TRISO-FPT-019B**

```math
A=\frac{b}{\sinh(\lambda b)}.
```

Therefore

**Equation TRISO-FPT-019C**

```math
v(r)=b\frac{\sinh(\lambda r)}{\sinh(\lambda b)}.
```

Divide by $r$:

**Equation TRISO-FPT-019D**

```math
\boxed{
H(r,s)=
\frac{b}{r}
\frac{\sinh(\lambda r)}{\sinh(\lambda b)}.
}
```

At the centre, use $\sinh(\lambda r)\sim\lambda r$:

**Equation TRISO-FPT-019E**

```math
H(0,s)
=
\frac{b\lambda}{\sinh(\lambda b)}.
```

Thus the centred-ball kernel used by the accelerated renewal is derived from the same backward diffusion equation, with regularity at the origin and absorption at the ball surface.

# 22. Exact spherical-shell first-passage theory

## 22.1 Outer-exit joint transform

[DEFINITION] Consider a homogeneous shell

**Equation TRISO-FPT-020**

```math
a<r<b.
```

Define the first exit time $T$ and the outer-exit joint transform

**Equation TRISO-FPT-021**

```math
G_b(r,s)
=
\mathbb E_r\left[e^{-sT}\mathbf 1_{\{R_T=b\}}\right].
```

TRISO-FPT-010 gives

**Equation TRISO-FPT-022**

```math
D\left(G_b''+\frac{2}{r}G_b'\right)=sG_b.
```

The outer-exit boundary conditions are

**Equation TRISO-FPT-023**

```math
G_b(a,s)=0,
```

**Equation TRISO-FPT-024**

```math
G_b(b,s)=1.
```

Introduce

**Equation TRISO-FPT-025**

```math
v(r)=rG_b(r,s).
```

Then

**Equation TRISO-FPT-026**

```math
G_b=\frac{v}{r}.
```

Differentiate:

**Equation TRISO-FPT-027**

```math
G_b'
=
\frac{v'}{r}
-
\frac{v}{r^2}.
```

Differentiate again:

**Equation TRISO-FPT-028**

```math
G_b''
=
\frac{v''}{r}
-
\frac{2v'}{r^2}
+
\frac{2v}{r^3}.
```

Also,

**Equation TRISO-FPT-029**

```math
\frac{2}{r}G_b'
=
\frac{2v'}{r^2}
-
\frac{2v}{r^3}.
```

Add TRISO-FPT-028 and TRISO-FPT-029:

**Equation TRISO-FPT-030**

```math
G_b''+\frac{2}{r}G_b'
=
\frac{v''}{r}.
```

Substituting the transformed derivative into Eq. (TRISO-FPT-022) gives:

**Equation TRISO-FPT-031**

```math
D\frac{v''}{r}
=
s\frac{v}{r}.
```

Multiply by $r$:

**Equation TRISO-FPT-032**

```math
Dv''=sv.
```

Divide by $D$:

**Equation TRISO-FPT-033**

```math
v''=\frac{s}{D}v.
```

Define

**Equation TRISO-FPT-034**

```math
\lambda=\sqrt{\frac{s}{D}}.
```

Then

**Equation TRISO-FPT-035**

```math
v''-\lambda^2v=0.
```

A convenient general solution measured from $a$ is

**Equation TRISO-FPT-036**

```math
v(r)
=
A\sinh[\lambda(r-a)]
+
B\cosh[\lambda(r-a)].
```

At $r=a$,

**Equation TRISO-FPT-037**

```math
v(a)=aG_b(a,s)=0.
```

Substitute $r=a$ into TRISO-FPT-036:

**Equation TRISO-FPT-038**

```math
0=A\sinh0+B\cosh0.
```

Use $sinh0=0$ and $cosh0=1$:

**Equation TRISO-FPT-039**

```math
B=0.
```

Thus

**Equation TRISO-FPT-040**

```math
v(r)=A\sinh[\lambda(r-a)].
```

At $r=b$,

**Equation TRISO-FPT-041**

```math
v(b)=bG_b(b,s)=b.
```

Substituting Eq. (TRISO-FPT-040) at $r=b$ gives:

**Equation TRISO-FPT-042**

```math
b=A\sinh[\lambda(b-a)].
```

Divide:

**Equation TRISO-FPT-043**

```math
A
=
\frac{b}{\sinh[\lambda(b-a)]}.
```

Insert this coefficient into TRISO-FPT-040:

**Equation TRISO-FPT-044**

```math
v(r)
=
b
\frac{\sinh[\lambda(r-a)]}
{\sinh[\lambda(b-a)]}.
```

Use $G_b=v/r$:

**Equation TRISO-FPT-045**

```math
\boxed{
G_b(r,s)
=
\frac{b}{r}
\frac{\sinh[\lambda(r-a)]}
{\sinh[\lambda(b-a)]}.
}
```

## 22.2 Inner-exit joint transform

Define

**Equation TRISO-FPT-046**

```math
G_a(r,s)
=
\mathbb E_r\left[e^{-sT}\mathbf 1_{\{R_T=a\}}\right].
```

The boundary values are

**Equation TRISO-FPT-047**

```math
G_a(a,s)=1,
```

**Equation TRISO-FPT-048**

```math
G_a(b,s)=0.
```

The same substitution $v=rG_a$ gives

**Equation TRISO-FPT-049**

```math
v''-\lambda^2v=0.
```

Choose a form that satisfies the zero condition at $b$:

**Equation TRISO-FPT-050**

```math
v(r)=C\sinh[\lambda(b-r)].
```

At $r=a$,

**Equation TRISO-FPT-051**

```math
v(a)=aG_a(a,s)=a.
```

Therefore

**Equation TRISO-FPT-052**

```math
a=C\sinh[\lambda(b-a)].
```

Divide:

**Equation TRISO-FPT-053**

```math
C
=
\frac{a}{\sinh[\lambda(b-a)]}.
```

Substitute:

**Equation TRISO-FPT-054**

```math
v(r)
=
a
\frac{\sinh[\lambda(b-r)]}
{\sinh[\lambda(b-a)]}.
```

Divide by $r$:

**Equation TRISO-FPT-055**

```math
\boxed{
G_a(r,s)
=
\frac{a}{r}
\frac{\sinh[\lambda(b-r)]}
{\sinh[\lambda(b-a)]}.
}
```

## 22.3 Exit probabilities and conditional time

Use

**Equation TRISO-FPT-056**

```math
\sinh z\sim z
\qquad(z\rightarrow0).
```

As $s\rightarrow0$, $\lambda\rightarrow0$.

Apply TRISO-FPT-056 to TRISO-FPT-045:

**Equation TRISO-FPT-057**

```math
G_b(r,0)
=
\frac br
\frac{\lambda(r-a)}
{\lambda(b-a)}.
```

Cancel $\lambda$:

**Equation TRISO-FPT-058**

```math
\boxed{
P_r(R_T=b)
=
\frac{b(r-a)}{r(b-a)}.
}
```

Likewise,

**Equation TRISO-FPT-059**

```math
G_a(r,0)
=
\frac ar
\frac{\lambda(b-r)}
{\lambda(b-a)}.
```

Cancel $\lambda$:

**Equation TRISO-FPT-060**

```math
\boxed{
P_r(R_T=a)
=
\frac{a(b-r)}{r(b-a)}.
}
```

The conditional outer-exit transform is

**Equation TRISO-FPT-061**

```math
\boxed{
\mathbb E_r[e^{-sT}\mid R_T=b]
=
\frac{G_b(r,s)}{G_b(r,0)}.
}
```

The conditional inner-exit transform is

**Equation TRISO-FPT-062**

```math
\boxed{
\mathbb E_r[e^{-sT}\mid R_T=a]
=
\frac{G_a(r,s)}{G_a(r,0)}.
}
```

Differentiate the outer joint transform:

**Equation TRISO-FPT-063**

```math
\frac{\partial G_b}{\partial s}
=
\mathbb E[-Te^{-sT}\mathbf1_b].
```

Set $s=0$:

**Equation TRISO-FPT-064**

```math
\left.\frac{\partial G_b}{\partial s}\right|_{s=0}
=
-\mathbb E[T\mathbf1_b].
```

Hence

**Equation TRISO-FPT-065**

```math
\boxed{
\mathbb E[T\mathbf1_b]
=
-
\left.\frac{\partial G_b}{\partial s}\right|_{s=0}.
}
```

[VERIFIED] The shell exit probabilities, conditional moments and conditional CDF were checked against analytical references and direct WOS before multilayer coupling.


# 23. Accelerated exact-interface renewal

[DEFINITION] Process B replaces repeated homogeneous-region WOS wandering by exact first-passage events while retaining the frozen stochastic interface law.

[IMPORTANT] Process A stops when the distance to an interface is at most $\epsilon$. Process B stops at the physical interface. The two stopping times are not identical.

## 23.1 Initial finite-capture mass

For uniform births in an inner sphere of radius $a$, the total sphere volume is

**Equation TRISO-WOS-020**

```math
V_a=\frac{4\pi}{3}a^3.
```

The volume inside radius $a-\epsilon$ is

**Equation TRISO-WOS-021**

```math
V_{a-\epsilon}
=
\frac{4\pi}{3}(a-\epsilon)^3.
```

The capture-shell volume is

**Equation TRISO-WOS-022**

```math
V_{\rm cap}
=
V_a-V_{a-\epsilon}.
```

The capture-shell probability is

**Equation TRISO-WOS-023**

```math
P_{\rm cap}
=
\frac{V_{\rm cap}}{V_a}.
```

Substituting Eqs. (TRISO-WOS-020)–(TRISO-WOS-022) gives:

**Equation TRISO-WOS-024**

```math
P_{\rm cap}
=
\frac{a^3-(a-\epsilon)^3}{a^3}.
```

Divide by $a^3$:

**Equation TRISO-WOS-025**

```math
\boxed{
P_{\rm cap}
=
1-\left(1-\frac{\epsilon}{a}\right)^3.
}
```

For $a=50\,\mu\mathrm m$ and $\epsilon=0.1\,\mu\mathrm m$,

**Equation TRISO-WOS-026**

```math
\frac{\epsilon}{a}=0.002.
```

Therefore

**Equation TRISO-WOS-027**

```math
P_{\rm cap}=1-(0.998)^3.
```

Numerically,

**Equation TRISO-WOS-028**

```math
\boxed{P_{\rm cap}=0.005988008.}
```

[EMPIRICAL COMPATIBILITY] The executed Process-A/Process-B controlled discrepancy is small at the declared finite-$\epsilon$ statistical precision, but it is not an identity.

## 23.2 Two-layer accelerated verification

[VERIFIED] The corrected two-layer accelerator uses the exact centred-ball kernel in the inner region and the exact shell kernel in the outer region.

The executed accelerated/FV RMS CDF difference is

**Equation TRISO-VER-200**

```math
\boxed{
\mathrm{RMS}(F_B-F_{FV})
=
0.002895.
}
```

The maximum absolute difference is

**Equation TRISO-VER-201**

```math
\boxed{
\max_t|F_B-F_{FV}|
=
0.005215.
}
```

The direct 100-nm WOS benchmark required approximately (1207.2) production steps per history.

The accelerated benchmark required approximately (139.04) renewal events per history.

The event-count ratio is

**Equation TRISO-VER-202**

```math
\frac{1207.2}{139.04}
=
8.68.
```

Thus the controlled event-count reduction is approximately

**Equation TRISO-VER-203**

```math
\boxed{8.68\times.}
```

# 24. Five-layer computational pathology

[VERIFIED] Direct production WOS produced 8/8 censored histories at $10^6$ steps per history in the targeted five-layer diagnostic.

[VERIFIED] The exact-shell accelerated five-layer diagnostic produced 0/16 releases and 16/16 capped histories at 100000 renewals per history.

The renewal counts by layer were

**Equation TRISO-WOS-030**

```math
(N_K,N_B,N_I,N_S,N_O)
=
(24,1599974,2,0,0).
```

The total number of renewals was

**Equation TRISO-WOS-031**

```math
N_{\rm tot}=1600000.
```

The Buffer renewal fraction is

**Equation TRISO-WOS-032**

```math
f_B
=
\frac{1599974}{1600000}.
```

Numerically,

**Equation TRISO-WOS-033**

```math
\boxed{
f_B=0.99998375.
}
```

[VERIFIED] Removing homogeneous-region wandering did not remove the computational pathology.

[INFERRED] The dominant remaining cost is repeated rare interface-state recurrence.

# 25. Interface-state Markov-renewal reduction

## 25.1 Eight transient states

There are four physical internal interfaces.

Each interface has two post-interface material sides.

Therefore

**Equation TRISO-MR-001**

```math
N_{\rm states}
=
4\times2.
```

Hence

**Equation TRISO-MR-002**

```math
\boxed{N_{\rm states}=8.}
```

Define

**Equation TRISO-MR-003**

```math
S_0=\text{Kernel side of Kernel/Buffer}.
```

**Equation TRISO-MR-004**

```math
S_1=\text{Buffer side of Kernel/Buffer}.
```

**Equation TRISO-MR-005**

```math
S_2=\text{Buffer side of Buffer/IPyC}.
```

**Equation TRISO-MR-006**

```math
S_3=\text{IPyC side of Buffer/IPyC}.
```

**Equation TRISO-MR-007**

```math
S_4=\text{IPyC side of IPyC/SiC}.
```

**Equation TRISO-MR-008**

```math
S_5=\text{SiC side of IPyC/SiC}.
```

**Equation TRISO-MR-009**

```math
S_6=\text{SiC side of SiC/OPyC}.
```

**Equation TRISO-MR-010**

```math
S_7=\text{OPyC side of SiC/OPyC}.
```

The exterior release state is absorbing.

## 25.2 First-step equation from the Kernel-side state

Define

**Equation TRISO-MR-011**

```math
\Phi_i(s)
=
\mathbb E_i[e^{-sT_{\rm rel}}].
```

From $S_0$, the exact centred-ball transform to the Kernel/Buffer interface is $H_K(s)$.

At that interface, reflection returns to $S_0$.

Transmission enters $S_1$.

Thus

**Equation TRISO-MR-012**

```math
\Phi_0
=
H_Kp_{K\rightarrow K}\Phi_0
+
H_Kp_{K\rightarrow B}\Phi_1.
```

Identify

**Equation TRISO-MR-013**

```math
K_{00}
=
H_Kp_{K\rightarrow K}.
```

Identify

**Equation TRISO-MR-014**

```math
K_{01}
=
H_Kp_{K\rightarrow B}.
```

All other row-0 entries are zero.

## 25.3 First-step equation from Buffer side of the inner interface

From $S_1$, the Buffer shell can first exit inward with transform $G_B^-$.

It can first exit outward with transform $G_B^+$.

If it exits inward, transmission enters Kernel state $S_0$.

If it exits inward and reflects, it returns to Buffer state $S_1$.

If it exits outward and reflects, it moves to Buffer state $S_2$.

If it exits outward and transmits, it enters IPyC state $S_3$.

Therefore

**Equation TRISO-MR-015**

```math
\Phi_1
=
G_B^-p_{B\rightarrow K}\Phi_0
+
G_B^-p_{B\rightarrow B}^{(I_0)}\Phi_1
+
G_B^+p_{B\rightarrow B}^{(I_1)}\Phi_2
+
G_B^+p_{B\rightarrow I}\Phi_3.
```

The first coefficient is

**Equation TRISO-MR-016**

```math
K_{10}
=
G_B^-p_{B\rightarrow K}.
```

The second is

**Equation TRISO-MR-017**

```math
K_{11}
=
G_B^-p_{B\rightarrow B}^{(I_0)}.
```

The third is

**Equation TRISO-MR-018**

```math
K_{12}
=
G_B^+p_{B\rightarrow B}^{(I_1)}.
```

The fourth is

**Equation TRISO-MR-019**

```math
K_{13}
=
G_B^+p_{B\rightarrow I}.
```

For $S_2$, the physical destinations are identical to $S_1$, but the shell transforms are evaluated from the outer-side Buffer reinsertion radius. Denote them $G_{B,2}^-$ and $G_{B,2}^+$.

**Equation TRISO-MR-034**

```math
\Phi_2=
G_{B,2}^-p_{B\rightarrow K}\Phi_0+
G_{B,2}^-p_{B\rightarrow B}^{(I_0)}\Phi_1+
G_{B,2}^+p_{B\rightarrow B}^{(I_1)}\Phi_2+
G_{B,2}^+p_{B\rightarrow I}\Phi_3.
```

Hence

**Equation TRISO-MR-035**

```math
K_{20}=G_{B,2}^-p_{B\rightarrow K},
\quad
K_{21}=G_{B,2}^-p_{B\rightarrow B}^{(I_0)}.
```

and

**Equation TRISO-MR-036**

```math
K_{22}=G_{B,2}^+p_{B\rightarrow B}^{(I_1)},
\quad
K_{23}=G_{B,2}^+p_{B\rightarrow I}.
```

## 25.4 IPyC state rows

From $S_3$, inward IPyC exit reaches the Buffer/IPyC interface and outward exit reaches IPyC/SiC:

**Equation TRISO-MR-037**

```math
\Phi_3=
G_{I,3}^-p_{I\rightarrow B}\Phi_2+
G_{I,3}^-p_{I\rightarrow I}^{(I_1)}\Phi_3+
G_{I,3}^+p_{I\rightarrow I}^{(I_2)}\Phi_4+
G_{I,3}^+p_{I\rightarrow S}\Phi_5.
```

Therefore

**Equation TRISO-MR-038**

```math
K_{32}=G_{I,3}^-p_{I\rightarrow B},
\quad
K_{33}=G_{I,3}^-p_{I\rightarrow I}^{(I_1)},
```

**Equation TRISO-MR-039**

```math
K_{34}=G_{I,3}^+p_{I\rightarrow I}^{(I_2)},
\quad
K_{35}=G_{I,3}^+p_{I\rightarrow S}.
```

From $S_4$,

**Equation TRISO-MR-040**

```math
\Phi_4=
G_{I,4}^-p_{I\rightarrow B}\Phi_2+
G_{I,4}^-p_{I\rightarrow I}^{(I_1)}\Phi_3+
G_{I,4}^+p_{I\rightarrow I}^{(I_2)}\Phi_4+
G_{I,4}^+p_{I\rightarrow S}\Phi_5.
```

Thus

**Equation TRISO-MR-041**

```math
K_{42}=G_{I,4}^-p_{I\rightarrow B},
\quad
K_{43}=G_{I,4}^-p_{I\rightarrow I}^{(I_1)},
```

**Equation TRISO-MR-042**

```math
K_{44}=G_{I,4}^+p_{I\rightarrow I}^{(I_2)},
\quad
K_{45}=G_{I,4}^+p_{I\rightarrow S}.
```

## 25.5 SiC state rows

From $S_5$,

**Equation TRISO-MR-043**

```math
\Phi_5=
G_{S,5}^-p_{S\rightarrow I}\Phi_4+
G_{S,5}^-p_{S\rightarrow S}^{(I_2)}\Phi_5+
G_{S,5}^+p_{S\rightarrow S}^{(I_3)}\Phi_6+
G_{S,5}^+p_{S\rightarrow O}\Phi_7.
```

Hence

**Equation TRISO-MR-044**

```math
K_{54}=G_{S,5}^-p_{S\rightarrow I},
\quad
K_{55}=G_{S,5}^-p_{S\rightarrow S}^{(I_2)},
```

**Equation TRISO-MR-045**

```math
K_{56}=G_{S,5}^+p_{S\rightarrow S}^{(I_3)},
\quad
K_{57}=G_{S,5}^+p_{S\rightarrow O}.
```

From $S_6$,

**Equation TRISO-MR-046**

```math
\Phi_6=
G_{S,6}^-p_{S\rightarrow I}\Phi_4+
G_{S,6}^-p_{S\rightarrow S}^{(I_2)}\Phi_5+
G_{S,6}^+p_{S\rightarrow S}^{(I_3)}\Phi_6+
G_{S,6}^+p_{S\rightarrow O}\Phi_7.
```

Therefore

**Equation TRISO-MR-047**

```math
K_{64}=G_{S,6}^-p_{S\rightarrow I},
\quad
K_{65}=G_{S,6}^-p_{S\rightarrow S}^{(I_2)},
```

**Equation TRISO-MR-048**

```math
K_{66}=G_{S,6}^+p_{S\rightarrow S}^{(I_3)},
\quad
K_{67}=G_{S,6}^+p_{S\rightarrow O}.
```

## 25.5 OPyC state and direct release

From $S_7$, inner shell exit reaches the SiC/OPyC interface.

Outer shell exit reaches the absorbing particle exterior.

The inward OPyC exit reaches the SiC/OPyC interface. Transmission to SiC gives $S_6$; reflection in OPyC gives $S_7$. The outward exit releases directly. Therefore

**Equation TRISO-MR-049**

```math
\Phi_7=
G_O^-p_{O\rightarrow S}\Phi_6+
G_O^-p_{O\rightarrow O}\Phi_7+
G_O^+.
```

The two transient entries are

**Equation TRISO-MR-050**

```math
K_{76}=G_O^-p_{O\rightarrow S},
\qquad
K_{77}=G_O^-p_{O\rightarrow O}.
```

The direct release transform is

**Equation TRISO-MR-020**

```math
\boxed{
B_7(s)=G_O^+(s).
}
```

For every other transient state,

**Equation TRISO-MR-021**

```math
B_i(s)=0,
\qquad i\ne7.
```

## 25.6 Matrix rearrangement without skipped algebra

Collect the eight first-step equations:

**Equation TRISO-MR-022**

```math
\boldsymbol\Phi
=
\mathbf K\boldsymbol\Phi
+
\mathbf B.
```

Subtract $\mathbf K\boldsymbol\Phi$ from both sides:

**Equation TRISO-MR-023**

```math
\boldsymbol\Phi
-
\mathbf K\boldsymbol\Phi
=
\mathbf B.
```

Write $\boldsymbol\Phi=\mathbf I\boldsymbol\Phi$:

**Equation TRISO-MR-024**

```math
\mathbf I\boldsymbol\Phi
-
\mathbf K\boldsymbol\Phi
=
\mathbf B.
```

Factor $\boldsymbol\Phi$:

**Equation TRISO-MR-025**

```math
(\mathbf I-\mathbf K)\boldsymbol\Phi
=
\mathbf B.
```

When $\mathbf I-\mathbf K$ is nonsingular, left-multiply by its inverse:

**Equation TRISO-MR-026**

```math
(\mathbf I-\mathbf K)^{-1}
(\mathbf I-\mathbf K)
\boldsymbol\Phi
=
(\mathbf I-\mathbf K)^{-1}\mathbf B.
```

Use the inverse identity:

**Equation TRISO-MR-027**

```math
\boxed{
\boldsymbol\Phi
=
(\mathbf I-\mathbf K)^{-1}\mathbf B.
}
```

## 25.7 Neumann-series path interpretation

If

**Equation TRISO-MR-028**

```math
\rho(\mathbf K)<1,
```

then

**Equation TRISO-MR-029**

```math
(\mathbf I-\mathbf K)^{-1}
=
\mathbf I
+
\mathbf K
+
\mathbf K^2
+
\mathbf K^3
+\cdots.
```

Multiply by $\mathbf B$:

**Equation TRISO-MR-030**

```math
\boldsymbol\Phi
=
\mathbf B
+
\mathbf K\mathbf B
+
\mathbf K^2\mathbf B
+
\mathbf K^3\mathbf B
+\cdots.
```

The term $\mathbf B$ is direct absorption without another transient renewal.

The term $\mathbf K\mathbf B$ is absorption after one transient renewal.

The term $\mathbf K^2\mathbf B$ is absorption after two transient renewals.

Thus the inverse sums arbitrarily long repeated interface-renewal paths without sampling each path individually.

## 25.8 Uniform-volume initial source

The kernel radial probability density is

**Equation TRISO-MR-031**

```math
f_R(r)
=
\frac{3r^2}{R_1^3},
\qquad
0\le r\le R_1.
```

The exact kernel first-exit transform from radius $r$ is $H_K(r,s)$.

After reaching Kernel/Buffer, reflection gives $S_0$ and transmission gives $S_1$.

Define

**Equation TRISO-MR-032**

```math
Q(s)
=
p_{K\rightarrow K}\Phi_0(s)
+
p_{K\rightarrow B}\Phi_1(s).
```

Average over the birth distribution:

**Equation TRISO-MR-033**

```math
\boxed{
\Phi_{\rm init}(s)
=
\int_0^{R_1}
\frac{3r^2}{R_1^3}
H_K(r,s)
Q(s)\,dr.
}
```

## 25.9 Process A, B and C distinction

[DEFINITION] Process A is finite-capture production WOS.

[DEFINITION] Process B is explicit accelerated exact-interface renewal.

[DEFINITION] Process C is the deterministic matrix reduction of Process B.

[EXACT TARGET] The mathematical reduction target is $B=C$.

[EMPIRICAL COMPATIBILITY] Process A and Process B/C have controlled finite-$\epsilon$ compatibility evidence.

[IMPORTANT] Neither statement is a proof of the $\epsilon\rightarrow0$ continuum limit.


# 26. Verification linked to the matrix derivation

## 26.1 Controlled two-layer B/C reconciliation

[VERIFIED] The explicit Process-B sample size was

**Equation TRISO-VER-300**

```math
N_B=20000.
```

At every predeclared positive transform point, define the standardized matrix discrepancy

**Equation TRISO-VER-301**

```math
z(s)=
\frac{\Phi_B(s)-\Phi_C(s)}
{\mathrm{SE}[\Phi_B(s)]}.
```

The executed maximum magnitude was

**Equation TRISO-VER-302**

```math
\boxed{
\max_s|z(s)|=1.004.
}
```

At zero transform frequency,

**Equation TRISO-VER-303**

```math
\boxed{
\Phi_{\rm init}(0)=1
}
```

to the predeclared numerical tolerance.

Thus the deterministic matrix is compatible with the explicit exact-interface renewal at the controlled Monte-Carlo precision.

## 26.2 Genuine three-layer multistate verification

The controlled radii were

**Equation TRISO-VER-304**

```math
(R_1,R_2,R_3)
=
(50,75,100)\,\mu\mathrm m.
```

The controlled diffusivities were

**Equation TRISO-VER-305**

```math
(D_1,D_2,D_3)
=
(1,2,5)\times10^{-9}\,\mathrm{m^2s^{-1}}.
```

The four-state Process-B/Process-C transform comparison gave

**Equation TRISO-VER-306**

```math
\boxed{
\max_s|z(s)|=0.729.
}
```

Against the independently refined FV reference,

**Equation TRISO-VER-307**

```math
\boxed{
\mathrm{RMS}(F_B-F_{FV})
=
1.0395\times10^{-3}.
}
```

The maximum absolute CDF discrepancy was

**Equation TRISO-VER-308**

```math
\boxed{
\max_t|F_B-F_{FV}|
=
1.7794\times10^{-3}.
}
```

[VERIFIED] The FV spatial/time refinement changes were smaller than the stochastic uncertainty, so the continuum-reference discretization error did not dominate this comparison.
