# 20 — Production WOS Stochastic Formulation

## Purpose

This document derives the mathematical meaning of one production Walk-on-Spheres (WOS) hop from the actual supervisor implementation. It does not claim that the current WOS process solves the frozen source-plus-Robin benchmark.

## 1. Continuous equation represented inside one homogeneous layer

In a single layer with constant diffusivity D_i and no reaction during the hop:

∂c_i/∂t = D_i ∇²c_i.

A local WOS sphere is the ball

B_ρ(x) = { y : ||y-x|| < ρ }.

The radius ρ is chosen so that the ball remains inside the current homogeneous material.

## 2. Local first-passage problem

Because D_i is constant inside B_ρ, the local problem is the ordinary homogeneous Brownian diffusion problem.

The sphere is centred on the current walker position.

The first time at which the walker reaches the sphere surface is the first-passage time τ.

## 3. Exit location

Rotational symmetry means that no direction on the sphere is preferred.

Therefore the exit point can be written as

x_{n+1} = x_n + ρ_n n_n,

where n_n is a uniformly distributed unit vector.

The supervisor source implements this through its isotropic-direction sampler.

[EXACT] This is the harmonic measure for a Brownian particle starting at the centre of a sphere.

## 4. Exit time

Define the dimensionless exit time

θ_n = D_i τ_n / ρ_n².

The exact centre-start mean is

E[τ_n] = ρ_n²/(6D_i).

The supervisor source uses the survival series

S(θ) = 2 sum over m of (-1)^(m+1) exp(-m²π²θ).

It samples θ and reconstructs

τ_n = θ_n ρ_n² / D_i.

Therefore an ordinary WOS hop advances both position and simulated time.

## 5. One-hop state transition

The stochastic state changes from

(x_n,t_n)

to

x_{n+1}=x_n+ρ_n n_n,

and

t_{n+1}=t_n+τ_n.

There is no globally prescribed Δt.

This is why WOS is event-driven rather than a conventional explicit time-stepping method.

## 6. Relationship to the diffusion operator

The WOS sphere is chosen so that it contains no material interface.

Thus the local coefficient is constant and the local differential operator is

D_i ∇².

For a stationary source-free problem, Brownian first-exit expectations are harmonic and satisfy

D_i ∇²u = 0.

Therefore the local WOS construction is a probabilistic representation of the same homogeneous diffusion operator.

For transient transport, the random first-passage time supplies the time propagation instead of an imposed fixed timestep.

## 7. Important forward/backward distinction

The basic first-exit identity is naturally a backward-expectation statement.

The physical TRISO concentration model is a forward conservation equation:

∂c/∂t = ∇·(D∇c) + S.

Therefore the local WOS identity is not by itself a proof that the ensemble concentration satisfies the forward PDE.

The missing proof must connect the ensemble probability density and the complete interface/boundary process to the forward conservation law.

Status: [DERIVATION GAP].

## 8. Piecewise diffusivity

The five-layer medium can be represented as

D(x)=D_i inside material region Ω_i.

Ordinary WOS hops stay inside one Ω_i.

When the walker reaches a material interface, the ordinary homogeneous hop is stopped and a separate interface rule is applied.

Thus the heterogeneous process is a sequence of homogeneous Brownian subproblems joined by stochastic interface events.

## 9. Interface event

The current supervisor implementation uses

p_1→2 = K D_2/(D_1 + K D_2).

For K=1:

p_1→2 = D_2/(D_1 + D_2).

Reflection is the complementary probability.

Earlier Ray work derives this rule from the encounter-rate/detailed-balance construction used by this WOS implementation.

Status: [CONDITIONALLY VERIFIED FOR THE STATED STOCHASTIC EVENT MODEL].

Full transient PDE equivalence remains [UNVERIFIED].

## 10. Finite interface capture

The mathematical WOS limit would keep shrinking the local sphere until the interface is reached.

The implementation instead triggers the interface logic when

ρ ≤ ε_capture.

This introduces a finite geometric approximation.

Status: [APPROXIMATION].

## 11. Reinsertion

After the interface decision the walker is placed a small distance away from the interface:

δ_reinsert = α ε_capture.

The current implementation does not assign a physical time increment to this reinsertion.

Therefore the interface-resolution operation is treated as an instantaneous numerical event.

Status: [APPROXIMATION].

## 12. First-passage inverse-CDF approximation

The exact first-passage distribution is continuous.

The implementation stores a finite lookup table and obtains θ by interpolation.

Thus the actual sample is an approximation to the exact inverse distribution:

θ_hat = F_hat^{-1}(U).

Status: [APPROXIMATION].

## 13. Maximum-step truncation

The production walk stops after a configured maximum number of steps.

If that limit is reached first, the mathematical walk has not necessarily reached its physical stopping condition.

Status: [APPROXIMATION / SAFETY TRUNCATION].

## 14. Monte-Carlo ensemble

For N histories and a scalar history observable q_m, the estimated quantity is

Q_hat_N = (1/N) sum q_m.

For independent histories with finite variance, statistical uncertainty decreases approximately as

SE(Q_hat_N) proportional to N^(-1/2).

This is sampling error, not deterministic discretisation error.

## 15. Current outer boundary

The current multilayer WOS driver treats reaching the OPyC outer radius as release.

The corresponding ideal continuum boundary condition is

c(R,t)=0.

The frozen numerical benchmark instead uses

-D_5 c_5'(R,t) = h[c_5(R,t)-c_infty].

These are different boundary models.

Status: [CODE MISMATCH].

## 16. Current source semantics

The current release/live ensemble creates walkers initially in the kernel, including a volume-uniform kernel birth distribution.

This represents an initial-value release problem.

It is not direct continuous injection according to a volumetric PDE source S_0.

Status: [CODE MISMATCH].

A future source-driven WOS formulation would need an explicit birth process or an equivalent source-weighted estimator.

## 17. Optional decay/transmutation

The depletion path samples reaction events as competing stochastic clocks.

With total reaction rate q, the reaction waiting time is exponentially distributed:

P(T > t)=exp(-q t).

An event can pre-empt a proposed diffusion hop.

This is a stochastic representation of reaction kinetics, but the complete mapping to the continuum sink/source term remains a separate derivation obligation.

Status: [DERIVATION GAP].

## 18. Actual production numerical model, as currently implemented

The best current description is:

piecewise-constant diffusion;
first-passage event-driven spatial movement;
stochastic interface transmission/reflection;
absorbing outer release;
optional competing reaction clocks;
finite interface capture;
finite reinsertion;
finite first-passage lookup interpolation;
finite step cap;
finite Monte-Carlo sampling.

This is the implementation model. It must not be silently relabelled as the frozen Robin/source benchmark.

## 19. Review-2 numerical derivation contract

The next numerical derivation should establish, in order:

1. the exact stochastic state;
2. the local first-passage kernel;
3. the interface state transition;
4. the stopping boundary;
5. the ensemble observable;
6. each numerical approximation;
7. the continuum problem to which the stochastic process is expected to converge.

Only after those are explicit should production code be changed.

## 20. Current highest-value unresolved questions

[QUESTION FOR SUPERVISOR] Is production release intended to use the current absorbing OPyC boundary, or finite external mass-transfer resistance?

[QUESTION FOR SUPERVISOR] Is the production observable an initial-kernel release curve, or a continuously generated fission-product source?

[QUESTION FOR SUPERVISOR] Is the first production benchmark reaction-free, with decay/transmutation treated as later extensions?

[QUESTION FOR SUPERVISOR] Is K=1 intended for every canonical interface?

## 21. Conclusion

The local WOS hop has now been connected explicitly to the homogeneous diffusion operator without pretending that this proves the complete multilayer process.

The production implementation and the frozen continuum benchmark are now mathematically separated.

No production WOS code was changed in this pass.

The next task is to decide which of the two model contracts is the intended production target, then derive the corresponding complete numerical formulation.