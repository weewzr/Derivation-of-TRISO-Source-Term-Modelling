# Independent Discrete-Mathematics Audit — Resolution

Immutable audit: `reviews/independent_discrete_mathematics_audit.md`

Reviewed commit: `34d450649db4fe70e9c8ade8d4222d63f3d3fa1f`

Audit persistence commit: `4db0132dbff897b6d5f47bdbfbab6409478b3bc8`

Gate decision: **PASS FOR ACCURACY / CONVERGENCE STUDY**.

This resolution reconciles the persisted audit against the current `ray/triso-foundation` canonical mathematics. It does not alter the independent review.

## R2-D01 — MAJOR — VALID — OPEN

Finding: the retained FTCS Robin ghost boundary was incorrectly described as having complete local spatial consistency `O(Delta r^2)`.

Independent reconciliation: VALID. The centred Robin first derivative is second-order, but the ghost-value error is `O(Delta r^3)` and enters the second derivative divided by `Delta r^2`, producing a generic `O(Delta r)` surface-operator error.

Remediation performed: Section 16.9 of `docs/triso/00_consolidated_mathematical_foundation.md` now derives the complete ghost/surface truncation error in `TRISO-DIS-452`–`TRISO-DIS-470` and states generic boundary consistency `O(Delta t)+O(Delta r)`. The previous second-order surface claim is removed.

Canonical remediation commit: `c3d88a9cabf72fafb98d55ee4e48822cdcfb9b8f`.

Register reconciliation commit: `627b22cdf73bec7347e9986d998b85a55fb54b21`.

Status: OPEN because the independent reviewer explicitly requires an actual grid-refinement study in addition to the analytical Taylor expansion for closure. That study is outside this reconciliation-only pass.

Downstream consequence: the FTCS benchmark must not be used as a formally second-order boundary/grid-convergence reference until R2-D01 is independently closed. The review explicitly states that this does not block the canonical five-layer FV accuracy/convergence route.

## R2-D02 — MODERATE — VALID — CLOSED

Finding: the FV derivation defined `C_P` as an exact cell average but did not freeze the representative coordinate used by two-point face reconstructions.

Independent reconciliation: VALID.

Remediation performed: Section 18.1.1 now freezes the canonical representation:

- `C_P` is the exact spherical volume average;
- `r_P` is the spherical volume-centroid coordinate;
- physical faces are `r_{P+1/2}`;
- centre-to-centre and centre-to-face distances are defined from those coordinates;
- material interfaces are face-aligned;
- `delta r_R = R-r_{M-1}` at the outer boundary.

The volume centroid is explicitly derived in `TRISO-FV-109A`–`TRISO-FV-109E`. The document also states that treating cell averages as reconstructed values at the centroids is an approximation whose order remains to be established in the accuracy study.

Closure evidence: canonical remediation commit `c3d88a9cabf72fafb98d55ee4e48822cdcfb9b8f`; register commit `627b22cdf73bec7347e9986d998b85a55fb54b21`.

Downstream consequence: formal FV consistency/order analysis can now begin from an unambiguous mesh/unknown definition.

## R2-D03 — NOTE — VALID — OPEN INFORMATIONAL

Finding: the harmonic/resistance interface flux is mathematically sound and conservative, but it is a two-point discrete approximation rather than the exact finite-radius spherical shell resistance; its spatial order remains unproved.

Independent reconciliation: VALID. No defect in the coefficient equations was found and no redesign is required.

Remediation: none required in this pass. The canonical derivation already labels the global spatial order across discontinuous `D` as UNVERIFIED.

Downstream consequence: interface consistency/order belongs to the next accuracy/convergence study.

## R2-D04 — MINOR — VALID — CLOSED

Finding: stable FV equation IDs overlapped.

Independent reconciliation: VALID. The canonical derivation contained duplicate numeric tags `TRISO-FV-180` through `TRISO-FV-185` for two different equation families.

Remediation performed: the ordinary-cell positivity equations were moved to unique IDs `TRISO-FV-600`–`TRISO-FV-605`; the outer Robin IDs retain `TRISO-FV-180`–`TRISO-FV-185`. The canonical register was updated accordingly.

Closure evidence: remediation commit `c3d88a9cabf72fafb98d55ee4e48822cdcfb9b8f`; register commit `627b22cdf73bec7347e9986d998b85a55fb54b21`. A current canonical scan finds no duplicate numeric `TRISO-FV-###` tags.

Downstream consequence: stable-ID traceability is restored for the reviewed FV equations.

## R2-D05 — MINOR — VALID — CLOSED

Finding: stale final-status wording still said Review 1 was not yet the next step.

Independent reconciliation: VALID.

Remediation performed: the obsolete statement was replaced with the actual milestone chronology: continuous audit passed for discretisation; discrete audit passed for accuracy/convergence; R2-D01 remains open for the retained FTCS benchmark.

Closure evidence: canonical remediation commit `c3d88a9cabf72fafb98d55ee4e48822cdcfb9b8f`.

Downstream consequence: none scientific; repository status now matches the actual review history.

## Gate after reconciliation

The independent gate remains **PASS FOR ACCURACY / CONVERGENCE STUDY**.

R2-D01 remains an OPEN MAJOR finding, but according to the independent audit it is isolated to the retained FTCS Robin benchmark and does not block the canonical conservative FV accuracy/convergence study.

R2-D02 is CLOSED, so the mesh/unknown representation prerequisite for formal FV accuracy analysis is now satisfied.

R2-B01 remains a separate WOS implementation-verification finding and is not changed by this resolution.
