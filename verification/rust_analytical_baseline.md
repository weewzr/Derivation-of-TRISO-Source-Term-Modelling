# Analytical Rust Baseline — Separate Reference Track

## Purpose
Preserve the user's supplied Rust structure as the independent analytical reference, **not** as a finite-volume time-stepping solver.

## Source structure from uploaded example
- Part I: homogeneous sphere, constant source and Robin boundary; eigenvalue roots and modal coefficients.
- Part II: five-material-layer transfer-matrix eigenproblem, modal coefficients, shutdown response and reconstruction.
- Uses `uom` typed SI quantities, `csv` outputs, `plotters` figures.
- Existing example uses illustrative radii, diffusivities, Robin mass-transfer coefficient and time settings, not verified literature TRISO inputs.

## Suggested files
```text
src/analytical/homogeneous.rs
src/analytical/five_layer.rs
src/analytical/mod.rs
```

## Preservation and scientific cautions
Keep analytical calculations distinct from the production FV solver. Validate eigenvalue residuals, interface flux/concentration conditions, orthogonality and homogeneous reduction before treating this analytical baseline as verified.

The supplied reference's Robin, continuously generated and shutdown scenario is different from the primary FV initial-inventory, absorbing-boundary release scenario. Only compare them when their boundary, source and initial conditions match.

Do not silently substitute physical values. Use explicitly marked literature-value placeholders and a separate synthetic verification configuration.

The original uploaded Rust text remains the authoritative reference for its detailed function bodies; this file is a structural handoff, not a claim that the entire uploaded source has been committed.
