> HISTORICAL ARCHIVE: superseded working document.\n\n# Two-Layer Transient Reference Data

Parameter set:

- `a = 2`
- `R = 5`
- `D1 = 3`
- `D2 = 0.25`
- `S0 = 2`

These are dimensionless demonstration values. They are not TRISO material properties.

The values below come from an independent numerical evaluation of the closed-form two-layer steady solution and the derived transient eigenproblem.

## Important numerical lesson

Two different checks are required:

1. **Eigenvalue accuracy:** does a numerical root satisfy the characteristic equation?
2. **Series convergence:** does adding more eigenmodes stop changing the transient value?

A tiny eigenvalue residual does not prove that a six-mode transient reconstruction is accurate.

## First six positive decay rates

| n | lambda_n | residual |
|---:|---:|---:|
| 1 | 1.015877382065339e-01 | about 1e-14 |
| 2 | 4.821203461986593e-01 | about 1e-13 |
| 3 | 1.309112742347257e+00 | about 1e-12 |
| 4 | 2.650558505301671e+00 | about 1e-12 |
| 5 | 4.512867578662497e+00 | about 1e-13 |
| 6 | 6.881436389856379e+00 | about 1e-13 |

The first positive decay rate is therefore approximately

$$
lambda_1 = 0.1015877382065.
$$

## Convergence at selected points

The table below shows the value obtained with 6, 12, 24 and 39 computed modes.

| r | t | 6 modes | 12 modes | 24 modes | 39 modes |
|---:|---:|---:|---:|---:|---:|
| 0 | 0.1 | 2.09686043884e-01 | 1.98008234776e-01 | 1.98389212688e-01 | 1.98389227358e-01 |
| 2 | 0.1 | 1.42848047109e-01 | 1.44588180469e-01 | 1.44616422407e-01 | 1.44616422746e-01 |
| 3 | 0.1 | -2.73115961570e-03 | 4.87380346423e-05 | 6.42988897681e-08 | 6.37334798270e-08 |
| 4 | 0.1 | -9.47441341356e-04 | 4.66801767465e-05 | -2.50557352643e-10 | -6.51301235166e-12 |
| 0 | 0.5 | 8.11048562410e-01 | 8.10807872086e-01 | 8.10807872601e-01 | 8.10807872601e-01 |
| 2 | 0.5 | 6.35091277275e-01 | 6.35123786761e-01 | 6.35123786789e-01 | 6.35123786789e-01 |
| 3 | 0.5 | 5.37520204695e-03 | 5.39702398196e-03 | 5.39702385858e-03 | 5.39702385858e-03 |
| 4 | 0.5 | -7.77300661192e-06 | 2.25573158930e-06 | 2.25563207712e-06 | 2.25563207712e-06 |
| 0 | 1.0 | 1.35937061932e+00 | 1.35936897096e+00 | 1.35936897096e+00 | 1.35936897096e+00 |
| 2 | 1.0 | 1.12360788526e+00 | 1.12360809242e+00 | 1.12360809242e+00 | 1.12360809242e+00 |
| 3 | 1.0 | 4.84878222418e-02 | 4.84879259611e-02 | 4.84879259611e-02 | 4.84879259611e-02 |
| 4 | 1.0 | 5.24522920284e-04 | 5.24565415098e-04 | 5.24565415098e-04 | 5.24565415098e-04 |
| 0 | 2.0 | 2.16657191953e+00 | 2.16657191943e+00 | 2.16657191943e+00 | 2.16657191943e+00 |
| 2 | 2.0 | 1.87136596394e+00 | 1.87136596395e+00 | 1.87136596395e+00 | 1.87136596395e+00 |
| 3 | 2.0 | 2.18999540710e-01 | 2.18999540716e-01 | 2.18999540716e-01 | 2.18999540716e-01 |
| 4 | 2.0 | 1.38914152954e-02 | 1.38914152974e-02 | 1.38914152974e-02 | 1.38914152974e-02 |

## Interpretation

At early time `t=0.1`, the outer-region values need many modes before they settle. A six-mode result can even become slightly negative during cancellation.

That does not mean the physical concentration is negative. It means the truncated mathematical series is not yet sufficiently resolved at that point.

By `t=0.5`, most listed points have stabilized by 12–24 modes.

By `t>=1` for these sample points, 12 modes are already essentially unchanged by adding more modes.

Therefore a final reference-data generator should choose the mode count adaptively rather than always using an arbitrary fixed number.

## Origin treatment

The numerical calculation must not evaluate

$$
\\frac{\\sin(k r)}{r}
$$

literally at `r=0`.

Use the mathematical limit:

$$
\\lim_{r\\to0}\\frac{\\sin(kr)}r=k.
$$

This was a real bug in the first independent oracle attempt and is now explicitly recorded as a verification requirement.

## Recommended convergence rule

For a requested point `(r,t)`, compute the partial sums with increasing mode counts:

$$
c_N(r,t).
$$

Then compare successive resolutions:

$$
e_N(r,t)=|c_N(r,t)-c_{N/2}(r,t)|.
$$

Stop only when `e_N` is below a declared absolute and relative tolerance over the full reference grid.

Also check that the reconstructed concentration is physically non-negative where the underlying initial-value problem guarantees non-negativity.

## Verification status

[VERIFIED] Positive eigenvalues can be found for the controlled two-layer case.

[VERIFIED] The characteristic equation reduces to the standard one-region spherical condition when D1=D2.

[VERIFIED] The singular-looking eigenfunction at r=0 has a finite analytic limit.

[VERIFIED] The transient series stabilizes as mode count increases for the sample points shown above.

[UNVERIFIED] A complete adaptive reference generator has been implemented in Rust.

[UNVERIFIED] WOS has been compared against the converged transient reference.

[UNVERIFIED] Finite capture epsilon and reinsertion introduce negligible transient bias.

## Next gate

The next scientific implementation should generate the reference curve automatically and store it with the exact parameter set and convergence tolerance.

Only then should WOS-versus-continuum discrepancies be quantified.