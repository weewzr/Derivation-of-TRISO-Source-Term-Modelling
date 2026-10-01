# Hard 8x8 High-Precision Numerical Remediation Gate

## Scope

Numerical-diagnostics remediation of the already frozen hard five-layer matrix only.

No physics, topology, real-s points, or acceptance criteria change.
No inverse Laplace and no release CDF.

Canonical f64 evidence: run 36866849598 / commit 5171a9ae214881db3c3a87696b9aa04af0c56dc5.

## Precision sequence

Predeclared decimal working precisions:

- 50 digits;
- 80 digits (primary reference);
- 120 digits.

The sequence is fixed before execution. Precision is adequate only if 80- and 120-digit results agree comfortably beyond the original 1e-9 normalization and 1e-10 residual criteria.

## Frozen benchmark

Exactly the hard gate values:
R=[2.125e-4,3.125e-4,3.525e-4,3.875e-4,4.275e-4] m.
D=[1.2502982636347968e-13,1e-8,4.062299125614697e-14,9.227773168241615e-17,4.062299125614697e-14] m2/s.
epsilon=100 nm; alpha=2; delta=200 nm; K=1.
Same eight states, same transition table, same uniform-volume kernel source.

Real-s points remain:
[0,1e-9,1e-8,1e-7,1e-6,1e-5,1e-4] s^-1.

## High-precision implementation

Python mpmath arbitrary-precision arithmetic:
- exact same stable shell/ball transform formulas;
- mp.lu_solve for (I-K)Phi=B;
- mp.eig for spectral radius at s=0;
- mp.svd for singular values / spectral 2-norm condition number;
- high-precision residual norm.

NumPy float64 SVD is also evaluated on the same matrix as an independent robust ordinary-precision conditioning cross-check.

The custom Rust Jacobi sigma_min is not used as trusted evidence.

## Initial-radius integration

Use the analytical radial integral of the centered-ball transform where numerically stable, independently cross-checked against the already converged 100k midpoint result.

For s=0 the source factor is exactly one.

For s>0, numerical high-precision quadrature is used only to compare against the stored converged f64 midpoint values; it is not allowed to change the matrix.

## Original gates retained

Normalization at s=0:
max_i |Phi_i-1| < 1e-9;
|Phi_init-1| < 1e-9.

Relative residual:
||A Phi-B||2/max(||B||2,1) < 1e-10.

rho(K(0))<1.

All transforms finite and in [-1e-10,1+1e-10].
Monotonicity tolerance 1e-10.

## Required comparison

At each s report:
- stored f64 Phi_init;
- 50/80/120-digit Phi_init;
- f64 minus 80-digit absolute/relative difference;
- stored f64 residual;
- 80-digit residual;
- kappa_2 and sigma_min from high-precision SVD;
- NumPy f64 kappa_2 and sigma_min.

At s=0 report high-precision rho and 1-rho.

For s>0 spectral radius is printed as NOT_EVALUATED, not NaN.

## Classification

A: high precision passes original gates and precision convergence shows f64 failures are conditioning error.
B: high precision still fails an original gate, indicating deeper issue.
C: high-precision evidence remains unresolved.

Do not select classification before execution.
