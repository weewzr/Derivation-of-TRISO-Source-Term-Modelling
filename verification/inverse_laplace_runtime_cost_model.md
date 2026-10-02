# Controlled inverse-Laplace execution cost and staged remediation

## Run 36991554900 classification

**EXECUTION / COMPUTATIONAL TIMEOUT.**

Synthetic inversion benchmarks completed successfully. The physical Process-C step emitted no numerical failure before GitHub Actions cancelled the job at the 60-minute workflow timeout. No physical artifact was uploaded.

## Old monolithic cost model

The old physical script evaluates 81 predeclared times at 80 requested digits with both methods, then repeats five representative times at 50/80/120 digits for both methods.

From mpmath 1.3.0's inversion implementation:

- Stehfest default degree at requested dps p is the next even integer at least 2.93p. At 80 dps this is 234 positive-real transform evaluations per inversion.
- de Hoog uses dps_goal=floor(1.36p), degree=max(10,dps_goal), and 2M+1 transform evaluations. At 80 dps this is 217 complex transform evaluations per inversion.
- 81-point primary Stehfest: 81*234 = 18,954 Phi evaluations.
- 81-point primary de Hoog: 81*217 = 17,577 Phi evaluations.
- Primary total: 36,531.
- Five-point 50/80/120 convergence reruns:
  - Stehfest degrees 148 + 234 + 352 = 734 per representative time => 3,670.
  - de Hoog evaluations 137 + 217 + 327 = 681 per representative time => 3,405.
- Convergence rerun total: 7,075.
- Before minor complex-gate/reconstruction calls, old design therefore requires about **43,606 Phi evaluations / 8x8 high-precision solves**.

The 80-dps convergence calculations duplicate work already performed on the 80-dps primary grid. The old implementation has no transform cache.

## Dominant avoidable cost

Every Phi(s) call also invokes adaptive high-precision `mp.quad` for the initial uniform-volume kernel source factor. This quadrature is nested inside every inverse-Laplace transform evaluation.

For z=R1 sqrt(s/D1), the source factor is exactly

Q(s)=3[z coth(z)-1]/z^2,

with Q(0)=1.

Derivation:

Q = integral_0^R 3r^2/R^3 * (R/r) sinh(lambda r)/sinh(lambda R) dr

and integral r sinh(lambda r) dr = r cosh(lambda r)/lambda - sinh(lambda r)/lambda^2.

No interpolation is introduced. IL-1 will cross-check this exact expression against the historical adaptive quadrature at real and complex s.

## Exact caching

Cache Phi(s) only for mathematically identical complex/real s at the same working precision. This permits exact reuse between CDF and survival inversions and repeated diagnostic calls. No interpolation of Phi is allowed.

## Staged architecture

### IL-1 — smoke only

Predeclared times: **1e2, 1e4, 1e6, 1e8, 1e10 s**.

- 50 requested decimal digits.
- Stehfest CDF from Phi(s)/s.
- independently inverted survival from [1-Phi(s)]/s.
- require raw reporting of bounds, monotonicity and F+S-1.
- exact-source-factor versus adaptive-quadrature cross-check.
- complex-s continuation gate only (no physical de Hoog inversion): real-axis equivalence, conjugacy, principal sqrt, stable complex sinh ratio.
- instrument transform-call count, unique cache misses and runtime.

No physical conclusion.

### IL-2 — method pilot

Predeclared times: **1e2, 1e4, 1e6, 1e8, 1e10 s**.

- Stehfest and de Hoog.
- primary 80 digits.
- precision convergence 50/80/120 at **1e2, 1e6, 1e10 s**.
- report method discrepancy, bounds, monotonicity, cost and transform counts.
- no final release curve.

### IL-3 — full production

The original **81-point log grid from 1e2 to 1e10 s is unchanged**.

- full grid at justified primary precision (currently predeclared 80 digits);
- 50/80/120 convergence only at the predeclared representative times 1e2, 1e6, 1e10 s;
- two independent methods;
- exact transform caching;
- forward-transform reconstruction and physical checks.

IL-3 is not triggered automatically by IL-1 or IL-2.

## Hard-matrix review reconciliation

- R3-HM01: remains a non-blocking historical verification-record limitation; inverse-Laplace work focuses on Phi_init and does not misstate full-state precision certification.
- R3-HM02: all conditioning statements use directly measured hard 8x8 kappa2/sigma_min, not lower-order extrapolation.
- R3-HM03: positive-s spectral radius is reported as NOT_EVALUATED in current high-precision evidence; historical f64 NaN is not interpreted as a numerical failure.
- R3-HM04: the hard matrix is independently reviewed and cleared for controlled inversion; status documents must not call that gate pending independent review.

R2-WOS-02 remains OPEN. R2-B01 remains OPEN. Method 3 is NOT STARTED.
