# Ray to Outram Park — TRISO Source-Term Research Deliverables

This folder is a clean human-readable deliverable view of Ray's canonical TRISO research on branch `main` of `weewzr/Derivation-of-TRISO-Source-Term-Modelling`.

It does **not** replace the canonical evidence architecture in `docs/`, `verification/`, and `reviews/`.

## Files

- `TRISO_Source_Term_Derivation.md` — canonical human-readable consolidated derivation for discussion/transfer.
- `TRISO_Source_Term_Derivation.tex` — synchronized standalone LaTeX manuscript.
- `references.bib` — bibliography entries whose provenance is already supported.
- `TRACEABILITY.md` — map from manuscript claims to canonical derivations, workflow runs, evidence, and independent reviews.

The historical `notes/raw/TRISO Fuel Derivation Ray V1.tex` remains untouched.

## Current Method-2 status

The continuous foundation, deterministic FV reference, controlled production-WOS verification, exact first-passage kernels, controlled accelerated renewal, controlled two-layer matrix and genuine three-layer multistate matrix have passed their applicable verification/review gates.

The frozen hard five-layer 8x8 transform is extremely recurrent and ill-conditioned in ordinary f64. Run `36869649217` completed the predeclared 50/80/120-decimal-digit cross-check and satisfies the original normalization/residual gates under reliable arithmetic. That result is **PENDING INDEPENDENT REVIEW**.

No inverse-Laplace recovery has been verified. No final five-layer release CDF exists.

- R2-WOS-02: **OPEN**
- R2-B01: **OPEN**

## Supervisor repository boundary

`theodoreOnzGit/outram-park-backend` was **not modified**. It remains read-only unless Ray explicitly authorizes a future transfer.

Any eventual scientifically justified Outram Park integration should first be documented and maintained in this research repository with exact supervisor-base provenance and verification evidence. This folder currently contains research deliverables only, not an automatically applicable supervisor patch.
