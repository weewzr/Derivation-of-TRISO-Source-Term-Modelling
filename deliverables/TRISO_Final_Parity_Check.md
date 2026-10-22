# Final Markdown / LaTeX / Rust Parity Check

Executed after delivery workflow 39711910260.

## Markdown vs LaTeX

The concise Markdown and LaTeX deliverables carry the same scientific chain and assumptions: control-volume conservation; local conservation law; Fick's law; spherical multilayer PDE; centre/interface/outer conditions; separate analytical/modal reference; exact spherical FV geometry; series-resistance interface conductance; centre and absorbing outer treatment; Backward-Euler tridiagonal equation; Forward-Euler verification-only bound; inventory/release conservation; and the numerical verification contract.

No scientific case is silently merged: the analytical Robin/source reference remains distinct from the FV initial-inventory/absorbing release case.

## Derivation vs Rust

triso-rust/src/fv.rs implements five physical regions with arbitrary per-region cell counts, interface-aligned faces, exact shell volumes and volume centroids, conservative series-resistance conductances, zero-area centre treatment, absorbing outer conductance, Backward-Euler tridiagonal solve, boundary-flux release accumulation, inventory/release conservation diagnostics, and the explicit verification-only timestep bound.

triso-rust/src/analytical.rs remains separate and implements the homogeneous Robin eigenvalue residual/root search and homogeneous-layer transfer building block. It is not claimed to be a complete reconstruction of the unavailable original analytical upload.

## Executed verification

Workflow: 39711910260.

- rustfmt: PASS
- clippy with -D warnings: PASS
- cargo test: PASS
- release build: PASS
- homogeneous FV/analytical error: 2.780e-5
- controlled two-layer 32 s error: 1.847e-10
- five-layer spatial refinement differences: 1.6441e-5, 4.1107e-6, 1.0276e-6, 2.5729e-7 (approximately 4x reduction)
- Backward-Euler temporal differences: 2.4270e-4, 1.2147e-4, 6.0766e-5 (approximately 2x reduction)
- five-layer synthetic final release fraction: 0.4324286068846
- final concentration minimum: positive
- PDF compile/render: PASS, 3 pages, representative pages 1/2/3 visually inspected.

## Classification

Parity check: **PASS within the stated MVP scope and analytical-code limitation**.
