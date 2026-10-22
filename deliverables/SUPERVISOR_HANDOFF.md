# Supervisor Handoff - TRISO Diffusion/Release Package

## Package purpose

This package provides the concise derivation and a standalone Rust implementation of the reviewed conservative spherical finite-volume model for five physical TRISO regions. The primary production case is initial kernel inventory, no continuing source after t=0, ideal interface continuity and an absorbing OPyC outer boundary.

## Deliverables

- \`TRISO_Final_Mathematical_Derivation.md\`
- \`TRISO_Final_Mathematical_Derivation.tex\`
- \`TRISO_Final_Mathematical_Derivation.pdf\` (generated/persisted by final verification)
- \`TRISO_FV_Pseudocode.md\`
- \`../triso-rust/src/analytical.rs\`
- \`../triso-rust/src/fv.rs\`
- \`../triso-rust/config/literature_inputs.template.toml\`
- \`../triso-rust/config/synthetic_five_layer.toml\`

## Build and run

~~~bash
cd triso-rust
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release
cargo run --release -- config/synthetic_five_layer.toml
cargo run --release --example verify
~~~

## Literature values

Do not edit solver logic to enter physical data. Copy \`config/literature_inputs.template.toml\`, replace every \`INSERT_VALUE_HERE\` using literature-verified SI values, and retain citation provenance. The supplied synthetic case is numerical test data only.

## CSV interpretation

\`timeseries.csv\` reports remaining inventory, cumulative boundary release, release fraction, boundary release rate, balance residual, and minimum concentration. \`final_profile.csv\` reports cell index, material index, volume-centroid radius and final concentration.

## Scientific assumptions

Five concentric physical regions; spherical symmetry; piecewise-constant diffusivity; ideal concentration/flux interface continuity; interface-aligned FV faces; exact spherical shell volumes; conservative series-resistance face conductance; Backward Euler; absorbing outer boundary.

## Analytical module

The analytical module is intentionally separate. It implements a homogeneous Robin eigenvalue residual/root finder and a homogeneous-layer transfer building block. The repository contains a structural summary of the user's supplied analytical Rust example, but not its original full function bodies; this package therefore does **not** claim verbatim reconstruction or complete validation of the original five-layer modal code.

## Verification and provenance

Final executed evidence is produced by \`.github/workflows/final-supervisor-delivery.yml\` and persisted under \`deliverables/\`. The handoff is supervisor-ready only after that workflow passes Rust formatting, clippy, tests, release build, numerical benchmarks, five-layer execution and PDF compilation/render checks.

Existing reviewed FV evidence independently showed approximately second-order spatial convergence on the aligned steady synthetic benchmark and first-order Forward-Euler temporal self-convergence. The final package adds arbitrary per-region meshes and Backward Euler.

## Known limitations

- Synthetic inputs are not experimental/physical validation.
- Temperature/fluence-dependent diffusivity is not implemented.
- Continuing source schedules are deferred.
- Production Robin boundary mode is deferred; absorbing release is implemented.
- Analytical five-layer modal completeness/truncation remains outside primary FV acceptance.
- WOS/Process-C remain separate verification tracks and are not proof of FV/WOS equivalence.
- No GUI, CFD or Method-3 functionality is included.

## Repository policy

All work is in the research repository. Nothing is pushed to Theodore's backend.


## Final executed acceptance

Final acceptance workflow: **39711910260**.

Artifact: **12396634301**, digest **sha256:19062d34e795cc053ebccec863d2249c0826ace6853352009a1d68babe85c8a2**.

Source commit exercised: **803bfd4a8c44b981406290253cbbaa2c3314433f**.

Persisted evidence commit: **cd935cd4b75725aaf170c69d8837d580a058817b**.

Mandatory gates:
- rustfmt PASS;
- clippy -D warnings PASS;
- cargo test PASS;
- release build PASS;
- homogeneous analytical benchmark PASS, absolute error 2.780e-5;
- controlled two-layer benchmark PASS, 32 s absolute error 1.847e-10;
- five-layer synthetic spatial refinement shows approximately second-order reduction;
- Backward-Euler temporal refinement shows approximately first-order reduction;
- positive concentrations and conservative inventory/release accounting PASS at the declared numerical scale;
- executable five-layer synthetic case PASS with final release fraction 0.4324286068846;
- LaTeX PDF compilation PASS;
- PDF: 3 A4 pages; rendered pages 1, 2 and 3 visually inspected with no clipping, overlap or broken mathematical glyphs.

See TRISO_Final_Parity_Check.md and the persisted numerical result files for the detailed handoff evidence.
