# Final Markdown / LaTeX / Rust Parity Check

Executed after the audit corrections, delivery workflow 39729930098 on `main`.

## Markdown vs LaTeX

`TRISO_Finite_Volume_Derivation.md` and `.tex` carry the same 22 equation IDs, the same section structure (the PDF leaves the symbol table unnumbered, so section numbers agree) and the same equations. Checked by script: no ID appears in only one file. The chain is: control-volume conservation, local conservation law, Fick's law, spherical multilayer PDE, centre/interface/outer conditions, exact eigenfunction solution (with the interface rule `D(u' - u/r)` continuous), exact spherical FV geometry, exact cell balance, series-resistance interface conductance, centre and absorbing outer treatment, Backward-Euler tridiagonal equation with subtraction-free elimination, Forward-Euler verification-only bound, inventory and release conservation.

The analytical Robin/source benchmark (Case B) remains distinct from the initial-inventory absorbing release case (Case A). Only Case A is compared between FV and the exact series.

## Derivation vs Rust

- `deliverables/finite_volume_code/src/discrete_finite_volume_tridiagonal.rs`: five regions, per-region cell counts, interface-aligned faces, exact shell volumes and volume centroids, series-resistance conductances (TRISO-FV-140), zero-area centre, absorbing outer conductance (TRISO-DEL-030), Backward Euler solved by subtraction-free elimination (TRISO-DEL-040), boundary-flux release accumulation, explicit bound (TRISO-FV-300).
- `deliverables/finite_volume_code/src/analytical_sturm_liouville.rs`: transfer step (TRISO-DEL-010), interface jump (TRISO-DEL-011), characteristic function and Sturm-count root isolation (TRISO-DEL-012), modal weights and release series (TRISO-DEL-020, TRISO-DEL-021), homogeneous Robin roots.
- `deliverables/finite_volume_code/src/compare.rs`: FV against exact series and refinement studies against the exact answer.

## Executed verification (workflow 39729930098, Linux, rustc 1.99.1)

Values are in `TRISO_FV_Numerical_Verification.txt` and `TRISO_Five_Layer_Run.txt`.

- rustfmt check, clippy with `-D warnings`, `cargo test`, release build, verification suite, five-layer run, PDF compile: all passed.
- Exact five-layer release fraction at $t=2\times10^5$ s: 0.4325556390928 (Rust). A separate Python implementation gives 0.4325556390930.
- FV spatial convergence against the exact solution: observed order 2.000 on every mesh pair, 100 to 1600 cells.
- FV temporal convergence against the exact solution: observed order 0.98, 0.97, 0.94, 0.89.
- Conservation residual at most $3\times10^{-13}$ relative, minimum concentration positive on every mesh.
- PDF: 7 A4 pages, every page rendered and inspected (`pdf_render_check/`).

## Classification

Parity check: **PASS**, within the limitations listed in `deliverables/HANDOFF.md`.
