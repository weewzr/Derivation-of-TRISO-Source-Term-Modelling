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

## 2. Conservation

Take an arbitrary control volume V. The amount inside it is N = integral over V of c dV.

[EXACT] Accumulation equals production minus outward flux:

d/dt (integral over V of c dV) = - integral over boundary of V of J dot n dA + integral over V of S dV.

[EXACT] By the divergence theorem:

integral over boundary of V of J dot n dA = integral over V of div J dV.

For a fixed control volume, move the time derivative inside the integral and then use the arbitrariness of V:

∂c/∂t + ∇·J = S.

## 3. Constitutive law and general PDE

[CONSTITUTIVE] Fickian diffusion:

J = -D ∇c.

Substitute into conservation:

∂c/∂t + ∇·(-D∇c) = S.

Therefore:

∂c/∂t = ∇·(D∇c) + S.

[IMPORTANT] The original notebook's D∇²c + S is the constant-D special case. For the five-layer problem D stays inside the divergence until a single layer is selected.

[SOURCE] Hales et al. 2021, BISON TRISO diffusion modelling: https://www.osti.gov/servlets/purl/1829987.

## 4. Spherical symmetry

[ASSUMPTION] c=c(r,t). The radial flux is

J_r = -D(r,t) ∂c/∂r.

For a radial vector:

∇·J = (1/r²) ∂/∂r [r² J_r].

Substitution gives the heterogeneous radial equation:

∂c/∂t = (1/r²) ∂/∂r [r² D(r,t) ∂c/∂r] + S(r,t).

Inside one layer where D_i is constant:

∂c_i/∂t = D_i (1/r²) ∂/∂r [r² ∂c_i/∂r] + S_i.

Only in such a constant-D region may the operator be expanded:

(1/r²) ∂/∂r (r² c_r) = c_rr + 2c_r/r.

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