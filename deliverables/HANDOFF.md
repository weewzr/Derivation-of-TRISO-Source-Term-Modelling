# Handoff: TRISO Diffusion and Release Package

## What this is

A TRISO fuel particle is a tiny sphere built from five layers: Kernel, Buffer, IPyC, SiC and OPyC. At the start a substance (for example a fission product) sits inside the kernel. Over time it spreads outward through the layers and some of it escapes from the particle.

This package calculates how much has escaped after a given time. It does this in two separate ways and compares the answers:

- **Code B** cuts the particle into thin shells and steps forward in time on a computer (the finite-volume approach).
- **Code C** uses an exact formula built from sine waves (the Sturm-Liouville approach).

**Code A**, the main file, reads one input file, runs B and C on it and writes the results and figures.

## What is in this folder

| File or folder | What it is |
|---|---|
| `TRISO_Finite_Volume_Derivation.pdf` (also `.md` and `.tex`) | The maths, step by step, from "atoms are not created or destroyed" to the equations the program solves |
| `TRISO_Finite_Volume_Pseudocode.md` | The method as a short list of steps |
| `finite_volume_code/` | The program you can run |
| `finite_volume_code/src/main.rs` | **Code A, main file** |
| `finite_volume_code/src/discrete_finite_volume_tridiagonal.rs` | **Code B, discrete approach** (finite volume, tridiagonal solve) |
| `finite_volume_code/src/analytical_sturm_liouville.rs` | **Code C, analytical approach** (Sturm-Liouville eigenfunction series) |
| `finite_volume_code/config/literature_inputs.template.toml` | Empty input file for your literature values |
| `finite_volume_code/config/synthetic_five_layer.toml` | Ready-to-run example input (made-up numbers for testing) |

The other files in `finite_volume_code/src/` handle reading the input (`config.rs`), comparing B with C (`compare.rs`) and drawing the figures (`plots.rs`).

## Try it

1. Install Rust once from https://rustup.rs.
2. Open a terminal in the folder `deliverables/finite_volume_code`.
3. Run:

```bash
cargo run --release -- config/synthetic_five_layer.toml
```

4. Open the folder `results/synthetic-five-layer/`. It contains the tables (`.csv`) and three figures (`.svg`, open them in a web browser).

The terminal prints the fraction of the substance released, the exact answer from code C, and how quickly the finite-volume error shrinks when the mesh or the time step is refined.

To run all the built-in checks:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run --release --example verify
```

## Using your own numbers

1. Copy `config/literature_inputs.template.toml` to a new file.
2. Replace every `INSERT_VALUE_HERE` with a value from the literature, in SI units, and keep the reference in the `source` comment next to it.
3. Run `cargo run --release -- your_file.toml`.

If any `INSERT_VALUE_HERE` is left, the program stops and lists every value you still need to fill. You never need to edit the program itself. The example file contains made-up numbers for testing and is not real TRISO data. Diffusivities are typed in directly, so `species` and `temperature_k` are only recorded in the file and do not change the calculation.

## Results the program writes

Files go to the folder named under `[output]` in the input file.

| File | Content |
|---|---|
| `timeseries.csv` | time, amount still inside, amount released, fraction released, release rate, balance check, lowest concentration |
| `final_profile.csv` | concentration against radius at the end time |
| `comparison_fv_vs_analytical.csv` | fraction released from code B and from code C at each output time, and their difference |
| `convergence_study.csv` | error against the exact answer for finer meshes and smaller time steps |
| `release_fv_vs_analytical.svg`, `convergence.svg`, `final_profile.svg` | figures |

Two options for code A: `--fv-only` skips code C and the figures that need it, and `--no-convergence` skips the mesh and time-step study.

## What the model assumes

- Five nested spherical layers, with the properties depending only on the distance from the centre.
- Each layer has one constant diffusivity.
- Layers touch perfectly: the concentration is the same on both sides of a boundary and nothing is lost there.
- At the start the substance is only in the kernel, nothing new is made afterwards, and the outer surface removes everything that reaches it.

## Limits

- The example numbers are for testing only. No literature values are included.
- Temperature-dependent diffusivity, ongoing sources and a partly blocking outer surface (Robin condition) are not implemented in the program.
- The exact answer from code C loses accuracy when two neighbouring layers differ in diffusivity by more than about a million times. The program prints how accurate code C is for each run and warns if the comparison is too close to call.
- Code B steps forward in time with Backward Euler, which is first-order accurate. Choose the time step with the help of the refinement study the program prints.

## Last full check

GitHub Actions run 39729930098 on `main` (Ubuntu, Rust 1.99.1) passed the format check, `clippy`, all tests, the release build, the verification example, the example run and the PDF build. Stored outputs are in `verification/delivery_checks/`.
