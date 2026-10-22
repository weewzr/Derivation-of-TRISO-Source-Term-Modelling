# TRISO FV Supervisor Pseudocode

## Governing equation

[
rac{partial c_i}{partial t}
=
rac{1}{r^2}rac{partial}{partial r}
left(r^2D_irac{partial c_i}{partial r}ight)+S_i.
]

## Conservative FV equation

[
V_jrac{dc_j}{dt}
=
G_W(c_{j-1}-c_j)+G_E(c_{j+1}-c_j)+S_jV_j,
qquad
G_f=rac{A_f}{delta_P/D_P+delta_E/D_E}.
]

## Backward Euler

[
-G_Wc_{j-1}^{n+1}
+left(rac{V_j}{Delta t}+G_W+G_Eight)c_j^{n+1}
-G_Ec_{j+1}^{n+1}
=
rac{V_j}{Delta t}c_j^n+S_j^{n+1}V_j.
]

## Forward Euler verification-only bound

[
Delta tlemin_jrac{V_j}{G_W+G_E}.
]

This is a sufficient positivity/stability check for explicit verification. Production uses Backward Euler.

## Release and conservation

[
M^n=sum_jV_jc_j^n,qquad
M_{m rel}^{n+1}=M_{m rel}^{n}+Delta t,G_Rc_N^{n+1},
]

[
F^n=rac{M_{m rel}^n}{M^0},qquad
M^n+M_{m rel}^n-M^0approx0.
]

## Calculation sequence

```text
READ five radii, five diffusivities, per-region cell counts, dt, t_end
READ initial inventory/concentration and supported source/boundary settings

BUILD five-region interface-aligned spherical mesh
FOR each cell:
    compute exact shell volume
    compute spherical volume-centroid radius
    assign physical material and diffusivity

FOR each internal face:
    compute face area
    compute left/right half-cell distances
    compute series-resistance conductance

SET centre west conductance = 0
SET absorbing outer boundary conductance

INITIALISE kernel inventory; zero coatings for primary release case

FOR each timestep:
    assemble Backward-Euler tridiagonal coefficients
    solve tridiagonal system
    compute boundary release rate
    accumulate released inventory using the same implicit boundary flux
    compute remaining inventory and mass-balance residual
    check concentration >= numerical tolerance
    check 0 <= release fraction <= 1 within tolerance
    write requested time-series/profile CSV rows

REPEAT with refined cell counts at fixed small dt
REPORT spatial convergence of release fraction/inventory

REPEAT with refined dt at fixed fine mesh
REPORT temporal convergence

REPORT runtime, positivity and maximum conservation residual
```
