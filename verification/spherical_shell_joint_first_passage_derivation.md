# Exact Spherical-Shell Joint First-Passage Kernel

## Status

MATHEMATICAL DERIVATION FOR METHOD-2 ACCELERATION DESIGN.

Not yet an implementation. Not R2-B01 closure.

## Problem

Let a Brownian diffusion with constant diffusivity D start at radius r in the spherical shell

a < r < b.

Let T be the first time the radial process reaches either boundary.

We require the joint law of:

- which boundary is reached first;
- elapsed physical time T.

This is the information an accelerated shell-to-shell estimator must preserve.

## Backward equation

Define the outer-exit Laplace transform

G_b(r,s) = E_r[ exp(-s T) 1{R_T=b} ].

For s >= 0 it satisfies the backward equation

D [ G_b'' + (2/r) G_b' ] = s G_b

with

G_b(a,s)=0,
G_b(b,s)=1.

Set

v(r)=r G_b(r,s)

and

lambda = sqrt(s/D).

Then

v'' - lambda^2 v = 0.

Imposing the boundary conditions gives

G_b(r,s)
=
(b/r)
sinh[lambda(r-a)]
/
sinh[lambda(b-a)].

Similarly, for exit through the inner sphere,

G_a(r,s) = E_r[ exp(-s T) 1{R_T=a} ]

satisfies

G_a(a,s)=1,
G_a(b,s)=0,

and therefore

G_a(r,s)
=
(a/r)
sinh[lambda(b-r)]
/
sinh[lambda(b-a)].

These two transforms encode the joint exit-side/time distribution.

## Zero-frequency limit: exit-side probabilities

As s -> 0, sinh(lambda x) ~ lambda x.

Thus

P_r(R_T=b)
=
G_b(r,0)
=
(b/r)(r-a)/(b-a).

This is algebraically identical to

(1/a - 1/r)/(1/a - 1/b).

The inner-exit probability is

P_r(R_T=a)
=
G_a(r,0)
=
(a/r)(b-r)/(b-a).

Their sum is one.

## Conditional time transforms

The conditional Laplace transform given outer exit is

E_r[e^{-sT} | R_T=b]
=
G_b(r,s)/G_b(r,0).

Likewise,

E_r[e^{-sT} | R_T=a]
=
G_a(r,s)/G_a(r,0).

Therefore the exact acceleration problem can be separated into:

1. sample exit side using the exact harmonic probability;
2. sample T from the corresponding conditional first-passage distribution.

No arbitrary physical time increment is permitted.

## Mean joint moments

Differentiating at s=0 gives side-weighted first moments:

E[T 1{R_T=b}] = -partial_s G_b(r,s)|_{s=0},

E[T 1{R_T=a}] = -partial_s G_a(r,s)|_{s=0}.

These provide strong numerical checks for any inverse-Laplace or spectral sampler.

## Why this helps the five-layer problem

The current production WOS repeatedly resolves local spheres and interface recrossings. Inside a homogeneous shell, the formulas above can marginalise all interface-free radial wandering between the two bounding spheres into one exact first-exit event.

They do NOT by themselves marginalise repeated stochastic transmission/reflection across material interfaces. A full accelerated multilayer estimator must combine verified shell kernels with the existing interface rule as a Markov-renewal process.

## Numerical implementation options to verify

Possible implementations include:

1. spectral expansion of the conditional exit-time density/CDF;
2. numerical inversion of the exact Laplace transforms;
3. pretabulated dimensionless conditional CDFs with controlled interpolation error.

The choice must be made on accuracy and reproducibility, not convenience.

## Mandatory verification before five-layer use

For selected (a,b,r,D):

- exit-side frequencies must agree with the exact harmonic probabilities;
- unconditional and side-conditioned time moments must agree with derivatives of G_a/G_b;
- sampled time CDFs must agree with an independently evaluated reference;
- direct production WOS and the shell kernel must agree in a computationally feasible homogeneous-shell test;
- the combined interface/shell renewal implementation must reproduce the already-closed two-layer transient benchmark.

Only after these gates may an accelerated five-layer release ensemble be attempted.

## Scope note

This derivation exploits spherical symmetry of the frozen benchmark. It is not a general acceleration for arbitrary non-spherical or spatially heterogeneous geometry.
