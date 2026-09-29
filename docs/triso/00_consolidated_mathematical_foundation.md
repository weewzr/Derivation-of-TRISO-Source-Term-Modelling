# 00 — Consolidated TRISO Mathematical Foundation

This is the single continuous derivation path for the Ray TRISO project. It preserves the user's original notebook as a separate evidence stream and incorporates the corrections established during the foundation audit.

The supplied LaTeX notebook contains 33 substantive displayed equations. Every one is mapped in 13_canonical_equation_register.md.

## 1. Physical problem

A TRISO particle has five concentric regions: fuel kernel, buffer, IPyC, SiC, and OPyC. A fission-product species is created in the kernel, diffuses through the layers, may decay or undergo other modelled reactions, and may leave the particle.

Let r be distance from the particle centre, t be time, c_i(r,t) be concentration in layer i, and D_i be the layer diffusivity.

[ASSUMPTION] One representative spherical particle.
[ASSUMPTION] Spherical symmetry.
[ASSUMPTION] Each layer is homogeneous during one model evaluation.
[ASSUMPTION] Initial ideal interfaces have no interfacial storage or resistance.
[QUESTION FOR SUPERVISOR] The first canonical species, decay/trapping model, partition coefficients, and coolant concentration still need to be fixed.

## 2. Conservation from a control volume

[EXACT] Begin with the amount of the conserved species inside an arbitrary fixed control volume V.

$$
N_V(t)=\int_V c(\mathbf{x},t)\,dV.
\tag{TRISO-GOV-020}
$$

The units are

$$
[N_V]=\mathrm{mol}.
\tag{TRISO-GOV-021}
$$

[EXACT] The rate of accumulation is

$$
\frac{dN_V}{dt}=\frac{d}{dt}\int_V c\,dV.
\tag{TRISO-GOV-022}
$$

Let \(\mathbf J\) be the diffusive molar flux vector. Its units are

$$
[\mathbf J]=\mathrm{mol\,m^{-2}\,s^{-1}}.
\tag{TRISO-GOV-023}
$$

Let \(S\) be the net volumetric production rate. Its units are

$$
[S]=\mathrm{mol\,m^{-3}\,s^{-1}}.
\tag{TRISO-GOV-024}
$$

[EXACT] For outward unit normal \(\mathbf n\), the outward amount crossing a boundary element in time \(dt\) is proportional to \(\mathbf J\cdot\mathbf n\). The outward rate is therefore

$$
\dot N_{\mathrm{out}}=\int_{\partial V}\mathbf J\cdot\mathbf n\,dA.
\tag{TRISO-GOV-025}
$$

[EXACT] The production rate inside the control volume is

$$
\dot N_{\mathrm{gen}}=\int_V S\,dV.
\tag{TRISO-GOV-026}
$$

[EXACT] Accumulation equals production minus outward flux:

$$
\frac{dN_V}{dt}=-\dot N_{\mathrm{out}}+\dot N_{\mathrm{gen}}.
\tag{TRISO-GOV-027}
$$

Substitute the definitions of the two rates:

$$
\frac{d}{dt}\int_V c\,dV=-\int_{\partial V}\mathbf J\cdot\mathbf n\,dA+\int_V S\,dV.
\tag{TRISO-GOV-028}
$$

[EXACT] Because \(V\) is fixed in space, the time derivative passes through the volume integral:

$$
\frac{d}{dt}\int_V c\,dV=\int_V\frac{\partial c}{\partial t}\,dV.
\tag{TRISO-GOV-029}
$$

[EXACT] Apply the divergence theorem to the surface term:

$$
\int_{\partial V}\mathbf J\cdot\mathbf n\,dA=\int_V\nabla\cdot\mathbf J\,dV.
\tag{TRISO-GOV-030}
$$

Substitute this result:

$$
\int_V\frac{\partial c}{\partial t}\,dV=-\int_V\nabla\cdot\mathbf J\,dV+\int_VS\,dV.
\tag{TRISO-GOV-031}
$$

Move the flux and source terms into one integrand:

$$
\int_V\left(\frac{\partial c}{\partial t}+\nabla\cdot\mathbf J-S\right)dV=0.
\tag{TRISO-GOV-032}
$$

[EXACT] Since the control volume is arbitrary, the integrand must vanish almost everywhere:

$$
\boxed{\frac{\partial c}{\partial t}+\nabla\cdot\mathbf J=S.}
\tag{TRISO-GOV-033}
$$


## 3. Constitutive law and general heterogeneous diffusion equation

[CONSTITUTIVE] Fickian diffusion relates flux to the concentration gradient:

$$
\boxed{\mathbf J=-D\nabla c.}
\tag{TRISO-GOV-034}
$$

The gradient has units

$$
[\nabla c]=\mathrm{mol\,m^{-4}}.
\tag{TRISO-GOV-035}
$$

Multiplying by \(D\) gives

$$
[D\nabla c]=\mathrm{m^2\,s^{-1}}\times\mathrm{mol\,m^{-4}}.
\tag{TRISO-GOV-036}
$$

Hence

$$
[D\nabla c]=\mathrm{mol\,m^{-2}\,s^{-1}},
\tag{TRISO-GOV-037}
$$
which matches the flux units.

[EXACT] Substitute Fick's law into conservation:

$$
\frac{\partial c}{\partial t}+\nabla\cdot(-D\nabla c)=S.
\tag{TRISO-GOV-038}
$$

[EXACT] Pull the minus sign through the divergence:

$$
\frac{\partial c}{\partial t}-\nabla\cdot(D\nabla c)=S.
\tag{TRISO-GOV-039}
$$

[EXACT] Rearrange:

$$
\boxed{\frac{\partial c}{\partial t}=\nabla\cdot(D\nabla c)+S.}
\tag{TRISO-GOV-040}
$$

[IMPORTANT] This is the general conservative form. The diffusivity must remain inside the divergence until a later layer-specific assumption establishes that it is constant with respect to the differentiated coordinate.


## 4. Spherical-coordinate derivation

[EXACT] In spherical coordinates, the gradient of a scalar field is

$$
\nabla c=\mathbf e_r\frac{\partial c}{\partial r}+\mathbf e_\theta\frac1r\frac{\partial c}{\partial\theta}+\mathbf e_\varphi\frac1{r\sin\theta}\frac{\partial c}{\partial\varphi}.
\tag{TRISO-SPH-020}
$$

[EXACT] Write a general vector flux as \(\mathbf J=J_r\mathbf e_r+J_\theta\mathbf e_\theta+J_\varphi\mathbf e_\varphi\).

[EXACT] Its spherical divergence is

$$
\nabla\cdot\mathbf J=\frac1{r^2}\frac{\partial}{\partial r}(r^2J_r)+\frac1{r\sin\theta}\frac{\partial}{\partial\theta}(\sin\theta J_\theta)+\frac1{r\sin\theta}\frac{\partial J_\varphi}{\partial\varphi}.
\tag{TRISO-SPH-021}
$$

[ASSUMPTION] Spherical symmetry means the concentration is independent of both angular coordinates:

$$
c=c(r,t).
\tag{TRISO-SPH-022}
$$

Therefore

$$
\frac{\partial c}{\partial\theta}=0.
\tag{TRISO-SPH-023}
$$

and

$$
\frac{\partial c}{\partial\varphi}=0.
\tag{TRISO-SPH-024}
$$

Substitute these zero angular derivatives into the gradient:

$$
\nabla c=\mathbf e_r\frac{\partial c}{\partial r}.
\tag{TRISO-SPH-025}
$$

[CONSTITUTIVE] Fick's law therefore becomes radial:

$$
\boxed{\mathbf J=-D(r,t)\frac{\partial c}{\partial r}\mathbf e_r.}
\tag{TRISO-SPH-026}
$$

Thus

$$
J_\theta=0.
\tag{TRISO-SPH-027}
$$

and

$$
J_\varphi=0.
\tag{TRISO-SPH-028}
$$

[EXACT] Insert the zero angular fluxes into the divergence:

$$
\nabla\cdot\mathbf J=\frac1{r^2}\frac{\partial}{\partial r}(r^2J_r).
\tag{TRISO-SPH-029}
$$

The radial flux component is

$$
J_r=-D(r,t)\frac{\partial c}{\partial r}.
\tag{TRISO-SPH-030}
$$

Substitution gives

$$
\nabla\cdot\mathbf J=\frac1{r^2}\frac{\partial}{\partial r}\left(-r^2D(r,t)\frac{\partial c}{\partial r}\right).
\tag{TRISO-SPH-031}
$$

Insert this into conservation:

$$
\frac{\partial c}{\partial t}+\frac1{r^2}\frac{\partial}{\partial r}\left(-r^2D(r,t)\frac{\partial c}{\partial r}\right)=S(r,t).
\tag{TRISO-SPH-032}
$$

[EXACT] Move the negative term to the right:

$$
\boxed{\frac{\partial c}{\partial t}=\frac1{r^2}\frac{\partial}{\partial r}\left(r^2D(r,t)\frac{\partial c}{\partial r}\right)+S(r,t).}
\tag{TRISO-SPH-033}
$$


## 4.1 Specialisation to one homogeneous layer

[ASSUMPTION] In one material layer \(i\), the benchmark assumes the diffusivity is constant with respect to radius and time during the analysis:

$$
D(r,t)=D_i.
\tag{TRISO-SPH-034}
$$

[EXACT] Substitute \(D_i\) into the conservative equation:

$$
\frac{\partial c_i}{\partial t}=\frac1{r^2}\frac{\partial}{\partial r}\left(r^2D_i\frac{\partial c_i}{\partial r}\right)+S_i.
\tag{TRISO-SPH-035}
$$

[EXACT] Because \(D_i\) is constant with respect to \(r\), take it outside the derivative:

$$
\frac{\partial c_i}{\partial t}=\frac{D_i}{r^2}\frac{\partial}{\partial r}\left(r^2\frac{\partial c_i}{\partial r}\right)+S_i.
\tag{TRISO-SPH-036}
$$

[EXACT] Apply the product rule:

$$
\frac{\partial}{\partial r}\left(r^2\frac{\partial c_i}{\partial r}\right)=\frac{\partial r^2}{\partial r}\frac{\partial c_i}{\partial r}+r^2\frac{\partial^2c_i}{\partial r^2}.
\tag{TRISO-SPH-037}
$$

Differentiate \(r^2\):

$$
\frac{\partial r^2}{\partial r}=2r.
\tag{TRISO-SPH-038}
$$

Substitute:

$$
\frac{\partial}{\partial r}\left(r^2\frac{\partial c_i}{\partial r}\right)=2r\frac{\partial c_i}{\partial r}+r^2\frac{\partial^2c_i}{\partial r^2}.
\tag{TRISO-SPH-039}
$$

Divide by \(r^2\):

$$
\frac1{r^2}\frac{\partial}{\partial r}\left(r^2\frac{\partial c_i}{\partial r}\right)=\frac2r\frac{\partial c_i}{\partial r}+\frac{\partial^2c_i}{\partial r^2}.
\tag{TRISO-SPH-040}
$$

Therefore:

$$
\boxed{\frac{\partial c_i}{\partial t}=D_i\left(\frac{\partial^2c_i}{\partial r^2}+\frac2r\frac{\partial c_i}{\partial r}\right)+S_i.}
\tag{TRISO-SPH-041}
$$

[IMPORTANT] Equation TRISO-SPH-041 is a within-layer constant-diffusivity equation. It must not be differentiated through a discontinuous material interface.

## 5. Source and reaction terms

The conservation equation contains a net volumetric source term. We now separate the physical processes that may contribute to that term.

[EXACT] Define the net source in material layer i as

$$
S_i=S_{i,\mathrm{gen}}+S_{i,\mathrm{other}}+S_{i,\mathrm{release}}-S_{i,\mathrm{decay}}-S_{i,\mathrm{trap}}.
\tag{TRISO-GOV-100}
$$

Each term has units

$$
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
\tag{TRISO-GOV-101}
$$

This equation is a bookkeeping definition. It does not yet choose a constitutive model for trapping, release, or generation.

### 5.1 Fission-product generation

Let (S_{i,\mathrm{gen}}) denote the local rate at which the tracked species is created.

[ASSUMPTION] For the simplest TRISO source benchmark, generation is confined to the fuel kernel:

$$
S_{i,\mathrm{gen}}=
\begin{cases}
S_0,&0\le r<r_1,\\
0,&r_1<r<R.
\end{cases}
\tag{TRISO-GOV-102}
$$

Here

$$
[S_0]=\mathrm{mol\,m^{-3}\,s^{-1}}.
\tag{TRISO-GOV-103}
$$

A microscopic fission-based expression such as a fission rate multiplied by a product yield would require a selected species, fission cross section, neutron flux, and yield correlation. Those inputs are not fixed by the current project evidence.

[SOURCE NEEDED] Species-specific generation correlation and parameter values.

### 5.2 Radioactive decay

Suppose the tracked atoms disappear by radioactive decay independently with a constant decay probability per unit time.

Let (lambda_d) be the decay constant:

$$
[\lambda_d]=\mathrm{s^{-1}}.
\tag{TRISO-GOV-104}
$$

Consider an amount (N) of the tracked species.

During a short time interval (dt), the expected fraction that decays is proportional to (\lambda_d dt):

$$
dN_{\mathrm{decay}}=\lambda_d N\,dt.
\tag{TRISO-GOV-105}
$$

Because decay removes atoms from the tracked species, the change in tracked inventory is negative:

$$
dN=-dN_{\mathrm{decay}}.
\tag{TRISO-GOV-106}
$$

Substitute the decay amount:

$$
dN=-\lambda_dN\,dt.
\tag{TRISO-GOV-107}
$$

Divide by (dt):

$$
\frac{dN}{dt}=-\lambda_dN.
\tag{TRISO-GOV-108}
$$

For a fixed volume element, (N=c\,dV). Therefore

$$
\frac{d(c\,dV)}{dt}=-\lambda_dc\,dV.
\tag{TRISO-GOV-109}
$$

For a fixed volume element, (dV) is constant in time:

$$
\frac{\partial c}{\partial t}=-\lambda_dc.
\tag{TRISO-GOV-110}
$$

Thus the local decay sink has the form

$$
\boxed{
S_{i,\mathrm{decay}}=\lambda_{d,i}c_i
}
\tag{TRISO-GOV-111}
$$

and it enters the conservation equation with a minus sign.

Dimensional check:

$$
[\lambda_dc]
=
\mathrm{s^{-1}}\times\mathrm{mol\,m^{-3}}
=
\mathrm{mol\,m^{-3}\,s^{-1}}.
\tag{TRISO-GOV-112}
$$

[CONSTITUTIVE] The first-order decay assumption is the mathematical statement that each tracked atom has the same constant decay hazard (lambda_d), independent of concentration.

[ASSUMPTION] The current base benchmark sets this decay contribution to zero:

$$
S_{i,\mathrm{decay}}=0.
\tag{TRISO-GOV-113}
$$

### 5.3 Trapping and release from traps

Trapping is a transfer of the tracked species from the mobile population into a trapped population. Release is the reverse transfer.

Define

$$
S_{i,\mathrm{trap}}
\tag{TRISO-GOV-114}
$$

as the positive rate at which mobile species are removed into traps, and

$$
S_{i,\mathrm{release}}
\tag{TRISO-GOV-115}
$$

as the positive rate at which trapped species are returned to the mobile population.

Their units are

$$
[S_{i,\mathrm{trap}}]
=
[S_{i,\mathrm{release}}]
=
\mathrm{mol\,m^{-3}\,s^{-1}}.
\tag{TRISO-GOV-116}
$$

The mobile-species equation therefore contains

$$
S_{i,\mathrm{release}}-S_{i,\mathrm{trap}}.
\tag{TRISO-GOV-117}
$$

No first-order, saturation, occupancy, or irradiation-dependent trapping law is assumed here.

[QUESTION FOR SUPERVISOR] Select the physical trapping/release constitutive model, if trapping is required in the final species model.

### 5.4 General reaction-inclusive equation

Substitute the separated source terms into the conservation equation:

$$
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
\tag{TRISO-GOV-118}
$$

This is the general layer-wise source/reaction form under spherical symmetry.

[QUESTION FOR SUPERVISOR] Confirm whether the final physical model should contain decay and/or trapping in addition to diffusion.

### 5.5 Two source problems must remain distinct

There are two different physical initial-value problems in this project.

**Problem A: initially empty particle with continuing generation**

The initial inventory is zero:

$$
c_i(r,0)=0.
\tag{TRISO-IC-100}
$$

The kernel generation remains active for (t>0):

$$
S_{1,\mathrm{gen}}=S_0.
\tag{TRISO-IC-101}
$$

This is the source-driven problem used by the homogeneous Part-I steady analytical benchmark.

**Problem B: initially loaded kernel with no continuing generation**

The kernel initially contains mobile species:

$$
c_1(r,0)=c_0
\qquad
0\le r<r_1,
\tag{TRISO-IC-102}
$$

while the coatings initially contain none:

$$
c_i(r,0)=0
\qquad
r_1<r<R.
\tag{TRISO-IC-103}
$$

After (t=0), the benchmark source is zero:

$$
S_{i,\mathrm{gen}}=0.
\tag{TRISO-IC-104}
$$

This is the initial-inventory release problem used by the production WOS verification contract.

These problems have different source terms and different physical histories. They must not be merged into one equation by notation alone.

## 6. Initial, centre, interface and outer conditions

### 6.1 Centre condition from spherical symmetry

At the exact centre there is no preferred radial direction.

[ASSUMPTION] Extend the radial concentration profile evenly through the mathematical origin for the purpose of the local limit:

$$
c(-r,t)=c(r,t).
\tag{TRISO-BC-100}
$$

Differentiate this relation with respect to (r):

$$
-c_r(-r,t)=c_r(r,t).
\tag{TRISO-BC-101}
$$

Set (r=0):

$$
-c_r(0,t)=c_r(0,t).
\tag{TRISO-BC-102}
$$

Therefore

$$
\boxed{c_r(0,t)=0.}
\tag{TRISO-BC-103}
$$

This is the mathematical expression of spherical symmetry at the centre.

It also agrees with the physical interpretation: a nonzero radial derivative at the centre would select one direction as different from the opposite direction.

### 6.2 Apparent singularity at the origin

The spherical operator contains

$$
\frac{2}{r}c_r.
\tag{TRISO-BC-104}
$$

At (r=0), this expression is of the form (0/0) for a smooth symmetric field, so it must not be evaluated by direct substitution.

Because (c_r(0,t)=0), apply L'Hôpital's rule:

$$
\lim_{r\to0}\frac{c_r(r,t)}{r}
=
\lim_{r\to0}\frac{c_{rr}(r,t)}{1}.
\tag{TRISO-BC-105}
$$

Therefore

$$
\lim_{r\to0}\frac{c_r(r,t)}{r}
=
c_{rr}(0,t).
\tag{TRISO-BC-106}
$$

Multiply by 2:

$$
\lim_{r\to0}\frac{2}{r}c_r(r,t)
=
2c_{rr}(0,t).
\tag{TRISO-BC-107}
$$

The full spherical diffusion operator therefore has the centre limit

$$
\lim_{r\to0}
\left(
c_{rr}+\frac2r c_r
\right)
=
c_{rr}(0,t)+2c_{rr}(0,t).
\tag{TRISO-BC-108}
$$

Collect the two identical curvature contributions:

$$
\boxed{
\lim_{r\to0}
\left(
c_{rr}+\frac2r c_r
\right)
=
3c_{rr}(0,t).
}
\tag{TRISO-BC-109}
$$

This is the continuum origin of the factor 3 that later becomes the factor 6 in the central FTCS stencil.

[ASSUMPTION] The limit requires sufficient smoothness of the radial field near the origin.

### 6.3 Material-interface conservation

Consider an infinitesimally thin spherical control volume surrounding interface (r=r_k).

Let its inner radius be (r_k-\varepsilon) and its outer radius be (r_k+\varepsilon).

The volume is

$$
V_\varepsilon
=
\frac{4\pi}{3}
\left[
(r_k+\varepsilon)^3-(r_k-\varepsilon)^3
\right].
\tag{TRISO-INT-100}
$$

As (\varepsilon\to0),

$$
V_\varepsilon\to0.
\tag{TRISO-INT-101}
$$

The inward diffusive amount rate crossing the inner surface is

$$
4\pi(r_k-\varepsilon)^2J_{r,k}^{-}.
\tag{TRISO-INT-102}
$$

The outward diffusive amount rate crossing the outer surface is

$$
4\pi(r_k+\varepsilon)^2J_{r,k}^{+}.
\tag{TRISO-INT-103}
$$

Let (\Gamma_k) be any explicitly modelled interfacial inventory per unit area, and let (g_k) be any explicitly modelled interfacial production rate per unit area.

Their units are

$$
[\Gamma_k]=\mathrm{mol\,m^{-2}},
\qquad
[g_k]=\mathrm{mol\,m^{-2}\,s^{-1}}.
\tag{TRISO-INT-104}
$$

The interface balance is

$$
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
\tag{TRISO-INT-105}
$$

Divide by (4\pi r_k^2):

$$
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
\tag{TRISO-INT-106}
$$

Take the zero-thickness limit:

$$
\frac{d\Gamma_k}{dt}
=
J_{r,k}^{-}
-
J_{r,k}^{+}
+
g_k.
\tag{TRISO-INT-107}
$$

Therefore the general interface jump condition is

$$
\boxed{
J_{r,k}^{-}-J_{r,k}^{+}
=
\frac{d\Gamma_k}{dt}-g_k.
}
\tag{TRISO-INT-108}
$$

For an ideal interface with no interfacial storage and no interfacial generation,

$$
\Gamma_k=0,
\qquad
g_k=0.
\tag{TRISO-INT-109}
$$

Hence

$$
J_{r,k}^{-}=J_{r,k}^{+}.
\tag{TRISO-INT-110}
$$

Thus

$$
\boxed{
J_{r,k}^{-}=J_{r,k}^{+}.
}
\tag{TRISO-INT-111}
$$

Now substitute Fick's law on the inner side:

$$
J_{r,k}^{-}
=
-D_k
\frac{\partial c_k}{\partial r}
\bigg|_{r_k^-}.
\tag{TRISO-INT-112}
$$

Substitute Fick's law on the outer side:

$$
J_{r,k}^{+}
=
-D_{k+1}
\frac{\partial c_{k+1}}{\partial r}
\bigg|_{r_k^+}.
\tag{TRISO-INT-113}
$$

Equate them:

$$
\boxed{
-D_k
\frac{\partial c_k}{\partial r}
\bigg|_{r_k^-}
=
-D_{k+1}
\frac{\partial c_{k+1}}{\partial r}
\bigg|_{r_k^+}.
}
\tag{TRISO-INT-114}
$$

This condition came from conservation. It did not require concentration continuity.

### 6.4 Concentration continuity is a separate interface assumption

Flux continuity answers the question:

> Is species amount conserved across the zero-thickness interface?

It does not answer:

> What equilibrium relation connects the two concentrations at the interface?

For an ideal perfectly equilibrated interface, one may impose concentration continuity:

$$
\boxed{
c_k(r_k,t)=c_{k+1}(r_k,t).
}
\tag{TRISO-INT-115}
$$

[ASSUMPTION] This is an ideal-interface constitutive/equilibrium assumption.

A species-specific partition coefficient (K_k) instead gives a different relation:

$$
\boxed{
c_{k+1}(r_k,t)=K_kc_k(r_k,t).
}
\tag{TRISO-INT-116}
$$

The value and definition of (K_k) depend on the species and the two materials.

[SOURCE NEEDED] Species-specific partition/solubility data if (K_k\ne1) is required physically.

### 6.5 Interfacial resistance is a third, distinct model

Partitioning and interfacial resistance are not the same statement.

A finite interfacial mass-transfer coefficient (h_{\mathrm{int}}) can instead be used in a constitutive resistance law such as

$$
J_{r,k}
=
h_{\mathrm{int}}
\left(
c_k-\frac{c_{k+1}}{K_k}
\right).
\tag{TRISO-INT-117}
$$

The units are

$$
[h_{\mathrm{int}}]=\mathrm{m\,s^{-1}}.
\tag{TRISO-INT-118}
$$

This relation introduces a finite concentration jump for finite resistance.

[SOURCE NEEDED] The physical interfacial-resistance law and coefficient, if such resistance is required.

[ASSUMPTION] The frozen numerical benchmark uses (K_k=1) and no explicit interfacial resistance, so (TRISO-INT-115) and (TRISO-INT-117) are not simultaneously imposed.

### 6.6 Outer boundary conditions

At the outer surface (r=R), define the outward radial flux as

$$
J_R=J_r(R,t).
\tag{TRISO-BC-110}
$$

By Fick's law,

$$
J_R
=
-D_5
\frac{\partial c_5}{\partial r}
\bigg|_{R}.
\tag{TRISO-BC-111}
$$

#### Dirichlet: absorbing or prescribed surface concentration

The simplest absorbing boundary is

$$
\boxed{
c_5(R,t)=0.
}
\tag{TRISO-BC-112}
$$

More generally, a prescribed surface concentration (c_b(t)) is

$$
c_5(R,t)=c_b(t).
\tag{TRISO-BC-113}
$$

The absorbing case is (c_b=0).

[ASSUMPTION] The production WOS verification benchmark uses the absorbing case.

#### Neumann: prescribed outward flux

A prescribed outward flux is written

$$
\boxed{
J_R=J_b(t).
}
\tag{TRISO-BC-114}
$$

Substitute the diffusive flux:

$$
-D_5
\frac{\partial c_5}{\partial r}
\bigg|_R
=
J_b(t).
\tag{TRISO-BC-115}
$$

The units are

$$
[J_b]=\mathrm{mol\,m^{-2}\,s^{-1}}.
\tag{TRISO-BC-116}
$$

#### Robin: finite external mass transfer

Let the external coolant concentration be (c_\infty(t)).

[CONSTITUTIVE] A linear external mass-transfer law is

$$
J_R
=
h
\left[
c_5(R,t)-c_\infty(t)
\right].
\tag{TRISO-BC-117}
$$

The units of (h) are

$$
[h]=\mathrm{m\,s^{-1}}.
\tag{TRISO-BC-118}
$$

Substitute the diffusive surface flux:

$$
-D_5
\frac{\partial c_5}{\partial r}
\bigg|_R
=
h
\left[
c_5(R,t)-c_\infty(t)
\right].
\tag{TRISO-BC-119}
$$

Both sides have units

$$
\mathrm{mol\,m^{-2}\,s^{-1}}.
\tag{TRISO-BC-120}
$$

For zero bulk concentration,

$$
c_\infty=0,
\tag{TRISO-BC-121}
$$

so

$$
-D_5c_5'(R,t)=hc_5(R,t).
\tag{TRISO-BC-122}
$$

For very large (h), a finite flux requires

$$
c_5(R,t)-c_\infty(t)\to0.
\tag{TRISO-BC-123}
$$

Thus the Robin condition approaches the Dirichlet condition

$$
c_5(R,t)=c_\infty(t).
\tag{TRISO-BC-124}
$$

For (h\to0),

$$
J_R\to0,
\tag{TRISO-BC-125}
$$

which approaches the zero-flux Neumann condition.

The three outer-boundary models are therefore mathematically distinct:

$$
\text{Dirichlet: concentration prescribed},
$$

$$
\text{Neumann: flux prescribed},
$$

$$
\text{Robin: flux responds to concentration difference}.
$$

[ASSUMPTION] The Part-I analytical benchmark uses Robin with (c_\infty=0).

[ASSUMPTION] The production WOS verification benchmark uses absorbing Dirichlet (c_5(R,t)=0).

### 6.7 Initial-condition summary

The initial condition must be selected together with the source model.

Problem A:

$$
c_i(r,0)=0,
\qquad
S_{1,\mathrm{gen}}=S_0.
\tag{TRISO-IC-105}
$$

Problem B:

$$
c_1(r,0)=c_0,
\qquad
c_{2..5}(r,0)=0,
\qquad
S_{i,\mathrm{gen}}=0.
\tag{TRISO-IC-106}
$$

The first homogeneous analytical Part-I benchmark belongs to Problem A.

The production WOS release benchmark belongs to Problem B.

## 7. Part I homogeneous analytical benchmark

The following solution belongs to **Problem A**: an initially empty homogeneous sphere with a continuing uniform source and a finite-transfer Robin boundary.

[ASSUMPTION] Replace the five-layer particle temporarily by one homogeneous sphere of radius (R) and constant diffusivity (D).

The governing equation is

$$
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
\tag{TRISO-ANA-100}
$$

At steady state the concentration no longer changes with time:

$$
\frac{\partial w}{\partial t}=0.
\tag{TRISO-ANA-101}
$$

Therefore

$$
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
\tag{TRISO-ANA-102}
$$

Move the source term to the other side:

$$
D
\left(
w''
+
\frac2r w'
\right)
=
-S_0.
\tag{TRISO-ANA-103}
$$

Divide by (D):

$$
w''
+
\frac2r w'
=
-\frac{S_0}{D}.
\tag{TRISO-ANA-104}
$$

Multiply by (r^2):

$$
r^2w''
+
2rw'
=
-\frac{S_0}{D}r^2.
\tag{TRISO-ANA-105}
$$

Recognise the product derivative on the left. Verify it explicitly using the product rule:

$$
\frac{d}{dr}(r^2w')
=
\frac{d r^2}{dr}w'
+
r^2w''.
\tag{TRISO-ANA-106}
$$

Differentiate (r^2):

$$
\frac{d r^2}{dr}=2r.
\tag{TRISO-ANA-107}
$$

Therefore

$$
\frac{d}{dr}(r^2w')
=
2rw'+r^2w''.
\tag{TRISO-ANA-108}
$$

Hence the steady equation becomes

$$
\frac{d}{dr}(r^2w')
=
-\frac{S_0}{D}r^2.
\tag{TRISO-ANA-109}
$$

Integrate both sides with respect to (r):

$$
\int
\frac{d}{dr}(r^2w')\,dr
=
-\frac{S_0}{D}
\int r^2\,dr.
\tag{TRISO-ANA-110}
$$

The left-hand integral is

$$
r^2w'.
\tag{TRISO-ANA-111}
$$

The right-hand integral is

$$
-\frac{S_0}{D}\frac{r^3}{3}.
\tag{TRISO-ANA-112}
$$

Introduce the integration constant (A):

$$
r^2w'
=
-\frac{S_0r^3}{3D}
+
A.
\tag{TRISO-ANA-113}
$$

At the centre, regularity requires (w'(0)) to remain finite.

If (A\ne0), then division by (r^2) gives a term proportional to (1/r^2), which diverges.

Therefore

$$
A=0.
\tag{TRISO-ANA-114}
$$

Substitute (A=0):

$$
r^2w'
=
-\frac{S_0r^3}{3D}.
\tag{TRISO-ANA-115}
$$

For (r>0), divide by (r^2):

$$
w'
=
-\frac{S_0r}{3D}.
\tag{TRISO-ANA-116}
$$

Integrate again:

$$
\int dw
=
-\frac{S_0}{3D}\int r\,dr.
\tag{TRISO-ANA-117}
$$

The left-hand integral is

$$
w.
\tag{TRISO-ANA-118}
$$

The right-hand integral is

$$
-\frac{S_0}{3D}\frac{r^2}{2}.
\tag{TRISO-ANA-119}
$$

Introduce the second integration constant (B):

$$
w
=
B
-
\frac{S_0r^2}{6D}.
\tag{TRISO-ANA-120}
$$

Now apply the Robin boundary condition at (r=R):

$$
-Dw'(R)=hw(R)
\tag{TRISO-ANA-121}
$$

because (c_\infty=0) for this benchmark.

Evaluate the derivative at (R):

$$
w'(R)
=
-\frac{S_0R}{3D}.
\tag{TRISO-ANA-122}
$$

Multiply by (-D):

$$
-Dw'(R)
=
\frac{S_0R}{3}.
\tag{TRISO-ANA-123}
$$

Evaluate the concentration at (R):

$$
w(R)
=
B
-
\frac{S_0R^2}{6D}.
\tag{TRISO-ANA-124}
$$

Substitute both expressions into the Robin condition:

$$
\frac{S_0R}{3}
=
h
\left(
B
-
\frac{S_0R^2}{6D}
\right).
\tag{TRISO-ANA-125}
$$

Divide by (h):

$$
\frac{S_0R}{3h}
=
B
-
\frac{S_0R^2}{6D}.
\tag{TRISO-ANA-126}
$$

Add the quadratic term to both sides:

$$
B
=
\frac{S_0R}{3h}
+
\frac{S_0R^2}{6D}.
\tag{TRISO-ANA-127}
$$

Substitute (B) into the profile:

$$
w(r)
=
\frac{S_0R}{3h}
+
\frac{S_0R^2}{6D}
-
\frac{S_0r^2}{6D}.
\tag{TRISO-ANA-128}
$$

Collect the two quadratic terms:

$$
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
\tag{TRISO-ANA-129}
$$

### 7.1 Dimensional check

The first term has units

$$
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
\tag{TRISO-ANA-130}
$$

The second term has units

$$
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
\tag{TRISO-ANA-131}
$$

Both terms therefore have concentration units.

### 7.2 Independent global generation/release balance

The total generation rate in the homogeneous sphere is

$$
\dot N_{\mathrm{gen}}
=
\int_0^R
S_0\,4\pi r^2\,dr.
\tag{TRISO-ANA-132}
$$

Pull out constants:

$$
\dot N_{\mathrm{gen}}
=
4\pi S_0
\int_0^Rr^2\,dr.
\tag{TRISO-ANA-133}
$$

Evaluate the integral:

$$
\int_0^Rr^2\,dr
=
\frac{R^3}{3}.
\tag{TRISO-ANA-134}
$$

Therefore

$$
\boxed{
\dot N_{\mathrm{gen}}
=
\frac{4\pi R^3S_0}{3}.
}
\tag{TRISO-ANA-135}
$$

At steady state, this must equal the outward surface release rate:

$$
\dot N_{\mathrm{out}}
=
4\pi R^2
\left[
h w(R)
\right].
\tag{TRISO-ANA-136}
$$

Set generation equal to release:

$$
\frac{4\pi R^3S_0}{3}
=
4\pi R^2h w(R).
\tag{TRISO-ANA-137}
$$

Cancel (4\pi R^2):

$$
\frac{S_0R}{3}
=
h w(R).
\tag{TRISO-ANA-138}
$$

Divide by (h):

$$
w(R)=\frac{S_0R}{3h}.
\tag{TRISO-ANA-139}
$$

Evaluate the analytical profile at (r=R):

$$
w(R)
=
\frac{S_0R}{3h}
+
\frac{S_0}{6D}(R^2-R^2).
\tag{TRISO-ANA-140}
$$

The second term is zero:

$$
w(R)
=
\frac{S_0R}{3h}.
\tag{TRISO-ANA-141}
$$

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

The steady solution (w(r)) has already been obtained. We now remove the steady part so that the remaining transient problem has no source term.

### 8.1 Define the transient deviation

Define

$$
v(r,t)=c(r,t)-w(r).
\tag{TRISO-ANA-200}
$$

Rearrange this definition:

$$
c(r,t)=v(r,t)+w(r).
\tag{TRISO-ANA-201}
$$

Because (w) is a steady solution, it does not depend on time:

$$
\frac{\partial w}{\partial t}=0.
\tag{TRISO-ANA-202}
$$

Differentiate (c=v+w) with respect to time:

$$
\frac{\partial c}{\partial t}
=
\frac{\partial v}{\partial t}
+
\frac{\partial w}{\partial t}.
\tag{TRISO-ANA-203}
$$

Substitute (TRISO-ANA-202):

$$
\boxed{
\frac{\partial c}{\partial t}
=
\frac{\partial v}{\partial t}.
}
\tag{TRISO-ANA-204}
$$

Differentiate (c=v+w) with respect to radius:

$$
\frac{\partial c}{\partial r}
=
\frac{\partial v}{\partial r}
+
\frac{dw}{dr}.
\tag{TRISO-ANA-205}
$$

Differentiate once more:

$$
\frac{\partial^2 c}{\partial r^2}
=
\frac{\partial^2v}{\partial r^2}
+
\frac{d^2w}{dr^2}.
\tag{TRISO-ANA-206}
$$

Start from the full homogeneous source-driven PDE:

$$
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
\tag{TRISO-ANA-207}
$$

Substitute the time derivative from (TRISO-ANA-204):

$$
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
\tag{TRISO-ANA-208}
$$

Substitute the second spatial derivative from (TRISO-ANA-206):

$$
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
\tag{TRISO-ANA-209}
$$

Substitute the first spatial derivative from (TRISO-ANA-205):

$$
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
\tag{TRISO-ANA-210}
$$

Distribute the factor (D):

$$
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
\tag{TRISO-ANA-211}
$$

Rearrange the terms into transient and steady groups:

$$
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
\tag{TRISO-ANA-212}
$$

The steady solution satisfies

$$
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
\tag{TRISO-ANA-213}
$$

Substitute (TRISO-ANA-213) into (TRISO-ANA-212):

$$
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
\tag{TRISO-ANA-214}
$$

The source has disappeared because the steady part (w) already accounts for the long-time source balance.

### 8.2 Transform the centre condition

The original centre condition is

$$
\frac{\partial c}{\partial r}(0,t)=0.
\tag{TRISO-ANA-215}
$$

Substitute (TRISO-ANA-205):

$$
\frac{\partial v}{\partial r}(0,t)+w'(0)=0.
\tag{TRISO-ANA-216}
$$

The steady solution has

$$
w'(r)=-\frac{S_0r}{3D}.
\tag{TRISO-ANA-217}
$$

Evaluate it at the centre:

$$
w'(0)=0.
\tag{TRISO-ANA-218}
$$

Therefore

$$
\boxed{
\frac{\partial v}{\partial r}(0,t)=0.
}
\tag{TRISO-ANA-219}
$$

### 8.3 Transform the outer Robin condition

The original Robin condition is

$$
-Dc_r(R,t)=hw(R,t).
\tag{TRISO-ANA-220}
$$

Substitute (c=v+w):

$$
-D
\left[
v_r(R,t)+w'(R)
\right]
=
h
\left[
v(R,t)+w(R)
\right].
\tag{TRISO-ANA-221}
$$

Rearrange the transient and steady terms:

$$
-Dv_r(R,t)-hw(R)
=
hv(R,t)+Dw'(R).
\tag{TRISO-ANA-222}
$$

The steady solution satisfies

$$
-Dw'(R)=hw(R).
\tag{TRISO-ANA-223}
$$

Therefore

$$
-Dv_r(R,t)=hv(R,t).
\tag{TRISO-ANA-224}
$$

Hence the transient Robin condition is

$$
\boxed{
-Dv_r(R,t)=hv(R,t).
}
\tag{TRISO-ANA-225}
$$

### 8.4 Transform the initial condition

The original initial condition for Problem A is

$$
c(r,0)=0.
\tag{TRISO-ANA-226}
$$

Apply the definition (v=c-w):

$$
v(r,0)=c(r,0)-w(r).
\tag{TRISO-ANA-227}
$$

Substitute (c(r,0)=0):

$$
\boxed{
v(r,0)=-w(r).
}
\tag{TRISO-ANA-228}
$$

Thus the complete transient deviation problem is

$$
v_t
=
D
\left(
v_{rr}+\frac2r v_r
\right),
$$

with

$$
v_r(0,t)=0,
$$

$$
-Dv_r(R,t)=hv(R,t),
$$

and

$$
v(r,0)=-w(r).
$$

## 9. Separation of variables

### 9.1 Assume a separated solution

Seek a non-zero transient mode in the form

$$
v(r,t)=\phi(r)T(t).
\tag{TRISO-ANA-229}
$$

Differentiate with respect to time:

$$
v_t=\phi(r)T'(t).
\tag{TRISO-ANA-230}
$$

Differentiate with respect to radius:

$$
v_r=\phi'(r)T(t).
\tag{TRISO-ANA-231}
$$

Differentiate once more:

$$
v_{rr}=\phi''(r)T(t).
\tag{TRISO-ANA-232}
$$

Substitute these three expressions into the transient PDE:

$$
\phi T'
=
D
\left[
\phi''T
+
\frac2r\phi'T
\right].
\tag{TRISO-ANA-233}
$$

Factor out (T) on the right:

$$
\phi T'
=
DT
\left(
\phi''+\frac2r\phi'
\right).
\tag{TRISO-ANA-234}
$$

Divide by (D\phi T), assuming the separated factors are non-zero at the point considered:

$$
\frac{T'}{DT}
=
\frac{\phi''+2\phi'/r}{\phi}.
\tag{TRISO-ANA-235}
$$

The left side depends only on (t), while the right side depends only on (r).

For one separated mode to satisfy the equation for every (r) and (t), both sides must equal the same constant.

Choose the separation constant as (-k^2):

$$
\frac{T'}{DT}=-k^2.
\tag{TRISO-ANA-236}
$$

Then the time equation is

$$
T'=-Dk^2T.
\tag{TRISO-ANA-237}
$$

The radial equation is

$$
\boxed{
\phi''
+
\frac2r\phi'
+
k^2\phi
=
0.
}
\tag{TRISO-ANA-238}
$$

### 9.2 Dimensions of the separation constant

The left side of (TRISO-ANA-236) has units

$$
\left[
\frac{T'}{DT}
\right]
=
\frac{\mathrm{s^{-1}}}{\mathrm{m^2\,s^{-1}}}
=
\mathrm{m^{-2}}.
\tag{TRISO-ANA-239}
$$

Therefore

$$
[k^2]=\mathrm{m^{-2}}.
\tag{TRISO-ANA-240}
$$

Hence

$$
[k]=\mathrm{m^{-1}}.
\tag{TRISO-ANA-241}
$$

Define the temporal decay rate

$$
\boxed{
\Lambda=Dk^2.
}
\tag{TRISO-ANA-242}
$$

Then

$$
[\Lambda]
=
\mathrm{m^2\,s^{-1}}
\times
\mathrm{m^{-2}}
=
\mathrm{s^{-1}}.
\tag{TRISO-ANA-243}
$$

Thus (k) is a spatial wave number, while (Lambda) is a temporal decay rate.

### 9.3 Solve the temporal equation

Starting from

$$
T'=-\Lambda T,
\tag{TRISO-ANA-244}
$$

divide by (T):

$$
\frac{T'}{T}=-\Lambda.
\tag{TRISO-ANA-245}
$$

Write the derivative as a differential:

$$
\frac{dT}{T}=-\Lambda\,dt.
\tag{TRISO-ANA-246}
$$

Integrate:

$$
\int\frac{dT}{T}
=
-\Lambda\int dt.
\tag{TRISO-ANA-247}
$$

Therefore

$$
\ln|T|
=
-\Lambda t+C.
\tag{TRISO-ANA-248}
$$

Exponentiate:

$$
|T|=e^{C}e^{-\Lambda t}.
\tag{TRISO-ANA-249}
$$

Absorb the constant into an arbitrary amplitude (C_T):

$$
T(t)=C_Te^{-\Lambda t}.
\tag{TRISO-ANA-250}
$$

The constant (C_T) can be absorbed into the spatial amplitude, so take

$$
\boxed{
T(t)=e^{-\Lambda t}.
}
\tag{TRISO-ANA-251}
$$

Therefore each separated mode has the form

$$
v(r,t)=\phi(r)e^{-\Lambda t}.
\tag{TRISO-ANA-252}
$$

## 10. Radial eigenproblem, Robin condition, and modal expansion

### 10.1 Transform the radial eigenproblem with (u=r\phi)

Start from

$$
\phi''
+
\frac2r\phi'
+
k^2\phi
=
0.
\tag{TRISO-ANA-253}
$$

Introduce

$$
u(r)=r\phi(r).
\tag{TRISO-ANA-254}
$$

Solve the definition for (phi):

$$
\phi(r)=\frac{u(r)}{r}.
\tag{TRISO-ANA-255}
$$

Differentiate using the quotient rule:

$$
\phi'
=
\frac{r u'-u}{r^2}.
\tag{TRISO-ANA-256}
$$

Differentiate again. Write the numerator as (n=ru'-u):

$$
n'=u'+ru''-u'.
\tag{TRISO-ANA-257}
$$

Therefore

$$
n'=ru''.
\tag{TRISO-ANA-258}
$$

Apply the quotient rule to (n/r^2):

$$
\phi''
=
\frac{n'r^2-n(2r)}{r^4}.
\tag{TRISO-ANA-259}
$$

Substitute (n'=ru'') and (n=ru'-u):

$$
\phi''
=
\frac{r^3u''-2r(ru'-u)}{r^4}.
\tag{TRISO-ANA-260}
$$

Expand the numerator:

$$
\phi''
=
\frac{r^3u''-2r^2u'+2ru}{r^4}.
\tag{TRISO-ANA-261}
$$

Divide each term by (r^4):

$$
\phi''
=
\frac{u''}{r}
-
\frac{2u'}{r^2}
+
\frac{2u}{r^3}.
\tag{TRISO-ANA-262}
$$

Now substitute (TRISO-ANA-256), (TRISO-ANA-255), and (TRISO-ANA-262) into (TRISO-ANA-253):

$$
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
\tag{TRISO-ANA-263}
$$

Expand the second term:

$$
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
\tag{TRISO-ANA-264}
$$

Cancel the (u') terms explicitly:

$$
\frac{u''}{r}
+
k^2\frac{u}{r}
=
0.
\tag{TRISO-ANA-265}
$$

Multiply by (r):

$$
\boxed{
u''+k^2u=0.
}
\tag{TRISO-ANA-266}
$$

### 10.2 Solve the transformed radial equation

The characteristic equation is

$$
m^2+k^2=0.
\tag{TRISO-ANA-267}
$$

Its roots are

$$
m=\pm ik.
\tag{TRISO-ANA-268}
$$

Therefore the real-valued solution is

$$
\boxed{
u(r)=A\sin(kr)+B\cos(kr).
}
\tag{TRISO-ANA-269}
$$

Substitute into (phi=u/r):

$$
\phi(r)
=
\frac{A\sin(kr)+B\cos(kr)}{r}.
\tag{TRISO-ANA-270}
$$

### 10.3 Centre regularity

Examine the cosine contribution as (r	o0).

Use the known limit

$$
\lim_{r\to0}\cos(kr)=1.
\tag{TRISO-ANA-271}
$$

Therefore

$$
\lim_{r\to0}\frac{B\cos(kr)}{r}
=
\lim_{r\to0}\frac{B}{r}.
\tag{TRISO-ANA-272}
$$

For (B\ne0), this diverges.

A physical concentration perturbation must remain finite at the particle centre.

Therefore

$$
B=0.
\tag{TRISO-ANA-273}
$$

The eigenfunction becomes

$$
\phi(r)=A\frac{\sin(kr)}{r}.
\tag{TRISO-ANA-274}
$$

For the sine term, use

$$
\sin(kr)=kr+O(r^3)
\qquad
(r\to0).
\tag{TRISO-ANA-275}
$$

Divide by (r):

$$
\frac{\sin(kr)}{r}
=
k+O(r^2).
\tag{TRISO-ANA-276}
$$

Therefore

$$
\boxed{
\lim_{r\to0}\phi(r)=Ak.
}
\tag{TRISO-ANA-277}
$$

The apparent (1/r) singularity is removable for the sine branch.

It is often convenient to absorb (k) into the modal amplitude. Define

$$
C=A k.
\tag{TRISO-ANA-278}
$$

Then the same mode may be written as

$$
\phi(r)=C\frac{\sin(kr)}{kr}.
\tag{TRISO-ANA-279}
$$

The normalization is arbitrary; only the relative spatial shape matters for the eigenvalue problem.

### 10.4 Derive the Robin eigencondition

Start from the normalized form

$$
\phi(r)=C\frac{\sin(kr)}{kr}.
\tag{TRISO-ANA-280}
$$

The derivative is easier to obtain by treating (C/k) as a constant:

$$
\phi(r)=\frac{C}{k}\frac{\sin(kr)}{r}.
\tag{TRISO-ANA-281}
$$

Differentiate (\sin(kr)/r) using the quotient rule:

$$
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
\tag{TRISO-ANA-282}
$$

Therefore

$$
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
\tag{TRISO-ANA-283}
$$

At the outer boundary,

$$
-D\phi'(R)=h\phi(R).
\tag{TRISO-ANA-284}
$$

Substitute (phi'(R)):

$$
-D
\frac{C}{k}
\frac{
kR\cos(kR)-\sin(kR)
}{
R^2
}
=
h\phi(R).
\tag{TRISO-ANA-285}
$$

Substitute (phi(R)=C\sin(kR)/(kR)):

$$
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
\tag{TRISO-ANA-286}
$$

Multiply both sides by (kR^2/C), assuming (C\ne0):

$$
-D
\left[
kR\cos(kR)-\sin(kR)
\right]
=
hR\sin(kR).
\tag{TRISO-ANA-287}
$$

Multiply by (-1):

$$
D
\left[
\sin(kR)-kR\cos(kR)
\right]
=
hR\sin(kR).
\tag{TRISO-ANA-288}
$$

Define the dimensionless eigenvariable

$$
\mu=kR.
\tag{TRISO-ANA-289}
$$

Its dimensions are

$$
[\mu]=[k][R]=\mathrm{m^{-1}}\times\mathrm m=1.
\tag{TRISO-ANA-290}
$$

Define the Biot number

$$
\mathrm{Bi}=\frac{hR}{D}.
\tag{TRISO-ANA-291}
$$

Its dimensions are

$$
[\mathrm{Bi}]
=
\frac{
\mathrm{m\,s^{-1}}\times\mathrm m
}{
\mathrm{m^2\,s^{-1}}
}
=
1.
\tag{TRISO-ANA-292}
$$

Substitute (\mu=kR) and (hR/D=\mathrm{Bi}) into (TRISO-ANA-288):

$$
\boxed{
\sin\mu-\mu\cos\mu
=
\mathrm{Bi}\sin\mu.
}
\tag{TRISO-ANA-293}
$$

Rearrange:

$$
(1-\mathrm{Bi})\sin\mu
=
\mu\cos\mu.
\tag{TRISO-ANA-294}
$$

For (\sin\mu\ne0), divide by (\sin\mu):

$$
1-\mathrm{Bi}
=
\mu\frac{\cos\mu}{\sin\mu}.
\tag{TRISO-ANA-295}
$$

Use (\cot\mu=\cos\mu/\sin\mu):

$$
\boxed{
\mu\cot\mu=1-\mathrm{Bi}.
}
\tag{TRISO-ANA-296}
$$

### 10.5 Check whether division by (sinmu) loses roots

The undivided equation (TRISO-ANA-293) must be used for this check.

Suppose

$$
\sin\mu=0.
\tag{TRISO-ANA-297}
$$

Then (mu=n\pi) for integer (n).

Substitute into (TRISO-ANA-293):

$$
0-n\pi\cos(n\pi)=0.
\tag{TRISO-ANA-298}
$$

Because

$$
\cos(n\pi)=(-1)^n,
\tag{TRISO-ANA-299}
$$

this becomes

$$
-n\pi(-1)^n=0.
\tag{TRISO-ANA-300}
$$

For positive (n), this is not zero.

Therefore no positive eigenvalue is lost when dividing by (sinmu).

The only simultaneous zero is (mu=0), which does not satisfy the positive transient-mode condition for the Robin problem with (h>0).

### 10.6 Eigenvalue definitions

Let (mu_n) denote the positive roots of (TRISO-ANA-293).

Then

$$
k_nR=\mu_n.
\tag{TRISO-ANA-301}
$$

Therefore

$$
\boxed{
k_n=\frac{\mu_n}{R}.
}
\tag{TRISO-ANA-302}
$$

Since

$$
\Lambda_n=Dk_n^2,
\tag{TRISO-ANA-303}
$$

substitute (TRISO-ANA-302):

$$
\Lambda_n
=
D
\left(
\frac{\mu_n}{R}
\right)^2.
\tag{TRISO-ANA-304}
$$

Thus

$$
\boxed{
\Lambda_n
=
D\frac{\mu_n^2}{R^2}.
}
\tag{TRISO-ANA-305}
$$

The dimensions are

$$
[\Lambda_n]
=
\mathrm{m^2\,s^{-1}}
\times
\mathrm{m^{-2}}
=
\mathrm{s^{-1}}.
\tag{TRISO-ANA-306}
$$

The separated transient mode is therefore

$$
v_n(r,t)=\phi_n(r)e^{-\Lambda_nt}.
\tag{TRISO-ANA-307}
$$

### 10.7 Sturm–Liouville form

Start from the radial eigenproblem:

$$
\phi_n''
+
\frac2r\phi_n'
+
k_n^2\phi_n
=
0.
\tag{TRISO-SL-200}
$$

Multiply by (r^2):

$$
r^2\phi_n''
+
2r\phi_n'
+
k_n^2r^2\phi_n
=
0.
\tag{TRISO-SL-201}
$$

The first two terms are a product derivative because

$$
\frac{d}{dr}(r^2\phi_n')
=
2r\phi_n'
+
r^2\phi_n''.
\tag{TRISO-SL-202}
$$

Therefore

$$
\frac{d}{dr}(r^2\phi_n')
+
k_n^2r^2\phi_n
=
0.
\tag{TRISO-SL-203}
$$

Move the eigenvalue term to the other side:

$$
-\frac{d}{dr}(r^2\phi_n')
=
k_n^2r^2\phi_n.
\tag{TRISO-SL-204}
$$

This is the self-adjoint Sturm–Liouville form

$$
-\frac{d}{dr}
\left(
p(r)\frac{d\phi_n}{dr}
\right)
+
q(r)\phi_n
=
\lambda_n w(r)\phi_n
$$

with the identifications

$$
p(r)=r^2,
\tag{TRISO-SL-205}
$$

$$
q(r)=0,
\tag{TRISO-SL-206}
$$

$$
w(r)=r^2,
\tag{TRISO-SL-207}
$$

and

$$
\lambda_n=k_n^2.
\tag{TRISO-SL-208}
$$

The eigenvalue in this Sturm–Liouville problem is therefore (k_n^2), with units (mathrm{m^{-2}}), not the temporal decay rate (Lambda_n).

The interval is (0<r<R).

The centre condition is regularity of (phi_n), equivalent for these modes to a finite (phi_n(0)) and zero radial derivative at the centre.

The outer boundary is the homogeneous Robin condition

$$
-D\phi_n'(R)=h\phi_n(R).
\tag{TRISO-SL-209}
$$

[THEOREM / STANDARD FORM] Sturm–Liouville theory provides the framework for eigenvalues and eigenfunctions of self-adjoint second-order problems. See the NIST Digital Library of Mathematical Functions, §1.13(viii), which identifies Sturm–Liouville eigenvalues/eigenfunctions and the Liouville form. \cite{DLMFSturmLiouville}.

Because the centre endpoint has (p(0)=0), it is more precise to call this a radial **singular** Sturm–Liouville endpoint rather than an ordinary regular endpoint. The orthogonality used below can nevertheless be derived directly for the present eigenfunctions, so no stronger theorem is needed.

### 10.8 Derive orthogonality directly

Take two distinct eigenfunctions (phi_m) and (phi_n) with eigenvalues (k_m^2) and (k_n^2):

$$
-\frac{d}{dr}
(r^2\phi_m')
=
k_m^2r^2\phi_m,
\tag{TRISO-SL-210}
$$

and

$$
-\frac{d}{dr}
(r^2\phi_n')
=
k_n^2r^2\phi_n.
\tag{TRISO-SL-211}
$$

Multiply the first equation by (phi_n):

$$
-\phi_n\frac{d}{dr}(r^2\phi_m')
=
k_m^2r^2\phi_m\phi_n.
\tag{TRISO-SL-212}
$$

Multiply the second equation by (phi_m):

$$
-\phi_m\frac{d}{dr}(r^2\phi_n')
=
k_n^2r^2\phi_m\phi_n.
\tag{TRISO-SL-213}
$$

Subtract the second equation from the first:

$$
-\phi_n\frac{d}{dr}(r^2\phi_m')
+
\phi_m\frac{d}{dr}(r^2\phi_n')
=
(k_m^2-k_n^2)r^2\phi_m\phi_n.
\tag{TRISO-SL-214}
$$

Recognise the left side as a derivative:

$$
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
\tag{TRISO-SL-215}
$$

Integrate from (0) to (R):

$$
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
\tag{TRISO-SL-216}
$$

Evaluate the left-hand integral:

$$
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
\tag{TRISO-SL-217}
$$

At (r=R), both eigenfunctions satisfy the same Robin condition:

$$
\phi_m'(R)=-\frac{h}{D}\phi_m(R),
\tag{TRISO-SL-218}
$$

and

$$
\phi_n'(R)=-\frac{h}{D}\phi_n(R).
\tag{TRISO-SL-219}
$$

Therefore the outer boundary term is zero:

$$
R^2
[
\phi_m(R)\phi_n'(R)
-
\phi_n(R)\phi_m'(R)
]
=0.
\tag{TRISO-SL-220}
$$

At the centre, the regular eigenfunctions are finite and their derivatives remain bounded, while (r^2\to0).

Hence

$$
\lim_{r\to0}
r^2
[
\phi_m\phi_n'
-
\phi_n\phi_m'
]
=0.
\tag{TRISO-SL-221}
$$

Therefore the complete boundary term is zero:

$$
(k_n^2-k_m^2)
\int_0^R
r^2\phi_m\phi_n\,dr
=
0.
\tag{TRISO-SL-222}
$$

For distinct eigenvalues,

$$
k_n^2\ne k_m^2,
\tag{TRISO-SL-223}
$$

so

$$
\boxed{
\int_0^R
r^2\phi_m(r)\phi_n(r)\,dr
=
0,
\qquad m\ne n.
}
\tag{TRISO-SL-224}
$$

The weight is therefore

$$
\boxed{
w(r)=r^2.
}
\tag{TRISO-SL-225}
$$

The same (r^2) weight also follows directly from spherical volume (dV=4\pi r^2dr).

### 10.9 Modal coefficient projection

At (t=0), (TRISO-ANA-228) gives

$$
v(r,0)=-w(r).
\tag{TRISO-SL-226}
$$

Represent the initial transient as an eigenfunction series:

$$
-w(r)
=
\sum_{n=1}^{\infty}
A_n\phi_n(r).
\tag{TRISO-SL-227}
$$

Multiply both sides by (r^2\phi_m(r)):

$$
-r^2w(r)\phi_m(r)
=
\sum_{n=1}^{\infty}
A_n r^2\phi_n(r)\phi_m(r).
\tag{TRISO-SL-228}
$$

Integrate from (0) to (R):

$$
-\int_0^R
r^2w(r)\phi_m(r)\,dr
=
\sum_{n=1}^{\infty}
A_n
\int_0^R
r^2\phi_n(r)\phi_m(r)\,dr.
\tag{TRISO-SL-229}
$$

For (n\ne m), orthogonality makes the corresponding integrals zero:

$$
\int_0^R
r^2\phi_n\phi_m\,dr
=
0.
\tag{TRISO-SL-230}
$$

The remaining (n=m) term is

$$
-\int_0^R
r^2w(r)\phi_m(r)\,dr
=
A_m
\int_0^R
r^2\phi_m(r)^2\,dr.
\tag{TRISO-SL-231}
$$

Divide by the non-zero mode norm:

$$
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
\tag{TRISO-SL-232}
$$

Rename (m) to (n):

$$
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
\tag{TRISO-SL-233}
$$

Each mode evolves with (e^{-\Lambda_nt}), so

$$
v(r,t)
=
\sum_{n=1}^{\infty}
A_n\phi_n(r)e^{-\Lambda_nt}.
\tag{TRISO-SL-234}
$$

Since (c=v+w),

$$
\boxed{
c(r,t)
=
w(r)
+
\sum_{n=1}^{\infty}
A_n\phi_n(r)e^{-\Lambda_nt}.
}
\tag{TRISO-SL-235}
$$

### 10.10 Checks on the transient series

Each mode has decay factor

$$
e^{-\Lambda_nt}.
\tag{TRISO-SL-236}
$$

At (t=0),

$$
e^{-\Lambda_n\cdot0}=1.
\tag{TRISO-SL-237}
$$

Therefore

$$
c(r,0)
=
w(r)
+
\sum_{n=1}^{\infty}A_n\phi_n(r).
\tag{TRISO-SL-238}
$$

Using the defining expansion (TRISO-SL-227),

$$
\sum_{n=1}^{\infty}A_n\phi_n(r)
=
-w(r).
\tag{TRISO-SL-239}
$$

Hence, formally,

$$
\boxed{
c(r,0)=0.
}
\tag{TRISO-SL-240}
$$

As (t\to\infty), every mode with (Lambda_n>0) satisfies

$$
e^{-\Lambda_nt}\to0.
\tag{TRISO-SL-241}
$$

Therefore, provided the modal expansion has the required convergence,

$$
\boxed{
c(r,t)\to w(r).
}
\tag{TRISO-SL-242}
$$

The convergence of the infinite series itself has not been numerically established here. The statements above are the formal consequences of the eigen-expansion framework.

## 11. Five-layer steady analytical formulation

### 11.1 Geometry, domains, and layer quantities

Define the concentric radii

$$
r_0=0<r_1<r_2<r_3<r_4<r_5=R.
\tag{TRISO-ML-300}
$$

The five material regions are

$$
\Omega_1=(r_0,r_1)
\quad\text{fuel kernel},
\tag{TRISO-ML-301}
$$

$$
\Omega_2=(r_1,r_2)
\quad\text{buffer},
\tag{TRISO-ML-302}
$$

$$
\Omega_3=(r_2,r_3)
\quad\text{IPyC},
\tag{TRISO-ML-303}
$$

$$
\Omega_4=(r_3,r_4)
\quad\text{SiC},
\tag{TRISO-ML-304}
$$

and

$$
\Omega_5=(r_4,r_5)
\quad\text{OPyC}.
\tag{TRISO-ML-305}
$$

In material layer \(i\), define

$$
c_i(r,t),
\qquad
r_{i-1}<r<r_i.
\tag{TRISO-ML-306}
$$

The concentration units are

$$
[c_i]=\mathrm{mol\,m^{-3}}.
\tag{TRISO-ML-307}
$$

Let \(D_i\) be the diffusivity in layer \(i\):

$$
[D_i]=\mathrm{m^2\,s^{-1}}.
\tag{TRISO-ML-308}
$$

Let \(S_i\) be the net volumetric source in layer \(i\):

$$
[S_i]=\mathrm{mol\,m^{-3}\,s^{-1}}.
\tag{TRISO-ML-309}
$$

The coordinates \(r,t\), outer radius \(R\), transfer coefficient \(h\), and external concentration \(c_\infty\) are global quantities. The fields \(c_i\), diffusivities \(D_i\), and sources \(S_i\) are material-layer quantities.

For the source-driven five-layer benchmark,

$$
S_1=S_0,
\tag{TRISO-ML-310}
$$

while

$$
S_i=0,
\qquad
i=2,3,4,5.
\tag{TRISO-ML-311}
$$

[ASSUMPTION] Each \(D_i>0\) is constant within its material layer for this analytical benchmark.

[ASSUMPTION] Interfaces have zero storage, zero interfacial source, \(K_i=1\), and no explicit interfacial resistance.

### 11.2 Kernel steady solution

The steady conservative equation in the kernel is

$$
0=
\frac1{r^2}
\frac{d}{dr}
\left(
r^2D_1\frac{dc_1}{dr}
\right)
+S_0.
\tag{TRISO-ML-312}
$$

Move the source term to the right:

$$
\frac1{r^2}
\frac{d}{dr}
\left(
r^2D_1\frac{dc_1}{dr}
\right)
=-S_0.
\tag{TRISO-ML-313}
$$

Multiply by \(r^2\):

$$
\frac{d}{dr}
\left(
r^2D_1\frac{dc_1}{dr}
\right)
=-S_0r^2.
\tag{TRISO-ML-314}
$$

Because \(D_1\) is constant in the kernel,

$$
D_1
\frac{d}{dr}
\left(
r^2\frac{dc_1}{dr}
\right)
=-S_0r^2.
\tag{TRISO-ML-315}
$$

Divide by \(D_1\):

$$
\frac{d}{dr}
\left(
r^2\frac{dc_1}{dr}
\right)
=-\frac{S_0}{D_1}r^2.
\tag{TRISO-ML-316}
$$

Integrate:

$$
r^2\frac{dc_1}{dr}
=
-\frac{S_0r^3}{3D_1}
+C_1.
\tag{TRISO-ML-317}
$$

Divide by \(r^2\), for \(r>0\):

$$
\frac{dc_1}{dr}
=
-\frac{S_0r}{3D_1}
+\frac{C_1}{r^2}.
\tag{TRISO-ML-318}
$$

Centre regularity requires \(dc_1/dr\) to remain finite as \(r\to0\). The term \(C_1/r^2\) diverges unless

$$
C_1=0.
\tag{TRISO-ML-319}
$$

Therefore

$$
\frac{dc_1}{dr}
=
-\frac{S_0r}{3D_1}.
\tag{TRISO-ML-320}
$$

Integrate again:

$$
c_1(r)
=
-\frac{S_0r^2}{6D_1}
+A_1.
\tag{TRISO-ML-321}
$$

Hence the regular kernel profile is

$$
\boxed{
c_1(r)
=
A_1-\frac{S_0r^2}{6D_1}.
}
\tag{TRISO-ML-322}
$$

### 11.3 Source-free coating solutions

For \(i=2,3,4,5\),

$$
S_i=0.
\tag{TRISO-ML-323}
$$

The steady equation is

$$
0=
\frac1{r^2}
\frac{d}{dr}
\left(
r^2D_i\frac{dc_i}{dr}
\right).
\tag{TRISO-ML-324}
$$

Multiply by \(r^2\):

$$
\frac{d}{dr}
\left(
r^2D_i\frac{dc_i}{dr}
\right)=0.
\tag{TRISO-ML-325}
$$

Use constant \(D_i\):

$$
D_i
\frac{d}{dr}
\left(
r^2\frac{dc_i}{dr}
\right)=0.
\tag{TRISO-ML-326}
$$

Divide by \(D_i>0\):

$$
\frac{d}{dr}
\left(
r^2\frac{dc_i}{dr}
\right)=0.
\tag{TRISO-ML-327}
$$

Integrate:

$$
r^2\frac{dc_i}{dr}=C_i.
\tag{TRISO-ML-328}
$$

Divide by \(r^2\):

$$
\frac{dc_i}{dr}=\frac{C_i}{r^2}.
\tag{TRISO-ML-329}
$$

Integrate:

$$
c_i(r)=C_i\int r^{-2}dr+A_i.
\tag{TRISO-ML-330}
$$

Since

$$
\int r^{-2}dr=-\frac1r,
\tag{TRISO-ML-331}
$$

we obtain

$$
c_i(r)=A_i-\frac{C_i}{r}.
\tag{TRISO-ML-332}
$$

Define \(B_i=-C_i\). Then

$$
\boxed{
c_i(r)=A_i+\frac{B_i}{r},
\qquad i=2,3,4,5.
}
\tag{TRISO-ML-333}
$$

The coating layers do not include \(r=0\), so their \(1/r\) terms are finite within their own domains and are not removed by centre regularity.

### 11.4 Total kernel generation and common steady flux

The total kernel generation rate is

$$
\dot N_{\mathrm{gen}}
=
4\pi
\int_0^{r_1}
S_0r^2\,dr.
\tag{TRISO-ML-334}
$$

Pull the constants outside:

$$
\dot N_{\mathrm{gen}}
=
4\pi S_0
\int_0^{r_1}r^2\,dr.
\tag{TRISO-ML-335}
$$

Evaluate the integral:

$$
\int_0^{r_1}r^2\,dr
=
\left[\frac{r^3}{3}\right]_0^{r_1}.
\tag{TRISO-ML-336}
$$

Therefore

$$
\int_0^{r_1}r^2\,dr
=
\frac{r_1^3}{3}.
\tag{TRISO-ML-337}
$$

Hence

$$
\boxed{
\dot N_{\mathrm{gen}}
=
\frac{4\pi S_0r_1^3}{3}.
}
\tag{TRISO-ML-338}
$$

At steady state, with no coating source, reaction, or storage, the same total amount rate crosses every sphere outside the kernel:

$$
4\pi r^2J_r(r)
=
\dot N_{\mathrm{gen}},
\qquad r>r_1.
\tag{TRISO-ML-339}
$$

Substitute (TRISO-ML-338):

$$
4\pi r^2J_r(r)
=
\frac{4\pi S_0r_1^3}{3}.
\tag{TRISO-ML-340}
$$

Cancel \(4\pi\):

$$
r^2J_r(r)=\frac{S_0r_1^3}{3}.
\tag{TRISO-ML-341}
$$

Divide by \(r^2\):

$$
\boxed{
J_r(r)=\frac{S_0r_1^3}{3r^2}.
}
\tag{TRISO-ML-342}
$$

The positive sign is outward.

### 11.5 Recover shell gradients from Fick's law

In shell \(i\),

$$
J_r=-D_i\frac{dc_i}{dr}.
\tag{TRISO-ML-343}
$$

Substitute (TRISO-ML-342):

$$
-D_i\frac{dc_i}{dr}
=
\frac{S_0r_1^3}{3r^2}.
\tag{TRISO-ML-344}
$$

Divide by \(-D_i\):

$$
\boxed{
\frac{dc_i}{dr}
=
-\frac{S_0r_1^3}{3D_ir^2}.
}
\tag{TRISO-ML-345}
$$

Integrate from \(r\) to the outer radius \(r_i\) of that shell:

$$
\int_{c_i(r)}^{c_i(r_i)}dc_i
=
-\frac{S_0r_1^3}{3D_i}
\int_r^{r_i}\rho^{-2}d\rho.
\tag{TRISO-ML-346}
$$

The radial integral is

$$
\int_r^{r_i}\rho^{-2}d\rho
=
\frac1r-\frac1{r_i}.
\tag{TRISO-ML-347}
$$

Therefore

$$
c_i(r_i)-c_i(r)
=
-\frac{S_0r_1^3}{3D_i}
\left(
\frac1r-\frac1{r_i}
\right).
\tag{TRISO-ML-348}
$$

Multiply by \(-1\):

$$
\boxed{
c_i(r)-c_i(r_i)
=
\frac{S_0r_1^3}{3D_i}
\left(
\frac1r-\frac1{r_i}
\right).
}
\tag{TRISO-ML-349}
$$

Differentiating \(A_i+B_i/r\) gives

$$
\frac{dc_i}{dr}=-\frac{B_i}{r^2}.
\tag{TRISO-ML-350}
$$

Compare with (TRISO-ML-345):

$$
-\frac{B_i}{r^2}
=
-\frac{S_0r_1^3}{3D_ir^2}.
\tag{TRISO-ML-351}
$$

Hence

$$
\boxed{
B_i=\frac{S_0r_1^3}{3D_i}.
}
\tag{TRISO-ML-352}
$$

This independently agrees with the direct shell ODE solution.

### 11.6 Interface matching

At \(r=r_1\), flux continuity is

$$
-D_1c_1'(r_1)
=
-D_2c_2'(r_1),
\tag{TRISO-ML-353}
$$

and ideal concentration continuity is

$$
c_1(r_1)=c_2(r_1).
\tag{TRISO-ML-354}
$$

At \(r=r_2\),

$$
-D_2c_2'(r_2)
=
-D_3c_3'(r_2),
\tag{TRISO-ML-355}
$$

and

$$
c_2(r_2)=c_3(r_2).
\tag{TRISO-ML-356}
$$

At \(r=r_3\),

$$
-D_3c_3'(r_3)
=
-D_4c_4'(r_3),
\tag{TRISO-ML-357}
$$

and

$$
c_3(r_3)=c_4(r_3).
\tag{TRISO-ML-358}
$$

At \(r=r_4\),

$$
-D_4c_4'(r_4)
=
-D_5c_5'(r_4),
\tag{TRISO-ML-359}
$$

and

$$
c_4(r_4)=c_5(r_4).
\tag{TRISO-ML-360}
$$

The flux equations are already satisfied by the common steady amount rate. The concentration equations relate the additive constants.

Across shell \(i\),

$$
c_i(r_{i-1})-c_i(r_i)
=
\frac{S_0r_1^3}{3D_i}
\left(
\frac1{r_{i-1}}-\frac1{r_i}
\right),
\qquad i=2,3,4,5.
\tag{TRISO-ML-361}
$$

Thus each inner interface concentration is obtained from the next outer interface concentration by adding that shell's concentration drop.

### 11.7 Outer Robin condition and inward propagation

At \(R=r_5\),

$$
-D_5c_5'(R)
=
h[c_5(R)-c_\infty].
\tag{TRISO-ML-362}
$$

For the benchmark,

$$
c_\infty=0.
\tag{TRISO-ML-363}
$$

Therefore

$$
-D_5c_5'(R)=hc_5(R).
\tag{TRISO-ML-364}
$$

The common flux gives

$$
-D_5c_5'(R)
=
\frac{S_0r_1^3}{3R^2}.
\tag{TRISO-ML-365}
$$

Equate the two expressions:

$$
hc_5(R)
=
\frac{S_0r_1^3}{3R^2}.
\tag{TRISO-ML-366}
$$

Divide by \(h\):

$$
\boxed{
c_5(R)
=
\frac{S_0r_1^3}{3hR^2}.
}
\tag{TRISO-ML-367}
$$

Move inward through OPyC:

$$
c_5(r_4)
=
c_5(R)
+
\frac{S_0r_1^3}{3D_5}
\left(
\frac1{r_4}-\frac1R
\right).
\tag{TRISO-ML-368}
$$

Apply continuity:

$$
c_4(r_4)=c_5(r_4).
\tag{TRISO-ML-369}
$$

Move inward through SiC:

$$
c_4(r_3)
=
c_4(r_4)
+
\frac{S_0r_1^3}{3D_4}
\left(
\frac1{r_3}-\frac1{r_4}
\right).
\tag{TRISO-ML-370}
$$

Apply continuity:

$$
c_3(r_3)=c_4(r_3).
\tag{TRISO-ML-371}
$$

Move inward through IPyC:

$$
c_3(r_2)
=
c_3(r_3)
+
\frac{S_0r_1^3}{3D_3}
\left(
\frac1{r_2}-\frac1{r_3}
\right).
\tag{TRISO-ML-372}
$$

Apply continuity:

$$
c_2(r_2)=c_3(r_2).
\tag{TRISO-ML-373}
$$

Move inward through the buffer:

$$
c_2(r_1)
=
c_2(r_2)
+
\frac{S_0r_1^3}{3D_2}
\left(
\frac1{r_1}-\frac1{r_2}
\right).
\tag{TRISO-ML-374}
$$

Apply continuity:

$$
c_1(r_1)=c_2(r_1).
\tag{TRISO-ML-375}
$$

From the kernel profile,

$$
c_1(r)-c_1(r_1)
=
\frac{S_0}{6D_1}(r_1^2-r^2).
\tag{TRISO-ML-376}
$$

Hence

$$
\boxed{
c_1(r)
=
c_2(r_1)
+
\frac{S_0}{6D_1}(r_1^2-r^2).
}
\tag{TRISO-ML-377}
$$

Equations (TRISO-ML-367) through (TRISO-ML-377) give the complete steady five-layer solution recursively.

### 11.8 Spherical resistance formulation

Define the common steady amount rate outside the kernel:

$$
\dot N=4\pi r^2J_r.
\tag{TRISO-ML-378}
$$

In shell \(i\),

$$
\dot N
=
-4\pi r^2D_i\frac{dc_i}{dr}.
\tag{TRISO-ML-379}
$$

Rearrange:

$$
dc_i
=
-\frac{\dot N}{4\pi D_i}\frac{dr}{r^2}.
\tag{TRISO-ML-380}
$$

Integrate from \(r_{i-1}\) to \(r_i\):

$$
c_i(r_i)-c_i(r_{i-1})
=
-\frac{\dot N}{4\pi D_i}
\int_{r_{i-1}}^{r_i}r^{-2}dr.
\tag{TRISO-ML-381}
$$

Evaluate the radial integral:

$$
\int_{r_{i-1}}^{r_i}r^{-2}dr
=
\frac1{r_{i-1}}-\frac1{r_i}.
\tag{TRISO-ML-382}
$$

Therefore

$$
c_i(r_{i-1})-c_i(r_i)
=
\dot N
\frac1{4\pi D_i}
\left(
\frac1{r_{i-1}}-\frac1{r_i}
\right).
\tag{TRISO-ML-383}
$$

Define

$$
\boxed{
\mathcal R_i
=
\frac1{4\pi D_i}
\left(
\frac1{r_{i-1}}-\frac1{r_i}
\right),
\qquad i=2,3,4,5.
}
\tag{TRISO-ML-384}
$$

Then

$$
c_i(r_{i-1})-c_i(r_i)=\dot N\mathcal R_i.
\tag{TRISO-ML-385}
$$

Its units are

$$
[\mathcal R_i]
=
\mathrm{s\,m^{-3}}.
\tag{TRISO-ML-386}
$$

For external transfer,

$$
\dot N
=
4\pi R^2h[c_5(R)-c_\infty].
\tag{TRISO-ML-387}
$$

Rearrange:

$$
c_5(R)-c_\infty
=
\dot N
\frac1{4\pi R^2h}.
\tag{TRISO-ML-388}
$$

Define

$$
\boxed{
\mathcal R_h
=
\frac1{4\pi R^2h}.
}
\tag{TRISO-ML-389}
$$

Its units are also

$$
[\mathcal R_h]=\mathrm{s\,m^{-3}}.
\tag{TRISO-ML-390}
$$

Because the same \(\dot N\) passes through every coating and the external film, the concentration drops add:

$$
c_2(r_1)-c_\infty
=
\dot N
\left(
\mathcal R_2+\mathcal R_3+\mathcal R_4+\mathcal R_5+\mathcal R_h
\right).
\tag{TRISO-ML-391}
$$

Thus

$$
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
\tag{TRISO-ML-392}
$$

For the benchmark \(c_\infty=0\) and

$$
\dot N=\frac{4\pi S_0r_1^3}{3}.
\tag{TRISO-ML-393}
$$

Substituting (TRISO-ML-384), (TRISO-ML-389), and (TRISO-ML-393) into (TRISO-ML-392) reproduces exactly the inward-recursion concentration at \(r_1\).

The kernel itself is source-containing, so it is not represented by the same source-free shell resistance. Its centre-to-interface concentration rise is instead

$$
c_1(0)-c_1(r_1)
=
\frac{S_0r_1^2}{6D_1}.
\tag{TRISO-ML-394}
$$

This distinction prevents a source-containing kernel from being incorrectly treated as an ordinary source-free series resistance.

## 12. Five-layer transient analytical formulation

### 12.1 Define the steady reference and transient deviation

Let

$$
c_{i,\mathrm{ss}}(r)
\tag{TRISO-ML-400}
$$

denote the steady five-layer solution derived in Section 11.

Define the transient deviation in each layer:

$$
\boxed{
v_i(r,t)
=
c_i(r,t)-c_{i,\mathrm{ss}}(r).
}
\tag{TRISO-ML-401}
$$

Rearrange:

$$
c_i(r,t)
=
v_i(r,t)+c_{i,\mathrm{ss}}(r).
\tag{TRISO-ML-402}
$$

Because the steady reference is time independent,

$$
\frac{\partial c_{i,\mathrm{ss}}}{\partial t}=0.
\tag{TRISO-ML-403}
$$

Therefore

$$
\frac{\partial c_i}{\partial t}
=
\frac{\partial v_i}{\partial t}.
\tag{TRISO-ML-404}
$$

Similarly,

$$
\frac{\partial c_i}{\partial r}
=
\frac{\partial v_i}{\partial r}
+
\frac{dc_{i,\mathrm{ss}}}{dr},
\tag{TRISO-ML-405}
$$

and

$$
\frac{\partial^2c_i}{\partial r^2}
=
\frac{\partial^2v_i}{\partial r^2}
+
\frac{d^2c_{i,\mathrm{ss}}}{dr^2}.
\tag{TRISO-ML-406}
$$

The full layer equation is

$$
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
\tag{TRISO-ML-407}
$$

Substitute (TRISO-ML-404) through (TRISO-ML-406):

$$
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
\tag{TRISO-ML-408}
$$

Group transient and steady terms:

$$
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
\tag{TRISO-ML-409}
$$

The steady solution satisfies

$$
D_i
\left(
c_{i,\mathrm{ss}}''
+\frac2r c_{i,\mathrm{ss}}'
\right)
+
S_i
=
0.
\tag{TRISO-ML-410}
$$

Therefore

$$
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
\tag{TRISO-ML-411}
$$

### 12.2 Homogeneous transient conditions

At the centre, both \(c_1\) and \(c_{1,\mathrm{ss}}\) satisfy zero radial derivative. Therefore

$$
\boxed{
v_{1,r}(0,t)=0.
}
\tag{TRISO-ML-412}
$$

At interface \(r=r_i\), both the full and steady solutions satisfy concentration continuity. Subtracting the steady relation from the full relation gives

$$
\boxed{
v_i(r_i,t)=v_{i+1}(r_i,t).
}
\tag{TRISO-ML-413}
$$

Both the full and steady solutions also satisfy flux continuity. Subtraction gives

$$
\boxed{
-D_iv_i'(r_i,t)
=
-D_{i+1}v_{i+1}'(r_i,t).
}
\tag{TRISO-ML-414}
$$

At \(R\), the full Robin condition is

$$
-D_5c_5'(R,t)=h[c_5(R,t)-c_\infty].
\tag{TRISO-ML-415}
$$

The steady solution satisfies

$$
-D_5c_{5,\mathrm{ss}}'(R)
=
h[c_{5,\mathrm{ss}}(R)-c_\infty].
\tag{TRISO-ML-416}
$$

Subtract (TRISO-ML-416) from (TRISO-ML-415):

$$
\boxed{
-D_5v_5'(R,t)=hv_5(R,t).
}
\tag{TRISO-ML-417}
$$

For the initially empty source-driven problem,

$$
c_i(r,0)=0.
\tag{TRISO-ML-418}
$$

Hence

$$
\boxed{
v_i(r,0)
=
-c_{i,\mathrm{ss}}(r).
}
\tag{TRISO-ML-419}
$$

### 12.3 Separation in each layer

For one global transient mode, assume

$$
v_i(r,t)
=
\phi_i(r)e^{-\Lambda t}.
\tag{TRISO-ML-420}
$$

The same temporal factor must apply in every layer because the interface conditions couple the layer amplitudes at the same physical time. A single global eigenmode cannot use independent exponential time factors on the two sides of one interface and still satisfy the interface equations for all \(t\), except in a degenerate zero-amplitude case.

Differentiate with respect to time:

$$
\frac{\partial v_i}{\partial t}
=
-\Lambda\phi_i e^{-\Lambda t}.
\tag{TRISO-ML-421}
$$

Differentiate with respect to radius:

$$
\frac{\partial v_i}{\partial r}
=
\phi_i'e^{-\Lambda t}.
\tag{TRISO-ML-422}
$$

Differentiate again:

$$
\frac{\partial^2v_i}{\partial r^2}
=
\phi_i''e^{-\Lambda t}.
\tag{TRISO-ML-423}
$$

Substitute into (TRISO-ML-411):

$$
-\Lambda\phi_i e^{-\Lambda t}
=
D_i
\left(
\phi_i''
+\frac2r\phi_i'
\right)
e^{-\Lambda t}.
\tag{TRISO-ML-424}
$$

Cancel the non-zero exponential factor:

$$
-\Lambda\phi_i
=
D_i
\left(
\phi_i''
+\frac2r\phi_i'
\right).
\tag{TRISO-ML-425}
$$

Divide by \(D_i\):

$$
\phi_i''
+\frac2r\phi_i'
+
\frac{\Lambda}{D_i}\phi_i
=
0.
\tag{TRISO-ML-426}
$$

Define

$$
\boxed{
k_i^2=\frac{\Lambda}{D_i}.
}
\tag{TRISO-ML-427}
$$

Then

$$
[k_i^2]
=
\frac{\mathrm{s^{-1}}}{\mathrm{m^2\,s^{-1}}}
=
\mathrm{m^{-2}},
\tag{TRISO-ML-428}
$$

so

$$
[k_i]=\mathrm{m^{-1}}.
\tag{TRISO-ML-429}
$$

Each layer generally has a different \(k_i\), because each layer has a different \(D_i\), even though all layers in one global mode share the same \(\Lambda\).

The radial equation is

$$
\boxed{
\phi_i''
+\frac2r\phi_i'
+k_i^2\phi_i
=
0.
}
\tag{TRISO-ML-430}
$$

### 12.4 Apply the proven transformation \(u_i=r\phi_i\)

Section 10 proved that the transformation

$$
u_i=r\phi_i
\tag{TRISO-ML-431}
$$

maps (TRISO-ML-430) to

$$
\boxed{
u_i''+k_i^2u_i=0.
}
\tag{TRISO-ML-432}
$$

Therefore

$$
\boxed{
u_i(r)
=
A_i\sin(k_ir)+B_i\cos(k_ir).
}
\tag{TRISO-ML-433}
$$

In the kernel,

$$
\phi_1(r)=\frac{u_1(r)}{r}.
\tag{TRISO-ML-434}
$$

The cosine contribution \(B_1\cos(k_1r)/r\) diverges as \(r\to0\), exactly as proved in Section 10.

Therefore

$$
\boxed{
B_1=0.
}
\tag{TRISO-ML-435}
$$

Thus

$$
u_1(r)=A_1\sin(k_1r).
\tag{TRISO-ML-436}
$$

No coating layer contains the origin, so \(B_i\) is not forced to zero for \(i=2,3,4,5\).

### 12.5 Transform concentration continuity

At interface \(r=r_i\),

$$
\phi_i(r_i)=\phi_{i+1}(r_i).
\tag{TRISO-ML-437}
$$

Use \(\phi_i=u_i/r\):

$$
\frac{u_i(r_i)}{r_i}
=
\frac{u_{i+1}(r_i)}{r_i}.
\tag{TRISO-ML-438}
$$

Multiply by the common non-zero radius \(r_i\):

$$
\boxed{
u_i(r_i)=u_{i+1}(r_i).
}
\tag{TRISO-ML-439}
$$

### 12.6 Transform flux continuity

Start from

$$
\phi_i(r)=\frac{u_i(r)}{r}.
\tag{TRISO-ML-440}
$$

Differentiate using the quotient rule:

$$
\phi_i'(r)
=
\frac{ru_i'(r)-u_i(r)}{r^2}.
\tag{TRISO-ML-441}
$$

Separate the two terms:

$$
\boxed{
\phi_i'(r)
=
\frac{u_i'(r)}{r}
-
\frac{u_i(r)}{r^2}.
}
\tag{TRISO-ML-442}
$$

Flux continuity at \(r=r_i\) is

$$
-D_i\phi_i'(r_i)
=
-D_{i+1}\phi_{i+1}'(r_i).
\tag{TRISO-ML-443}
$$

Cancel the common minus sign:

$$
D_i\phi_i'(r_i)
=
D_{i+1}\phi_{i+1}'(r_i).
\tag{TRISO-ML-444}
$$

Substitute (TRISO-ML-442) on both sides:

$$
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
\tag{TRISO-ML-445}
$$

Multiply by the common factor \(r_i\):

$$
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
\tag{TRISO-ML-446}
$$

This is the transformed ideal flux-continuity condition.

### 12.7 Transform the outer Robin condition

The transient Robin condition is

$$
-D_5\phi_5'(R)=h\phi_5(R).
\tag{TRISO-ML-447}
$$

Use

$$
\phi_5'(R)
=
\frac{u_5'(R)}{R}
-
\frac{u_5(R)}{R^2},
\tag{TRISO-ML-448}
$$

and

$$
\phi_5(R)=\frac{u_5(R)}{R}.
\tag{TRISO-ML-449}
$$

Substitute both:

$$
-D_5
\left[
\frac{u_5'(R)}{R}
-
\frac{u_5(R)}{R^2}
\right]
=
h\frac{u_5(R)}{R}.
\tag{TRISO-ML-450}
$$

Multiply by \(R\):

$$
\boxed{
-D_5
\left[
u_5'(R)-\frac{u_5(R)}{R}
\right]
=
hu_5(R).
}
\tag{TRISO-ML-451}
$$

Equivalently,

$$
D_5u_5'(R)
+
\left(
h-\frac{D_5}{R}
\right)u_5(R)
=
0.
\tag{TRISO-ML-452}
$$

### 12.8 Count the unknown coefficients

Before centre regularity, five layers would provide ten coefficients:

$$
(A_1,B_1,A_2,B_2,A_3,B_3,A_4,B_4,A_5,B_5).
\tag{TRISO-ML-453}
$$

Centre regularity fixes

$$
B_1=0.
\tag{TRISO-ML-454}
$$

Therefore nine independent coefficients remain.

Define

$$
\boxed{
\mathbf a
=
(A_1,A_2,B_2,A_3,B_3,A_4,B_4,A_5,B_5)^T.
}
\tag{TRISO-ML-455}
$$

Thus

$$
\mathbf a\in\mathbb R^9
\tag{TRISO-ML-456}
$$

for real \(\Lambda>0\).

### 12.9 Define interface shorthand

For compact matrix notation, define at interface \(r=r_j\)

$$
s_{ij}=\sin(k_ir_j),
\qquad
c_{ij}=\cos(k_ir_j).
\tag{TRISO-ML-457}
$$

For a sine basis term,

$$
u_i=A_i\sin(k_ir),
\tag{TRISO-ML-458}
$$

and

$$
u_i'=A_ik_i\cos(k_ir).
\tag{TRISO-ML-459}
$$

Therefore the transformed flux factor for the sine basis at \(r_j\) is

$$
F^{(s)}_{ij}
=
D_i
\left(
k_ic_{ij}-\frac{s_{ij}}{r_j}
\right).
\tag{TRISO-ML-460}
$$

For a cosine basis term,

$$
u_i=B_i\cos(k_ir),
\tag{TRISO-ML-461}
$$

and

$$
u_i'=-B_ik_i\sin(k_ir).
\tag{TRISO-ML-462}
$$

Therefore its transformed flux factor is

$$
F^{(c)}_{ij}
=
D_i
\left(
-k_is_{ij}-\frac{c_{ij}}{r_j}
\right).
\tag{TRISO-ML-463}
$$

These quantities depend on \(\Lambda\) through \(k_i=\sqrt{\Lambda/D_i}\).

### 12.10 Assemble the global homogeneous coefficient system

There are two equations at each of the four internal interfaces:

- one concentration-continuity equation;
- one flux-continuity equation.

This gives

$$
4\times2=8
\tag{TRISO-ML-464}
$$

interface equations.

The outer Robin boundary supplies one more equation:

$$
8+1=9.
\tag{TRISO-ML-465}
$$

These nine equations determine the nine coefficients up to an arbitrary overall modal normalization when \(\Lambda\) is an eigenvalue.

Write

$$
\boxed{
\mathbf M(\Lambda)\mathbf a=\mathbf0.
}
\tag{TRISO-ML-466}
$$

The matrix dimensions are

$$
\boxed{
\mathbf M(\Lambda)\in\mathbb R^{9\times9}.
}
\tag{TRISO-ML-467}
$$

Using the coefficient order in (TRISO-ML-455), rows 1–2 correspond to \(r_1\), rows 3–4 to \(r_2\), rows 5–6 to \(r_3\), rows 7–8 to \(r_4\), and row 9 to the outer Robin condition.

The explicit matrix is

$$
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
\tag{TRISO-ML-468}
$$

The outer-row coefficients follow from (TRISO-ML-452).

For the sine basis,

$$
G_s
=
D_5k_5\cos(k_5R)
+
\left(
h-\frac{D_5}{R}
\right)
\sin(k_5R).
\tag{TRISO-ML-469}
$$

For the cosine basis,

$$
G_c
=
-D_5k_5\sin(k_5R)
+
\left(
h-\frac{D_5}{R}
\right)
\cos(k_5R).
\tag{TRISO-ML-470}
$$

Centre regularity does not appear as a matrix row because it has already been used to eliminate \(B_1\) from the unknown vector.

### 12.11 Global eigenvalue condition

For a generic value of \(\Lambda\), the homogeneous system

$$
\mathbf M(\Lambda)\mathbf a=\mathbf0
\tag{TRISO-ML-471}
$$

has only the trivial solution

$$
\mathbf a=\mathbf0
\tag{TRISO-ML-472}
$$

when \(\mathbf M\) is nonsingular.

A non-zero global eigenmode requires a non-trivial coefficient vector:

$$
\mathbf a\ne\mathbf0.
\tag{TRISO-ML-473}
$$

A square homogeneous linear system has a non-trivial solution only if its matrix is singular.

Therefore

$$
\boxed{
\det\mathbf M(\Lambda)=0.
}
\tag{TRISO-ML-474}
$$

Define

$$
\boxed{
F(\Lambda)=\det\mathbf M(\Lambda).
}
\tag{TRISO-ML-475}
$$

The global modal decay rates are the positive roots

$$
F(\Lambda_n)=0.
\tag{TRISO-ML-476}
$$

For each root \(\Lambda_n\), the layer wave numbers are

$$
\boxed{
k_{i,n}
=
\sqrt{\frac{\Lambda_n}{D_i}}.
}
\tag{TRISO-ML-477}
$$

Thus one global decay rate \(\Lambda_n\) generates five material-dependent spatial wave numbers.

### 12.12 Global conservative self-adjoint structure

Return to the separated transient equation before the substitution \(u_i=r\phi_i\).

Equation (TRISO-ML-425) is

$$
-\Lambda\phi_i
=
D_i
\left(
\phi_i''
+
\frac2r\phi_i'
\right).
\tag{TRISO-ML-478}
$$

Multiply by \(r^2\):

$$
-\Lambda r^2\phi_i
=
D_i
\left(
r^2\phi_i''
+
2r\phi_i'
\right).
\tag{TRISO-ML-479}
$$

Because \(D_i\) is constant inside layer \(i\),

$$
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
\tag{TRISO-ML-480}
$$

Therefore

$$
-\Lambda r^2\phi_i
=
\frac{d}{dr}
\left(
r^2D_i\phi_i'
\right).
\tag{TRISO-ML-481}
$$

Multiply by \(-1\):

$$
\boxed{
-\frac{d}{dr}
\left(
r^2D_i\phi_i'
\right)
=
\Lambda r^2\phi_i.
}
\tag{TRISO-ML-482}
$$

This is the conservative eigen-equation in layer \(i\).

Define the piecewise diffusivity

$$
D(r)=D_i,
\qquad
r_{i-1}<r<r_i.
\tag{TRISO-ML-483}
$$

Then the piecewise Sturm–Liouville coefficient is

$$
\boxed{
p(r)=r^2D(r).
}
\tag{TRISO-ML-484}
$$

There is no zeroth-order potential term:

$$
\boxed{
q(r)=0.
}
\tag{TRISO-ML-485}
$$

Comparing

$$
-\frac{d}{dr}
\left(
p(r)\phi'
\right)
+
q(r)\phi
=
\Lambda w(r)\phi
\tag{TRISO-ML-486}
$$

with (TRISO-ML-482) shows that the weight is

$$
\boxed{
w(r)=r^2.
}
\tag{TRISO-ML-487}
$$

Thus the weight \(r^2\) follows from the physical conservative eigen-equation; it is not imported by analogy with the homogeneous sphere.

### 12.13 Layerwise Lagrange identity for two global modes

Let global mode \(m\) have eigenvalue \(\Lambda_m\) and layer functions \(\phi_i^{(m)}\).

In layer \(i\),

$$
-\frac{d}{dr}
\left(
r^2D_i\frac{d\phi_i^{(m)}}{dr}
\right)
=
\Lambda_m r^2\phi_i^{(m)}.
\tag{TRISO-ML-488}
$$

Let global mode \(n\) have eigenvalue \(\Lambda_n\):

$$
-\frac{d}{dr}
\left(
r^2D_i\frac{d\phi_i^{(n)}}{dr}
\right)
=
\Lambda_n r^2\phi_i^{(n)}.
\tag{TRISO-ML-489}
$$

Multiply the \(m\)-equation by \(\phi_i^{(n)}\):

$$
-\phi_i^{(n)}
\frac{d}{dr}
\left(
r^2D_i\phi_i^{(m)\prime}
\right)
=
\Lambda_m r^2
\phi_i^{(m)}
\phi_i^{(n)}.
\tag{TRISO-ML-490}
$$

Multiply the \(n\)-equation by \(\phi_i^{(m)}\):

$$
-\phi_i^{(m)}
\frac{d}{dr}
\left(
r^2D_i\phi_i^{(n)\prime}
\right)
=
\Lambda_n r^2
\phi_i^{(m)}
\phi_i^{(n)}.
\tag{TRISO-ML-491}
$$

Subtract (TRISO-ML-491) from (TRISO-ML-490):

$$
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
\tag{TRISO-ML-492}
$$

Define

$$
P_i(r)=r^2D_i.
\tag{TRISO-ML-493}
$$

Use the product rule:

$$
\frac{d}{dr}
\left[
P_i
\left(
\phi_i^{(m)}\phi_i^{(n)\prime}
-
\phi_i^{(n)}\phi_i^{(m)\prime}
\right)
\right]
$$

$$
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
\tag{TRISO-ML-494}
$$

Therefore (TRISO-ML-492) becomes

$$
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
\tag{TRISO-ML-495}
$$

Integrate over layer \(i\):

$$
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
$$

$$
=
(\Lambda_m-\Lambda_n)
\int_{r_{i-1}}^{r_i}
r^2\phi_i^{(m)}\phi_i^{(n)}\,dr.
\tag{TRISO-ML-496}
$$

Evaluate the derivative integral:

$$
\left[
r^2D_i
\left(
\phi_i^{(m)}\phi_i^{(n)\prime}
-
\phi_i^{(n)}\phi_i^{(m)\prime}
\right)
\right]_{r_{i-1}}^{r_i}
$$

$$
=
(\Lambda_m-\Lambda_n)
\int_{r_{i-1}}^{r_i}
r^2\phi_i^{(m)}\phi_i^{(n)}\,dr.
\tag{TRISO-ML-497}
$$

### 12.14 Sum over all five layers

Define the layer boundary expression

$$
\mathcal B_i(r)
=
r^2D_i
\left(
\phi_i^{(m)}\phi_i^{(n)\prime}
-
\phi_i^{(n)}\phi_i^{(m)\prime}
\right).
\tag{TRISO-ML-498}
$$

Equation (TRISO-ML-497) is

$$
\mathcal B_i(r_i)
-
\mathcal B_i(r_{i-1})
=
(\Lambda_m-\Lambda_n)
\int_{r_{i-1}}^{r_i}
r^2\phi_i^{(m)}\phi_i^{(n)}\,dr.
\tag{TRISO-ML-499}
$$

Sum from \(i=1\) to \(5\):

$$
\sum_{i=1}^{5}
\left[
\mathcal B_i(r_i)
-
\mathcal B_i(r_{i-1})
\right]
$$

$$
=
(\Lambda_m-\Lambda_n)
\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2\phi_i^{(m)}\phi_i^{(n)}\,dr.
\tag{TRISO-ML-500}
$$

Write the left side explicitly:

$$
\mathcal B_1(r_1)-\mathcal B_1(0)
+
\mathcal B_2(r_2)-\mathcal B_2(r_1)
$$

$$
+
\mathcal B_3(r_3)-\mathcal B_3(r_2)
+
\mathcal B_4(r_4)-\mathcal B_4(r_3)
+
\mathcal B_5(R)-\mathcal B_5(r_4).
\tag{TRISO-ML-501}
$$

Group the four internal-interface contributions:

$$
-\mathcal B_1(0)
+
\left[
\mathcal B_1(r_1)-\mathcal B_2(r_1)
\right]
+
\left[
\mathcal B_2(r_2)-\mathcal B_3(r_2)
\right]
$$

$$
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
\tag{TRISO-ML-502}
$$

### 12.15 Explicit cancellation at one internal interface

Consider interface \(r=r_j\) between layers \(j\) and \(j+1\).

The contribution from the left layer is

$$
\mathcal B_j(r_j)
=
r_j^2D_j
\left[
\phi_j^{(m)}\phi_j^{(n)\prime}
-
\phi_j^{(n)}\phi_j^{(m)\prime}
\right]_{r_j}.
\tag{TRISO-ML-503}
$$

The contribution from the right layer enters with a minus sign:

$$
-\mathcal B_{j+1}(r_j)
=
-r_j^2D_{j+1}
\left[
\phi_{j+1}^{(m)}\phi_{j+1}^{(n)\prime}
-
\phi_{j+1}^{(n)}\phi_{j+1}^{(m)\prime}
\right]_{r_j}.
\tag{TRISO-ML-504}
$$

For ideal concentration continuity,

$$
\phi_j^{(m)}(r_j)
=
\phi_{j+1}^{(m)}(r_j),
\tag{TRISO-ML-505}
$$

and

$$
\phi_j^{(n)}(r_j)
=
\phi_{j+1}^{(n)}(r_j).
\tag{TRISO-ML-506}
$$

For ideal flux continuity,

$$
D_j\phi_j^{(m)\prime}(r_j)
=
D_{j+1}\phi_{j+1}^{(m)\prime}(r_j),
\tag{TRISO-ML-507}
$$

and

$$
D_j\phi_j^{(n)\prime}(r_j)
=
D_{j+1}\phi_{j+1}^{(n)\prime}(r_j).
\tag{TRISO-ML-508}
$$

Use (TRISO-ML-505) and (TRISO-ML-508) in the first product of (TRISO-ML-503):

$$
D_j
\phi_j^{(m)}
\phi_j^{(n)\prime}
=
\phi_{j+1}^{(m)}
D_{j+1}\phi_{j+1}^{(n)\prime}.
\tag{TRISO-ML-509}
$$

Use (TRISO-ML-506) and (TRISO-ML-507) in the second product:

$$
D_j
\phi_j^{(n)}
\phi_j^{(m)\prime}
=
\phi_{j+1}^{(n)}
D_{j+1}\phi_{j+1}^{(m)\prime}.
\tag{TRISO-ML-510}
$$

Therefore

$$
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
\tag{TRISO-ML-511}
$$

The right side of (TRISO-ML-511) is exactly

$$
\mathcal B_{j+1}(r_j).
\tag{TRISO-ML-512}
$$

Hence

$$
\boxed{
\mathcal B_j(r_j)-\mathcal B_{j+1}(r_j)=0.
}
\tag{TRISO-ML-513}
$$

The same argument applies independently at \(r_1,r_2,r_3,r_4\), including when adjacent diffusivities are unequal.

Thus all four internal-interface terms in (TRISO-ML-502) cancel pairwise.

### 12.16 Centre boundary contribution

At the centre,

$$
\mathcal B_1(0)
=
\lim_{r\to0}
r^2D_1
\left(
\phi_1^{(m)}\phi_1^{(n)\prime}
-
\phi_1^{(n)}\phi_1^{(m)\prime}
\right).
\tag{TRISO-ML-514}
$$

Regularity gives finite centre values for both eigenfunctions:

$$
|\phi_1^{(m)}(0)|<\infty,
\qquad
|\phi_1^{(n)}(0)|<\infty.
\tag{TRISO-ML-515}
$$

Spherical symmetry gives

$$
\phi_1^{(m)\prime}(0)=0,
\qquad
\phi_1^{(n)\prime}(0)=0.
\tag{TRISO-ML-516}
$$

For regular eigenfunctions, the bracketed quantity remains bounded as \(r\to0\).

Since

$$
r^2D_1\to0
\qquad
(r\to0),
\tag{TRISO-ML-517}
$$

we obtain

$$
\boxed{
\mathcal B_1(0)=0.
}
\tag{TRISO-ML-518}
$$

### 12.17 Outer Robin boundary contribution

At \(r=R\),

$$
\mathcal B_5(R)
=
R^2D_5
\left[
\phi_5^{(m)}(R)\phi_5^{(n)\prime}(R)
-
\phi_5^{(n)}(R)\phi_5^{(m)\prime}(R)
\right].
\tag{TRISO-ML-519}
$$

Both modes satisfy the same homogeneous Robin condition:

$$
-D_5\phi_5^{(m)\prime}(R)
=
h\phi_5^{(m)}(R),
\tag{TRISO-ML-520}
$$

and

$$
-D_5\phi_5^{(n)\prime}(R)
=
h\phi_5^{(n)}(R).
\tag{TRISO-ML-521}
$$

Solve the first condition for the derivative:

$$
D_5\phi_5^{(m)\prime}(R)
=
-h\phi_5^{(m)}(R).
\tag{TRISO-ML-522}
$$

Similarly,

$$
D_5\phi_5^{(n)\prime}(R)
=
-h\phi_5^{(n)}(R).
\tag{TRISO-ML-523}
$$

Substitute into (TRISO-ML-519):

$$
\mathcal B_5(R)
=
R^2
\left[
-h\phi_5^{(m)}(R)\phi_5^{(n)}(R)
+
h\phi_5^{(n)}(R)\phi_5^{(m)}(R)
\right].
\tag{TRISO-ML-524}
$$

The two products are identical and have opposite signs:

$$
\boxed{
\mathcal B_5(R)=0.
}
\tag{TRISO-ML-525}
$$

### 12.18 Global multilayer orthogonality

All terms on the left side of (TRISO-ML-500) now vanish:

- centre term by (TRISO-ML-518);
- four interface pairs by (TRISO-ML-513);
- outer Robin term by (TRISO-ML-525).

Therefore

$$
0
=
(\Lambda_m-\Lambda_n)
\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2
\phi_i^{(m)}(r)
\phi_i^{(n)}(r)
\,dr.
\tag{TRISO-ML-526}
$$

For distinct eigenvalues,

$$
\Lambda_m\ne\Lambda_n.
\tag{TRISO-ML-527}
$$

Divide by \(\Lambda_m-\Lambda_n\):

$$
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
\tag{TRISO-ML-528}
$$

Thus the correct global weight is

$$
\boxed{
w(r)=r^2.
}
\tag{TRISO-ML-529}
$$

Define the global piecewise eigenfunction

$$
\Phi_n(r)
=
\phi_i^{(n)}(r),
\qquad
r_{i-1}<r<r_i.
\tag{TRISO-ML-530}
$$

Then define the weighted inner product

$$
\boxed{
\langle f,g\rangle_w
=
\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2f_i(r)g_i(r)\,dr.
}
\tag{TRISO-ML-531}
$$

Global orthogonality is

$$
\boxed{
\langle\Phi_m,\Phi_n\rangle_w=0,
\qquad
m\ne n.
}
\tag{TRISO-ML-532}
$$

### 12.19 Modal norm and normalization

Define the norm of mode \(n\):

$$
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
\tag{TRISO-ML-533}
$$

If \(\phi_i\) carries concentration units, then

$$
[N_n]
=
\mathrm{m^3}
[\phi]^2.
\tag{TRISO-ML-534}
$$

If instead the eigenfunctions are chosen dimensionless, then

$$
[N_n]=\mathrm{m^3}.
\tag{TRISO-ML-535}
$$

The eigenvalue problem determines each coefficient vector only up to an arbitrary non-zero scale.

If

$$
\Phi_n\to\alpha_n\Phi_n,
\tag{TRISO-ML-536}
$$

then

$$
N_n\to\alpha_n^2N_n.
\tag{TRISO-ML-537}
$$

The corresponding modal amplitude transforms inversely:

$$
A_n\to\frac{A_n}{\alpha_n}.
\tag{TRISO-ML-538}
$$

Therefore the physical product

$$
A_n\Phi_n
\tag{TRISO-ML-539}
$$

is unchanged.

One convenient convention is unit weighted norm:

$$
N_n=1.
\tag{TRISO-ML-540}
$$

No such normalization is required for the projection formula below.

### 12.20 Project the initial transient state

For the source-driven benchmark,

$$
v_i(r,0)
=
-c_{i,\mathrm{ss}}(r).
\tag{TRISO-ML-541}
$$

Assume the global modal representation

$$
v_i(r,t)
=
\sum_{n=1}^{\infty}
A_n
\phi_i^{(n)}(r)
e^{-\Lambda_nt}.
\tag{TRISO-ML-542}
$$

At \(t=0\),

$$
e^{-\Lambda_n0}=1.
\tag{TRISO-ML-543}
$$

Therefore

$$
-c_{i,\mathrm{ss}}(r)
=
\sum_{n=1}^{\infty}
A_n\phi_i^{(n)}(r),
\qquad
r_{i-1}<r<r_i.
\tag{TRISO-ML-544}
$$

Multiply the equation in layer \(i\) by

$$
r^2\phi_i^{(m)}(r).
\tag{TRISO-ML-545}
$$

This gives

$$
-r^2
c_{i,\mathrm{ss}}(r)
\phi_i^{(m)}(r)
=
\sum_{n=1}^{\infty}
A_n
r^2
\phi_i^{(n)}(r)
\phi_i^{(m)}(r).
\tag{TRISO-ML-546}
$$

Integrate over layer \(i\):

$$
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
\tag{TRISO-ML-547}
$$

Sum all five layers:

$$
-\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2
c_{i,\mathrm{ss}}
\phi_i^{(m)}
\,dr
$$

$$
=
\sum_{n=1}^{\infty}
A_n
\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2
\phi_i^{(n)}
\phi_i^{(m)}
\,dr.
\tag{TRISO-ML-548}
$$

For every \(n\ne m\), global orthogonality gives

$$
\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2
\phi_i^{(n)}
\phi_i^{(m)}
\,dr
=
0.
\tag{TRISO-ML-549}
$$

Therefore only the \(n=m\) term remains:

$$
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
\tag{TRISO-ML-550}
$$

The denominator is \(N_m\):

$$
-\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2
c_{i,\mathrm{ss}}
\phi_i^{(m)}
\,dr
=
A_mN_m.
\tag{TRISO-ML-551}
$$

Divide by \(N_m>0\):

$$
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
\tag{TRISO-ML-552}
$$

This is the multilayer modal coefficient formula, conditional on the assumed completeness of the global eigenfunction family for representing the initial transient state.

### 12.21 Complete formal five-layer transient solution

Because

$$
c_i=v_i+c_{i,\mathrm{ss}},
\tag{TRISO-ML-553}
$$

substitute the modal expansion (TRISO-ML-542):

$$
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
\tag{TRISO-ML-554}
$$

#### Initial-time check

At \(t=0\),

$$
c_i(r,0)
=
c_{i,\mathrm{ss}}(r)
+
\sum_{n=1}^{\infty}
A_n\phi_i^{(n)}(r).
\tag{TRISO-ML-555}
$$

If the eigenfunction expansion represents the initial transient,

$$
\sum_{n=1}^{\infty}
A_n\phi_i^{(n)}(r)
=
-c_{i,\mathrm{ss}}(r).
\tag{TRISO-ML-556}
$$

Therefore

$$
c_i(r,0)=0.
\tag{TRISO-ML-557}
$$

This recovery is formal and depends on the completeness/convergence statement discussed below.

#### Long-time check

For every positive decay rate,

$$
\Lambda_n>0,
\tag{TRISO-ML-558}
$$

so

$$
e^{-\Lambda_nt}\to0
\qquad
(t\to\infty).
\tag{TRISO-ML-559}
$$

Formally,

$$
\boxed{
c_i(r,t)\to c_{i,\mathrm{ss}}(r)
\qquad
(t\to\infty).
}
\tag{TRISO-ML-560}
$$

#### Interface check

Every global eigenmode separately satisfies

$$
\phi_i^{(n)}(r_i)
=
\phi_{i+1}^{(n)}(r_i),
\tag{TRISO-ML-561}
$$

and

$$
D_i\phi_i^{(n)\prime}(r_i)
=
D_{i+1}\phi_{i+1}^{(n)\prime}(r_i).
\tag{TRISO-ML-562}
$$

A linear combination of such modes therefore preserves both homogeneous interface conditions.

Adding the steady solution restores the corresponding full interface conditions.

#### Centre check

Every kernel eigenfunction has \(B_1=0\), so it is regular at \(r=0\).

Therefore each transient mode is finite at the centre and satisfies the centre symmetry condition.

#### Outer-boundary check

Every mode satisfies

$$
-D_5\phi_5^{(n)\prime}(R)
=
h\phi_5^{(n)}(R).
\tag{TRISO-ML-563}
$$

Multiplication by the scalar factor \(A_ne^{-\Lambda_nt}\) preserves this relation.

Therefore the complete transient sum satisfies the homogeneous Robin condition whenever termwise boundary evaluation is justified.

Adding the steady solution restores the full Robin boundary condition.

### 12.22 Completeness and convergence status

The orthogonality result (TRISO-ML-528) was derived directly from the conservative differential equations, interface conditions, centre regularity, and outer Robin condition.

It does not require a completeness theorem.

[VERIFIED] Distinct global eigenmodes are orthogonal in the weighted inner product with \(w(r)=r^2\).

The projection algebra leading to (TRISO-ML-552) is also valid once an expansion in the eigenfunctions is admitted.

[DERIVED CONDITIONALLY] The modal projection formula follows from orthogonality under the assumption that the initial transient lies in the closure of the global eigenfunction span.

Completeness is a separate spectral statement.

Standard regular Sturm–Liouville completeness theorems provide completeness in an appropriate weighted \(L^2\) space for regular self-adjoint problems on finite intervals. The present TRISO problem is more delicate because:

1. \(p(r)=r^2D(r)\) vanishes at \(r=0\), so the centre is a singular endpoint;
2. \(D(r)\) is piecewise constant and discontinuous at four internal interfaces;
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

$$
D_1=D_2=D_3=D_4=D_5=D.
\tag{TRISO-ML-564}
$$

Then

$$
p(r)=r^2D
\tag{TRISO-ML-565}
$$

throughout the sphere.

Flux continuity becomes

$$
D\phi_i'(r_i)=D\phi_{i+1}'(r_i).
\tag{TRISO-ML-566}
$$

Cancel \(D>0\):

$$
\phi_i'(r_i)=\phi_{i+1}'(r_i).
\tag{TRISO-ML-567}
$$

Together with concentration continuity,

$$
\phi_i(r_i)=\phi_{i+1}(r_i),
\tag{TRISO-ML-568}
$$

the artificial internal material boundaries become transparent to the homogeneous eigenfunction.

The global inner product becomes

$$
\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2\phi_m\phi_n\,dr.
\tag{TRISO-ML-569}
$$

Because the five intervals partition \((0,R)\),

$$
\sum_{i=1}^{5}
\int_{r_{i-1}}^{r_i}
r^2\phi_m\phi_n\,dr
=
\int_0^R
r^2\phi_m\phi_n\,dr.
\tag{TRISO-ML-570}
$$

Therefore the multilayer orthogonality relation reduces to

$$
\boxed{
\int_0^R
r^2\phi_m(r)\phi_n(r)\,dr
=
0,
\qquad
m\ne n,
}
\tag{TRISO-ML-571}
$$

which is exactly the homogeneous spherical weight derived in Section 10.

### 12.24 Unequal-diffusivity interface check

The interface cancellation did not require

$$
D_j=D_{j+1}.
\tag{TRISO-ML-572}
$$

Instead it used the physical transmission condition

$$
D_j\phi_j'
=
D_{j+1}\phi_{j+1}'.
\tag{TRISO-ML-573}
$$

Therefore unequal adjacent diffusivities remain fully compatible with the global orthogonality proof.

The diffusivity discontinuity is carried by \(p(r)=r^2D(r)\), while the weight remains \(r^2\).

### 12.25 Updated analytical status

[VERIFIED] Five-layer steady analytical solution under the frozen ideal benchmark assumptions.

[VERIFIED] Explicit global eigenvalue system and determinant condition.

[VERIFIED] Piecewise conservative self-adjoint/Lagrange structure.

[VERIFIED] Pairwise cancellation of all four ideal-interface boundary terms.

[VERIFIED] Centre and Robin boundary cancellation.

[VERIFIED] Global orthogonality with weight \(r^2\).

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

$$
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
\tag{TRISO-DIS-100}
$$

[ASSUMPTION] \(D\) is constant over the local stencil.

[ASSUMPTION] The mesh is uniform for this Part-I benchmark.

### 13.2 Define the spatial mesh

Divide the interval

$$
0\le r\le R
\tag{TRISO-DIS-101}
$$

into \(N\) equal intervals.

Define

$$
\Delta r=\frac{R}{N}.
\tag{TRISO-DIS-102}
$$

The node locations are

$$
r_i=i\Delta r,
\qquad
i=0,1,\ldots,N.
\tag{TRISO-DIS-103}
$$

Therefore

$$
r_0=0,
\tag{TRISO-DIS-104}
$$

and

$$
r_N=R.
\tag{TRISO-DIS-105}
$$

### 13.3 Define the temporal mesh

Let the time step be

$$
\Delta t>0.
\tag{TRISO-DIS-106}
$$

Define

$$
t_j=j\Delta t,
\qquad
j=0,1,2,\ldots.
\tag{TRISO-DIS-107}
$$

Let

$$
C_i^j
\tag{TRISO-DIS-108}
$$

denote the numerical approximation to

$$
c(r_i,t_j).
\tag{TRISO-DIS-109}
$$

Thus

$$
C_i^j\approx c(r_i,t_j).
\tag{TRISO-DIS-110}
$$

The units remain

$$
[C_i^j]=\mathrm{mol\,m^{-3}}.
\tag{TRISO-DIS-111}
$$

### 13.4 Forward approximation of the time derivative

At fixed radius \(r_i\), Taylor-expand the exact solution from \(t_j\) to \(t_j+\Delta t\):

$$
c(r_i,t_j+\Delta t)
=
c(r_i,t_j)
+
\Delta t
\frac{\partial c}{\partial t}(r_i,t_j)
+
O(\Delta t^2).
\tag{TRISO-DIS-112}
$$

Subtract \(c(r_i,t_j)\) from both sides:

$$
c(r_i,t_j+\Delta t)-c(r_i,t_j)
=
\Delta t
\frac{\partial c}{\partial t}(r_i,t_j)
+
O(\Delta t^2).
\tag{TRISO-DIS-113}
$$

Divide by \(\Delta t\):

$$
\frac{
c(r_i,t_j+\Delta t)-c(r_i,t_j)
}{
\Delta t
}
=
\frac{\partial c}{\partial t}(r_i,t_j)
+
O(\Delta t).
\tag{TRISO-DIS-114}
$$

Rearrange:

$$
\frac{\partial c}{\partial t}(r_i,t_j)
=
\frac{
c(r_i,t_j+\Delta t)-c(r_i,t_j)
}{
\Delta t
}
+
O(\Delta t).
\tag{TRISO-DIS-115}
$$

Replace the exact nodal values by the numerical unknowns:

$$
\boxed{
\frac{\partial c}{\partial t}(r_i,t_j)
\approx
\frac{C_i^{j+1}-C_i^j}{\Delta t}.
}
\tag{TRISO-DIS-116}
$$

The forward-time approximation is first-order accurate in time.

### 13.5 Centred approximation of the first radial derivative

At fixed \(t_j\), Taylor-expand about \(r_i\) toward \(r_i+\Delta r\):

$$
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
\tag{TRISO-DIS-117}
$$

Taylor-expand toward \(r_i-\Delta r\):

$$
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
\tag{TRISO-DIS-118}
$$

Subtract the backward expansion from the forward expansion:

$$
c(r_i+\Delta r,t_j)
-
c(r_i-\Delta r,t_j)
=
2\Delta r\,c_{r,i}
+
\frac{\Delta r^3}{3}c_{rrr,i}
+
O(\Delta r^5).
\tag{TRISO-DIS-119}
$$

Divide by \(2\Delta r\):

$$
\frac{
c(r_i+\Delta r,t_j)-c(r_i-\Delta r,t_j)
}{
2\Delta r
}
=
c_{r,i}
+
O(\Delta r^2).
\tag{TRISO-DIS-120}
$$

Therefore

$$
\boxed{
\frac{\partial c}{\partial r}(r_i,t_j)
\approx
\frac{
C_{i+1}^j-C_{i-1}^j
}{
2\Delta r
}.
}
\tag{TRISO-DIS-121}
$$

### 13.6 Centred approximation of the second radial derivative

Add the two Taylor expansions (TRISO-DIS-117) and (TRISO-DIS-118):

$$
c(r_i+\Delta r,t_j)
+
c(r_i-\Delta r,t_j)
=
2c_i
+
\Delta r^2c_{rr,i}
+
O(\Delta r^4).
\tag{TRISO-DIS-122}
$$

Subtract \(2c_i\):

$$
c(r_i+\Delta r,t_j)
-
2c_i
+
c(r_i-\Delta r,t_j)
=
\Delta r^2c_{rr,i}
+
O(\Delta r^4).
\tag{TRISO-DIS-123}
$$

Divide by \(\Delta r^2\):

$$
\frac{
c(r_i+\Delta r,t_j)-2c_i+c(r_i-\Delta r,t_j)
}{
\Delta r^2
}
=
c_{rr,i}
+
O(\Delta r^2).
\tag{TRISO-DIS-124}
$$

Therefore

$$
\boxed{
\frac{\partial^2c}{\partial r^2}(r_i,t_j)
\approx
\frac{
C_{i-1}^j-2C_i^j+C_{i+1}^j
}{
\Delta r^2
}.
}
\tag{TRISO-DIS-125}
$$

### 13.7 Substitute the discrete derivatives into the PDE

Evaluate the continuous equation at \((r_i,t_j)\):

$$
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
\tag{TRISO-DIS-126}
$$

Substitute the forward-time approximation:

$$
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
\tag{TRISO-DIS-127}
$$

Substitute the centred second derivative:

$$
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
\tag{TRISO-DIS-128}
$$

Substitute the centred first derivative:

$$
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
\tag{TRISO-DIS-129}
$$

Cancel the factor \(2\) in the radial first-derivative term:

$$
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
\tag{TRISO-DIS-130}
$$

Use the uniform-mesh identity

$$
r_i=i\Delta r.
\tag{TRISO-DIS-131}
$$

Then

$$
r_i\Delta r
=
i\Delta r^2.
\tag{TRISO-DIS-132}
$$

Therefore

$$
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
\tag{TRISO-DIS-133}
$$

### 13.8 Collect neighbour coefficients

Expand the \(1/i\) term:

$$
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
\tag{TRISO-DIS-134}
$$

Collect the \(C_{i-1}^j\) terms:

$$
C_{i-1}^j
-
\frac1iC_{i-1}^j
=
\left(
1-\frac1i
\right)
C_{i-1}^j.
\tag{TRISO-DIS-135}
$$

Collect the \(C_{i+1}^j\) terms:

$$
C_{i+1}^j
+
\frac1iC_{i+1}^j
=
\left(
1+\frac1i
\right)
C_{i+1}^j.
\tag{TRISO-DIS-136}
$$

Thus

$$
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
\tag{TRISO-DIS-137}
$$

Multiply by \(\Delta t\):

$$
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
\tag{TRISO-DIS-138}
$$

Add \(C_i^j\) to both sides:

$$
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
\tag{TRISO-DIS-139}
$$

### 13.9 Define the Fourier number

Define

$$
\boxed{
\mathrm{Fo}
=
\frac{D\Delta t}{\Delta r^2}.
}
\tag{TRISO-DIS-140}
$$

Its units are

$$
[\mathrm{Fo}]
=
\frac{
\mathrm{m^2\,s^{-1}}\mathrm{s}
}{
\mathrm{m^2}
}
=
1.
\tag{TRISO-DIS-141}
$$

Therefore \(\mathrm{Fo}\) is dimensionless.

Substitute the definition into (TRISO-DIS-139):

$$
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
\tag{TRISO-DIS-142}
$$

Distribute \(\mathrm{Fo}\):

$$
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
\tag{TRISO-DIS-143}
$$

Collect the central coefficient:

$$
C_i^j-2\mathrm{Fo}C_i^j
=
(1-2\mathrm{Fo})C_i^j.
\tag{TRISO-DIS-144}
$$

Hence the original Ray interior FTCS update is

$$
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
\tag{TRISO-DIS-145}
$$

This is algebraically equivalent to the ordering used in the original notebook.

### 13.10 Dimensional check of the source increment

The source contribution is

$$
S_i^j\Delta t.
\tag{TRISO-DIS-146}
$$

Its units are

$$
[S_i^j\Delta t]
=
\mathrm{mol\,m^{-3}\,s^{-1}}
\times
\mathrm{s}.
\tag{TRISO-DIS-147}
$$

Therefore

$$
[S_i^j\Delta t]
=
\mathrm{mol\,m^{-3}},
\tag{TRISO-DIS-148}
$$

which matches the concentration units of every other term in (TRISO-DIS-145).

### 13.11 Local consistency order

The forward-time derivative has truncation error

$$
O(\Delta t).
\tag{TRISO-DIS-149}
$$

The centred first derivative has truncation error

$$
O(\Delta r^2).
\tag{TRISO-DIS-150}
$$

The centred second derivative has truncation error

$$
O(\Delta r^2).
\tag{TRISO-DIS-151}
$$

Therefore, away from \(r=0\), material interfaces, and the outer boundary, the local differential approximation is formally

$$
\boxed{
O(\Delta t)+O(\Delta r^2).
}
\tag{TRISO-DIS-152}
$$

This is a consistency statement only. It is not by itself a proof of stability or convergence.

### 13.12 Scope and limitations of the interior stencil

Equation (TRISO-DIS-145) assumes:

1. \(i\ge1\), so the \(1/i\) factor is defined;
2. the stencil lies inside one homogeneous constant-\(D\) region;
3. the spatial mesh is uniform;
4. the time step is uniform;
5. the source value \(S_i^j\) is known explicitly at time level \(j\).

It must **not** be applied unchanged:

- at \(r=0\);
- across a discontinuous material interface;
- at the outer boundary.

Those cases require separate derivations.

The original notebook's FTCS method is retained as a transparent deterministic benchmark. It does not redefine the production WOS method and it does not yet establish the preferred discretisation for the full five-layer discontinuous-\(D\) problem.

## 14. Centre discretisation

The interior stencil in Section 13 cannot be evaluated at \(i=0\) because it contains the factor \(1/i\).

The centre must therefore be derived from the regular spherical limit.

### 14.1 Continuous centre operator

Inside the homogeneous kernel, the radial diffusion operator is

$$
\mathcal L[c]
=
\frac{\partial^2c}{\partial r^2}
+
\frac2r\frac{\partial c}{\partial r}.
\tag{TRISO-DIS-200}
$$

For a sufficiently smooth spherically symmetric field,

$$
\frac{\partial c}{\partial r}(0,t)=0.
\tag{TRISO-DIS-201}
$$

Consider the apparently singular term

$$
\lim_{r\to0}
\frac2r
\frac{\partial c}{\partial r}.
\tag{TRISO-DIS-202}
$$

Because the numerator tends to zero,

$$
\lim_{r\to0}
\frac{\partial c/\partial r}{r}
$$

has the indeterminate form \(0/0\).

Apply L'Hôpital's rule with respect to \(r\):

$$
\lim_{r\to0}
\frac{\partial c/\partial r}{r}
=
\lim_{r\to0}
\frac{\partial^2c/\partial r^2}{1}.
\tag{TRISO-DIS-203}
$$

Therefore

$$
\lim_{r\to0}
\frac1r
\frac{\partial c}{\partial r}
=
\frac{\partial^2c}{\partial r^2}(0,t).
\tag{TRISO-DIS-204}
$$

Multiply by \(2\):

$$
\lim_{r\to0}
\frac2r
\frac{\partial c}{\partial r}
=
2
\frac{\partial^2c}{\partial r^2}(0,t).
\tag{TRISO-DIS-205}
$$

Hence

$$
\mathcal L[c](0,t)
=
\frac{\partial^2c}{\partial r^2}(0,t)
+
2\frac{\partial^2c}{\partial r^2}(0,t).
\tag{TRISO-DIS-206}
$$

Collect the terms:

$$
\boxed{
\mathcal L[c](0,t)
=
3
\frac{\partial^2c}{\partial r^2}(0,t).
}
\tag{TRISO-DIS-207}
$$

This is the exact smooth-origin limit of the spherical radial operator.

### 14.2 Introduce a symmetric ghost point

The uniform mesh has

$$
r_0=0,
\qquad
r_1=\Delta r.
\tag{TRISO-DIS-208}
$$

For the purpose of constructing a centred derivative at the origin, introduce a mathematical ghost point

$$
r_{-1}=-\Delta r.
\tag{TRISO-DIS-209}
$$

Spherical symmetry corresponds to an even extension of the radial concentration:

$$
c(-r,t)=c(r,t).
\tag{TRISO-DIS-210}
$$

Evaluate this at \(r=\Delta r\):

$$
c(-\Delta r,t)=c(\Delta r,t).
\tag{TRISO-DIS-211}
$$

In nodal notation,

$$
\boxed{
C_{-1}^j=C_1^j.
}
\tag{TRISO-DIS-212}
$$

The ghost point is a mathematical device. It does not represent a physical negative-radius material region.

### 14.3 Centre second derivative

Use the centred second-derivative formula at \(i=0\):

$$
\frac{\partial^2c}{\partial r^2}(0,t_j)
\approx
\frac{
C_{-1}^j-2C_0^j+C_1^j
}{
\Delta r^2
}.
\tag{TRISO-DIS-213}
$$

Substitute the symmetry relation \(C_{-1}^j=C_1^j\):

$$
\frac{\partial^2c}{\partial r^2}(0,t_j)
\approx
\frac{
C_1^j-2C_0^j+C_1^j
}{
\Delta r^2
}.
\tag{TRISO-DIS-214}
$$

Add the two \(C_1^j\) terms:

$$
\boxed{
\frac{\partial^2c}{\partial r^2}(0,t_j)
\approx
\frac{
2(C_1^j-C_0^j)
}{
\Delta r^2
}.
}
\tag{TRISO-DIS-215}
$$

### 14.4 Discrete spherical operator at the centre

From the exact centre limit,

$$
\mathcal L[c](0,t)
=
3c_{rr}(0,t).
\tag{TRISO-DIS-216}
$$

Substitute the discrete second derivative (TRISO-DIS-215):

$$
\mathcal L[c](0,t_j)
\approx
3
\frac{
2(C_1^j-C_0^j)
}{
\Delta r^2
}.
\tag{TRISO-DIS-217}
$$

Multiply the factors \(3\) and \(2\):

$$
\boxed{
\mathcal L[c](0,t_j)
\approx
\frac{
6(C_1^j-C_0^j)
}{
\Delta r^2
}.
}
\tag{TRISO-DIS-218}
$$

This is the origin of the factor \(6\) in the Ray notebook's centre update.

### 14.5 Apply the centre PDE

In the homogeneous kernel, the PDE at the centre is interpreted through the regular limit:

$$
\frac{\partial c}{\partial t}(0,t)
=
D_1\mathcal L[c](0,t)
+
S_0.
\tag{TRISO-DIS-219}
$$

Use the forward-time approximation:

$$
\frac{
C_0^{j+1}-C_0^j
}{
\Delta t
}
=
D_1\mathcal L[c](0,t_j)
+
S_0.
\tag{TRISO-DIS-220}
$$

Substitute (TRISO-DIS-218):

$$
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
\tag{TRISO-DIS-221}
$$

Multiply by \(\Delta t\):

$$
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
\tag{TRISO-DIS-222}
$$

Define the kernel Fourier number

$$
\boxed{
\mathrm{Fo}_1
=
\frac{D_1\Delta t}{\Delta r^2}.
}
\tag{TRISO-DIS-223}
$$

Substitute it:

$$
C_0^{j+1}-C_0^j
=
6\mathrm{Fo}_1
(C_1^j-C_0^j)
+
S_0\Delta t.
\tag{TRISO-DIS-224}
$$

Add \(C_0^j\) to both sides:

$$
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
\tag{TRISO-DIS-225}
$$

Expand the difference:

$$
C_0^{j+1}
=
C_0^j
+
6\mathrm{Fo}_1C_1^j
-
6\mathrm{Fo}_1C_0^j
+
S_0\Delta t.
\tag{TRISO-DIS-226}
$$

Collect the centre coefficient:

$$
\boxed{
C_0^{j+1}
=
(1-6\mathrm{Fo}_1)C_0^j
+
6\mathrm{Fo}_1C_1^j
+
S_0\Delta t.
}
\tag{TRISO-DIS-227}
$$

### 14.6 Consistency of the centre approximation

For a smooth even radial field, expand about \(r=0\):

$$
c(\Delta r,t)
=
c(0,t)
+
\frac{\Delta r^2}{2}c_{rr}(0,t)
+
\frac{\Delta r^4}{24}c_{rrrr}(0,t)
+
O(\Delta r^6).
\tag{TRISO-DIS-228}
$$

Subtract \(c(0,t)\):

$$
c(\Delta r,t)-c(0,t)
=
\frac{\Delta r^2}{2}c_{rr}(0,t)
+
\frac{\Delta r^4}{24}c_{rrrr}(0,t)
+
O(\Delta r^6).
\tag{TRISO-DIS-229}
$$

Multiply by \(2/\Delta r^2\):

$$
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
\tag{TRISO-DIS-230}
$$

Therefore the centre second derivative in (TRISO-DIS-215) is second-order accurate in space:

$$
c_{rr}(0,t)
=
\frac{
2(C_1-C_0)
}{
\Delta r^2
}
+
O(\Delta r^2).
\tag{TRISO-DIS-231}
$$

Multiplying by the exact factor \(3\) does not change the spatial order:

$$
\mathcal L[c](0,t)
=
\frac{
6(C_1-C_0)
}{
\Delta r^2
}
+
O(\Delta r^2).
\tag{TRISO-DIS-232}
$$

Combined with forward Euler time stepping, the centre equation is locally

$$
\boxed{
O(\Delta t)+O(\Delta r^2).
}
\tag{TRISO-DIS-233}
$$

### 14.7 Scope of the centre formula

The centre formula requires:

- spherical symmetry;
- sufficient smoothness at \(r=0\);
- the centre to lie inside one homogeneous kernel material;
- constant \(D_1\) over the centre stencil.

It does not determine the treatment of material interfaces or the outer surface.

The coefficient \(1-6\mathrm{Fo}_1\) will later enter the FTCS monotonicity/stability discussion, but no stability conclusion is drawn here.

## 15. Material-interface discretisation for discontinuous diffusivity

The homogeneous interior FTCS stencil from Section 13 must not be centred across a material interface because the diffusivity is discontinuous there.

This section derives an interface-aligned discrete constraint directly from the two physical interface conditions already established in the continuous model:

1. ideal concentration continuity;
2. diffusive flux continuity.

The derivation is for the frozen ideal benchmark with \(K=1\) and no explicit interfacial resistance.

### 15.1 Place a mesh node at the material interface

Let a material interface occur at

$$
r=r_I.
\tag{TRISO-DIS-300}
$$

Choose the mesh so that one numerical node lies exactly at the interface:

$$
r_I=r_{i}.
\tag{TRISO-DIS-301}
$$

Let the material immediately inside the interface have diffusivity

$$
D^-,
\tag{TRISO-DIS-302}
$$

and the material immediately outside have diffusivity

$$
D^+.
\tag{TRISO-DIS-303}
$$

For a uniform mesh,

$$
r_{i-1}=r_I-\Delta r,
\tag{TRISO-DIS-304}
$$

and

$$
r_{i+1}=r_I+\Delta r.
\tag{TRISO-DIS-305}
$$

The interface concentration is represented by one nodal unknown:

$$
C_I^j.
\tag{TRISO-DIS-306}
$$

### 15.2 Discrete concentration continuity

For the ideal \(K=1\) interface, the continuous condition is

$$
c^-(r_I,t)=c^+(r_I,t).
\tag{TRISO-DIS-307}
$$

Represent both limiting concentrations by the same interface unknown:

$$
c^-(r_I,t_j)\approx C_I^j,
\tag{TRISO-DIS-308}
$$

and

$$
c^+(r_I,t_j)\approx C_I^j.
\tag{TRISO-DIS-309}
$$

Thus concentration continuity is built directly into the interface-node representation.

No averaging of the two material concentrations is required because there is only one ideal-interface concentration degree of freedom.

### 15.3 Continuous flux continuity

The exact ideal-interface condition is

$$
-D^-
\left.
\frac{\partial c^-}{\partial r}
\right|_{r_I^-}
=
-D^+
\left.
\frac{\partial c^+}{\partial r}
\right|_{r_I^+}.
\tag{TRISO-DIS-310}
$$

The minus signs occur because the outward radial diffusive flux is

$$
J_r=-D\frac{\partial c}{\partial r}.
\tag{TRISO-DIS-311}
$$

### 15.4 Approximate the inner-side gradient

On the inner material side, use the interface node and its inner neighbour.

The radial distance is

$$
r_I-r_{i-1}=\Delta r.
\tag{TRISO-DIS-312}
$$

A first-order one-sided approximation is

$$
\left.
\frac{\partial c^-}{\partial r}
\right|_{r_I^-}
\approx
\frac{
C_I^j-C_{i-1}^j
}{
\Delta r
}.
\tag{TRISO-DIS-313}
$$

Therefore the inner-side outward flux is approximated by

$$
J_I^-
\approx
-D^-
\frac{
C_I^j-C_{i-1}^j
}{
\Delta r
}.
\tag{TRISO-DIS-314}
$$

### 15.5 Approximate the outer-side gradient

On the outer material side,

$$
r_{i+1}-r_I=\Delta r.
\tag{TRISO-DIS-315}
$$

The one-sided gradient is

$$
\left.
\frac{\partial c^+}{\partial r}
\right|_{r_I^+}
\approx
\frac{
C_{i+1}^j-C_I^j
}{
\Delta r
}.
\tag{TRISO-DIS-316}
$$

Therefore

$$
J_I^+
\approx
-D^+
\frac{
C_{i+1}^j-C_I^j
}{
\Delta r
}.
\tag{TRISO-DIS-317}
$$

### 15.6 Enforce discrete flux continuity

Set the two approximated fluxes equal:

$$
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
\tag{TRISO-DIS-318}
$$

Cancel the common factor \(-1/\Delta r\):

$$
D^-
\left(
C_I^j-C_{i-1}^j
\right)
=
D^+
\left(
C_{i+1}^j-C_I^j
\right).
\tag{TRISO-DIS-319}
$$

Expand both sides:

$$
D^-C_I^j-D^-C_{i-1}^j
=
D^+C_{i+1}^j-D^+C_I^j.
\tag{TRISO-DIS-320}
$$

Add \(D^+C_I^j\) to both sides:

$$
(D^-+D^+)C_I^j-D^-C_{i-1}^j
=
D^+C_{i+1}^j.
\tag{TRISO-DIS-321}
$$

Add \(D^-C_{i-1}^j\) to both sides:

$$
(D^-+D^+)C_I^j
=
D^-C_{i-1}^j
+
D^+C_{i+1}^j.
\tag{TRISO-DIS-322}
$$

Divide by \(D^-+D^+>0\):

$$
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
\tag{TRISO-DIS-323}
$$

This is an algebraic interface constraint, not a homogeneous-material FTCS update.

### 15.7 Unequal grid spacing

The same derivation can be retained if the interface is not equally spaced from its neighbouring nodes.

Define

$$
\Delta r^-=r_I-r_{i-1},
\tag{TRISO-DIS-324}
$$

and

$$
\Delta r^+=r_{i+1}-r_I.
\tag{TRISO-DIS-325}
$$

Then the two flux approximations are

$$
J_I^-
\approx
-D^-
\frac{
C_I-C_{i-1}
}{
\Delta r^-
},
\tag{TRISO-DIS-326}
$$

and

$$
J_I^+
\approx
-D^+
\frac{
C_{i+1}-C_I
}{
\Delta r^+
}.
\tag{TRISO-DIS-327}
$$

Flux continuity gives

$$
\frac{D^-}{\Delta r^-}
(C_I-C_{i-1})
=
\frac{D^+}{\Delta r^+}
(C_{i+1}-C_I).
\tag{TRISO-DIS-328}
$$

Expand:

$$
\frac{D^-}{\Delta r^-}C_I
-
\frac{D^-}{\Delta r^-}C_{i-1}
=
\frac{D^+}{\Delta r^+}C_{i+1}
-
\frac{D^+}{\Delta r^+}C_I.
\tag{TRISO-DIS-329}
$$

Collect the interface unknown:

$$
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
\tag{TRISO-DIS-330}
$$

Therefore

$$
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
\tag{TRISO-DIS-331}
$$

Equation (TRISO-DIS-323) is recovered when

$$
\Delta r^-=\Delta r^+=\Delta r.
\tag{TRISO-DIS-332}
$$

### 15.8 Equivalent two-node conductance and harmonic diffusivity

Sometimes the interface concentration is eliminated so that the flux is written directly between the two neighbouring material nodes.

Start from the inner-side flux relation:

$$
J_I
=
-D^-
\frac{
C_I-C_{i-1}
}{
\Delta r^-
}.
\tag{TRISO-DIS-333}
$$

Rearrange for the inner concentration drop:

$$
C_{i-1}-C_I
=
J_I
\frac{\Delta r^-}{D^-}.
\tag{TRISO-DIS-334}
$$

From the outer-side relation,

$$
J_I
=
-D^+
\frac{
C_{i+1}-C_I
}{
\Delta r^+
}.
\tag{TRISO-DIS-335}
$$

Rearrange:

$$
C_I-C_{i+1}
=
J_I
\frac{\Delta r^+}{D^+}.
\tag{TRISO-DIS-336}
$$

Add the two concentration drops:

$$
C_{i-1}-C_{i+1}
=
J_I
\left(
\frac{\Delta r^-}{D^-}
+
\frac{\Delta r^+}{D^+}
\right).
\tag{TRISO-DIS-337}
$$

Solve for the flux:

$$
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
\tag{TRISO-DIS-338}
$$

The denominator is the sum of the two local diffusion resistances per unit area.

Define the total node-to-node distance

$$
\Delta r_{\mathrm{tot}}
=
\Delta r^-+\Delta r^+.
\tag{TRISO-DIS-339}
$$

Define an effective diffusivity \(D_{\mathrm{eff}}\) by

$$
J_I
=
D_{\mathrm{eff}}
\frac{
C_{i-1}-C_{i+1}
}{
\Delta r_{\mathrm{tot}}
}.
\tag{TRISO-DIS-340}
$$

Equate (TRISO-DIS-338) and (TRISO-DIS-340):

$$
\frac{D_{\mathrm{eff}}}{\Delta r_{\mathrm{tot}}}
=
\frac1{
\dfrac{\Delta r^-}{D^-}
+
\dfrac{\Delta r^+}{D^+}
}.
\tag{TRISO-DIS-341}
$$

Multiply by \(\Delta r_{\mathrm{tot}}\):

$$
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
\tag{TRISO-DIS-342}
$$

For equal half-distances,

$$
\Delta r^-=\Delta r^+,
\tag{TRISO-DIS-343}
$$

the effective diffusivity becomes

$$
D_{\mathrm{eff}}
=
\frac{2}{
\dfrac1{D^-}
+
\dfrac1{D^+}
}.
\tag{TRISO-DIS-344}
$$

Thus

$$
\boxed{
D_{\mathrm{eff}}
=
\frac{
2D^-D^+
}{
D^-+D^+
}.
}
\tag{TRISO-DIS-345}
$$

This is the harmonic mean of the adjacent diffusivities.

It appears because diffusion resistances add in series; it is not an arbitrary averaging rule.

### 15.9 Limiting checks

If

$$
D^-=D^+=D,
\tag{TRISO-DIS-346}
$$

then (TRISO-DIS-323) becomes

$$
C_I
=
\frac{
DC_{i-1}+DC_{i+1}
}{
2D
}.
\tag{TRISO-DIS-347}
$$

Cancel \(D\):

$$
\boxed{
C_I
=
\frac{
C_{i-1}+C_{i+1}
}{2}.
}
\tag{TRISO-DIS-348}
$$

This is the expected linear interpolation for equal diffusivity and equal spacing.

If

$$
D^+\ll D^-,
\tag{TRISO-DIS-349}
$$

then the low-diffusivity outer material contributes the dominant diffusion resistance in (TRISO-DIS-338).

The interface concentration correspondingly approaches the concentration on the high-diffusivity side only according to the resistance-weighted relation; it is not obtained from an arithmetic diffusivity average.

### 15.10 Accuracy and scope

The one-sided interface gradients in (TRISO-DIS-313) and (TRISO-DIS-316) are first-order approximations to the limiting interface derivatives when used with only one neighbouring node on each side.

Therefore the algebraic interface relation derived here is conservative with respect to the approximated flux, but this particular gradient construction is not automatically second-order accurate at the interface.

A higher-order interface treatment would require additional same-material nodes or a finite-volume formulation with carefully defined face fluxes.

[IMPORTANT] The present result establishes the correct **discrete transmission logic**:

- one ideal-interface concentration;
- no differentiation of \(D\) through its jump;
- equal discrete flux on both sides;
- resistance-weighted, rather than arithmetic, diffusivity coupling.

It does not yet establish the globally preferred five-layer spatial discretisation.

## 16. Outer Robin boundary: ghost-point elimination and surface update

The outer surface is

$$
r_N=R.
\tag{TRISO-DIS-400}
$$

For the homogeneous Part-I FTCS benchmark, the outer material has constant diffusivity \(D\) and the external bulk concentration is

$$
c_\infty=0.
\tag{TRISO-DIS-401}
$$

The continuous Robin condition is therefore

$$
-D
\frac{\partial c}{\partial r}(R,t)
=
h c(R,t).
\tag{TRISO-DIS-402}
$$

The left side is the outward diffusive flux from the particle. The right side is the outward external mass-transfer flux.

### 16.1 Introduce the outer ghost point

The last physical node is

$$
r_N=R.
\tag{TRISO-DIS-403}
$$

The adjacent physical interior node is

$$
r_{N-1}=R-\Delta r.
\tag{TRISO-DIS-404}
$$

Introduce a mathematical ghost point outside the particle:

$$
r_{N+1}=R+\Delta r.
\tag{TRISO-DIS-405}
$$

Its numerical value is denoted

$$
C_{N+1}^j.
\tag{TRISO-DIS-406}
$$

The ghost value is not an external physical concentration. It is an algebraic device used to retain a centred derivative at \(r=R\).

### 16.2 Centred approximation of the surface gradient

At time \(t_j\), approximate the radial derivative by

$$
\frac{\partial c}{\partial r}(R,t_j)
\approx
\frac{
C_{N+1}^j-C_{N-1}^j
}{
2\Delta r
}.
\tag{TRISO-DIS-407}
$$

Substitute this approximation into the Robin condition:

$$
-D
\frac{
C_{N+1}^j-C_{N-1}^j
}{
2\Delta r
}
=
hC_N^j.
\tag{TRISO-DIS-408}
$$

Multiply both sides by \(2\Delta r\):

$$
-D
\left(
C_{N+1}^j-C_{N-1}^j
\right)
=
2h\Delta r\,C_N^j.
\tag{TRISO-DIS-409}
$$

Divide by \(-D\):

$$
C_{N+1}^j-C_{N-1}^j
=
-\frac{2h\Delta r}{D}C_N^j.
\tag{TRISO-DIS-410}
$$

Add \(C_{N-1}^j\) to both sides:

$$
C_{N+1}^j
=
C_{N-1}^j
-
\frac{2h\Delta r}{D}C_N^j.
\tag{TRISO-DIS-411}
$$

Define the dimensionless mesh transfer parameter

$$
\boxed{
\kappa
=
\frac{h\Delta r}{D}.
}
\tag{TRISO-DIS-412}
$$

Its units are

$$
[\kappa]
=
\frac{
\mathrm{m\,s^{-1}}\mathrm m
}{
\mathrm{m^2\,s^{-1}}
}
=
1.
\tag{TRISO-DIS-413}
$$

Therefore the ghost relation is

$$
\boxed{
C_{N+1}^j
=
C_{N-1}^j
-
2\kappa C_N^j.
}
\tag{TRISO-DIS-414}
$$

### 16.3 Surface approximation of the second radial derivative

Use the centred second derivative at node \(N\):

$$
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
\tag{TRISO-DIS-415}
$$

Substitute the ghost relation (TRISO-DIS-414):

$$
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
\tag{TRISO-DIS-416}
$$

Collect the two interior-neighbour terms:

$$
C_{N-1}^j+C_{N-1}^j
=
2C_{N-1}^j.
\tag{TRISO-DIS-417}
$$

Collect the two surface terms:

$$
-2C_N^j-2\kappa C_N^j
=
-2(1+\kappa)C_N^j.
\tag{TRISO-DIS-418}
$$

Therefore

$$
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
\tag{TRISO-DIS-419}
$$

### 16.4 Surface approximation of the first radial derivative

The centred derivative is

$$
\frac{\partial c}{\partial r}(R,t_j)
\approx
\frac{
C_{N+1}^j-C_{N-1}^j
}{
2\Delta r
}.
\tag{TRISO-DIS-420}
$$

Substitute (TRISO-DIS-414):

$$
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
\tag{TRISO-DIS-421}
$$

Cancel the two \(C_{N-1}^j\) terms:

$$
\frac{\partial c}{\partial r}(R,t_j)
\approx
-\frac{
2\kappa C_N^j
}{
2\Delta r
}.
\tag{TRISO-DIS-422}
$$

Cancel the factor \(2\):

$$
\boxed{
\frac{\partial c}{\partial r}(R,t_j)
\approx
-\frac{\kappa}{\Delta r}C_N^j.
}
\tag{TRISO-DIS-423}
$$

Using \(\kappa=h\Delta r/D\),

$$
-\frac{\kappa}{\Delta r}C_N^j
=
-\frac hD C_N^j.
\tag{TRISO-DIS-424}
$$

Thus the eliminated ghost relation reproduces the discrete Robin gradient exactly within the chosen centred boundary approximation.

### 16.5 Discrete spherical operator at the outer surface

The spherical operator is

$$
\mathcal L[c](R,t)
=
c_{rr}(R,t)
+
\frac2R c_r(R,t).
\tag{TRISO-DIS-425}
$$

Substitute the discrete second derivative (TRISO-DIS-419):

$$
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
\tag{TRISO-DIS-426}
$$

Substitute the discrete first derivative (TRISO-DIS-423):

$$
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
\tag{TRISO-DIS-427}
$$

Use

$$
R=N\Delta r.
\tag{TRISO-DIS-428}
$$

Therefore

$$
R\Delta r
=
N\Delta r^2.
\tag{TRISO-DIS-429}
$$

Hence

$$
\frac{2\kappa}{R\Delta r}
=
\frac{2\kappa}{N\Delta r^2}.
\tag{TRISO-DIS-430}
$$

Substitute:

$$
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
\tag{TRISO-DIS-431}
$$

Put the terms over the common denominator:

$$
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
\tag{TRISO-DIS-432}
$$

Collect the surface coefficient:

$$
(1+\kappa)
+
\frac{\kappa}{N}
=
1+\kappa
\left(
1+\frac1N
\right).
\tag{TRISO-DIS-433}
$$

Therefore

$$
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
\tag{TRISO-DIS-434}
$$

### 16.6 Apply the surface PDE

For the homogeneous benchmark,

$$
\frac{\partial c}{\partial t}(R,t)
=
D\mathcal L[c](R,t)
+
S_R.
\tag{TRISO-DIS-435}
$$

For the original homogeneous source benchmark,

$$
S_R=S_0.
\tag{TRISO-DIS-436}
$$

For the physical five-layer kernel-confined source problem, the OPyC source would instead be zero. This distinction must be preserved when the boundary formula is reused outside the Part-I benchmark.

Use forward Euler:

$$
\frac{
C_N^{j+1}-C_N^j
}{
\Delta t
}
=
D\mathcal L[c](R,t_j)
+
S_R^j.
\tag{TRISO-DIS-437}
$$

Substitute (TRISO-DIS-434):

$$
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
\tag{TRISO-DIS-438}
$$

Multiply by \(\Delta t\):

$$
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
\tag{TRISO-DIS-439}
$$

Use

$$
\mathrm{Fo}
=
\frac{D\Delta t}{\Delta r^2}.
\tag{TRISO-DIS-440}
$$

Then

$$
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
\tag{TRISO-DIS-441}
$$

Add \(C_N^j\) to both sides:

$$
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
\tag{TRISO-DIS-442}
$$

Collect the surface coefficient:

$$
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
\tag{TRISO-DIS-443}
$$

This recovers the surface update written in the original Ray derivation.

### 16.7 Dimensional checks

The Fourier number is dimensionless:

$$
[\mathrm{Fo}]=1.
\tag{TRISO-DIS-444}
$$

The mesh transfer parameter is dimensionless:

$$
[\kappa]=1.
\tag{TRISO-DIS-445}
$$

Therefore every coefficient multiplying a concentration in (TRISO-DIS-443) is dimensionless.

The source increment has units

$$
[S_R\Delta t]
=
\mathrm{mol\,m^{-3}}.
\tag{TRISO-DIS-446}
$$

Thus every term in the update has concentration units.

### 16.8 Limiting checks

If

$$
h=0,
\tag{TRISO-DIS-447}
$$

then

$$
\kappa=0.
\tag{TRISO-DIS-448}
$$

The ghost relation becomes

$$
C_{N+1}^j=C_{N-1}^j.
\tag{TRISO-DIS-449}
$$

This is the expected symmetric zero-gradient ghost condition for a zero-flux Neumann boundary.

The surface update becomes

$$
C_N^{j+1}
=
2\mathrm{Fo}C_{N-1}^j
+
(1-2\mathrm{Fo})C_N^j
+
S_R^j\Delta t.
\tag{TRISO-DIS-450}
$$

For finite \(h>0\), increasing \(h\) increases \(\kappa\), which strengthens the outward-transfer contribution in the surface coefficient.

The formal limit \(h\to\infty\) is more delicate for this explicit ghost formulation because

$$
\kappa=\frac{h\Delta r}{D}\to\infty
\tag{TRISO-DIS-451}
$$

at fixed \(\Delta r\).

The continuum Robin condition approaches the absorbing Dirichlet condition \(c(R,t)=0\), but the explicit ghost update becomes increasingly stiff rather than automatically turning into a numerically well-conditioned Dirichlet update.

Therefore an absorbing Dirichlet boundary should be imposed directly when that is the intended numerical model, rather than obtained by taking \(\kappa\to\infty\) in (TRISO-DIS-443).

### 16.9 Accuracy of the boundary approximation

Taylor-expand around \(R\):

$$
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
\tag{TRISO-DIS-452}
$$

Similarly,

$$
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
\tag{TRISO-DIS-453}
$$

Subtract the second expansion from the first:

$$
c(R+\Delta r)-c(R-\Delta r)
=
2\Delta r\,c_r(R)
+
\frac{\Delta r^3}{3}c_{rrr}(R)
+
O(\Delta r^5).
\tag{TRISO-DIS-454}
$$

Divide by \(2\Delta r\):

$$
\frac{
c(R+\Delta r)-c(R-\Delta r)
}{
2\Delta r
}
=
c_r(R)
+
O(\Delta r^2).
\tag{TRISO-DIS-455}
$$

Thus the centred Robin derivative used to eliminate the ghost point is second-order accurate for a sufficiently smooth continuation through the boundary.

However, the ghost value itself is a mathematical continuation, not a physical exterior solution.

The complete surface update combines this boundary approximation with the centred second derivative and forward Euler time step. Its local consistency is therefore formally

$$
O(\Delta t)+O(\Delta r^2)
\tag{TRISO-DIS-456}
$$

for the homogeneous smooth benchmark, subject to the Robin boundary compatibility used in the ghost construction.

### 16.10 Scope

Equation (TRISO-DIS-443) belongs to the homogeneous Part-I FTCS benchmark.

For the physical five-layer problem, the same derivational structure may be applied to the OPyC layer by replacing \(D\) with \(D_5\) and using the physically selected outer source and boundary parameters.

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

$$
E_i^j
=
C_i^j-\widetilde C_i^j.
\tag{TRISO-DIS-500}
$$

Because both solutions have the same additive source, subtraction cancels that source.

Thus the error/perturbation equation is homogeneous.

Stability of the linear time-marching operator can therefore be analysed from the source-free amplification step

$$
\mathbf E^{j+1}
=
\mathbf A\mathbf E^j.
\tag{TRISO-DIS-501}
$$

### 17.2 Interior-row coefficients

For an interior homogeneous node, Section 13 gives

$$
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
\tag{TRISO-DIS-502}
$$

Define

$$
a_i
=
\mathrm{Fo}
\left(
1-\frac1i
\right),
\tag{TRISO-DIS-503}
$$

$$
b_i
=
1-2\mathrm{Fo},
\tag{TRISO-DIS-504}
$$

and

$$
d_i
=
\mathrm{Fo}
\left(
1+\frac1i
\right).
\tag{TRISO-DIS-505}
$$

For \(i\ge1\),

$$
1-\frac1i\ge0.
\tag{TRISO-DIS-506}
$$

Since \(\mathrm{Fo}\ge0\),

$$
a_i\ge0.
\tag{TRISO-DIS-507}
$$

Similarly,

$$
1+\frac1i>0,
\tag{TRISO-DIS-508}
$$

so

$$
d_i\ge0.
\tag{TRISO-DIS-509}
$$

The central coefficient is non-negative when

$$
1-2\mathrm{Fo}\ge0.
\tag{TRISO-DIS-510}
$$

Rearrange:

$$
2\mathrm{Fo}\le1.
\tag{TRISO-DIS-511}
$$

Therefore

$$
\boxed{
\mathrm{Fo}\le\frac12.
}
\tag{TRISO-DIS-512}
$$

Now add the three interior coefficients:

$$
a_i+b_i+d_i
=
\mathrm{Fo}\left(1-\frac1i\right)
+
1-2\mathrm{Fo}
+
\mathrm{Fo}\left(1+\frac1i\right).
\tag{TRISO-DIS-513}
$$

Expand:

$$
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
\tag{TRISO-DIS-514}
$$

Cancel the radial terms:

$$
-\frac{\mathrm{Fo}}i
+
\frac{\mathrm{Fo}}i
=
0.
\tag{TRISO-DIS-515}
$$

Cancel the diffusion contributions:

$$
\mathrm{Fo}-2\mathrm{Fo}+\mathrm{Fo}=0.
\tag{TRISO-DIS-516}
$$

Hence

$$
\boxed{
a_i+b_i+d_i=1.
}
\tag{TRISO-DIS-517}
$$

When (TRISO-DIS-512) holds, an interior update is therefore a convex combination of the previous-time neighbouring values.

### 17.3 Centre-row coefficients

Section 14 gives the homogeneous centre error update

$$
E_0^{j+1}
=
(1-6\mathrm{Fo})E_0^j
+
6\mathrm{Fo}E_1^j.
\tag{TRISO-DIS-518}
$$

The neighbour coefficient satisfies

$$
6\mathrm{Fo}\ge0.
\tag{TRISO-DIS-519}
$$

The centre coefficient is non-negative when

$$
1-6\mathrm{Fo}\ge0.
\tag{TRISO-DIS-520}
$$

Therefore

$$
6\mathrm{Fo}\le1.
\tag{TRISO-DIS-521}
$$

Hence

$$
\boxed{
\mathrm{Fo}\le\frac16.
}
\tag{TRISO-DIS-522}
$$

The centre-row sum is

$$
(1-6\mathrm{Fo})+6\mathrm{Fo}.
\tag{TRISO-DIS-523}
$$

Therefore

$$
\boxed{
(1-6\mathrm{Fo})+6\mathrm{Fo}=1.
}
\tag{TRISO-DIS-524}
$$

Under (TRISO-DIS-522), the centre row is also a convex combination.

### 17.4 Robin surface-row coefficients

Section 16 gives the homogeneous surface error update

$$
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
\tag{TRISO-DIS-525}
$$

The interior-neighbour coefficient is

$$
2\mathrm{Fo}\ge0.
\tag{TRISO-DIS-526}
$$

The surface coefficient is non-negative when

$$
1
-
2\mathrm{Fo}
\left(
1+\kappa\left(1+\frac1N\right)
\right)
\ge0.
\tag{TRISO-DIS-527}
$$

Move the second term to the other side:

$$
1
\ge
2\mathrm{Fo}
\left(
1+\kappa\left(1+\frac1N\right)
\right).
\tag{TRISO-DIS-528}
$$

For \(h\ge0\), \(D>0\), and \(\Delta r>0\),

$$
\kappa\ge0.
\tag{TRISO-DIS-529}
$$

Therefore the denominator below is positive.

Divide:

$$
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
\tag{TRISO-DIS-530}
$$

Now calculate the surface-row sum:

$$
2\mathrm{Fo}
+
1
-
2\mathrm{Fo}
\left(
1+\kappa\left(1+\frac1N\right)
\right).
\tag{TRISO-DIS-531}
$$

Expand the last term:

$$
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
\tag{TRISO-DIS-532}
$$

Cancel \(2\mathrm{Fo}-2\mathrm{Fo}\):

$$
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
\tag{TRISO-DIS-533}
$$

For \(\kappa\ge0\),

$$
\text{surface row sum}\le1.
\tag{TRISO-DIS-534}
$$

Under the non-negativity restriction (TRISO-DIS-530),

$$
\text{surface row sum}\ge0.
\tag{TRISO-DIS-535}
$$

Thus the Robin row is sub-convex: part of the previous concentration can leave through the external boundary.

### 17.5 Combined coefficient-non-negativity condition

The three restrictions are

$$
\mathrm{Fo}\le\frac12
\tag{TRISO-DIS-536}
$$

for ordinary interior nodes,

$$
\mathrm{Fo}\le\frac16
\tag{TRISO-DIS-537}
$$

for the centre,

and

$$
\mathrm{Fo}
\le
\frac{
1
}{
2\left[
1+\kappa\left(1+\frac1N\right)
\right]
}
\tag{TRISO-DIS-538}
$$

for the Robin surface.

Because

$$
\frac16<\frac12,
\tag{TRISO-DIS-539}
$$

the interior bound is never the controlling restriction once the centre node is included.

Therefore a sufficient coefficient-non-negativity condition for this homogeneous benchmark is

$$
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
\tag{TRISO-DIS-540}
$$

This is the corrected form of the original notebook restriction.

### 17.6 Amplification matrix

Collect the nodal errors into

$$
\mathbf E^j
=
(E_0^j,E_1^j,\ldots,E_N^j)^T.
\tag{TRISO-DIS-541}
$$

The homogeneous update is

$$
\boxed{
\mathbf E^{j+1}
=
\mathbf A\mathbf E^j.
}
\tag{TRISO-DIS-542}
$$

The matrix has size

$$
\mathbf A\in\mathbb R^{(N+1)\times(N+1)}.
\tag{TRISO-DIS-543}
$$

The centre row contains

$$
A_{0,0}=1-6\mathrm{Fo},
\tag{TRISO-DIS-544}
$$

and

$$
A_{0,1}=6\mathrm{Fo}.
\tag{TRISO-DIS-545}
$$

For an ordinary interior row \(i\),

$$
A_{i,i-1}
=
\mathrm{Fo}
\left(
1-\frac1i
\right),
\tag{TRISO-DIS-546}
$$

$$
A_{i,i}
=
1-2\mathrm{Fo},
\tag{TRISO-DIS-547}
$$

and

$$
A_{i,i+1}
=
\mathrm{Fo}
\left(
1+\frac1i
\right).
\tag{TRISO-DIS-548}
$$

The Robin surface row contains

$$
A_{N,N-1}=2\mathrm{Fo},
\tag{TRISO-DIS-549}
$$

and

$$
A_{N,N}
=
1
-
2\mathrm{Fo}
\left(
1+\kappa\left(1+\frac1N\right)
\right).
\tag{TRISO-DIS-550}
$$

All other entries are zero for this homogeneous tridiagonal benchmark.

### 17.7 \(\ell_\infty\) stability under the monotonicity restriction

The induced infinity norm of a matrix is

$$
\|\mathbf A\|_\infty
=
\max_i
\sum_j
|A_{ij}|.
\tag{TRISO-DIS-551}
$$

Under (TRISO-DIS-540), every non-zero entry of \(\mathbf A\) is non-negative.

Therefore

$$
|A_{ij}|=A_{ij}.
\tag{TRISO-DIS-552}
$$

For the centre row, the row sum is exactly

$$
1.
\tag{TRISO-DIS-553}
$$

For each ordinary interior row, the row sum is exactly

$$
1.
\tag{TRISO-DIS-554}
$$

For the Robin row, the row sum is at most

$$
1.
\tag{TRISO-DIS-555}
$$

Therefore

$$
\boxed{
\|\mathbf A\|_\infty\le1.
}
\tag{TRISO-DIS-556}
$$

Apply the matrix norm inequality:

$$
\|\mathbf E^{j+1}\|_\infty
=
\|\mathbf A\mathbf E^j\|_\infty
\le
\|\mathbf A\|_\infty
\|\mathbf E^j\|_\infty.
\tag{TRISO-DIS-557}
$$

Use (TRISO-DIS-556):

$$
\|\mathbf E^{j+1}\|_\infty
\le
\|\mathbf E^j\|_\infty.
\tag{TRISO-DIS-558}
$$

Repeat the inequality over \(j\) steps:

$$
\boxed{
\|\mathbf E^j\|_\infty
\le
\|\mathbf E^0\|_\infty.
}
\tag{TRISO-DIS-559}
$$

Thus the coefficient restriction (TRISO-DIS-540) is not merely a heuristic: for this assembled homogeneous benchmark it is a sufficient condition for non-amplification in the discrete \(\ell_\infty\) norm.

### 17.8 Positivity and discrete maximum-principle interpretation

Suppose

$$
E_i^j\ge0
\tag{TRISO-DIS-560}
$$

for every node.

Under (TRISO-DIS-540), every amplification coefficient is non-negative.

Therefore every component of

$$
\mathbf E^{j+1}
=
\mathbf A\mathbf E^j
\tag{TRISO-DIS-561}
$$

is also non-negative.

Hence the homogeneous update preserves non-negativity.

For an interior or centre row whose coefficients sum to one, the new value lies between the minimum and maximum of the contributing old values.

At the Robin boundary, the row sum is less than or equal to one because concentration can leave the domain.

This is the precise monotonicity/maximum-principle content of the notebook's coefficient argument.

### 17.9 Spectral-radius consequence

For every square matrix,

$$
\rho(\mathbf A)
\le
\|\mathbf A\|
\tag{TRISO-DIS-562}
$$

for any induced matrix norm.

Using the infinity norm,

$$
\rho(\mathbf A)
\le
\|\mathbf A\|_\infty.
\tag{TRISO-DIS-563}
$$

Under (TRISO-DIS-540),

$$
\|\mathbf A\|_\infty\le1.
\tag{TRISO-DIS-564}
$$

Therefore

$$
\boxed{
\rho(\mathbf A)\le1.
}
\tag{TRISO-DIS-565}
$$

So the monotonicity restriction also provides a sufficient spectral-radius bound for this homogeneous assembled amplification matrix.

This does **not** prove that (TRISO-DIS-540) is necessary for spectral stability.

There may be parameter values with some negative coefficients for which

$$
\rho(\mathbf A)\le1.
\tag{TRISO-DIS-566}
$$

Determining the exact necessary-and-sufficient spectral stability region would require analysis of the eigenvalues of the specific amplification matrix.

### 17.10 Relation to the standard explicit-Euler eigenvalue condition

Write a semi-discrete diffusion system abstractly as

$$
\frac{d\mathbf C}{dt}
=
\mathbf L\mathbf C.
\tag{TRISO-DIS-567}
$$

Forward Euler gives

$$
\mathbf C^{j+1}
=
\left(
\mathbf I+\Delta t\,\mathbf L
\right)
\mathbf C^j.
\tag{TRISO-DIS-568}
$$

Therefore

$$
\mathbf A
=
\mathbf I+\Delta t\,\mathbf L.
\tag{TRISO-DIS-569}
$$

If \(\lambda_\ell\) is an eigenvalue of \(\mathbf L\), then the corresponding amplification eigenvalue is

$$
g_\ell
=
1+\Delta t\,\lambda_\ell.
\tag{TRISO-DIS-570}
$$

For a real non-positive diffusion eigenvalue,

$$
\lambda_\ell\le0,
\tag{TRISO-DIS-571}
$$

the scalar forward-Euler stability requirement is

$$
|1+\Delta t\,\lambda_\ell|\le1.
\tag{TRISO-DIS-572}
$$

For real \(\lambda_\ell\le0\), this is equivalent to

$$
-1
\le
1+\Delta t\,\lambda_\ell
\le
1.
\tag{TRISO-DIS-573}
$$

Subtract \(1\):

$$
-2
\le
\Delta t\,\lambda_\ell
\le
0.
\tag{TRISO-DIS-574}
$$

Because \(\lambda_\ell<0\) for a decaying mode, the lower inequality gives

$$
\Delta t
\le
\frac{2}{|\lambda_\ell|}.
\tag{TRISO-DIS-575}
$$

For all modes,

$$
\boxed{
\Delta t
\le
\frac{2}{
\max_\ell|\lambda_\ell|
}
}
\tag{TRISO-DIS-576}
$$

would be the exact scalar forward-Euler restriction if the relevant semi-discrete operator has a real non-positive spectrum and is diagonalizable in the norm under consideration.

The present section does not compute \(\max|\lambda_\ell|\) for the spherical matrix.

Therefore (TRISO-DIS-576) is a framework for a sharper spectral analysis, not a completed numerical bound for this benchmark.

### 17.11 Important limitation: material interfaces

The amplification matrix in Sections 17.6–17.10 corresponds to the homogeneous benchmark using:

- the centre row from Section 14;
- homogeneous interior rows from Section 13;
- the Robin surface row from Section 16.

The full five-layer problem contains discontinuous diffusivities and the interface transmission treatment from Section 15.

Its assembled operator is therefore different.

The bound

$$
\mathrm{Fo}
\le
\min
\left\{
\frac16,
\frac1{
2[1+\kappa(1+1/N)]
}
\right\}
\tag{TRISO-DIS-577}
$$

must **not** be presented as a proved stability bound for an arbitrary five-layer discretisation.

A five-layer stability condition depends on the final chosen conservative spatial discretisation, the layer-specific diffusivities, mesh spacings, and interface treatment.

### 17.12 Correct status of the original notebook claim

The original notebook's coefficient argument is therefore classified as follows.

[VERIFIED] Interior coefficient non-negativity requires

$$
\mathrm{Fo}\le\frac12.
\tag{TRISO-DIS-578}
$$

[VERIFIED] Centre coefficient non-negativity requires

$$
\mathrm{Fo}\le\frac16.
\tag{TRISO-DIS-579}
$$

[VERIFIED] Robin-surface coefficient non-negativity requires

$$
\mathrm{Fo}
\le
\frac1{
2[1+\kappa(1+1/N)]
}.
\tag{TRISO-DIS-580}
$$

[VERIFIED] Their minimum is a sufficient monotonicity condition for the homogeneous benchmark.

[VERIFIED] Under that same condition, the assembled homogeneous amplification matrix satisfies

$$
\|\mathbf A\|_\infty\le1
\tag{TRISO-DIS-581}
$$

and consequently

$$
\rho(\mathbf A)\le1.
\tag{TRISO-DIS-582}
$$

[NOT PROVED] The condition is necessary for spectral stability.

[NOT APPLICABLE WITHOUT RE-DERIVATION] The same bound is the exact stability condition for the final discontinuous-\(D\), five-layer discretisation.

## 18. Implementation provenance

Repository: theodoreOnzGit/outram-park-backend.

Default branch: main.

Ray derivation branch: ray-triso-derivation.

Ray branch tip: fd9d5c287ce77ab04a89fa4764fcf0a6b9b3e90e.

The WOS implementation is inherited from main. Path-specific history for interface.rs, walk_on_spheres.rs and sphere_fpt.rs traces the relevant modules to commit 159d1fc4bd56916f5d598da80fa45a2311d6e594.

Relevant functions:

constructive_solid_geometry/mod.rs → TrisoCell::new, TrisoCell::new_crp6_geometry, TrisoCell::get_triso_region, TrisoCell::try_get_diffusion_coefficient.

first_passage/walk_on_spheres.rs → WoSWalker, step_multilayer, walk_until_released, nearest_interface_distance, shell_bounds, sample_uniform_in_ball.

first_passage/interface.rs → does_transmit.

first_passage/sphere_fpt.rs → homogeneous first-passage distribution and numerical lookup/interpolation.

verification_and_validation/crp6_case1_kernel_release_vs_crank.md → existing single-layer verification record.

docs/buffer_clt_failure_analysis.md → legacy Gaussian interface-overshoot analysis.

## 19. Foundation gate

All 33 original displayed equations are now accounted for in the central register.

The mathematical middle is now continuous from physical problem through conservation, Fickian transport, spherical reduction, source/decay convention, conditions, Part-I steady and transient analysis, Part-II steady analytical solution, Part-II transient eigenvalue framework, and Part-I FTCS.

Remaining foundation gaps are explicitly retained:

- physical source/decay/trapping closure;
- physical partition/interfacial-resistance choice;
- five-layer transient modal coefficient convergence;
- complete FTCS spectral stability proof;
- complete equation-to-code verification;
- WOS-to-continuum transient verification.

Review 1 is not yet the next step. The required gate is an independent mathematical audit of this consolidated foundation against the original 33 equations and the status register.