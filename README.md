# TRISO Fuel Particle Mathematical Derivation & Numerical Implementation

This repository develops the TRISO fuel-particle source-term model from first principles and connects the continuous mathematics to the supervisor's numerical implementation.

## Current project state

Active development branch: `ray/triso-foundation`

Default branch: `main`

Review 1: PASS WITH NON-BLOCKING FINDINGS; continuous-foundation findings reconciled.

Review 2: FAIL — remediation in progress. The current blocker is R2-B01, requiring executed evidence for the concrete five-layer release-time estimator. The focused test is present; closure requires a passing execution against the reviewed supervisor implementation.

Do not start Review 3 until R2-B01 is independently closed.

## ACTIVE / CANONICAL

- Original Ray derivation (raw evidence): `notes/raw/TRISO Fuel Derivation Ray V1.tex`
- Verified first-principles derivation: `docs/triso/00_consolidated_mathematical_foundation.md`
- Canonical equation register: `docs/triso/13_canonical_equation_register.md`
- Frozen continuous benchmark: `docs/triso/16_frozen_continuous_benchmark.md`
- Frozen production WOS contract: `docs/triso/21_frozen_production_wos_contract.md`
- Production WOS numerical formulation and equation-to-code mapping: `docs/triso/22_review2_numerical_formulation.md`
- Active R2-B01 verification test: `verification/r2_b01_supervisor_integration.rs`
- Active R2-B01 execution workflow: `.github/workflows/r2-b01-supervisor-integration.yml`
- Current status: this README

## FROZEN / HISTORICAL

Historical derivations, audits, intermediate benchmarks, handoffs, and completed remediation notes are preserved under `docs/triso/archive/`.

Review reports are preserved as immutable historical evidence under `reviews/`:

- `reviews/review_01_continuous_mathematical_foundation.md`
- `reviews/review_02_numerical_formulation.md`

Supporting Review-2 planning/specification notes are under `reviews/archive/`.

Other non-production numerical or diagnostic artifacts are preserved under `verification/archive/`.

Obsolete workflows are preserved under `.github/workflows/archive/` and are not active.

## Research navigation

Original derivation → verified derivation → canonical equation register → production WOS contract → numerical formulation / code mapping → verification

## Method distinction

The original notebook's FTCS scheme is retained as a transparent deterministic benchmark. The supervisor production method is first-passage Walk-on-Spheres. The two are not treated as interchangeable.

## Repository safety

Do not modify or merge into `main` from this research branch. Supervisor-repository source is authoritative for the production WOS implementation; Ray changes remain isolated until the user explicitly publishes them.