# Spherical-Shell Accelerated Kernel — Executed Verification Evidence

## Provenance

- Run: `36741901064`
- Conclusion: success
- Research commit: `6170babb63c6c7feba31e4a50ff999149ecd6a62`
- Supervisor commit: `8d31482d127e211614ebb3f66b1076e1ed6dea98`
- Artifact: `spherical-shell-kernel-results`
- Artifact ID: `11111450233`
- Artifact SHA-256: `1ba994aa41f2bf7b10861b94fa490bebdf2cb31fe505795c57886169b42fb233`

## Benchmark

- a = 50 um
- b = 100 um
- r0 = 75 um
- D = 1e-8 m2/s
- N = 20,000
- accelerated series terms = 2,000
- direct-WOS capture epsilon = 50 nm

## Exact references

Outer-exit probability:

[
P_b=2/3=0.6666666667.
]

Exact conditional mean time for both sides in this symmetric radial placement:

[
E[T|b]=E[T|a]=0.03125 s.
]

Accelerated-series mean truncation-tail bound:

[
9.50\times10^{-6} s.
]

## Executed statistics

| statistic | exact | accelerated | direct WOS |
|---|---:|---:|---:|
| P(outer) | 0.6666667 | 0.6713500 | 0.6663000 |
| mean T | outer, s | 0.0312500 | 0.0312289 | 0.0312362 |
| mean T | inner, s | 0.0312500 | 0.0307283 | 0.0309805 |

Errors versus exact:

Accelerated:
- outer probability +0.0046833;
- outer conditional mean -2.11e-5 s;
- inner conditional mean -5.22e-4 s.

Direct WOS:
- outer probability -0.0003667;
- outer conditional mean -1.38e-5 s;
- inner conditional mean -2.70e-4 s.

At N=20,000 these discrepancies are consistent with finite Monte-Carlo sampling scale; no systematic failure is identified by the tested exit probability or first moments.

## Gate interpretation

The exact Bernoulli-gated exponential shell sampler passes the first numerical gate:

- exit-side frequency is compatible with exact harmonic probability;
- conditional mean times are compatible with exact derivative-of-transform moments;
- truncation-tail scale is negligible relative to MC variation;
- direct production WOS provides an independent stochastic comparison and is likewise compatible.

However, this run did not record conditional time CDFs. The derivation gate explicitly requires CDF-level verification before interface coupling.

Therefore the shell kernel is **moment/exit-side verified, CDF verification pending**.

R2-B01 remains OPEN. No interface-coupled accelerated estimator is authorized yet.
