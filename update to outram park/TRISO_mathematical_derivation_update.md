# TRISO Source-Term Modelling — Mathematical Derivation Progress Update

**Prepared for:** Outram Park / supervisor update  
**Working repository:** `weewzr/Derivation-of-TRISO-Source-Term-Modelling`  
**Research state used for this update:** current `main` through commit `dafa2d9e33577e0c32daee31d99e503218f00cd8`  
**Purpose:** concise mathematical handover of the derivation and verification work completed so far. This document is an update, not a claim that the final five-layer Cs-137 release problem is closed.

---

## 1. Current modelling objective

The particle is represented as five concentric spherical regions,

[
	ext{Kernel}ightarrow	ext{Buffer}ightarrow	ext{IPyC}ightarrow	ext{SiC}ightarrow	ext{OPyC}ightarrow	ext{Release}.
]

For the current verification problem, the tracked species is initially loaded in the kernel, there is no continuing source after (t=0), the outer surface is absorbing, and the ideal-interface base case uses (K=1).

The central observable is the cumulative released fraction

[
F(t)=P(T_{m release}le t).
]

The project currently uses two first-principles Method-2 representations:

1. conservative spherical finite volume (FV);
2. stochastic first-passage / walk-on-spheres (WOS), including mathematically verified acceleration and a Markov-renewal reduction.

---

## 2. Governing diffusion equation

Starting from conservation in an arbitrary fixed control volume,

[
rac{d}{dt}int_V c,dV
=
-int_{partial V}mathbf Jcdotmathbf n,dA
+int_V S,dV,
]

and Fick's law

[
mathbf J=-D
abla c,
]

the conservative diffusion equation is

[
oxed{
rac{partial c}{partial t}
=

ablacdot(D
abla c)+S
}.
]

Under spherical symmetry, (c=c(r,t)),

[
oxed{
rac{partial c}{partial t}
=
rac{1}{r^2}
rac{partial}{partial r}
left(
r^2D(r,t)rac{partial c}{partial r}
ight)
+S(r,t)
}.
]

Within a homogeneous layer (i), where (D_i) is constant,

[
oxed{
rac{partial c_i}{partial t}
=
D_i
left(
rac{partial^2c_i}{partial r^2}
+
rac{2}{r}rac{partial c_i}{partial r}
ight)
+S_i
}.
]

The conservative form is retained across material discontinuities.

---

## 3. Centre, interface and outer conditions

Spherical symmetry gives

[
oxed{
left.rac{partial c}{partial r}ight|_{r=0}=0
}.
]

For an ideal zero-storage interface between layers (i) and (i+1), conservation gives flux continuity,

[
oxed{
-D_ileft.rac{partial c_i}{partial r}ight|_{r_k^-}
=
-D_{i+1}left.rac{partial c_{i+1}}{partial r}ight|_{r_k^+}
}.
]

For the present (K=1) base case, concentration continuity is additionally imposed,

[
oxed{
c_i(r_k,t)=c_{i+1}(r_k,t)
}.
]

The current release benchmark uses an absorbing outer boundary,

[
oxed{c(R,t)=0}.
]

---

## 4. Conservative finite-volume reference

For a spherical cell (j),

[
V_j=rac{4pi}{3}
left(r_{j+1/2}^3-r_{j-1/2}^3ight),
]

and face area

[
A_{j+1/2}=4pi r_{j+1/2}^2.
]

The semi-discrete conservative balance is

[
V_jrac{dc_j}{dt}
=
A_{j-1/2}J_{j-1/2}
-
A_{j+1/2}J_{j+1/2}
+
V_jS_j.
]

At a material interface, the face conductance is obtained from the two half-cell resistances,

[
oxed{
G_{j+1/2}
=
rac{A_{j+1/2}}
{Delta r_j/(2D_j)+Delta r_{j+1}/(2D_{j+1})}
}.
]

This preserves the discontinuous diffusivity without differentiating (D) through the interface.

The deterministic FV formulation has undergone independent accuracy/convergence review. In the controlled two-layer transient benchmark, the finest spatial change in (F(t)) was approximately (2.77	imes10^{-4}), the finest temporal change approximately (3.42	imes10^{-6}), and conservation residuals were near machine precision.

---

## 5. Direct WOS and the interface bottleneck

For the production finite-capture WOS rule with (K=1), the transmission probability from material (i) to (j) is

[
oxed{
p_{iightarrow j}
=
rac{D_j}{D_i+D_j}
}.
]

The controlled two-layer WOS/FV verification passed independent review at the declared Monte-Carlo precision. However, the frozen five-layer Cs-137 problem is computationally pathological.

For the frozen diffusivities

[
[D_K,D_B,D_I,D_S,D_O]
=
[1.2502983	imes10^{-13},
10^{-8},
4.0622991	imes10^{-14},
9.2277732	imes10^{-17},
4.0622991	imes10^{-14}]
 {m m^2,s^{-1}},
]

one important outward probability is approximately

[
p_{Bightarrow I}
approx4.06	imes10^{-6}.
]

Direct five-layer diagnostics consequently spent essentially all computational effort in repeated Buffer/interface events and did not reach SiC.

This motivated exact first-passage acceleration rather than changing the physical diffusivities or interface law.

---

## 6. Exact spherical-shell first-passage derivation

For homogeneous diffusion in a spherical shell

[
a<r<b,
]

let (T) be the first time either boundary is reached.

Define the joint outer-exit Laplace transform

[
G_b(r,s)
=
E_rleft[e^{-sT}mathbf 1_{{R_T=b}}ight].
]

The backward equation is

[
Dleft(G_b''+rac{2}{r}G_b'ight)=sG_b,
qquad
G_b(a,s)=0,
qquad
G_b(b,s)=1.
]

With

[
v=rG_b,qquad lambda=sqrt{rac{s}{D}},
]

the equation becomes

[
v''-lambda^2v=0.
]

Therefore

[
oxed{
G_b(r,s)
=
rac{b}{r}
rac{sinh[lambda(r-a)]}
{sinh[lambda(b-a)]}
}
]

and similarly for inner exit,

[
oxed{
G_a(r,s)
=
rac{a}{r}
rac{sinh[lambda(b-r)]}
{sinh[lambda(b-a)]}
}.
]

At (sightarrow0),

[
oxed{
P_r(R_T=b)
=
rac{b}{r}rac{r-a}{b-a}
=
rac{1/a-1/r}{1/a-1/b}
}
]

and

[
P_r(R_T=a)=1-P_r(R_T=b).
]

The conditional first-passage-time transforms are

[
E_r[e^{-sT}mid R_T=b]
=
rac{G_b(r,s)}{G_b(r,0)}
]

and

[
E_r[e^{-sT}mid R_T=a]
=
rac{G_a(r,s)}{G_a(r,0)}.
]

The shell kernel has been checked against analytical exit probabilities, conditional moments, conditional CDFs, and direct supervisor WOS.

---

## 7. Accelerated exact-interface renewal process

The verified accelerated process analytically marginalises diffusion inside each homogeneous region.

A renewal consists of

[
	ext{region first passage}
ightarrow
	ext{interface}
ightarrow
	ext{transmit/reflect}
ightarrow
	ext{reinsertion}
ightarrow
	ext{next first passage}.
]

The controlled two-layer accelerated implementation reproduced the FV/direct-WOS release behaviour and reduced the event count by about (8.68	imes) relative to the direct 100-nm WOS benchmark.

A key distinction established by independent review is:

- **Process A:** finite-capture production WOS;
- **Process B:** accelerated exact-interface renewal;
- **Process C:** deterministic matrix reduction of Process B.

Processes B and C are the exact-equivalence target. Process A is empirically compatible with B/C in the controlled benchmark but is not pathwise identical.

For (a=50,mu{m m}) and (epsilon=0.1,mu{m m}), the fraction of uniform-volume initial births lying in the finite capture shell is

[
oxed{
1-left(1-rac{epsilon}{a}ight)^3
=
0.005988008
}
]

or about (0.5988%).

---

## 8. Eight-state interface Markov-renewal reduction

The five-layer particle has four internal interfaces. Each interface has two post-interface material-side states, giving eight transient states:

[
S_0,ldots,S_7,
]

plus absorbing release (A).

For example,

[
S_0=	ext{Kernel side of Kernel/Buffer},
qquad
S_1=	ext{Buffer side of Kernel/Buffer},
]

and the pattern continues through Buffer/IPyC, IPyC/SiC and SiC/OPyC.

Let

[
Phi_i(s)
=
E_i[e^{-sT_{m release}}].
]

Each first-step equation has the form

[
oxed{
Phi_i(s)
=
sum_jK_{ij}(s)Phi_j(s)+B_i(s)
}.
]

Thus

[
oxed{
oldsymbolPhi(s)
=
K(s)oldsymbolPhi(s)+mathbf B(s)
}
]

and therefore

[
oxed{
oldsymbolPhi(s)
=
[I-K(s)]^{-1}mathbf B(s)
}.
]

The inverse is the finite-state analogue of summing the geometric renewal series

[
[I-K]^{-1}
=
I+K+K^2+cdots,
]

so arbitrarily many reflection/return cycles can be summed algebraically instead of sampled one at a time.

For uniform-in-volume kernel births,

[
f_R(r)=rac{3r^2}{R_0^3},
]

and the initial release transform is

[
oxed{
Phi_{m init}(s)
=
int_0^{R_0}
rac{3r^2}{R_0^3}
H_K(r,s)
left[
p_{Kightarrow K}Phi_0(s)
+
p_{Kightarrow B}Phi_1(s)
ight]dr
}.
]

The release CDF is related to the transform by

[
oxed{
mathcal L{F}(s)
=
rac{Phi_{m init}(s)}{s}
}.
]

---

## 9. Controlled matrix reconciliation

For the controlled two-layer problem, the matrix reduction was compared directly with explicit accelerated renewal at

[
s=[0,0.25,0.5,1,2,4] {m s^{-1}}.
]

With (N=20{,}000) explicit renewal histories, all matrix-versus-renewal discrepancies were within approximately (1.004) Monte-Carlo standard errors.

The normalization check gave

[
oxed{Phi_{m init}(0)=1}
]

to floating-point precision.

Linear-system residuals were of order

[
10^{-16}	ext{--}10^{-18}.
]

The independent closure audit therefore cleared the formulation for controlled multistate verification, while keeping the hard five-layer Cs-137 calculation open.

---

## 10. Genuine multistate verification

A controlled three-layer benchmark was then introduced:

[
R=[50,75,100] mu{m m},
]

[
D=[10^{-9},2	imes10^{-9},5	imes10^{-9}]
 {m m^2,s^{-1}}.
]

This gives four transient interface-side states and one absorbing release state.

The successful verification run used (N=20{,}000) explicit Process-B histories and transform points

[
s=[0,1,2,5,10,20] {m s^{-1}}.
]

The largest matrix-versus-explicit-renewal standardized discrepancy was

[
oxed{|z|_{max}=0.729}.
]

All transient states normalized to one at (s=0), and linear-system residuals were near machine precision.

An independently refined FV reference was also generated. The explicit renewal CDF agreed with FV with

[
oxed{mathrm{RMS}approx1.04	imes10^{-3}}
]

and

[
oxed{max|F_B-F_{m FV}|approx1.78	imes10^{-3}}.
]

No nonzero-time discrepancy exceeded about (1.03) binomial standard errors.

This demonstrates that the matrix construction works beyond a 2-state toy/control problem and composes correctly across multiple material interfaces in a non-pathological regime.

---

## 11. Current status and remaining work

The mathematical progression is now

[
	ext{conservation/Fick}
ightarrow
	ext{spherical multilayer PDE}
ightarrow
	ext{conservative FV}
ightarrow
	ext{direct WOS}
ightarrow
	ext{exact shell first passage}
ightarrow
	ext{accelerated renewal}
ightarrow
	ext{matrix Markov-renewal reduction}.
]

Current verified/independently reviewed milestones include:

- continuous spherical diffusion foundation;
- conservative discrete FV formulation and convergence;
- controlled direct-WOS/FV interface compatibility;
- homogeneous shell exit/time kernel;
- controlled accelerated two-layer coupling;
- finite-capture versus exact-interface distinction;
- controlled two-layer matrix reconciliation;
- controlled three-layer multistate matrix/FV verification.

The unresolved target remains the hard frozen five-layer Cs-137 release-time problem.

In particular,

[
oxed{	ext{R2-WOS-02 remains open}}
]

for the five-layer direct-production censoring issue, and

[
oxed{	ext{R2-B01 remains open}}
]

until a defensible integrated five-layer release-time result is established.

The hard five-layer matrix must still undergo the remaining gate/independent-review process before it is treated as a final Method-2 result.

---

## 12. Supervisor-facing summary

The main scientific development is that the project no longer attempts to brute-force millions of repeated rare WOS interface events.

Instead, the same first-passage/interface physics is being reduced to a finite Markov-renewal system whose transform solution is

[
oxed{
oldsymbolPhi(s)
=
[I-K(s)]^{-1}mathbf B(s)
}.
]

This reduction has now passed controlled two-layer and genuine multistate three-layer checks, while the distinction between finite-capture production WOS, exact-interface renewal, and continuum PDE relevance is kept explicit.

The next goal is not to alter the physical model, but to complete the remaining verification gate before applying the reduced formulation to the frozen five-layer Cs-137 release problem.
