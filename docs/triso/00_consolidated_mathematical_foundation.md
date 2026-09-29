# 00 — Consolidated TRISO Mathematical Foundation

This is the single continuous derivation path for the Ray TRISO project. It preserves the user's original notebook as a separate evidence stream and incorporates the corrections established during the foundation audit.

The supplied LaTeX notebook contains 33 substantive displayed equations. Every one is mapped in 13_central_equation_status_register.md.

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

## 5. Source and decay

The original source decomposition is useful bookkeeping but does not define signs. Use the safer convention

S_i = S_i,gen + S_i,other - λ_d,i c_i.

Here λ_d,i has units s^-1.

For kernel-only generation:

S_i,gen = S0 for i=1, and 0 for i=2,3,4,5.

[QUESTION FOR SUPERVISOR] Confirm whether decay and trapping/release belong in the first physical implementation.

## 6. Initial, centre, interface and outer conditions

Benchmark initial condition:

c(r,0)=0.

Centre regularity:

c_r(0,t)=0.

Ideal interface at r=r_k:

c_k(r_k,t)=c_{k+1}(r_k,t).

Flux continuity:

-D_k c_k'(r_k,t)=-D_{k+1} c_{k+1}'(r_k,t).

If a partition coefficient K_k is required, concentration continuity becomes c_{k+1}=K_k c_k.

General outer Robin condition:

-D_5 c_5'(R,t)=h[c_5(R,t)-c_infty(t)].

The notebook's boundary condition -D c_r = h c is recovered for c_infty=0 and the homogeneous Part-I replacement D_5=D.

## 7. Part I homogeneous analytical benchmark

Part I deliberately uses a uniform source over one homogeneous sphere:

∂c/∂t = D(c_rr + 2c_r/r) + S0.

At steady state, c becomes w(r):

D(w_rr + 2w_r/r)+S0=0.

Rewrite:

d/dr(r²w_r)=-(S0/D)r².

Integrate:

r²w_r=-S0r³/(3D)+A.

Boundedness at r=0 gives A=0.

Therefore w_r=-S0r/(3D).

Integrate again:

w=B-S0r²/(6D).

Apply -D w_r(R)=h w(R).

Since -D w_r(R)=S0R/3,

w(R)=S0R/(3h).

Therefore:

w(r)=S0R/(3h)+S0(R²-r²)/(6D).

Independent global balance:

(4πR³/3)S0=4πR²h w(R).

This returns the same surface concentration.

## 8. Homogeneous transient eigenproblem

Define v=c-w.

Then v_t=D(v_rr+2v_r/r), with v_r(0,t)=0, -Dv_r(R,t)=hv(R,t), and v(r,0)=-w(r).

Use v(r,t)=φ(r)T(t).

Substitution gives φT'=D(φ''+2φ'/r)T.

Divide by DφT:

T'/(DT)=(φ''+2φ'/r)/φ.

Set both sides equal to -k²:

T'=-Dk²T.

φ''+2φ'/r+k²φ=0.

Define the temporal decay rate Λ=Dk².

Here k has units m^-1 and Λ has units s^-1. This corrects the original notebook's mixed eigenvalue dimensions.

Let u=rφ.

Then φ=u/r and φ'=(ru'-u)/r².

The radial equation becomes u''+k²u=0.

Regularity at the centre removes the cosine branch, giving u=A sin(kr).

Thus φ=A sin(kr)/r.

At the origin use the limit lim[r→0] sin(kr)/r=k.

## 9. Robin eigencondition

Differentiate φ=A[kr cos(kr)-sin(kr)]/r².

Apply -Dφ'(R)=hφ(R).

Define μ=kR and Bi=hR/D.

The raw equation is:

sin μ - μ cos μ = Bi sin μ.

Therefore, away from sin μ=0:

μ cot μ = 1 - Bi.

The modal decay rate is Λ_n=D μ_n²/R².

## 10. Transient eigenfunction expansion

Distinct eigenmodes satisfy radial weighted orthogonality:

integral from 0 to R of r² φ_m φ_n dr = 0 for m not equal to n.

Expand -w(r)=sum over n of A_n φ_n(r).

Projection gives

A_n = - [integral of r² w φ_n dr] / [integral of r² φ_n² dr].

Therefore:

c(r,t)=w(r)+sum over n of A_n φ_n(r) exp(-Λ_n t).

This closes the Part-I analytical chain beyond the original raw eigencondition.

## 11. Exact steady five-layer analytical formulation

For kernel-only steady generation:

Ndot_gen = 4π integral from 0 to a of S0 r² dr.

So Ndot_gen=4πS0a³/3.

Every spherical surface outside the kernel carries the same total rate:

4πr²J_r=Ndot_gen.

Therefore J_r=S0a³/(3r²).

In outer layer i:

c_i'(r)=-S0a³/(3D_i r²).

Integrate:

c_i(r)=c(r_i)+S0a³/(3D_i)[1/r-1/r_i].

At the OPyC surface with c_infty=0:

c_5(R)=S0a³/(3hR²).

This gives an exact steady five-layer benchmark under the current ideal assumptions.

The original shell resistance is:

ρ_i=[1/(4πD_i)] [1/r_{i-1}-1/r_i].

This resistance is the steady diffusive opposition of shell i.

## 12. Five-layer transient analytical framework

In layer i:

v_i(r,t)=φ_i(r) exp(-Λt).

Then:

(1/r²)d/dr[r²D_i φ_i']+Λφ_i=0.

Define k_i²=Λ/D_i.

Let u_i=rφ_i. Then u_i''+k_i²u_i=0.

Therefore:

u_i=A_i sin(k_i r)+B_i cos(k_i r).

Centre regularity gives B_1=0.

At an ideal interface:

u_k(r_k)=u_{k+1}(r_k).

D_k[u_k'(r_k)-u_k(r_k)/r_k] = D_{k+1}[u_{k+1}'(r_k)-u_{k+1}(r_k)/r_k].

At R:

-D_5[u_5'(R)-u_5(R)/R]=h u_5(R).

These equations form a homogeneous linear system for the layer coefficients. A non-zero solution requires

F(Λ)=0,

where F is the determinant of the coefficient system.

This substantially advances the previously omitted multilayer analytical model. Numerical root enumeration and modal-coefficient convergence remain unverified.

## 13. Original FTCS discretisation

Uniform mesh:

r_i=iΔr, Δr=R/N.

Interior second derivative:

c_rr ≈ (C_{i-1}^j-2C_i^j+C_{i+1}^j)/Δr².

Interior first derivative:

c_r ≈ (C_{i+1}^j-C_{i-1}^j)/(2Δr).

Define Fo=DΔt/Δr².

The FTCS update is:

C_i^{j+1}=(1-2Fo)C_i^j+Fo(1+1/i)C_{i+1}^j+Fo(1-1/i)C_{i-1}^j+S_iΔt.

This is a Part-I benchmark discretisation, not the supervisor repository's production numerical method.

## 14. Centre discretisation

Symmetry gives C_-1^j=C_1^j.

Therefore c_rr(0)≈2(C_1^j-C_0^j)/Δr².

For a regular smooth solution, lim[r→0](2/r)c_r=2c_rr(0).

Thus the centre spherical operator is 6(C_1^j-C_0^j)/Δr².

Therefore C_0^{j+1}=C_0^j+6Fo(C_1^j-C_0^j)+S0Δt.

## 15. Robin ghost and surface

At R:

-D(C_{N+1}^j-C_{N-1}^j)/(2Δr)=hC_N^j.

With κ=hΔr/D:

C_{N+1}^j=C_{N-1}^j-2κC_N^j.

Substitution gives:

C_N^{j+1}=[1-2Fo(1+κ(1+1/N))]C_N^j+2FoC_{N-1}^j+S0Δt.

## 16. Corrected FTCS stability statement

The original notebook treats non-negative stencil coefficients as though that were a necessary and sufficient stability condition. It is only a sufficient monotonicity-style condition.

Interior positivity gives Fo≤1/2.

Centre positivity gives 1-6Fo≥0, so Fo≤1/6.

Surface positivity gives Fo≤1/[2(1+κ(1+1/N))].

Therefore:

Fo≤min{1/6, 1/[2(1+κ(1+1/N))]}.

A complete spectral stability analysis of the assembled amplification matrix remains future work.

## 17. Implementation provenance

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

## 18. Foundation gate

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