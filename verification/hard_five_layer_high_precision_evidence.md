# Hard Five-Layer 8x8 High-Precision Cross-Check — Executed Evidence

## Provenance

- Run: `36869649217`
- Conclusion: SUCCESS
- Research commit: `fdb62bd203bf2c618eb58efe026f0a3bf239a480`
- Artifact: `hard-five-layer-high-precision-results`
- Artifact ID: `11166566883`
- Artifact SHA-256: `3ee6b43e532c0b7680e52b98f60cdcd4d77a63bf90e33e46118a244c06d12cf0`
- Python 3.12.14
- NumPy 2.3.3
- mpmath 1.3.0
- Predeclared precision: 50 / 80 / 120 decimal digits; 80-digit primary reference.

The preceding run `36869168929` failed before scientific execution because mpmath 1.3.0 has no `mp.one`. Commit `fdb62bd...` replaced the unsupported API with `mp.mpf(1)`; no scientific parameter or criterion changed.

## High-precision result

At s=0:

- Phi_init(0) = 1.0 at the 80-digit reference;
- high-precision residual = 7.51e-85;
- kappa_2(I-K) = 5.2020352891e10;
- sigma_min = 2.71854703581e-11;
- sigma_max = 1.41419776154;
- rho(K(0)) = 0.99999999994994928880658084574605124;
- 1-rho = 5.0050711193419154e-11.

The original normalization criterion (<1e-9) and residual criterion (<1e-10) PASS under reliable arithmetic.

## Precision convergence

At s=0, |Phi_init(50 dps)-Phi_init(80 dps)| = 7.41e-42 and |Phi_init(80 dps)-Phi_init(120 dps)| = 7.91e-72.

Across all predeclared s points, 50/80/120-digit convergence is vastly tighter than the original acceptance criteria.

## f64 versus high precision

| s (1/s) | f64 Phi_init | 80-digit Phi_init | absolute difference | high-precision kappa_2 |
|---:|---:|---:|---:|---:|
| 0 | 0.999999887846684 | 1.0 | 1.1215e-7 | 5.2020e10 |
| 1e-9 | 0.955017133045444 | 0.955021769459339 | 4.6364e-6 | 4.9794e10 |
| 1e-8 | 0.675651754390438 | 0.675651450304049 | 3.0409e-7 | 3.5956e10 |
| 1e-7 | 0.147639243555108 | 0.147639152754869 | 9.0800e-8 | 9.5626e9 |
| 1e-6 | 0.00405621041650422 | 0.00405621033355241 | 8.2952e-11 | 1.1799e9 |
| 1e-5 | 3.22490186058e-7 | 3.22458722262e-7 | 3.1464e-11 | 1.2663e8 |
| 1e-4 | 1.06756671621e-19 | 1.06756671684e-19 | 6.2261e-29 | 1.4865e7 |

The f64 residual failure at s=1e-5 (1.138e-10) disappears: the 80-digit residual is 3.49e-85.

NumPy float64 SVD independently agrees with the high-precision spectral conditioning scale (e.g. kappa_2 about 5.20205e10 at s=0).

## Classification

**A. HIGH PRECISION PASSES AND DEMONSTRATES F64 CONDITIONING ERROR.**

The underlying frozen hard 8x8 mathematical system satisfies the original predeclared normalization and residual gates under reliable arithmetic. The ordinary-f64 failures are explained by the near-recurrent, highly conditioned system rather than by the reviewed topology.

This is candidate closure evidence only. The hard matrix remains **RESULT PENDING INDEPENDENT REVIEW**.

No inverse Laplace or five-layer release CDF has been computed.

R2-WOS-02 remains OPEN.
R2-B01 remains OPEN.
