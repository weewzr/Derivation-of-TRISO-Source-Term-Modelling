# TRISO Fuel Particle Mathematical Derivation & Numerical Implementation

This repository develops the TRISO fuel-particle source-term model from first principles and connects the continuous mathematics to the supervisor's numerical implementation.

## Current project state

Active development branch: `ray/triso-foundation`

Default branch: `main`

### Continuous-mathematics track

Review 1: PASS WITH NON-BLOCKING FINDINGS; original continuous-foundation findings reconciled.

Independent continuous-mathematics milestone audit: **PASS FOR DISCRETISATION**. The audit verified the continuous chain through the five-layer eigenproblem, global orthogonality, and conditional modal projection. Its report is `reviews/independent_continuous_mathematics_audit.md` and its current finding status is in `reviews/independent_continuous_mathematics_audit_resolution.md`.

Continuous mathematics is cleared to proceed to **detailed discrete mathematical derivation**. The remaining completeness theorem/provenance item is explicitly non-blocking and the modal series remains labelled formal/conditional.

### Deterministic discrete-mathematics track

Independent deterministic discrete-mathematics audit: **PASS FOR ACCURACY / CONVERGENCE STUDY**. The immutable report is `reviews/independent_discrete_mathematics_audit.md`; reconciliation is `reviews/independent_discrete_mathematics_audit_resolution.md`.

Current finding status:

- `R2-D01` MAJOR: **OPEN** — the FTCS Robin benchmark's complete boundary truncation is generically first-order in space; analytical overclaim corrected, grid-refinement closure evidence still pending.
- `R2-D02` MODERATE: **CLOSED** — FV unknown is frozen as exact spherical cell average with spherical volume-centroid representative coordinate.
- `R2-D04` MINOR: **CLOSED** — overlapping FV stable IDs repaired.
- `R2-D05` MINOR: **CLOSED** — stale review-status wording corrected.
- `R2-D03` NOTE: interface-flux accuracy remains intentionally unproved and belongs to the accuracy/convergence study.

The canonical conservative five-layer FV route is cleared to proceed to formal accuracy/convergence analysis. The open FTCS boundary finding does not block that FV study, but it prevents treating the retained FTCS Robin benchmark as a proven second-order boundary reference.

### Deterministic accuracy/convergence reconciliation

Independent accuracy/convergence audit: **PASS FOR DETERMINISTIC REFERENCE / WOS COMPARISON**. Immutable audit: `reviews/independent_accuracy_convergence_audit.md`; reconciliation: `reviews/independent_accuracy_convergence_audit_resolution.md`.

Current reconciliation:
- R3-m01: **VALID — CLOSED**. Post-remediation run `36661249962` verified actual executed timestep ratios and corrected temporal orders approximately equal to one.
- R3-m02: **VALID — CLOSED**. Post-remediation run `36661249962` explicitly selected stable Rust and recorded `rustc 1.98.1 (48a229cea 2026-09-01)` in the raw execution evidence.
- R3-m03: **VALID — OPEN / NON-BLOCKING**. Transient release-rate convergence remains deferred; no release-rate temporal convergence claim is made.
- R2-D01: **OPEN** and remains restricted to the retained FTCS Robin benchmark; FV convergence does not close it.
- R2-B01: **OPEN / SEPARATE** and remains a distinct production-WOS implementation-verification finding.

The evidence layers remain distinct: continuous analytical reference → deterministic FV reference → production WOS verification. The deterministic FV model is cleared to serve as the numerical reference for the subsequent deterministic/analytical ↔ WOS verification stage after this reconciliation is independently checked. This FV evidence does not validate WOS.

### Numerical / WOS implementation-verification track

Review 2: **FAIL — remediation required**. R2-B01 concerns executed evidence for the concrete five-layer release-time estimator. This finding is separate from the continuous-mathematics gate and is **not marked closed** here.

The independent continuous-mathematics audit occurred while Review 2 remained open. It is therefore preserved as a named milestone audit rather than being renumbered as Review 3.

Do not infer Review-2 closure from the continuous-mathematics PASS.

## ACTIVE / CANONICAL

- Original Ray derivation (raw evidence): `notes/raw/TRISO Fuel Derivation Ray V1.tex`
- Verified first-principles derivation: `docs/triso/00_consolidated_mathematical_foundation.md`
- Canonical equation register: `docs/triso/13_canonical_equation_register.md`
- Frozen continuous benchmark: `docs/triso/16_frozen_continuous_benchmark.md`
- Independent continuous-mathematics audit: `reviews/independent_continuous_mathematics_audit.md`
- Continuous-audit resolution: `reviews/independent_continuous_mathematics_audit_resolution.md`
- Independent discrete-mathematics audit: `reviews/independent_discrete_mathematics_audit.md`
- Discrete-audit resolution: `reviews/independent_discrete_mathematics_audit_resolution.md`
- Frozen production WOS contract: `docs/triso/21_frozen_production_wos_contract.md`
- Production WOS numerical formulation and equation-to-code mapping: `docs/triso/22_review2_numerical_formulation.md`
- Active R2-B01 verification test: `verification/r2_b01_supervisor_integration.rs`
- Active R2-B01 execution workflow: `.github/workflows/r2-b01-supervisor-integration.yml`
- Current status: this README

## FROZEN / HISTORICAL

Historical derivations, audits, intermediate benchmarks, handoffs, and completed remediation notes are preserved under `docs/triso/archive/`.

Review reports are preserved as immutable historical evidence under `reviews/`, including Review 1, Review 2, and the independent continuous-mathematics milestone audit.

Supporting Review-2 planning/specification notes are under `reviews/archive/`.

Other non-production numerical or diagnostic artifacts are preserved under `verification/archive/`.

Obsolete workflows are preserved under `.github/workflows/archive/` and are not active.

## Research navigation

Original derivation → verified continuous derivation → independent continuous audit → canonical deterministic discretisation → independent discrete audit → accuracy/convergence study → later implementation mapping / verification.

## Method distinction

The original notebook's FTCS scheme is retained as a transparent deterministic benchmark. The supervisor production method is first-passage Walk-on-Spheres. The two are not treated as interchangeable.

## Repository safety

Do not modify or merge into `main` from this research branch. Supervisor-repository source is authoritative for the production WOS implementation; Ray changes remain isolated until the user explicitly publishes them.
