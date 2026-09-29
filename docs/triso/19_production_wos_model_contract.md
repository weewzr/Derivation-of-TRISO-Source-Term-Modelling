# 19 — Production-WOS Model Contract (As Implemented)

## Purpose

This document records what the supervisor's current WOS implementation actually advances. It is not a replacement for the frozen continuum benchmark.

## A. Pure-diffusion WOS contract

State of one history:

- Cartesian position x(t);
- accumulated event time t;
- current nuclide identity;
- RNG state.

At each diffusion hop, the code identifies the current concentric material shell and computes the minimum distance rho to the shell's inner or outer interface.

It samples a local first-passage time:

tau = theta rho^2 / D_i,

where theta is sampled from the centre-start absorbing-sphere first-passage distribution.

It then samples an isotropic direction and moves to the corresponding point on the hop sphere.

Thus the base WOS step is event-driven rather than fixed-Delta-t.

## B. Internal interfaces

When the walker is within capture_eps of the nearest interface, the code resolves transmission/reflection.

For current side D1, destination side D2, partition parameter K:

p_transmit = K D2 / (D1 + K D2).

Reflection is the complementary event.

After the decision, the walker is reinserted at distance

delta = reinsert_factor * capture_eps

on the selected side.

This reinsertion has no sampled physical travel time in the current implementation.

## C. Outer boundary

The current production WOS code treats the outer OPyC radius as immediate release.

Mathematically the present code therefore corresponds to an absorbing condition:

c(R,t)=0.

It does not directly implement the frozen Robin condition:

-D_5 c_r(R,t)=h[c(R,t)-c_infty].

This is an explicit CODE MISMATCH with the frozen finite-resistance benchmark.

## D. Source semantics

The current ensemble constructor samples atoms initially inside the fuel kernel, uniformly in kernel volume for the live/release comparison path.

The current release-fraction driver therefore solves an initial-value release problem.

It does not inject new particles continuously according to a volumetric source S0.

Consequently, the current ensemble driver is not a direct Monte-Carlo discretisation of the continuously forced PDE:

partial c/partial t = div(D grad c) + S.

A source-driven production model would require an explicit birth process or an equivalent source-weighted estimator.

## E. Decay and transmutation

The depletion driver samples a reaction event time from an exponential clock with total rate

q = q_decay + q_transmutation.

The event can pre-empt the proposed WOS diffusion hop.

If it occurs first, the walker is restored to its pre-hop position and pre-hop time plus the sampled reaction time. Then a decay or transmutation channel is selected according to its rate.

Thus the implementation represents decay/transmutation as stochastic events attached to moving histories.

This is separate from the current frozen benchmark, where R_i=0.

## F. Exact current numerical target

Subject to the implementation's finite capture tolerance and reinsertion approximation, the current production WOS path is best described as:

piecewise-constant spatial diffusion;

+ first-passage event-driven movement;

+ stochastic interface transmission/reflection;

+ absorbing outer release;

+ optional competing reaction clocks;

+ finite-step safety cap;

+ finite Monte-Carlo sampling.

## G. Consequence for Review 2

Review 2 cannot honestly derive a single discrete scheme and then state that it reproduces both the frozen Robin/source PDE and the current WOS code.

There are two legitimate verification tracks:

Track 1 — Continuum benchmark:

five-layer diffusion + kernel source + ideal interfaces + Robin outer boundary.

Track 2 — Current WOS implementation:

initially loaded walkers + five-layer diffusion + ideal stochastic interfaces + absorbing outer boundary + optional reactions.

These tracks can be connected only after the production model semantics are intentionally aligned.

## H. Questions that now have direct implementation consequences

[QUESTION FOR SUPERVISOR] Should the production WOS model represent continuous fission-product births, or is the intended observable an initial-inventory release curve?

[QUESTION FOR SUPERVISOR] Should the outer boundary remain an absorbing OPyC release surface, or should WOS represent finite external mass-transfer resistance?

[QUESTION FOR SUPERVISOR] Should the canonical verification case set R_i=0, with decay handled only in a later extension?

[QUESTION FOR SUPERVISOR] Is K=1 intended at every interface for the canonical case?

## Current recommendation

Do not change production code yet.

First obtain the supervisor's intended production-model semantics. Then derive the numerical formulation against that declared target.

Until those semantics are fixed, FTCS remains a deterministic benchmark and WOS remains an independently documented production stochastic path.