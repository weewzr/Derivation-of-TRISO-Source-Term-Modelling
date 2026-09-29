# Canonical Equation Register — Single Source of Truth

Source: `notes/raw/TRISO Fuel Derivation Ray V1.tex` (881 lines, 33 labelled substantive equations).

Rule: this table is the only authoritative original-equation → stable-equation mapping. Other documents must link to or cite these IDs rather than invent alternate mappings.

| # | Original LaTeX label | Canonical stable ID | Status | Note / correction | Downstream dependency |
|---:|---|---|---|---|---|
| 1 | `eq:b:ic` | `TRISO-BC-001` | CONDITIONALLY VERIFIED | Zero initial concentration is a benchmark assumption. | Transient benchmark. |
| 2 | `eq:b:bc0` | `TRISO-BC-002` | VERIFIED | Regular spherical centre condition. | Centre treatment. |
| 3 | `eq:b:bcR` | `TRISO-BC-003` | CONDITIONALLY VERIFIED | General form uses D5 and c∞; notebook is c∞=0 homogeneous benchmark. | Outer boundary. |
| 4 | `eq:b:biot` | `TRISO-VER-001` | CONDITIONALLY VERIFIED | Part-I definition Bi=hR/D. | Part-I eigenproblem. |
| 5 | `eq:fick` | `TRISO-GOV-001` | CONDITIONALLY VERIFIED | Valid for constant D; general model is div(D grad c). | Five-layer PDE. |
| 6 | `eq:fick-spherical` | `TRISO-SPH-001` | CONDITIONALLY VERIFIED | Constant-D spherical-coordinate form. | Spherical reduction. |
| 7 | `eq:radial-conservative` | `TRISO-SPH-002` | CONDITIONALLY VERIFIED | Constant-D notebook equation; generalized form stored separately as TRISO-SPH-004. | Variable-D model. |
| 8 | `eq:product-rule` | `TRISO-SPH-003` | VERIFIED | Correct within constant-D region. | Part-I benchmark. |
| 9 | `eq:simplified-triso` | `TRISO-GOV-002` | CONDITIONALLY VERIFIED | Constant-D radial equation only. | Part-I benchmark. |
| 10 | `eq:kernel-source` | `TRISO-GOV-003` | CONDITIONALLY VERIFIED | Kernel-only generation assumption; S0 provenance still needed. | Numerical source model. |
| 11 | `eq:source-decomposition` | `TRISO-GOV-004` | POSSIBLE ERROR | Signs/forms of decay and trapping are not defined. | Reaction/source closure. |
| 12 | `eq:b:pde` | `TRISO-GOV-005` | VERIFIED | Homogeneous Part-I benchmark PDE. | Steady/transient/FTCS. |
| 13 | `eq:b:pde-divergence` | `TRISO-GOV-006` | VERIFIED | Equivalent divergence form for constant D. | Sturm-Liouville. |
| 14 | `eq:b:pqsigma` | `TRISO-VER-002` | CONDITIONALLY VERIFIED | Operator/weight structure correct; eigenvalue notation repaired. | Modal analysis. |
| 15 | `eq:b:w-firstint` | `TRISO-GOV-007` | VERIFIED | Steady first integration. | Steady profile. |
| 16 | `eq:b:w-secondint` | `TRISO-GOV-008` | CONDITIONALLY VERIFIED | Correct under c∞=0. | Steady profile. |
| 17 | `eq:b:w` | `TRISO-GOV-009` | VERIFIED | Part-I Robin steady profile. | Transient decomposition. |
| 18 | `eq:b:massbalance` | `TRISO-VER-003` | VERIFIED | Independent steady generation/release check. | Verification. |
| 19 | `eq:b:vproblem` | `TRISO-GOV-010` | VERIFIED | Homogeneous transient deviation problem. | Separation of variables. |
| 20 | `eq:b:sl-ode` | `TRISO-VER-004` | POSSIBLE ERROR → REPAIRED | Spatial k and temporal Λ must be distinct. | Eigenproblem. |
| 21 | `eq:b:harmonic` | `TRISO-VER-005` | POSSIBLE ERROR → REPAIRED | Corrected to u''+k²u=0. | Eigenfunction. |
| 22 | `eq:b:eigfun` | `TRISO-VER-006` | POSSIBLE ERROR → REPAIRED | Corrected to sin(kr)/r. | Robin eigencondition. |
| 23 | `eq:b:eigcond-raw` | `TRISO-VER-007` | VERIFIED AFTER REPAIR | Raw condition valid with μ=kR. | Modal roots. |
| 24 | `eq:central-2nd` | `TRISO-DIS-001` | CONDITIONALLY VERIFIED | Smooth interior, constant D only. | FTCS interior. |
| 25 | `eq:explicit-coeffs` | `TRISO-DIS-002` | CONDITIONALLY VERIFIED | Correct for uniform mesh and i≥1. | FTCS benchmark. |
| 26 | `eq:centre-ghost` | `TRISO-BC-004` | VERIFIED | Even-extension symmetry construction. | Centre stencil. |
| 27 | `eq:centre-stencil` | `TRISO-DIS-003` | CONDITIONALLY VERIFIED | Correct for this ghost-node FTCS construction. | Centre update. |
| 28 | `eq:lhopital` | `TRISO-BC-005` | VERIFIED | Correct regular-solution limit. | Centre operator. |
| 29 | `eq:centre-curvature-limit` | `TRISO-DIS-004` | CONDITIONALLY VERIFIED | Factor-of-six centre operator. | Centre update/stability. |
| 30 | `eq:centre-update` | `TRISO-DIS-005` | CONDITIONALLY VERIFIED | Centre coefficient positivity gives Fo≤1/6. | FTCS timestep. |
| 31 | `eq:b:ftcs-ghost` | `TRISO-BC-006` | CONDITIONALLY VERIFIED | Constant-D, zero-coolant Robin ghost relation. | Surface stencil. |
| 32 | `eq:b:ftcs-surface` | `TRISO-DIS-006` | CONDITIONALLY VERIFIED | Correct for the stated ghost construction. | Surface stability. |
| 33 | `eq:b:ftcs-stability` | `TRISO-VER-008` | POSSIBLE ERROR → REPAIRED | Sufficient coefficient-positivity condition only; centre restriction must also be included. | FTCS stability. |

## Stable replacement equations introduced after the notebook

| Stable ID | Purpose | Status |
|---|---|---|
| `TRISO-GOV-008` | General conservative PDE ∂c/∂t = ∇·(D∇c)+S | VERIFIED |
| `TRISO-SPH-004` | General heterogeneous spherical PDE | VERIFIED |
| `TRISO-BC-005` | General outer Robin condition with c∞ | VERIFIED |
| `TRISO-INT-001` | Ideal concentration continuity | CONDITIONALLY VERIFIED |
| `TRISO-INT-002` | Ideal flux continuity | VERIFIED |
| `TRISO-GOV-015` | Signed decay/source convention | CONDITIONALLY VERIFIED |
| `TRISO-GOV-050` | Five-layer transient eigen-equation | CONDITIONALLY VERIFIED |
| `TRISO-SYS-001` | Five-layer eigen-coefficient system / determinant framework | UNVERIFIED |
| `TRISO-VER-060` | Corrected sufficient FTCS coefficient-positivity bound | VERIFIED AS SUFFICIENT CONDITION |

## Provenance

The raw notebook was preserved from the supplied project file without mathematical edits. SHA-256: `5ce5f77e5148a234a7e7339a485923df982b54edfc79ff96796d26e0673158ce`.
## Canonical stable equations added during mathematical expansion

| Stable ID | Mathematical role | Status |
|---|---|---|
| `TRISO-GOV-020` | Control-volume conserved inventory definition | VERIFIED |
| `TRISO-GOV-027` | Control-volume accumulation = generation - outward flux | VERIFIED |
| `TRISO-GOV-029` | Integral conservation statement with flux and source | VERIFIED |
| `TRISO-GOV-034` | Local differential conservation law | VERIFIED |
| `TRISO-GOV-035` | Fickian constitutive relation | CONSTITUTIVE |
| `TRISO-GOV-038` | Fick substitution into conservation | VERIFIED |
| `TRISO-GOV-040` | General conservative diffusion equation | VERIFIED |
| `TRISO-SPH-020` | Spherical-coordinate gradient | VERIFIED |
| `TRISO-SPH-021` | Spherical-coordinate divergence | VERIFIED |
| `TRISO-SPH-022` | Spherical symmetry specialization c=c(r,t) | ASSUMPTION |
| `TRISO-SPH-025` | Radial gradient after angular elimination | VERIFIED |
| `TRISO-SPH-026` | Radial Fickian flux | CONSTITUTIVE |
| `TRISO-SPH-029` | Radial spherical divergence | VERIFIED |
| `TRISO-SPH-033` | General heterogeneous spherical conservative PDE | VERIFIED |
| `TRISO-SPH-034` | Layer-specific constant-D assumption | ASSUMPTION |
| `TRISO-SPH-036` | D_i moved outside radial derivative | CONDITIONALLY VERIFIED |
| `TRISO-SPH-037` | Explicit product-rule expansion | VERIFIED |
| `TRISO-SPH-039` | Product-rule result | VERIFIED |
| `TRISO-SPH-040` | Divide by r^2 and collect terms | VERIFIED |
| `TRISO-SPH-041` | Familiar within-layer radial diffusion equation | CONDITIONALLY VERIFIED |

These identifiers are now part of the active canonical derivation. They supplement the one-to-one mapping of the original 33 notebook equations; they do not replace any original mapping.