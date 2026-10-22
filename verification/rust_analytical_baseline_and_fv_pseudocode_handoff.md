# Rust implementation handoff — uploaded analytical baseline and concise FV pseudocode

## Scope

The user-supplied Rust reference is an **analytical eigenfunction/transfer-matrix benchmark** with:
- `uom` for typed SI quantities;
- `csv` for outputs;
- `plotters` for concentration history and radial profiles;
- homogeneous Robin sphere analytical series;
- five-layer transfer-matrix eigenproblem and modal coefficients.

Preserve it as a separate analytical reference. Do not mislabel it as a finite-volume solver. The first supervisor deliverable remains the generic five-physical-layer, arbitrary-N conservative FV Rust simulator.

## Minimal equations for the supervisor-facing code overview

Governing radial diffusion in material i:

```math
\frac{\partial c_i}{\partial t}=\frac{1}{r^2}\frac{\partial}{\partial r}\left(r^2D_i\frac{\partial c_i}{\partial r}\right)+S_i.
```

Conservative cell balance:

```math
V_j\frac{dc_j}{dt}=G_W(c_{j-1}-c_j)+G_E(c_{j+1}-c_j)+S_jV_j.
```

Backward Euler:

```math
-G_Wc_{j-1}^{n+1}+\left(\frac{V_j}{\Delta t}+G_W+G_E\right)c_j^{n+1}-G_Ec_{j+1}^{n+1}=\frac{V_j}{\Delta t}c_j^n+S_j^{n+1}V_j.
```

Forward Euler verification-only positivity/stability condition:

```math
\Delta t\leq\min_j\frac{V_j}{G_W+G_E}.
```

Backward Euler has no explicit diffusion CFL stability restriction for the standard linear monotone operator, but temporal convergence and positivity must still be checked.

Release fraction:

```math
F(t)=\frac{M_{\mathrm{released}}(t)}{M(0)}.
```

These are a **concise code overview**, not a substitute for the derivation of shell volumes, face resistance, boundary conditions, or interface laws. Retain those intermediate steps in the formal mathematical submission.

## Code design

- `analytical/homogeneous.rs` and `analytical/five_layer.rs`: retain/reference the supplied analytical benchmark.
- `fv/mesh.rs`: five physical radii, per-material cell allocations, interface-aligned spherical volumes.
- `fv/operator.rs`: conservative face conductances and boundary treatment.
- `fv/timestep.rs`: implicit Backward Euler and tridiagonal solve.
- `fv/release.rs`: cumulative flux, remaining inventory, release CDF, conservation residual.
- `verification/convergence.rs`: homogeneous/two-layer/five-layer tests and N, dt refinement.
- `output.rs`: CSV and optional plots.

## User-requested literature placeholder convention

Every missing physical input must be bracketed by exactly one explanatory comment above and one closing comment below. Use descriptive `INSERT_...` tokens, for example:

```rust
// INSERT LITERATURE-VERIFIED DIFFUSIVITIES BELOW
let diffusivities = [
    INSERT_KERNEL_D,
    INSERT_BUFFER_D,
    INSERT_IPYC_D,
    INSERT_SIC_D,
    INSERT_OPYC_D,
];
// END LITERATURE-VERIFIED DIFFUSIVITIES
```

Do not present a non-compiling placeholder configuration as a verified executable. Keep a separate `examples/synthetic_verified.toml` with explicitly synthetic, tested values, and a `examples/literature_inputs.template.toml` with documented placeholder fields and source-citation slots.

Never invent literature parameter values. Record nuclide, temperature, irradiation assumptions, units and citation for each filled diffusivity.

## Next implementation instruction

Use this handoff together with `DELIVERY_SPRINT_RUST_FV.md`, `WORKFLOW_HIGH_RESOLUTION.md`, and `verification/primary_arbitrary_n_fv_gate.md`.

The code should compile and execute the synthetic verification configuration, while the literature-input template remains unfilled until defensible values are supplied.

Do not modify the user's original uploaded analytical code unless it is added explicitly as a separate benchmark with traceable changes. Do not begin WOS, Method 3, or new manuscript expansion.
