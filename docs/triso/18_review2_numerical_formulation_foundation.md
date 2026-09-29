# 18 — Review-2 Numerical Formulation Foundation: Actual WOS Path

## Scope

This document begins Review-2 numerical preparation from the frozen continuous benchmark and the actual supervisor repository implementation.

No production WOS code is modified here.

## 1. First conclusion: WOS is not a mesh discretisation

The supervisor implementation does not use a fixed radial mesh or a finite-volume control-volume stencil for its production diffusion path.

The numerical state is a stochastic walker:

x_n = [x_n,y_n,z_n],

t_n,

and its current nuclide/material state.

The continuous PDE remains the physical reference. The numerical approximation is instead a sequence of exact-or-approximated first-passage events in locally homogeneous spheres.

Therefore the Review-2 derivation must use the equivalent stochastic propagation objects rather than inventing spherical finite-volume cells.

## 2. Local WOS propagation

At a current walker position x_n, identify the material region i.

Let rho_n be the minimum distance from x_n to the nearest material interface or outer particle surface.

The implementation constructs the largest sphere centred at x_n that does not cross that interface.

Conceptually:

B_n = {x : |x-x_n| < rho_n}.

Inside B_n the diffusivity is the current layer value D_i.

The local source-free diffusion equation is therefore the homogeneous equation in that sphere.

## 3. First-passage time

For a centre-started Brownian walker in a sphere of radius rho_n, define

theta_n = D_i tau_n / rho_n².

The implementation samples theta_n from the centre-start first-passage distribution.

The exact mean is

E[tau_n] = rho_n²/(6D_i).

The source code then reconstructs physical time as

tau_n = theta_n rho_n² / D_i.

The accumulated clock is

t_{n+1}=t_n+tau_n.

This is an event-driven time discretisation: there is no globally imposed Delta t.

## 4. Exit position

Because the local sphere is centred on the current walker and the Brownian process is isotropic, the first exit location is uniform over the sphere surface.

If n_hat_n is the sampled unit direction,

x_{n+1}=x_n+rho_n n_hat_n.

This is the numerical spatial update for an ordinary WOS hop.

## 5. Why a hop does not cross an interface

The hop radius is chosen from the nearest interface distance.

Therefore:

rho_n <= distance(x_n, any material interface).

So the open WOS sphere lies entirely within one material.

This directly avoids the older Gaussian-step failure mode in which one large free-space displacement could cross a thin layer without resolving the interface.

## 6. Interface event

The current code stops ordinary WOS propagation when

rho_n <= epsilon_capture.

At this point the event is treated as an explicit interface interaction.

Which interface is reached is determined by the comparison between the inward and outward distances from the walker.

For an internal interface between current diffusivity D1 and destination diffusivity D2, the code uses

p_{1->2}=K D2/(D1+K D2).

The complementary outcome is reflection.

The earlier Ray derivation establishes this as the detailed-balance rule for the stated WOS encounter/reinsertion construction.

## 7. Critical numerical approximation at the interface

The current implementation does not simulate a finite travel time for the interface-resolution event.

The walker is instead moved by a reinsertion distance

delta_reinsert = alpha epsilon_capture.

and the next ordinary WOS hop starts from the reinserted point.

Therefore the numerical time sequence contains:

ordinary WOS hop → positive sampled time increment;

interface decision → zero time increment;

reinsertion → zero time increment.

This is an approximation to the limiting first-passage process and must remain separate from the physical continuum model.

## 8. Centre treatment in the production WOS path

A WOS walker can start at the exact centre.

The repository handles the centre direction explicitly when it needs to reinsert a walker at a radial target.

The first-passage distribution itself is centred on the current WOS sphere, so the per-hop first-passage law remains well defined even when the physical particle centre is involved.

This differs from an Eulerian spherical-Laplacian centre stencil. The WOS path has no 1/r numerical singularity to evaluate at the global origin during an ordinary hop.

## 9. Outer boundary in the actual production path

The current WOS multilayer driver treats reaching the outer OPyC radius as release:

walk → outer OPyC surface → Released.

This is an absorbing boundary.

In continuum notation, the corresponding benchmark condition is

c_5(R,t)=0.

That is NOT the same boundary condition as the frozen Robin benchmark

-D_5 c_5'(R,t)=h[c_5(R,t)-c_infty].

Therefore the present production WOS path is a `CODE MISMATCH` with the frozen Robin benchmark.

No claim of WOS-versus-Robin equivalence should be made until the production boundary treatment is changed or a distinct absorbing-boundary benchmark is explicitly declared.

## 10. Source treatment in the actual ensemble drivers

The actual ensemble code contains two distinct use cases.

`parallel_kernel_release_fraction` samples an initial point uniformly inside the fuel kernel and then measures whether the walker reaches the absorbing surface before a requested time.

Thus it represents an initial-value release problem, not a PDE with an ongoing volumetric source term.

`parallel_advance_until` starts histories at the particle centre and advances diffusion/depletion until a requested time.

That also does not directly represent the frozen continuous model

S_1=S_0,

S_2=S_3=S_4=S_5=0,

unless an additional birth/source mechanism is layered around these histories.

Therefore the current production WOS ensemble is also a `CODE MISMATCH` with the frozen continuous source-driven benchmark.

## 11. What WOS is actually discretising

The current pure-diffusion multilayer process is better represented as a stochastic approximation to an initial-value diffusion problem:

piecewise D_i diffusion

+ ideal stochastic interface transmission/reflection

+ absorbing outer release

+ optional decay/transmutation clock.

It is not currently a direct numerical discretisation of the frozen Robin + continuous-kernel-source benchmark.

## 12. Monte-Carlo estimator

For N independent histories and a scalar observable q_m from history m, the ensemble estimator is

Q_hat_N = (1/N) sum from m=1 to N of q_m.

The statistical uncertainty decreases with sample size approximately as

standard error proportional to N^{-1/2},

provided the histories are independent and the variance is finite.

This is statistical convergence, not spatial or temporal truncation convergence.

The WOS method therefore has a different numerical-error structure from FTCS.

## 13. Numerical approximation inventory for Review 2

Exact local mathematical ingredients:

- homogeneous-sphere first-passage formulation;
- isotropic sphere-exit geometry;
- event-driven time accumulation for the ideal local sphere.

Implementation approximations:

- finite first-passage inverse-CDF lookup/interpolation;
- finite capture epsilon;
- finite reinsertion distance;
- finite maximum history steps;
- floating-point arithmetic;
- finite Monte-Carlo sample size.

Model mismatches with the frozen benchmark:

- absorbing outer boundary instead of Robin;
- current ensemble drivers do not directly apply the continuous kernel source;
- optional decay/transmutation is separated from the base pure-diffusion driver.

## 14. Numerical-stage decision

The frozen continuous benchmark remains the mathematical reference for the general project.

For the actual supervisor WOS implementation, the numerical problem must be documented separately until the boundary and source semantics are aligned.

Do not silently redefine the frozen PDE to match the code.

Do not silently modify the supervisor code to match the PDE.

The discrepancy must remain explicit until the intended production physical model is selected.

## 15. Highest-value next mathematical task

Before deriving detailed WOS interface algebra further, construct the exact equation-to-algorithm contract for the two possible numerical targets:

Target A: the frozen continuum benchmark with kernel source + Robin outer boundary;

Target B: the current production WOS initial-value / absorbing-boundary problem.

Then determine which target the supervisor intends as the production model and derive the corresponding discrete/stochastic formulation.

[QUESTION FOR SUPERVISOR] Is the intended production release boundary the current perfect absorbing OPyC surface, or should WOS ultimately represent the frozen finite-resistance Robin condition?

[QUESTION FOR SUPERVISOR] Is the production WOS ensemble intended to model an initially loaded kernel, or a continuously generated fission-product source?

These are now the two highest-value model-definition questions because they determine what numerical equations can legitimately be compared against the continuum benchmark.

## Review-2 status

Review-1 findings are closed for the frozen benchmark.

Review-2 numerical derivation has begun.

No production numerical code has been changed.

Full WOS correctness remains unverified.