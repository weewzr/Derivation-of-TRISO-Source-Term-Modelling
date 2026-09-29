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

The original 33 notebook mappings above are preserved. The equations introduced by later first-principles expansion use a separate 100-series namespace so that stable identifiers never collide.

| Stable ID | Mathematical role | Status |
|---|---|---|
| `TRISO-GOV-100` | Net source/reaction bookkeeping definition | VERIFIED AS DEFINITION |
| `TRISO-GOV-102` | Kernel-confined generation benchmark | ASSUMPTION |
| `TRISO-GOV-104` | Decay constant definition | VERIFIED AS DEFINITION |
| `TRISO-GOV-111` | First-order radioactive-decay sink | CONDITIONALLY VERIFIED / CONSTITUTIVE |
| `TRISO-GOV-118` | General layer-wise reaction-inclusive equation | CONDITIONALLY VERIFIED |
| `TRISO-IC-100` | Initially empty source-driven problem | ASSUMPTION |
| `TRISO-IC-102` | Initial kernel inventory condition | ASSUMPTION |
| `TRISO-IC-104` | No post-initial generation for release benchmark | ASSUMPTION |
| `TRISO-BC-100` | Even radial extension about centre | ASSUMPTION |
| `TRISO-BC-103` | Centre derivative condition | VERIFIED |
| `TRISO-BC-105` | L'Hopital limit of c_r/r | VERIFIED |
| `TRISO-BC-109` | Centre spherical-operator limit | VERIFIED |
| `TRISO-INT-100` | Infinitesimal interface-control-volume geometry | VERIFIED |
| `TRISO-INT-107` | General interface storage/source balance | VERIFIED |
| `TRISO-INT-111` | Flux continuity for ideal interface | VERIFIED |
| `TRISO-INT-114` | Fickian flux continuity | VERIFIED |
| `TRISO-INT-115` | Ideal concentration continuity | ASSUMPTION / CONSTITUTIVE |
| `TRISO-INT-116` | Partition coefficient condition | CONDITIONALLY VERIFIED / CONSTITUTIVE |
| `TRISO-INT-117` | Example finite interfacial resistance law | SOURCE NEEDED |
| `TRISO-BC-112` | Absorbing Dirichlet boundary | ASSUMPTION |
| `TRISO-BC-114` | Prescribed-flux Neumann boundary | VERIFIED AS DEFINITION |
| `TRISO-BC-117` | External mass-transfer law | CONSTITUTIVE |
| `TRISO-BC-119` | Robin boundary after flux matching | VERIFIED |
| `TRISO-BC-123` | Large-h limit to Dirichlet behaviour | CONDITIONALLY VERIFIED |
| `TRISO-BC-125` | Small-h limit to zero-flux behaviour | CONDITIONALLY VERIFIED |
| `TRISO-IC-106` | Initial-kernel release benchmark | ASSUMPTION |
| `TRISO-ANA-100` | Part-I homogeneous steady benchmark PDE | CONDITIONALLY VERIFIED |
| `TRISO-ANA-109` | Conservative steady equation | VERIFIED |
| `TRISO-ANA-113` | First integrated steady profile | VERIFIED |
| `TRISO-ANA-114` | First integration constant fixed by centre regularity | VERIFIED |
| `TRISO-ANA-120` | Second integrated steady profile | VERIFIED |
| `TRISO-ANA-121` | Robin condition applied at R | VERIFIED |
| `TRISO-ANA-127` | Steady integration constant B | VERIFIED |
| `TRISO-ANA-129` | Final homogeneous steady profile | VERIFIED |
| `TRISO-ANA-130` | First-term dimensional check | VERIFIED |
| `TRISO-ANA-131` | Second-term dimensional check | VERIFIED |
| `TRISO-ANA-135` | Total steady generation rate | VERIFIED |
| `TRISO-ANA-137` | Global steady generation/release equality | VERIFIED |
| `TRISO-ANA-139` | Surface concentration from global balance | VERIFIED |
| `TRISO-ANA-141` | Analytical surface concentration check | VERIFIED |

These IDs belong to the current canonical derivation and supplement, rather than replace, the one-to-one mapping of the original 33 notebook equations.


## Canonical stable equations: transient eigenproblem expansion

| Stable ID | Mathematical role | Status |
|---|---|---|
| `TRISO-ANA-200` | Definition of transient deviation v=c-w | VERIFIED |
| `TRISO-ANA-204` | Time derivative relation c_t=v_t | VERIFIED |
| `TRISO-ANA-210` | Full source-PDE substitution of c=v+w | VERIFIED |
| `TRISO-ANA-214` | Homogeneous transient PDE for v | VERIFIED |
| `TRISO-ANA-225` | Homogeneous transient Robin condition | VERIFIED |
| `TRISO-ANA-228` | Transformed initial condition v(r,0)=-w(r) | VERIFIED |
| `TRISO-ANA-229` | Separated form v=phi T | ASSUMPTION / METHOD |
| `TRISO-ANA-238` | Radial eigenproblem | VERIFIED |
| `TRISO-ANA-242` | Temporal decay rate Lambda=Dk^2 | VERIFIED |
| `TRISO-ANA-251` | Temporal mode exp(-Lambda t) | VERIFIED |
| `TRISO-ANA-266` | u=r phi transformed radial ODE | VERIFIED |
| `TRISO-ANA-269` | General sine/cosine solution for u | VERIFIED |
| `TRISO-ANA-273` | Centre regularity forces B=0 | VERIFIED |
| `TRISO-ANA-279` | Normalized regular eigenfunction form | VERIFIED |
| `TRISO-ANA-293` | Undivided Robin characteristic equation | VERIFIED |
| `TRISO-ANA-296` | Reduced Robin characteristic equation mu cot(mu)=1-Bi | CONDITIONALLY VERIFIED |
| `TRISO-ANA-305` | Modal decay rate Lambda_n=D mu_n^2/R^2 | VERIFIED |
| `TRISO-SL-204` | Self-adjoint Sturm–Liouville form | VERIFIED |
| `TRISO-SL-205` | Sturm–Liouville p(r)=r^2 | VERIFIED |
| `TRISO-SL-207` | Sturm–Liouville weight w(r)=r^2 | VERIFIED |
| `TRISO-SL-208` | Sturm–Liouville eigenvalue lambda_n=k_n^2 | VERIFIED |
| `TRISO-SL-215` | Lagrange-identity derivative for two modes | VERIFIED |
| `TRISO-SL-224` | Weighted eigenfunction orthogonality | VERIFIED |
| `TRISO-SL-233` | Modal coefficient projection | VERIFIED |
| `TRISO-SL-235` | Formal transient eigenfunction series | CONDITIONALLY VERIFIED |
| `TRISO-SL-242` | Formal t→infinity steady-state limit | CONDITIONALLY VERIFIED |

### Sturm–Liouville provenance

The canonical derivation uses a direct orthogonality derivation rather than relying on an unqualified regular-endpoint theorem, because the radial endpoint at r=0 has p(0)=0. The standard Sturm–Liouville framework and eigenvalue/eigenfunction terminology are supported by the NIST Digital Library of Mathematical Functions, §1.13(viii) and §3.7(iv): https://dlmf.nist.gov/1.13 and https://dlmf.nist.gov/3.7.

The direct orthogonality calculation in TRISO-SL-210 through TRISO-SL-224 is the project's own derivation for this radial problem.
