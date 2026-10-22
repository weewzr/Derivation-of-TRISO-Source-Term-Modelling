# TRISO Finite-Volume Code

This program calculates how much of a substance escapes from a TRISO fuel particle (a sphere with five layers: Kernel, Buffer, IPyC, SiC, OPyC) when it starts inside the kernel. It solves the problem in two independent ways and compares them.

- **Code A, main file** (`src/main.rs`): reads the input file, runs B and C, compares them and writes the tables and figures.
- **Code B, discrete approach** (`src/discrete_finite_volume_tridiagonal.rs`): finite volumes (thin spherical shells), Backward Euler in time, tridiagonal solve.
- **Code C, analytical approach** (`src/analytical_sturm_liouville.rs`): exact Sturm-Liouville eigenfunction series (sums of sine-wave shapes that fade with time).

## Run it

You need Rust, installed once from https://rustup.rs. Then, in this folder:

```bash
cargo run --release -- config/synthetic_five_layer.toml
```

Results appear in `results/synthetic-five-layer/`.

To run every built-in check:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run --release --example verify
```

Options for the main program: `--fv-only` skips code C, and `--no-convergence` skips the refinement study.

## Using literature values

1. Copy `config/literature_inputs.template.toml`.
2. Replace every `INSERT_VALUE_HERE` with a literature value in SI units and keep the reference in the `source` comment.
3. Run `cargo run --release -- <your file>.toml`.

If a placeholder is left, the program stops and lists every value that is still missing. The program itself never needs editing.

The supported case: all the substance starts in the kernel, nothing new is made, the outer surface removes everything that reaches it, and each layer has one constant diffusivity. `temperature_k` and `species` are recorded for your own bookkeeping and do not change the calculation, because diffusivities are typed in directly. The example file `config/synthetic_five_layer.toml` holds made-up numbers for testing.

## Files the program writes

| File | Content |
|---|---|
| `timeseries.csv` | time, amount inside, amount released, fraction released, release rate, balance check, lowest concentration |
| `final_profile.csv` | cell, material, radius and concentration at the end time |
| `comparison_fv_vs_analytical.csv` | fraction released from code B and code C at each output time, and the difference |
| `convergence_study.csv` | error against the exact answer for finer meshes and smaller time steps |
| `final_profile.svg`, `release_fv_vs_analytical.svg`, `convergence.svg` | figures (open in a web browser) |

The terminal summary also gives the exact release fraction, the observed convergence orders and the accuracy of code C for that run.

## Folder map

- `src/main.rs`: code A.
- `src/discrete_finite_volume_tridiagonal.rs`: code B (mesh, flow between shells, time stepping, tridiagonal solve, release and balance).
- `src/analytical_sturm_liouville.rs`: code C (exact release series, plus the root finder for the homogeneous Robin sphere).
- `src/compare.rs`: code B against code C, and the refinement studies.
- `src/plots.rs`: figures.
- `src/config.rs`: reads and checks the input file.
- `examples/verify.rs`: built-in checks whose printed claims are all assertions.
- `tests/finite_volume_vs_analytical.rs`: tests run by `cargo test`.

## Limits

Temperature-dependent diffusivity, ongoing sources and a partly blocking outer surface are not implemented. When two neighbouring layers differ in diffusivity by more than about a million times, code C loses accuracy. The program prints how accurate code C is and warns when the comparison is too close to call.
