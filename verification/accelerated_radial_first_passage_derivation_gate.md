# Candidate Accelerated Radial First-Passage Estimator — Derivation Gate

## Status

DESIGN / DERIVATION ONLY. Not production. Not R2-B01 closure.

## Motivation

Direct integrated five-layer WOS is computationally unusable at the frozen benchmark because rare outward interface transmissions and repeated near-interface excursions consume the step budget before SiC penetration.

The controlled two-layer interface law is independently verified at declared statistical precision. Any acceleration must preserve that physical stochastic process rather than tune it.

## Symmetry basis

The frozen benchmark is concentric and spherically symmetric:

- piecewise radial D(r);
- K=1 ideal interfaces;
- uniform-in-volume kernel initial inventory;
- no angularly varying source or material property;
- absorbing spherical outer boundary.

The requested observable F(t) is cumulative release through the outer sphere.

Therefore the continuum problem is radial. Angular coordinates contain no information needed for this benchmark observable once the process is conditioned only on radius and material state.

This symmetry permits investigation of a radial first-passage / Markov-renewal representation. It does NOT imply that an arbitrary radial random walk is equivalent to production WOS.

## Exact homogeneous spherical-shell hitting probability

For Brownian diffusion with constant D in a spherical shell a<r<b, the probability u(r) of reaching b before a solves

(1/r^2) d/dr [r^2 du/dr] = 0,
u(a)=0,
u(b)=1.

Hence

u(r) = (1/a - 1/r)/(1/a - 1/b).

This probability is independent of D; D controls the associated first-passage time distribution.

This exact result explains the small chance that a walker reinserted only 20 nm into IPyC reaches the far IPyC boundary before returning to Buffer.

## What an exact accelerated estimator would need

A valid shell/interface renewal kernel must sample the JOINT law of:

1. which neighbouring interface/boundary is hit next;
2. elapsed physical first-passage time;
3. interface transmit/reflect outcome under the frozen K=1 rule;
4. renewed state after the interface event.

Matching hitting probability alone is insufficient because R2-B01 concerns release-time statistics.

The kernel must preserve the same radial diffusion/interface process represented by

partial_t c = r^-2 partial_r(r^2 D(r) partial_r c)

with concentration and flux continuity at K=1 interfaces and absorbing c(R,t)=0.

## Verification hierarchy before adoption

1. derive shell exit-side probability analytically;
2. derive or independently compute shell exit-time law conditional/joint with exit side;
3. verify homogeneous shell statistics against direct production WOS;
4. verify controlled two-layer transient F(t) against the already accepted FV reference and direct-WOS plateau;
5. verify an integrated reduced five-layer regime where direct production WOS is feasible;
6. only then evaluate the frozen five-layer benchmark.

No production supervisor source is modified.

## Important boundary

The deterministic FV solver is not substituted for WOS. It remains an independent reference. The proposed estimator, if completed, remains a stochastic Method-2 first-passage implementation.

## Current gate

Do not implement a production accelerated five-layer release estimator until the joint shell first-passage time/exit-side law is derived and independently verified.
