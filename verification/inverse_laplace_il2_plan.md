# IL-2 predeclared method/precision pilot

Classification: verification pilot only; not a final five-layer release curve.

Predeclared before execution:

- times: 1e2, 1e4, 1e6, 1e8, 1e10 s;
- methods: high-precision Gaver-Stehfest and de Hoog;
- primary precision: 80 requested decimal digits;
- precision convergence: 50/80/120 requested digits at all five pilot times;
- independently invert Phi(s)/s for CDF and [1-Phi(s)]/s for survival at primary precision;
- preserve raw values; no clipping;
- report F+S-1;
- report signed and absolute method discrepancy;
- strict raw bounds and monotonicity flags;
- exact transform caching only; no interpolation.

The 1e4-s early-time point is retained specifically to test the IL-1 raw negative value. Interpretation as numerical noise is permitted only after method and precision evidence is observed.

IL-3 full 81-point production inversion is not automatically triggered.
