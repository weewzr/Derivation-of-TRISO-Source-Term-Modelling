# Two-Layer Transient Reference Data

Parameter set:

- `a = 2`
- `R = 5`
- `D1 = 3`
- `D2 = 0.25`
- `S0 = 2`

These are dimensionless demonstration values. They are not TRISO material properties.

The data below were generated with an independent numerical oracle from the closed-form two-layer steady solution and the derived transient eigenproblem.

## Important distinction

Two different numerical errors must be separated:

1. **Eigenvalue error** — whether a computed root really satisfies the characteristic equation.
2. **Series truncation error** — whether enough eigenmodes have been included to represent the transient accurately at the requested time.

A tiny eigenvalue residual does not prove that a 6-mode or 12-mode transient reconstruction is converged.

## First six positive decay rates

| n | lambda_n | residual |
|---:|---:|---:|
| 1 | 1.015877382065170e-01 | 1.52e-14 |
| 2 | 4.821203461986969e-01 | 5.92e-14 |
| 3 | 1.309112742347551e+00 | -8.59e-13 |
| 4 | 2.650558505302132e+00 | 2.08e-12 |
| 5 | 4.512867578662563e+00 | -3.57e-13 |
| 6 | 6.881436389856342e+00 | -2.06e-13 |

The first positive mode is therefore:

$$
lambda_1 \\approx 0.1015877382065.
$$

The characteristic-equation residual at the root is approximately `1.5e-14`.

## Example transient values

Using the modal reconstruction with many of the independently calculated modes:

| t | c(0,t) | c(a,t) | c(3,t) | c(4,t) |
|---:|---:|---:|---:|---:|
| 0.1 | 1.983892131893e-01 | 1.446164226465e-01 | 6.431128404189e-08 | -2.444756036749e-10 |
| 0.5 | 8.108078728108e-01 | 6.351237869236e-01 | 5.397023825486e-03 | 2.255637671427e-06 |
| 1.0 | 1.359368971064e+00 | 1.123608092502e+00 | 4.848792593923e-02 | 5.245654173102e-04 |
| 2.0 | 2.166571919482e+00 | 1.871365963998e+00 | 2.189995407156e-01 | 1.389141529571e-02 |
| 5.0 | 3.651547058289e+00 | 3.287978683990e+00 | 8.354193455754e-01 | 1.739037347699e-01 |
| 10.0 | 4.981220262150e+00 | 4.579378580175e+00 | 1.620927683709e+00 | 4.997698267830e-01 |

The tiny negative value at `t=0.1, r=4` is a warning about series truncation/cancellation at that very early time, not evidence of a physical negative concentration. The next executable reference should use enough modes and a truncation/convergence test before treating early-time values as reference data.

## What is safe to use now

Safe:

- the first eigenvalue and the characteristic equation residual;
- the steady-state solution;
- the equal-diffusivity limiting equation;
- later-time modal behaviour where the truncated series is demonstrably converged.

Not yet safe to label as final reference:

- early-time pointwise concentration values from a fixed small number of modes;
- WOS-vs-reference error from a comparison that has not established series convergence.

## Next numerical requirement

For every requested `(r,t)`, evaluate the modal sum with increasing mode counts `N` until:

$$
|c_N(r,t)-c_{N/2}(r,t)|
$$

is below a declared tolerance.

Only then should the value be written to the machine-readable reference dataset.