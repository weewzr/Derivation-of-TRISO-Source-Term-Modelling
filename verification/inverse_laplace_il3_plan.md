# IL-3 full Process-C production inversion — predeclared plan

Classification before execution: **CONTROLLED PRODUCTION EVIDENCE CANDIDATE — NOT YET AN ACCEPTED RELEASE CDF**.

This stage preserves the independently reviewed Process-C hard five-layer transform and all frozen physical parameters.

## Production grid

The original grid is unchanged:

- 81 logarithmically spaced times;
- t_j = 10^(2 + 0.1 j) s, j=0,...,80;
- range 1e2 to 1e10 s.

No grid point is selected or removed after observing results.

## Methods

Two independent numerical inversion routes:

1. high-precision Gaver-Stehfest (positive-real transform evaluations);
2. de Hoog Fourier/continued-fraction inversion (complex-s transform evaluations).

Primary requested precision: **80 decimal digits**.

## Targeted precision convergence

Predeclared representative points:

- 1e2 s — early unresolved/negligible-release regime;
- 1e6 s — release onset/resolved small CDF;
- 1e10 s — near-complete release.

At each, compare 50, 80 and 120 requested digits for both methods. The 80-digit value is reused from the production grid rather than recomputed. Only 50- and 120-digit auxiliary inversions are added.

## CDF and survival

At every production time and for both methods independently invert:

F(t) = L^-1{Phi(s)/s}

and

S(t) = L^-1{[1-Phi(s)]/s}.

Report raw F, raw S and F+S-1. No clipping is permitted.

## Physical/numerical checks

For each method:

- raw bounds 0 <= F <= 1;
- monotonicity on all 81 points;
- maximum |F+S-1|;
- method-to-method signed and absolute differences at every point;
- maximum method discrepancy and its time;
- exact transform-call count, unique evaluations, cache hits and runtime.

Early values whose sign/magnitude remain precision-dependent near numerical zero are preserved raw and classified using the IL-2 reporting rule, not clipped.

## Forward-transform reconstruction diagnostic

Use the recovered discrete CDF increments on the fixed 81-point log grid as a transparent quadrature diagnostic:

Phi_rec(s) ~= sum_i exp[-s sqrt(t_i t_{i-1})] [F(t_i)-F(t_{i-1})].

This is **not** used to generate the CDF and introduces no interpolation into Phi(s). It is a post-hoc discretization diagnostic only. Compare against direct Phi(s) at s = 1e-9, 1e-8, 1e-7, 1e-6, 1e-5 s^-1 and report absolute/relative error. The finite time window and grid discretization are explicitly part of this diagnostic error.

## Acceptance boundary

IL-3 results remain controlled production evidence until reconciled and independently reviewed. R2-WOS-02 and R2-B01 remain OPEN during execution. Method 3 remains out of scope.
