# IL-3 Process-C full production inversion — executed evidence

## Provenance

- Run: `37008548408`
- Research commit: `9668f76c6339871e6fc60d915242fcaa44ae3fef`
- Conclusion: **SUCCESS**
- Artifact: `inverse-laplace-il3-production-results`
- Artifact ID: `11226802949`
- Artifact digest: `sha256:c359c6a66a398693914ce87a81ad19ba0b8e32d0e226d9830da417d03ff8b9a9`
- Python 3.12.14; mpmath 1.3.0.
- Classification: **CONTROLLED PRODUCTION EVIDENCE CANDIDATE — INDEPENDENT REVIEW REQUIRED BEFORE ACCEPTED RELEASE-CDF CLAIM**.

## Grid and methods

81 fixed log-spaced points from 1e2 to 1e10 s, 0.1-decade spacing. Primary precision 80 requested decimal digits. Independent Gaver-Stehfest and de Hoog inversion of both CDF and survival.

## Method agreement

Maximum absolute Stehfest/de Hoog CDF discrepancy over all 81 points: **2.2e-60**, at t=125892.5411794166 s.

From the physically resolved release region onward, values are identical at the stored precision.

Representative de Hoog / resolved values:

| t (s) | F(t) |
|---:|---:|
| 1e4 | 9.0827360268e-147 (numerically unresolved from zero) |
| 1e5 | 2.0357115123e-21 |
| 1e6 | 5.8491286920709253e-4 |
| 1e7 | 0.1594036655523864 |
| 1e8 | 0.8872792421069171 |
| 1e9 | 0.9999999997881473 |
| 1e10 | 1.0 |

## Raw bounds and monotonicity

de Hoog: raw_bounds=PASS; raw_monotone=PASS.

Stehfest: strict raw_bounds=FAIL and raw_monotone=FAIL only in the numerically unresolved early tail. Ten points between about 3.16e2 and 1.58e4 s are negative; the largest negative magnitude is **5.2882749324e-87** at 1.5849e4 s. These values are retained raw and are consistent with the IL-2 precision/method diagnosis of alternating cancellation around an effectively zero CDF. No clipping is applied.

## CDF/survival consistency

Maximum absolute F+S-1:
- Stehfest: **1.5813296915e-80**.
- de Hoog: **1.5955319297e-74**.

## Targeted precision convergence

At 1e6 s:
- Stehfest |F50-F80|=1.8537e-37; |F80-F120|=0 at stored precision.
- de Hoog both successive differences=0 at stored precision.

At 1e10 s:
- Stehfest |F50-F80|=7.1164e-38; |F80-F120|=0.
- de Hoog |F50-F80|=5.4e-44; |F80-F120|=0.

At 1e2 s both methods drive the already negligible value rapidly toward zero with increasing precision; Stehfest can alternate sign. This remains classified as numerically unresolved from zero.

## Runtime and exact caching

Stehfest:
- runtime 135.15 s;
- transform calls 37,908;
- unique transform evaluations 18,834;
- cache hits 19,074.

de Hoog:
- runtime 227.27 s;
- transform calls 35,154;
- unique transform evaluations 17,577;
- cache hits 17,577.

Total primary production runtime is consistent with the observed ~6.5-minute workflow and is far below the historical one-hour timeout because the exact source-factor formula removed nested adaptive quadrature and CDF/survival reuse exact transform evaluations.

## Forward-transform reconstruction diagnostic

This is a post-hoc finite-grid quadrature diagnostic, not the inversion algorithm.

Relative errors versus direct Phi(s):
- s=1e-9: 9.1052e-5
- s=1e-8: 2.7725e-4
- s=1e-7: 1.5564e-3
- s=1e-6: 9.9870e-3
- s=1e-5: 9.4120e-2

Stehfest and de Hoog reconstruction values are identical because their recovered resolved CDF grids are effectively identical. The increasing high-s error is attributed to the deliberately coarse 0.1-decade grid and finite 1e2-s lower window used for this diagnostic. It must not be represented as a 9.4% disagreement between the two inversion methods.

## Gate status

The controlled inversion evidence is now complete enough to warrant independent review. It is not yet inserted into the canonical manuscript as an accepted five-layer release CDF.

R2-WOS-02 remains OPEN.
R2-B01 remains OPEN.
Method 3 remains NOT STARTED.
