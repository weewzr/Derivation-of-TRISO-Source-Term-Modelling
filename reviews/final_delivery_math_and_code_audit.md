# Final Delivery Math and Code Audit

**Date:** 22 October 2026
**Scope:** `deliverables/`, `verification/code_checks/`, literature template, CI workflow for the final delivery package.
**Method:** every equation re-derived by hand, every claim re-executed, FV results compared with an independent exact solution written in two languages (Rust and Python).
**Preceding audit:** the earlier final-delivery audit in `reviews/` (not modified).

## Result

The finite-volume scheme is correct: against the exact five-layer solution it converges at observed order 2.000 in space and 0.9 to 1.0 in time. The package as delivered had four substantive defects (two in the mathematics and verification, one in the numerics, one in the documents) and several smaller ones. All are fixed on this branch.

## Findings

| ID | Severity | Finding | Fix |
|---|---|---|---|
| FDA-01 | MAJOR | `analytical_sturm_liouville.rs` (then `analytical.rs`) documented `(u, D u')` as the state to carry through a five-layer transfer matrix. That state is not continuous across an interface with unequal diffusivities. The matching quantity is `D(u' - u/r)` (TRISO-ML-446), giving a jump `(D_next - D_prev) u / R_i` in `D u'`. Composing the documented building block gives wrong eigenvalues and a release fraction of about 0.98 instead of 0.43 on the synthetic case. | Interface rule added and documented. The module now contains the full exact five-layer release series. |
| FDA-02 | MAJOR | No FV result was compared with an exact multilayer answer. The "controlled two-layer reference" `0.99998700` is an earlier refined FV value, checked only at $t=32$ s where $F\approx1$ and almost any scheme agrees. Assertion tolerances were $10^{-2}$. | `verify` compares with the exact series at mid-transient times, measures observed orders against the exact answer and asserts them. |
| FDA-03 | MODERATE | Textbook Thomas elimination subtracts nearly equal numbers at large Fourier number. Relative balance residual was $2.6\times10^{-11}$ to $2.5\times10^{-9}$ and grew with refinement. A past workflow failure at $1.87\times10^{-9}$ was resolved by changing the simulation horizon. | Subtraction-free elimination (all terms non-negative). Residual is now at most $3\times10^{-13}$ and falls as the Fourier number rises. Test added at Fourier number $2.5\times10^{5}$. |
| FDA-04 | MODERATE | Derivation Markdown, cell balance: `A_{j-1/2} D dc/dr - A_{j+1/2} J` has the wrong sign on the first term (it must be `A_{j-1/2} J_{j-1/2} - A_{j+1/2} J_{j+1/2}`). | Corrected, with the integration step shown. |
| FDA-05 | MODERATE | The compiled PDF and `.tex` jumped from the PDE to the stencil in one sentence, omitted the interface rule and the exact solution, and described convergence evidence that predates the package. Markdown and LaTeX were not equivalent. | PDF and Markdown rewritten as one derivation with the same equation IDs. |
| FDA-06 | MODERATE | `TRISO_Finite_Volume_Pseudocode.md` equations were corrupted (`\frac` replaced by a form feed and `rac`, same for `\left`, `\le`, `\times`). This is RFD-M01 of the preceding audit, still open. The derivation Markdown used `\[ \]` and `\tag`, which the repository's own GitHub math standard rules out. | Both rewritten with `math` fences. Lint script confirms no control characters and balanced delimiters. |
| FDA-07 | MINOR | The headline release fraction $0.43243$ carries a first-order Backward-Euler error of $1.3\times10^{-4}$. The exact value is $0.43256$. | Both numbers are reported, with the time and space errors separated. |
| FDA-08 | MINOR | Unfilled template failed with a TOML type error. `species = "INSERT_VALUE_HERE"` passed validation. NaN and infinity were not rejected. | Readable list of every unfilled value, stricter validation. |
| FDA-09 | MINOR | CI ran `cargo fmt` (auto-fix) before `cargo fmt --check`, so the check could not fail. It wrote evidence into `deliverables/`, undoing the cleanup to five submission files, and pushed to `main` from any ref. | Check-only formatting, evidence in `verification/delivery_checks/`, push to the triggering branch. |
| FDA-10 | NOTE | the code README showed literal backslash-backtick sequences. | Fixed. |

## Residual limitations (stated in the derivation and handoff)

- The exact series loses accuracy to cancellation when adjacent diffusivities differ by more than about $10^{6}$. The program reports an accuracy floor and warns when results fall below it. In the stress tests (SiC from $10^{-15}$ to $10^{-20}\ \mathrm{m^2\,s^{-1}}$ next to IPyC at $10^{-13}$) the FV solver stayed positive with a relative balance residual below $3\times10^{-12}$ at every setting.
- Completeness of the eigenfunction set for this singular piecewise problem is verified numerically (Sturm count equals root count, weights sum to 1, agreement with FV to $10^{-7}$), not proved.
- Backward Euler is first order. The user must choose $\Delta t$ from the refinement study the program prints.
- Synthetic inputs are not physical validation. Nothing in the package contains literature values.

## Reproduce

```bash
cd deliverables
cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test
cargo run --release -- synthetic_five_layer.toml
cd ../verification/code_checks
cargo test
cargo run --release --example verify
```
