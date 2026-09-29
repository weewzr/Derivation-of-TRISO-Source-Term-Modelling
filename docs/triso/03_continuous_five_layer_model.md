# Continuous Five-Layer TRISO Transport Model

## Status

This document is the current continuous-model foundation on `ray/triso-foundation`.

It deliberately separates the homogeneous-sphere Part I benchmark from the physical five-layer TRISO model. No numerical discretisation is defined here.

Evidence classes follow `MASTER_INSTRUCTIONS.md`:
- [EXACT] mathematical identity or conservation statement
- [ASSUMPTION] project modelling assumption
- [CONSTITUTIVE] constitutive transport relation
- [APPROXIMATION] approximation
- [SOURCE] externally sourced model/evidence
- [INFERRED FROM CODE] not currently applicable because the repository has no scientific implementation

Unresolved items are retained explicitly.

## 1. Physical domain

The idealised particle contains five concentric material regions:

| Layer | Material | Radial domain |
|---|---|---|
| 1 | fuel kernel | (0 < r < r_1) |
| 2 | buffer | (r_1 < r < r_2) |
| 3 | IPyC | (r_2 < r < r_3) |
| 4 | SiC | (r_3 < r < r_4) |
| 5 | OPyC | (r_4 < r < r_5=R) |

The notation (a=r_1) used in the original notebook is retained as the kernel radius.

[ASSUMPTION] The layers are concentric and spherically symmetric.

[ASSUMPTION] Each layer is materially homogeneous for the purposes of the present continuous derivation, while its diffusivity may differ from neighbouring layers.

## 2. Transported quantity

Let (c(mathbf{x},t)) denote the molar concentration of one selected fission-product species.

[
[c] = mathrm{mol,m^{-3}}.
]

Let (mathbf J(mathbf{x},t)) denote its molar diffusive flux.

[
[mathbf J] = mathrm{mol,m^{-2},s^{-1}}.
]

Let (S_{mathrm{gen}}) denote its volumetric generation rate.

[
[S_{mathrm{gen}}] = mathrm{mol,m^{-3},s^{-1}}.
]

Let (R_{mathrm{net}}) denote any additional volumetric reaction/exchange contribution not already included in (S_{mathrm{gen}}).

[
[R_{mathrm{net}}] = mathrm{mol,m^{-3},s^{-1}}.
]

[QUESTION FOR SUPERVISOR] The final species model must specify which physical processes belong in (R_{mathrm{net}}): radioactive decay, trapping/release, or other exchange mechanisms.

## 3. Local conservation statement

Consider an arbitrary material control volume (V).

[EXACT] The rate of accumulation equals the net inward transport plus volumetric production:

[
rac{mathrm d}{mathrm dt}int_V c,mathrm dV
=
-int_{partial V}mathbf Jcdotmathbf n,mathrm dA
+
int_Vleft(S_{mathrm{gen}}+R_{mathrm{net}}ight),mathrm dV.
	ag{TRISO-GOV-001}
]

Here (mathbf n) is the outward unit normal.

[EXACT] Apply the divergence theorem to the flux term:

[
int_{partial V}mathbf Jcdotmathbf n,mathrm dA
=
int_V
ablacdotmathbf J,mathrm dV.
	ag{TRISO-GOV-002}
]

[EXACT] Substitute this identity into the conservation statement:

[
rac{mathrm d}{mathrm dt}int_V c,mathrm dV
=
-int_V
ablacdotmathbf J,mathrm dV
+
int_Vleft(S_{mathrm{gen}}+R_{mathrm{net}}ight),mathrm dV.
	ag{TRISO-GOV-003}
]

[EXACT] Combine the volume integrals:

[
rac{mathrm d}{mathrm dt}int_V c,mathrm dV
=
int_V
left[
-
ablacdotmathbf J
+
S_{mathrm{gen}}
+
R_{mathrm{net}}
ight]mathrm dV.
	ag{TRISO-GOV-004}
]

[EXACT] For a fixed control volume, move the time derivative inside the volume integral:

[
rac{mathrm d}{mathrm dt}int_V c,mathrm dV
=
int_Vrac{partial c}{partial t},mathrm dV.
	ag{TRISO-GOV-005}
]

[EXACT] Therefore

[
int_V
left[
rac{partial c}{partial t}
+

ablacdotmathbf J
-
S_{mathrm{gen}}
-
R_{mathrm{net}}
ight]mathrm dV
=
0.
	ag{TRISO-GOV-006}
]

[EXACT] Because the control volume is arbitrary, the local conservation equation is

[
oxed{
rac{partial c}{partial t}
+

ablacdotmathbf J
=
S_{mathrm{gen}}
+
R_{mathrm{net}}.
}
	ag{TRISO-GOV-007}
]

This is the conservation starting point for the rest of the derivation.

## 4. Constitutive transport law

[CONSTITUTIVE] Under the concentration-driven Fickian model,

[
mathbf J=-D
abla c.
	ag{TRISO-GOV-008}
]

For the five-layer particle, diffusivity is represented as a piecewise material property:

[
D(mathbf x,t)
=
D_i(mathbf x,t),
qquad
mathbf xinOmega_i.
	ag{TRISO-GOV-009}
]

[ASSUMPTION] The present model uses local Fickian diffusion and does not add a Soret/thermal-diffusion flux.

[ SOURCE] IAEA guidance describes fission-product transport in reactor materials using an effective Fickian diffusion equation and notes that effective diffusion coefficients are commonly represented as temperature dependent; the BISON TRISO workshop similarly assigns diffusion-coefficient material properties by material block. citeturn772970search35turn629504search1

## 5. General governing PDE

[EXACT] Substitute the Fickian relation into the conservation law:

[
rac{partial c}{partial t}
+

ablacdot(-D
abla c)
=
S_{mathrm{gen}}+R_{mathrm{net}}.
	ag{TRISO-GOV-010}
]

[EXACT] Use linearity of the divergence operator:

[
rac{partial c}{partial t}
-

ablacdot(D
abla c)
=
S_{mathrm{gen}}+R_{mathrm{net}}.
	ag{TRISO-GOV-011}
]

[EXACT] Rearrange:

[
oxed{
rac{partial c}{partial t}
=

ablacdot(D
abla c)
+
S_{mathrm{gen}}
+
R_{mathrm{net}}.
}
	ag{TRISO-GOV-012}
]

This is the conservative variable-diffusivity form.

It is the form that must be specialised to the five-layer particle. The constant-(D) equation in the original notebook is a special case, not the general multilayer equation.

## 6. Spherical representation

For a radially symmetric field,

[
c(mathbf x,t)=c(r,t).
	ag{TRISO-SPH-001}
]

For a piecewise radial diffusivity,

[
D(mathbf x,t)=D(r,t).
	ag{TRISO-SPH-002}
]

The radial Fickian flux is

[
J_r=-D(r,t)rac{partial c}{partial r}.
	ag{TRISO-SPH-003}
]

The divergence of a purely radial vector field is

[

ablacdotmathbf J
=
rac{1}{r^2}
rac{partial}{partial r}
left(r^2J_right).
	ag{TRISO-SPH-004}
]

[EXACT] Substitute the radial flux:

[

ablacdotmathbf J
=
rac{1}{r^2}
rac{partial}{partial r}
left(
-r^2D(r,t)rac{partial c}{partial r}
ight).
	ag{TRISO-SPH-005}
]

[EXACT] Insert this in the local conservation equation:

[
rac{partial c}{partial t}
-
rac{1}{r^2}
rac{partial}{partial r}
left(
-r^2D(r,t)rac{partial c}{partial r}
ight)
=
S_{mathrm{gen}}+R_{mathrm{net}}.
	ag{TRISO-SPH-006}
]

[EXACT] Therefore,

[
oxed{
rac{partial c}{partial t}
=
rac{1}{r^2}
rac{partial}{partial r}
left(
r^2D(r,t)rac{partial c}{partial r}
ight)
+
S_{mathrm{gen}}(r,t)
+
R_{mathrm{net}}(r,t).
}
	ag{TRISO-SPH-007}
]

This form is retained across material layers. No derivative of a discontinuous (D) is taken across an interface.

## 7. Piecewise material model

Define the radial domains

[
Omega_1=(0,r_1),
quad
Omega_2=(r_1,r_2),
quad
Omega_3=(r_2,r_3),
quad
Omega_4=(r_3,r_4),
quad
Omega_5=(r_4,R).
	ag{TRISO-GOV-013}
]

Define the piecewise diffusivity

[
D(r,t)
=
egin{cases}
D_1(t), & rinOmega_1,\
D_2(t), & rinOmega_2,\
D_3(t), & rinOmega_3,\
D_4(t), & rinOmega_4,\
D_5(t), & rinOmega_5.
end{cases}
	ag{TRISO-GOV-014}
]

The layer-specific time dependence is intentionally left generic.

[QUESTION FOR SUPERVISOR] Determine whether the intended repository model treats (D_i) as prescribed functions of temperature only, temperature plus fluence/burnup, or constants for the initial implementation.

## 8. Kernel-confined source

The original notebook correctly distinguishes source location from diffusivity.

Under the stated kernel-source assumption,

[
S_{mathrm{gen}}(r,t)
=
egin{cases}
S_1(r,t), & 0le r<r_1,\
0, & r_1<r<R.
end{cases}
	ag{TRISO-GOV-015}
]

For the simplest uniform-kernel benchmark,

[
S_1(r,t)=S_0(t).
	ag{TRISO-GOV-016}
]

[ASSUMPTION] The buffer and coating layers contain no volumetric fission generation.

[ SOURCE] BISON documentation describes a nonzero source generation rate in the kernel for in-pile stable-fission-product verification, while the IAEA treatment identifies the kernel and coating layers as distinct transport regions. citeturn629504search2turn772970search35

The homogeneous Part I source

[
S_{mathrm{gen}}=S_0
qquad 0le rle R
]

is therefore retained only as a verification benchmark and must not be confused with the physical five-layer source distribution.

## 9. Optional radioactive-decay specialisation

If the selected fission-product species has a first-order radioactive loss rate, introduce decay constant (lambda_d).

[CONSTITUTIVE]

[
R_{mathrm{decay}}=-lambda_d c.
	ag{TRISO-GOV-017}
]

The units are

[
[lambda_d]=mathrm{s^{-1}}.
	ag{TRISO-GOV-018}
]

If trapping/release is represented, it requires its own constitutive definition rather than an unspecified additive symbol.

[QUESTION FOR SUPERVISOR] Confirm whether the first implementation is a stable/long-lived species model with (R_{mathrm{net}}=0), or includes explicit radioactive decay and/or trapping-release kinetics.

With decay only,

[
R_{mathrm{net}}=-lambda_d c.
	ag{TRISO-GOV-019}
]

and the governing PDE becomes

[
oxed{
rac{partial c}{partial t}
=
rac{1}{r^2}
rac{partial}{partial r}
left(
r^2D(r,t)rac{partial c}{partial r}
ight)
+
S_{mathrm{gen}}(r,t)
-
lambda_d c.
}
	ag{TRISO-GOV-020}
]

BISON's published TRISO diffusion verification cases explicitly include diffusion, body-force source, and radioactive-decay terms as separate model contributions. citeturn629504search0turn772970search0

## 10. Centre condition

The spherical coordinate (r=0) is a geometric point, not a material interface.

[ASSUMPTION] Spherical symmetry implies no preferred radial direction at the centre.

Therefore the solution must be even in an extended radial coordinate:

[
c(-r,t)=c(r,t).
	ag{TRISO-BC-001}
]

Differentiate with respect to (r):

[
rac{partial c}{partial r}(-r,t)
=
-rac{partial c}{partial r}(r,t).
	ag{TRISO-BC-002}
]

Set (r=0):

[
oxed{
left.rac{partial c}{partial r}ight|_{r=0}=0.
}
	ag{TRISO-BC-003}
]

This is the continuous centre condition.

[ SOURCE] Spherical symmetry at the centre is also used in BISON's analytical TRISO diffusion verification problems. citeturn629504search0turn629504search2

The singular-looking factors (1/r) and (1/r^2) are therefore not to be evaluated naively at (r=0).

## 11. Interface conditions

Let (r=r_k) be an interface between layers (k) and (k+1).

The conservative formulation requires the outgoing flux from one side to equal the incoming flux on the other when no interfacial accumulation exists.

[ASSUMPTION] Ideal material contact is used initially, with no separate interfacial storage and no explicit interfacial resistance.

Then

[
J_{r,k}(r_k,t)=J_{r,k+1}(r_k,t).
	ag{TRISO-INT-001}
]

Using Fick's law on each side gives

[
-D_k
left.rac{partial c_k}{partial r}ight|_{r_k^-}
=
-D_{k+1}
left.rac{partial c_{k+1}}{partial r}ight|_{r_k^+}.
	ag{TRISO-INT-002}
]

[ASSUMPTION] Under ideal contact and a single concentration variable with no partition law, concentration is continuous:

[
c_k(r_k,t)=c_{k+1}(r_k,t).
	ag{TRISO-INT-003}
]

The pair

[
oxed{
c_k(r_k,t)=c_{k+1}(r_k,t)
}
	ag{TRISO-INT-004}
]

and

[
oxed{
D_k
left.rac{partial c_k}{partial r}ight|_{r_k^-}
=
D_{k+1}
left.rac{partial c_{k+1}}{partial r}ight|_{r_k^+}
}
	ag{TRISO-INT-005}
]

are the current ideal-interface model.

Do not replace these with an arithmetic average of (D_k) and (D_{k+1}) before a numerical face/interface formulation is derived.

A partition relation or interfacial resistance would change the second condition and possibly the first.

[QUESTION FOR SUPERVISOR] Confirm whether the intended species model assumes concentration continuity across every interface or a species/material partition coefficient at any interface.

[ SOURCE] TRISO diffusion literature identifies interface characteristics as relevant model inputs, and experimental work specifically studies fission-product accommodation at IPyC/SiC interfaces. This supports treating the interface law as a model choice requiring evidence rather than assuming an arbitrary average property. citeturn772970search4turn629504search5

## 12. Outer boundary

At (r=R), the transport leaves the OPyC into the coolant.

Let (c_infty(t)) be the coolant concentration.

[ASSUMPTION] The external transfer law is linear in the surface-to-coolant concentration difference:

[
J_r(R,t)=hleft[c(R,t)-c_infty(t)ight].
	ag{TRISO-BC-004}
]

Using the OPyC diffusivity (D_5),

[
-D_5
left.rac{partial c}{partial r}ight|_{R}
=
hleft[c(R,t)-c_infty(t)ight].
	ag{TRISO-BC-005}
]

Therefore,

[
oxed{
-D_5
left.rac{partial c}{partial r}ight|_{R}
=
hleft[c(R,t)-c_infty(t)ight].
}
	ag{TRISO-BC-006}
]

The original notebook uses (c_infty=0), so its benchmark condition is recovered as

[
-D_5
left.rac{partial c}{partial r}ight|_{R}
=
h,c(R,t).
	ag{TRISO-BC-007}
]

This also exposes an important correction to the homogeneous notation: the physical five-layer outer boundary uses the diffusivity of the outermost material, (D_5), not a generic interior (D).

## 13. Initial condition

The existing benchmark starts from an empty particle:

[
oxed{
c(r,0)=0.
}
	ag{TRISO-BC-008}
]

This is retained as the benchmark initial condition.

A more general irradiated-particle calculation may instead require a nonzero initial inventory.

[QUESTION FOR SUPERVISOR] Confirm whether the first physical case begins from zero inventory or an inherited irradiation history.

## 14. Complete continuous five-layer model

For each layer (i=1,ldots,5),

[
oxed{
rac{partial c_i}{partial t}
=
rac{1}{r^2}
rac{partial}{partial r}
left(
r^2D_i
rac{partial c_i}{partial r}
ight)
+
S_i
+
R_i.
}
	ag{TRISO-GOV-021}
]

The domains are

[
r_{i-1}<r<r_i,
qquad
r_0=0,
qquad
r_5=R.
	ag{TRISO-GOV-022}
]

The source is kernel-confined under the current model:

[
S_i=
egin{cases}
S_1, & i=1,\
0, & i=2,3,4,5.
end{cases}
	ag{TRISO-GOV-023}
]

For the decay-only extension,

[
R_i=-lambda_{d,i}c_i.
	ag{TRISO-GOV-024}
]

The centre condition is

[
left.rac{partial c_1}{partial r}ight|_{r=0}=0.
	ag{TRISO-BC-009}
]

At every interface (r=r_k), (k=1,ldots,4),

[
c_k(r_k,t)=c_{k+1}(r_k,t),
	ag{TRISO-INT-006}
]

and

[
D_k
left.rac{partial c_k}{partial r}ight|_{r_k^-}
=
D_{k+1}
left.rac{partial c_{k+1}}{partial r}ight|_{r_k^+}.
	ag{TRISO-INT-007}
]

At (r=R),

[
-D_5
left.rac{partial c_5}{partial r}ight|_{R}
=
hleft[c_5(R,t)-c_infty(t)ight].
	ag{TRISO-BC-010}
]

Together with the selected initial condition, these equations define the present continuous model before discretisation.

## 15. Part I relation to the continuous model

The existing Part I benchmark is recovered by setting

[
D_1=D_2=D_3=D_4=D_5=D
	ag{TRISO-GOV-025}
]

and

[
S_1=S_2=S_3=S_4=S_5=S_0.
	ag{TRISO-GOV-026}
]

With zero decay,

[
R_i=0.
	ag{TRISO-GOV-027}
]

The multilayer problem is therefore a generalisation of the homogeneous benchmark rather than a replacement for it.

## 16. Eigenvalue notation correction in the original notebook

The original Part I separation writes a separation constant as (-lambda D) and then assigns (lambda) units of (mathrm{s^{-1}}). That is dimensionally inconsistent with the subsequent factor (sqrt{lambda}r).

Retain two distinct quantities.

Define the spatial eigenvalue (k_n) by

[
[k_n]=mathrm{m^{-1}}.
	ag{TRISO-VER-001}
]

Then define

[
lambda_n=Dk_n^2.
	ag{TRISO-VER-002}
]

Therefore

[
[lambda_n]=mathrm{s^{-1}}.
	ag{TRISO-VER-003}
]

For the dimensionless particle coordinate, define

[
mu_n=k_nR.
	ag{TRISO-VER-004}
]

Then

[
lambda_n=rac{Dmu_n^2}{R^2}.
	ag{TRISO-VER-005}
]

This preserves the original intent while making the units consistent.

The existing relation

[
mu=sqrt{lambda},R
]

is therefore not retained with (lambda) interpreted as a rate. Its dimensionally correct replacement is

[
mu=kR.
	ag{TRISO-VER-006}
]

The homogeneous-sphere temporal factor is then

[
exp(-lambda_n t),
	ag{TRISO-VER-007}
]

while the spatial eigenfunction contains

[
sin(k_nr).
	ag{TRISO-VER-008}
]

## 17. What this document establishes

Verified or conditionally verified:

- conservation-based continuous formulation;
- conservative variable-diffusivity form;
- spherical radial form;
- kernel-localised source structure;
- centre symmetry condition;
- ideal-interface flux continuity;
- ideal-interface concentration continuity under the stated single-species model;
- OPyC-specific outer Robin condition;
- dimensional distinction between (k_n) and (lambda_n);
- relationship of Part I to the five-layer formulation.

Not yet resolved:

- exact species/source model;
- decay/trapping/release model;
- temperature/fluence dependence of each (D_i);
- partition coefficients or interfacial resistance;
- numerical treatment of discontinuous (D_i);
- numerical centre/interface/boundary discretisation;
- code mapping, because the current repository contains no scientific implementation;
- verification against the supervisor's intended implementation;
- local checked-out branch and `git status`, because no local repository is available in the present execution environment.

## References

- IAEA, TECDOC material on fission-product transport and TRISO fuel modelling. citeturn772970search35
- Hales, J. D., Jiang, W., Toptan, A., Gamble, K. A., *Modeling fission product diffusion in TRISO fuel particles with BISON*, Journal of Nuclear Materials 548 (2021) 152840. citeturn772970search0
- BISON TRISO workshop, fission-product diffusion model and material-specific diffusion coefficients. citeturn629504search1
- BISON TRISO analytical diffusion verification cases. citeturn629504search0turn629504search2
- Toptan et al., *Verification of Bison fission product species conservation under TRISO reactor conditions*, Journal of Nuclear Materials 573 (2023) 154105. citeturn772970search1
