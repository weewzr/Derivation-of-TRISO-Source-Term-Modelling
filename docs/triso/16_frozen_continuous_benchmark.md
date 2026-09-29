# 16 — Frozen Continuous Benchmark for Numerical Discretisation

This document freezes the exact conditional continuous model that the next numerical stage will discretise. It is a mathematical benchmark, not a claim that every future physical TRISO calculation must use these assumptions.

## Frozen model

Geometry: five concentric spherical layers with radii 0<r1<r2<r3<r4<R and a=r1.

Unknown: mobile fission-product concentration c_i(r,t), in mol m^-3.

Source/reaction choice:

R_i = 0.

Generation is steady and confined to the kernel:

S_i(r)=S0 in layer 1, and S_i(r)=0 in layers 2 to 5.

Diffusivity:

D_i > 0 and time independent.

Within each layer D_i is constant in space during the benchmark. Different layers may have different D_i.

Interfaces are ideal:

c_k(r_k,t)=c_{k+1}(r_k,t),

-D_k c_k'(r_k,t) = -D_{k+1} c_{k+1}'(r_k,t).

Therefore this benchmark uses K_k=1 and no interfacial resistance or interface storage.

Outer boundary:

-D_5 c_5'(R,t)=h c_5(R,t),

with h constant and c_infty=0.

Here c_infty=0 is a benchmark condition only. The general physical boundary remains -D_5 c_5'(R,t)=h[c_5(R,t)-c_infty(t)].

Initial condition:

c_i(r,0)=0 in every layer.

## Frozen continuous PDE

In each layer:

partial c_i / partial t = (1/r^2) partial/partial r [ r^2 D_i partial c_i/partial r ] + S_i(r).

Because D_i is constant within a layer, the layer equation may be expanded only inside that layer:

partial c_i / partial t = D_i (1/r^2) partial/partial r [ r^2 partial c_i/partial r ] + S_i(r).

The project must not differentiate a discontinuous D through a material interface.

## Frozen analytical eigenbenchmark

The steady source problem defines c_i,ss(r). Let

v_i(r,t)=c_i(r,t)-c_i,ss(r).

The transient benchmark therefore has no source term and no decay term:

partial v_i/partial t = (1/r^2) partial/partial r [r^2 D_i partial v_i/partial r].

The separated form

v_i(r,t)=phi_i(r) exp(-Lambda t)

is used only for this restricted benchmark because D_i, h, and interface properties are time independent and the reaction term is zero.

The layer wave number is

k_i^2=Lambda/D_i,

with k_i in m^-1 and Lambda in s^-1.

## Part-I benchmark

The original homogeneous-sphere benchmark is retained separately by setting all D_i equal to D and replacing the kernel-only source by a uniform source S0 throughout the sphere.

## Numerical-stage scope

FTCS remains a transparent deterministic benchmark.

The production method must be determined from the supervisor repository. It must not be assumed to be FTCS merely because the original notebook contains FTCS.

The next derivation must follow:

continuous conservation equation → chosen numerical degrees of freedom/control volumes → discrete fluxes → centre treatment → interface treatment → outer Robin treatment → time discretisation → algebraic system → algorithm → code mapping → verification.

## Explicitly unresolved physical questions

[QUESTION FOR SUPERVISOR] canonical fission-product species.
[QUESTION FOR SUPERVISOR] whether radioactive decay belongs in the production model.
[QUESTION FOR SUPERVISOR] whether trapping/release belongs in the production model.
[QUESTION FOR SUPERVISOR] whether K_k differs from 1 for any physical interface.
[QUESTION FOR SUPERVISOR] whether explicit interfacial resistance is required.
[QUESTION FOR SUPERVISOR] whether D_i are temperature/fluence/burnup/state dependent in the production model.
[QUESTION FOR SUPERVISOR] whether c_infty=0 is acceptable physically or only for benchmarking.
[QUESTION FOR SUPERVISOR] whether an irradiated initial inventory is required in the physical case.

These open physical choices do not invalidate the frozen benchmark; they must simply not be silently mixed into it.

## Review-1 closure

Review 1 PASS WITH NON-BLOCKING FINDINGS is now addressed by explicitly separating the general PDE from the narrower constant-D, reaction-free eigenbenchmark.