# Rust FV Solver — Minimal Equation and Pseudocode Handoff

## Purpose
Primary supervisor deliverable: a runnable five-physical-layer, arbitrary-N conservative spherical FV solver with Backward Euler and tridiagonal solution.

## Final equations

Five-layer diffusion PDE:
```math
\frac{\partial c_i}{\partial t}=\frac{1}{r^2}\frac{\partial}{\partial r}\left(r^2D_i\frac{\partial c_i}{\partial r}\right)+S_i.
```

Conservative FV cell balance:
```math
V_j\frac{dc_j}{dt}=G_W(c_{j-1}-c_j)+G_E(c_{j+1}-c_j)+S_jV_j.
```

Backward Euler:
```math
-G_Wc_{j-1}^{n+1}+\left(\frac{V_j}{\Delta t}+G_W+G_E\right)c_j^{n+1}-G_Ec_{j+1}^{n+1}=\frac{V_j}{\Delta t}c_j^n+S_j^{n+1}V_j.
```

Explicit Forward-Euler verification stability/positivity bound:
```math
\Delta t\leq\min_j\frac{V_j}{G_W+G_E}.
```

Release fraction:
```math
F(t)=\frac{M_{\mathrm{released}}(t)}{M(0)}.
```

The complete mathematical derivation must separately retain shell-volume, face-resistance, interface and boundary intermediate steps. These five equations are the compact code overview.

## Pseudocode
```text
READ five physical radii, diffusivities, cell allocations, dt, t_end
BUILD interface-aligned spherical FV mesh
CALCULATE shell volumes, representative radii, material indices
ASSEMBLE conservative internal face conductances and outer boundary conductance
INITIALISE inventory in kernel, zero elsewhere
FOR each physical timestep:
    ASSEMBLE Backward-Euler tridiagonal system
    SOLVE for updated cell concentrations
    CALCULATE outer release flux and accumulated release
    CHECK positivity, inventory balance, CDF bounds
    WRITE time-series data
WRITE radial concentration profiles
REPEAT for mesh/time refinement sequence
REPORT convergence, runtime, physical assumptions
```

## Rust structure
```text
src/fv/mesh.rs
src/fv/operator.rs
src/fv/timestep.rs
src/fv/release.rs
src/verification/convergence.rs
src/output.rs
```

## Literature placeholder convention
```rust
// INSERT LITERATURE-VERIFIED FIVE-LAYER DIFFUSIVITIES BELOW
let diffusivities = [
    INSERT_KERNEL_D,
    INSERT_BUFFER_D,
    INSERT_IPYC_D,
    INSERT_SIC_D,
    INSERT_OPYC_D,
];
// END LITERATURE-VERIFIED FIVE-LAYER DIFFUSIVITIES
```

The placeholder template intentionally cannot compile. Maintain a separate **working synthetic benchmark configuration** for reproducible build and tests. Physical values require nuclide, temperature, units, model assumptions and literature citations.

## Acceptance
Cargo build, unit tests, homogeneous/two-layer checks, five-layer execution, conservation, positivity, mesh/time convergence, CSV outputs, reproducible instructions. Do not claim code is verified before these execute.
