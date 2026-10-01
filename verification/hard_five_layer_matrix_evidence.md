# Hard Five-Layer 8x8 Matrix Diagnostic — Executed Reconciliation

## Run provenance

Canonical scientific execution:
- run: `36866849598`
- conclusion: SUCCESS
- research commit: `5171a9ae214881db3c3a87696b9aa04af0c56dc5`
- rustc: 1.99.0 (b940084d7 2026-09-28)
- artifact: `hard-five-layer-matrix-results`
- artifact ID: `11164242042`
- artifact SHA-256: `35f34def4ecb554f7bd5d13042b6c82bfcef01dd2ab0facdc1d8d43b1365742a`

Compilation-only predecessors:
- `36865761217`, commit `6ab0e745...`: Rust E0282 before scientific execution.
- `36866203368`, commit `72bd21e9...`: Rust E0282 before scientific execution.
Neither failed run contains scientific matrix evidence.

## Frozen benchmark

Unchanged from the predeclared gate:
Cs-137; radii [212.5,312.5,352.5,387.5,427.5] um; D=[1.2502982636e-13,1e-8,4.0622991256e-14,9.2277731682e-17,4.0622991256e-14] m2/s; epsilon=100 nm; alpha=2; K=1; uniform-volume kernel birth; absorbing OPyC exterior.

## Interface probabilities

I0 Kernel->Buffer = 9.9998749717368629e-1; Buffer->Kernel = 1.2502826313727662e-5.
I1 Buffer->IPyC = 4.0622826234075476e-6; IPyC->Buffer = 9.9999593771737660e-1.
I2 IPyC->SiC = 2.2664158674722207e-3; SiC->IPyC = 9.9773358413252777e-1.
I3 SiC->OPyC = 9.9773358413252777e-1; OPyC->SiC = 2.2664158674722207e-3.

These values explain the extreme recurrence.

## Stable transform tests

sinh-ratio tests:
- (1e-8,2e-8) -> 0.5;
- (2,3) -> 0.362038898880996;
- (1000,1200) -> 1.3838965267367376e-87.

The scaled large-argument path remains finite and does not overflow.

## s=0 probability / absorption gate

Executed:
rho(K(0)) = 9.9999999994964917e-1.
Gap 1-rho = 5.0350834612800099e-11.

Thus rho(K(0))<1, but the chain is extremely close to nonabsorbing, consistent with the rare-interface recurrence observed in explicit simulation.

All row transient probability plus direct absorption sums are 1 to floating-point precision.

State transforms at s=0 range from 0.999999887846684 to 0.9999999997622344.
Phi_init(0)=0.9999998878466840.

Normalization error ~1.12e-7 therefore **FAILS** the predeclared 1e-9 normalization tolerance in ordinary f64.

This is consistent with severe conditioning rather than a probability-topology defect, but it is not acceptable to silently waive the criterion.

## Positive-s rho=NaN investigation

The NaN values at positive s do NOT arise from K(s), transform evaluation, or the linear solve.

The harness source explicitly contains:

`let r = if s==0.0 { rho(k) } else { f64::NAN };`

Therefore rho was intentionally evaluated only at s=0, exactly where the predeclared spectral-radius gate applies. Positive-s `rho=NaN` means **not evaluated**, not numerical matrix failure.

Evidence against matrix NaN propagation:
- every positive-s Phi_i is finite;
- every Phi_init is finite;
- bounds checks are true;
- monotonicity checks are true;
- quadrature values are finite/convergent;
- residuals are finite.

Future output should print `rho=NOT_EVALUATED` or omit the field for s>0 to avoid ambiguity.

## Transform bounds and monotonicity

All executed `bounds=true`.
All executed `monotone=true`.

Phi_init decreases:
~0.9999999 at s=0
-> 0.9550171 at 1e-9
-> 0.6756518 at 1e-8
-> 0.1476392 at 1e-7
-> 0.00405621 at 1e-6
-> 3.22490e-7 at 1e-5
-> 1.06757e-19 at 1e-4.

No transform-bound or monotonicity defect is observed.

## Quadrature convergence

For Nq=1k,10k,100k the 10k->100k Phi_init changes are:
- s=0: 0;
- 1e-9: 2.364e-9;
- 1e-8: 1.674e-9;
- 1e-7: 3.685e-10;
- 1e-6: 1.088e-11;
- 1e-5: 1.454e-15;
- 1e-4: 2.225e-27.

Quadrature is well converged relative to the observed f64 matrix-solve limitations.

## Conditioning and residuals

Frobenius condition estimate kappa_F^est:
- s=0: 1.037038e11;
- 1e-9: 9.926466e10;
- 1e-8: 7.167475e10;
- 1e-7: 1.905236e10;
- 1e-6: 2.343578e9;
- 1e-5: 2.489517e8;
- 1e-4: 2.841687e7.

Residuals:
- s=0: 1.286e-13;
- 1e-9: 1.435e-13;
- 1e-8: 1.536e-13;
- 1e-7: 1.420e-13;
- 1e-6: 5.184e-13;
- 1e-5: 1.138e-10;
- 1e-4: 4.337e-19.

The predeclared residual criterion is 1e-10. The s=1e-5 value 1.138e-10 therefore **FAILS narrowly**. It must not be rounded into a pass.

The custom Jacobi-derived `sigma_min` values are not internally consistent with the very large Frobenius condition estimates at several s points and are therefore **not accepted as reliable singular-value evidence**. This diagnostic requires remediation or an independent robust SVD/eigenvalue implementation.

## Scientific classification

The hard matrix topology, stable transform evaluation, transform bounds, monotonicity, quadrature convergence, and s=0 spectral-radius inequality all behave coherently.

However the full predeclared hard diagnostic does **NOT yet pass** because:
1. f64 normalization at s=0 misses the 1e-9 tolerance (~1.12e-7 error);
2. the s=1e-5 residual is 1.138e-10 > 1e-10;
3. the custom smallest-singular-value estimator is not reliable enough to support the requested conditioning evidence.

These failures are numerically consistent with the extreme recurrence / ~1e11 conditioning and do not presently demonstrate a matrix-topology or physical-model defect.

The next justified action is a SMALL high-precision/robust-linear-algebra cross-check of the same frozen 8x8 matrices. This is explicitly permitted by the predeclared gate when f64 appears materially affected by conditioning.

No inverse Laplace or release CDF is authorized.

R2-WOS-02 remains OPEN.
R2-B01 remains OPEN.
