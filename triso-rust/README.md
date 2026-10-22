# TRISO Rust FV Solver

Standalone five-physical-layer spherical finite-volume diffusion/release solver.

## Run

~~~bash
cd triso-rust
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release
cargo run --release -- config/synthetic_five_layer.toml
cargo run --release --example verify
~~~

The synthetic configuration is executable numerical test data, **not literature TRISO data**.

For a physical case, copy and fill \`config/literature_inputs.template.toml\`. The current MVP supports piecewise-constant five-layer diffusivities, initial kernel concentration, no continuing source after t=0, and an absorbing outer boundary.

## Outputs

- \`timeseries.csv\`: time, remaining inventory, cumulative release, release fraction/rate, conservation residual, minimum concentration.
- \`final_profile.csv\`: cell/material/radius/final concentration.

## Structure

- \`src/fv.rs\` - mesh, conservative conductances, Backward Euler, tridiagonal solve, inventory/release.
- \`src/analytical.rs\` - separate homogeneous Robin root/transfer-matrix baseline.
- \`src/config.rs\` - five-region TOML input.
- \`examples/verify.rs\` - homogeneous, controlled two-layer and five-layer convergence checks.
- \`config/synthetic_five_layer.toml\` - executable synthetic case.
- \`config/literature_inputs.template.toml\` - user-editable literature placeholder template.

## Scope

Five physical layers remain five materials regardless of numerical cell count. Interfaces coincide with FV faces. The analytical Robin/source benchmark and FV initial-inventory/absorbing release problem are separate scientific cases.

Temperature/fluence-dependent diffusivity, arbitrary source schedules and production Robin boundary conditions are deferred.
