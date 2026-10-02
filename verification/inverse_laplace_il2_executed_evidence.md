# IL-2 Process-C method/precision pilot — executed evidence

## Provenance

- Run: `37007436157`
- Commit: `c97ec853b35a635b1698ffdf884bea2a594d2d4d`
- Conclusion: **SUCCESS**
- Artifact: `inverse-laplace-il2-pilot-results`
- Artifact ID: `11225954066`
- Artifact digest: `sha256:f1081de82e177dc8263767712a0d00bdaa065da923227dd36391594dc7379fcb`
- Python 3.12.14; mpmath 1.3.0.
- Classification: **IL-2 METHOD/PRECISION PILOT ONLY — NOT FINAL RELEASE CDF**.

## Primary 80-digit results

| t (s) | Stehfest F | de Hoog F | absolute method difference |
|---:|---:|---:|---:|
| 1e2 | 7.68778244e-340 | 2.18264184e-1352 | 7.68778244e-340 |
| 1e4 | 4.49120403e-99 | 9.08273603e-147 | 4.49120403e-99 |
| 1e6 | 0.000584912869207092530638761959671479190122 | same reported value | 0 |
| 1e8 | 0.8872792421069170706171971734365266692718 | same reported value | 0 |
| 1e10 | 1.0 | 1.0 | 0 |

Both primary method grids report raw_bounds=true and raw_monotone=true.

## CDF/survival consistency

Stehfest F+S-1: 0, 6.33e-81, 0, -8.43e-81, 2.11e-81.

de Hoog F+S-1: 0, 0, 0, -1.61e-78, -4.06e-75.

No clipping was applied.

## Precision convergence

At 1e6 s:
- Stehfest |F50-F80|=1.85e-37, |F80-F120|=0.
- de Hoog differences are 0 at reported precision.

At 1e8 s:
- Stehfest |F50-F80|=3.40e-37, |F80-F120|=0.
- de Hoog |F50-F80|=1e-45, |F80-F120|=0.

At 1e10 s:
- Stehfest |F50-F80|=7.12e-38, |F80-F120|=0.
- de Hoog |F50-F80|=5.4e-44, |F80-F120|=0.

## Early-time near-zero interpretation

At 1e4 s Stehfest gives:
- 50 dps: -7.3700218259e-73
- 80 dps: +4.4912040344e-99
- 120 dps: -1.0487842146e-130

de Hoog gives:
- 50 dps: +9.0827360268e-107
- 80 dps: +9.0827360268e-147
- 120 dps: +2.6247040218e-182

At 1e2 s the same pattern is even more extreme. Increasing precision drives the absolute magnitude rapidly toward zero, while the Stehfest sign can alternate. This is numerical inverse-Laplace cancellation/roundoff in a regime where the true CDF is below resolved numerical significance, not evidence of a physical negative probability.

Reporting rule for subsequent production evidence: retain raw numerical values in machine-readable evidence. In human presentation, values whose sign is unstable across precision/method while absolute magnitude decreases rapidly toward zero should be described as **numerically unresolved from zero / effectively zero at the demonstrated inversion precision**, not clipped and not reported as a negative physical probability.

## Method status

For the physically resolved pilot values (1e6, 1e8, 1e10 s), Stehfest and de Hoog agree to all reported digits and 80-to-120 convergence is zero at the printed precision. The complex-s de Hoog route is therefore supported for controlled IL-3 production alongside the independent real-axis Stehfest route.

R2-WOS-02 remains OPEN. R2-B01 remains OPEN. Independent review is not yet requested; IL-3 evidence is required before a release-time claim.
