# IL-1 Process-C inverse-Laplace smoke — executed evidence

## Provenance

- Workflow run: `37006915675`
- Research commit: `b07408659fbe3d57f39873161102bc148680c327`
- Workflow conclusion: **SUCCESS**
- Artifact: `inverse-laplace-il1-smoke-results`
- Artifact ID: `11226281189`
- Artifact digest: `sha256:c92c7a06bfe66ebb02030bd9e3995f3decb99208284e0a7a45efc383d0dfbf95`
- Python 3.12.14
- mpmath 1.3.0
- Requested precision: 50 decimal digits
- Classification: **IL-1 SMOKE EVIDENCE ONLY — NOT A FINAL FIVE-LAYER RELEASE CDF**

## Frozen physics

R = [2.125e-4, 3.125e-4, 3.525e-4, 3.875e-4, 4.275e-4] m.

D = [1.2502982636347968e-13, 1e-8, 4.062299125614697e-14, 9.227773168241615e-17, 4.062299125614697e-14] m^2/s.

epsilon = 1e-7 m, alpha=2, K=1, uniform kernel-volume birth, absorbing OPyC exterior, Process C.

## Exact source-factor cross-check

| s | exact source factor | adaptive quadrature | abs error |
|---|---|---|---:|
| 1e-8 | 0.999759306907125538525758153692 | same displayed value | 1.3030e-78 |
| 1e-5 | 0.820122252337440694779164035256 | same displayed value | 1.0542e-81 |
| 1e-6+2e-6 i | 0.973757769586987772409305028277 - 0.0448072321519039297422140223582 i | same displayed value | 3.0287e-81 |

The analytical uniform-volume source factor is therefore validated against the historical adaptive quadrature at the tested real and complex points.

## Complex-s prerequisite gate

Principal square root: PASS.

Positive-real implementation agreement with frozen reviewed transform:
- s=1e-9: abs difference 3.05625e-16
- s=1e-7: 1.44578e-16
- s=1e-5: 1.99311e-19

Conjugacy Phi(conj(s))=conj(Phi(s)): displayed absolute discrepancy 0 at all three tested complex points.

These are prerequisite evidence for a bounded de Hoog pilot; they do not by themselves validate a physical de Hoog release curve.

## Raw Stehfest smoke

| t (s) | raw CDF F(t) | raw survival S(t) | F+S-1 |
|---:|---:|---:|---:|
| 1e2 | 4.25993501885884e-253 | 1.0 | 0 |
| 1e4 | -7.37002182588948e-73 | 1.0 | 0 |
| 1e6 | 5.84912869207093e-4 | 0.999415087130793 | 0 |
| 1e8 | 0.887279242106917 | 0.112720757893083 | 0 |
| 1e10 | 1.0 | -7.11643406355149e-38 | 0 |

The tiny negative values are retained raw. The strict automated CDF bounds and monotonicity flags are false because of the negative 1e4 value. They are not clipped or silently reclassified. IL-2 must determine whether these are inverse-Laplace cancellation/roundoff about zero.

## Cost evidence

- transform calls: 1460
- unique transform evaluations: 727
- exact cache hits: 733
- measured inversion runtime: 2.1719074249 s
- individual CDF inversions: about 0.376–0.416 s
- survival inversions after CDF cache population: about 0.041–0.042 s

The old monolithic run required about 43,606 transform solves and performed adaptive quadrature inside each transform evaluation. IL-1 instead uses the exact source-factor formula, five smoke times, one physical inversion method, 50 digits, and exact transform caching.

R2-WOS-02 remains OPEN. R2-B01 remains OPEN.
