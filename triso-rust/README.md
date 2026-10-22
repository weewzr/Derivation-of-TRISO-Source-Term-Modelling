# TRISO Rust Solver

Five-physical-layer spherical diffusion and release model with two independent solvers for the same problem:

- a conservative finite-volume (FV) solver, Backward Euler in time (`src/fv.rs`);
- the exact eigenfunction-series solution (`src/analytical.rs`).

Both read the same input file. The program runs the FV solver, solves the same case analytically, compares the two and writes CSV files and SVG figures.

## Run

```bash
cd triso-rust
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release
cargo run --release -- config/synthetic_five_layer.toml
cargo run --release --example verify
```

Options for the main program: `--fv-only` skips the analytical cross-check, `--no-convergence` skips the refinement study.

The synthetic configuration is executable numerical test data, not literature TRISO data.

## Using literature values

1. Copy `config/literature_inputs.template.toml`.
2. Replace every `INSERT_VALUE_HERE` with a literature-verified SI value and keep the citation in the adjacent `source` comment.
3. Run `cargo run --release -- <your file>.toml`.

A file that still contains `INSERT_VALUE_HERE` is rejected with a list of every unfilled value and the instruction written above it. Solver code never needs editing.

The supported case is: initial kernel concentration, no source after $t=0$, absorbing outer surface, constant diffusivity in each region. `temperature_k` and `species` are recorded for provenance and do not change the calculation, because diffusivities are entered directly.

## Outputs (written to `output.directory`)

| File | Content |
|---|---|
| `timeseries.csv` | time, remaining inventory, cumulative release, release fraction and rate, conservation residual, minimum concentration |
| `final_profile.csv` | cell index, material index, volume-centroid radius, final concentration |
| `comparison_fv_vs_analytical.csv` | release fraction from FV and from the exact series at each output time, absolute error |
| `convergence_study.csv` | error against the exact answer for refined meshes and refined time steps |
| `final_profile.svg`, `release_fv_vs_analytical.svg`, `convergence.svg` | figures |

The console summary also reports the exact release fraction, observed convergence orders and the accuracy floor of the exact series.

## Structure

- `src/fv.rs`: mesh, conservative conductances, Backward Euler with a subtraction-free tridiagonal solve, inventory and release.
- `src/analytical.rs`: exact five-layer release series (propagation with the correct interface rule, Sturm-count root isolation, modal weights) and the homogeneous Robin root finder.
- `src/compare.rs`: FV against exact comparison and refinement studies.
- `src/plots.rs`: SVG figures.
- `src/config.rs`: TOML input and validation.
- `examples/verify.rs`: verification suite whose printed claims are all assertions.
- `tests/analytical_vs_fv.rs`: cross-module tests run by `cargo test`.

## Scope

Five physical layers remain five materials regardless of the number of numerical cells. Interfaces coincide with FV faces. Temperature and fluence dependent diffusivity, source schedules and a Robin outer boundary in the FV solver are not implemented.

When adjacent diffusivities differ by more than about $10^{6}$, the exact series loses accuracy to cancellation. The program prints its accuracy floor and warns when the FV and exact results differ by less than that floor.
