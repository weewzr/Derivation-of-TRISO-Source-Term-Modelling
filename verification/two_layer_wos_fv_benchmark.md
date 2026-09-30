# Controlled Two-Layer Transient WOS ↔ FV Interface Benchmark

## Gate

This benchmark reconciles the next experiment required by `reviews/independent_method2_wos_diagnostic_audit.md`.

It addresses transient interface transport and finite-capture behaviour without returning to the full five-layer production ensemble.

## Continuum problem

Spherical domain:

- interface radius: `a = 50 um`;
- absorbing outer radius: `R = 100 um`.

Piecewise diffusivity:

- inner material: `D1 = 1.0e-10 m^2/s`;
- outer material: `D2 = 1.0e-9 m^2/s`.

Thus `D2/D1 = 10`. The contrast is deliberately moderate and fixed before execution: it is large enough to exercise the production transmission rule but avoids reproducing the five-layer stiffness/pathology while interface correctness is being isolated.

Initial condition, normalized to unit total inventory:

- uniform concentration in `0 <= r < a`;
- zero concentration in `a < r < R`;
- no continuing source for `t > 0`.

Governing equation:

[
\frac{\partial c}{\partial t}
=
\frac{1}{r^2}\frac{\partial}{\partial r}
\left(r^2 D(r)\frac{\partial c}{\partial r}\right).
]

Centre:

[
\partial_r c(0,t)=0.
]

Ideal internal interface, `K=1`:

[
c_1(a,t)=c_2(a,t),
qquad
-D_1\partial_r c_1(a,t)=-D_2\partial_r c_2(a,t).
]

Outer absorbing boundary:

[
c(R,t)=0.
]

Observable:

[
F(t)=1-M(t)/M(0),
]

equivalently cumulative flux released through the absorbing outer boundary.

## Deterministic reference

Use the canonical conservative spherical finite-volume formulation:

- exact spherical cell volumes;
- volume-centroid representative coordinates;
- harmonic face resistance across the material interface;
- zero-flux symmetry at the centre;
- absorbing Dirichlet exterior represented by the outer half-cell resistance;
- no source after `t=0`.

The benchmark workflow must execute spatial and temporal refinement before WOS comparison. Reference quality is accepted only when changes in `F_FV(t)` between the two finest deterministic levels are small relative to the declared WOS Monte-Carlo uncertainty target.

## WOS mapping

The WOS benchmark uses the authoritative production primitives from supervisor commit `8d31482d127e211614ebb3f66b1076e1ed6dea98`:

- production first-passage sampler;
- production isotropic direction sampler;
- production `does_transmit` interface decision;
- finite capture epsilon;
- reinsertion `delta = alpha epsilon`;
- absorbing release at `R`.

A verification adapter supplies only this controlled two-layer geometry and constant `D1,D2`; it does not alter the stochastic transition law.

## Predeclared epsilon refinement

Hold reinsertion factor fixed:

[
\alpha=2.
]

Refine:

- 200 nm;
- 100 nm;
- 50 nm;
- 25 nm.

No epsilon is selected based on agreement with FV.

## Staged sample design

### Smoke

`N=32` per epsilon only to establish execution, censoring, event counts and runtime. Not validation.

### Pilot

If smoke succeeds, use a fixed pilot `N=400` per epsilon. This is chosen before WOS/FV agreement is inspected.

### Production uncertainty target

The later production ensemble will target a two-sided 95% normal-approximation half-width no larger than `0.02` in the worst-case binomial regime `F=0.5`.

Using

[
1.96\sqrt{0.25/N}\le0.02
]

gives

[
N\ge2401.
]

Therefore the predeclared production target is `N=2500` histories per epsilon, subject only to smoke/pilot feasibility and censoring diagnostics, not observed agreement.

## Findings

- R2-WOS-01 remains OPEN pending epsilon-refined transient agreement evidence.
- R2-WOS-02 remains OPEN pending five-layer release-CDF evidence.
- R2-WOS-03 remains OPEN pending executed transient two-layer comparison.
- R2-WOS-04 remains OPEN pending executed direct transmission/reflection diagnostics.
- R2-B01 remains OPEN.
