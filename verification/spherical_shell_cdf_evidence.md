# Spherical-Shell Conditional-Time CDF — Executed Verification Evidence

## Provenance

- Run: `36743542042`
- Conclusion: success
- Research commit: `a6a0da1849a1ca501653ed2fe931255e42586e1c`
- Supervisor commit: `8d31482d127e211614ebb3f66b1076e1ed6dea98`
- Artifact: `spherical-shell-cdf-results`
- Artifact ID: `11111702435`
- Artifact SHA-256: `54121549aa5072eda8b93ea6e86cf0d956cdf8366dcceb973a4cbda31b692deb`

N=20,000; 2,000 accelerated-series terms; same 50–100 um homogeneous shell and r0=75 um benchmark as the preceding moment test.

## CDF evidence

| t s | exact | accel outer | accel inner | direct outer | direct inner |
|---:|---:|---:|---:|---:|---:|
| .005 | .02484 | .02691 | .02568 | .02536 | .02517 |
| .010 | .15420 | .15937 | .15420 | .15871 | .15208 |
| .020 | .42225 | .42609 | .41171 | .43036 | .42613 |
| .030 | .61047 | .60952 | .60676 | .60941 | .61837 |
| .050 | .82313 | .81957 | .82823 | .82245 | .83084 |
| .080 | .94589 | .94280 | .94745 | .94484 | .94561 |
| .120 | .98884 | .98793 | .98694 | .98807 | .98876 |

Maximum absolute deviation from the analytical spectral CDF:

- accelerated outer: 0.00517;
- direct-WOS outer: 0.00812.

The conditional sample sizes are 13,340 accelerated outer / 6,660 accelerated inner and 13,326 direct outer / 6,674 direct inner. The observed discrepancies are consistent with finite Monte-Carlo variation at these sample sizes; no coherent time-dependent distortion is evident.

## Gate decision

The homogeneous spherical-shell kernel has now passed:

1. exact exit-side probability verification;
2. exact conditional first-moment verification;
3. conditional first-passage-time CDF verification against an analytical spectral reference;
4. independent comparison against direct supervisor WOS.

Therefore controlled interface coupling is justified.

This does NOT yet authorize five-layer acceleration. The next gate is the already-verified two-layer transient problem, with only within-shell wandering replaced by the exact shell kernel and the frozen interface rule retained.

R2-B01 remains OPEN.
