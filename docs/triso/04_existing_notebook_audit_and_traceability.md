# Existing Notebook Audit and Traceability

## Source notebook

The supplied file is **TRISO Fuel Derivation Ray V1.tex**, 881 lines.

Its own abstract states that it is a narrower record of independently derived work and deliberately omits the completed five-layer analytical solution, Arrhenius relation, and other material not personally derived. That statement is important provenance: the notebook is a research source, not the final validated model.

## Current state of the original derivation

| Original notebook component | Assessment | Reason / action |
|---|---|---|
| Five-layer conceptual geometry | [ASSUMPTION] | Useful physical scope; dimensions intentionally not to scale. |
| Concentration definition and units | [VERIFIED] | Consistent within the homogeneous benchmark. |
| Initial condition (c(r,0)=0) | [ASSUMPTION] | Appropriate benchmark, not necessarily universal. |
| Centre condition (c_r(0,t)=0) | [VERIFIED] | Follows from spherical symmetry for a regular solution. |
| Robin outer condition | [VERIFIED] within stated benchmark | For the physical multilayer model the diffusivity must be (D_5), and a nonzero coolant concentration should be retained until the zero-coolant assumption is explicitly selected. |
| General equation (partial_t c=D
abla^2c+S) | [VERIFIED] only for constant (D) | Not the general multilayer equation. |
| Spherical coordinate Fick equation | [VERIFIED] for constant (D) | Must be replaced by conservative variable-(D) form for the five-layer model. |
| Product-rule expansion | [VERIFIED] under constant (D) | Not applicable across discontinuous interfaces without care. |
| Kernel-localised source | [ASSUMPTION] | Correct model boundary for generation under the stated project scope. |
| Source decomposition (S=S_{gen}+S_{decay}+S_{trap}) | [UNVERIFIED] | Physical forms of decay/trapping terms were not established in the notebook. |
| Part I homogeneous sphere | [VERIFIED] as a mathematical benchmark | Not the complete physical TRISO model. |
| Part I steady solution | [VERIFIED] | Independently checked by global mass balance in the notebook. |
| Part I transient separation | [VERIFIED] after notation repair | Spatial eigenvalue and temporal decay rate must be distinct dimensional quantities. |
| Raw eigencondition | [UNVERIFIED] pending full independent modal/stability audit | Notebook stops at the raw condition. |
| FTCS interior stencil | [VERIFIED] for constant (D), uniform grid, interior nodes | Not the multilayer discretisation. |
| Centre FTCS treatment | [VERIFIED] at benchmark level | Should additionally be checked from the selected conservative numerical method. |
| Robin ghost-point update | [UNVERIFIED] | Discrete derivation should be re-derived and stability checked independently. |
| FTCS stability bound | [UNVERIFIED] | Non-negative coefficients are a sufficient monotonicity-style check, not by themselves a complete spectral stability proof. |
| Part II multilayer analytical model | [DERIVATION GAP] | Not present in this notebook. |
| Equation-to-code mapping | [CODE MISMATCH] / unavailable | The current GitHub repository has no scientific implementation to map against. |

## Original → stable equation mapping

The first stable IDs are assigned to the repaired continuous foundation rather than renumbering every old equation.

| Original notebook label | Stable project ID | Relationship |
|---|---|---|
| `eq:fick` | `TRISO-GOV-012` | General conservation + Fickian law; repaired to retain variable (D) inside divergence. |
| `eq:fick-spherical` | `TRISO-SPH-007` | Reconstructed from radial conservation rather than constant-(D) Laplacian expansion. |
| `eq:radial-conservative` | `TRISO-SPH-007` | Same intended radial structure, generalized to (D(r,t)). |
| `eq:kernel-source` | `TRISO-GOV-015` | Retained as the kernel-localised generation model. |
| `eq:b:pde` | `TRISO-GOV-025` + `TRISO-GOV-026` | Part I benchmark recovered by equal diffusivities and uniform source. |
| `eq:b:vproblem` | pending | Requires final eigenvalue notation audit before stable modal IDs. |
| `eq:b:sl-ode` | `TRISO-VER-001` to `TRISO-VER-005` | Dimensional repair separates (k_n,[mathrm{m^{-1}}]) from (lambda_n,[mathrm{s^{-1}}]). |
| `eq:b:centre-ghost` | pending numerical ID | Numerical treatment deliberately not advanced in this continuous-model increment. |
| `eq:b:ftcs-surface` | pending numerical ID | Requires independent stability audit before adoption. |

## Dependency map now established

```text
physical species
  ↓
local conservation
  ↓
control volume + divergence theorem
  ↓
Fickian flux law
  ↓
general conservative PDE
  ↓
spherical symmetry
  ↓
piecewise material coefficients
  ↓
piecewise kernel source
  ↓
centre condition
  ↓
four interface conditions
  ↓
outer OPyC Robin condition
  ↓
complete five-layer continuous model
  ↓
Part I recovered as a limiting benchmark
  ↓
future numerical discretisation
  ↓
future code mapping
  ↓
future verification
```

## Repository inspection status

The GitHub repository currently contains only the bootstrap README and the pre-existing access-test file. No scientific source tree, tests, CI configuration, mesh, solver, or material implementation is currently present.

Therefore the following required reverse map remains unavailable:

```text
source code
→ algorithm
→ discrete equation
→ continuous equation
→ physical model
```

This is a factual repository limitation, not an assumption about an unseen implementation.

The local execution environment also does not contain a checked-out Git repository, so local `git status` and checked-out-branch state cannot be independently reported.

## Questions for supervisor

1. Which fission-product species is the first target of the physical model?
2. Is the first physical model stable/long-lived, or should radioactive decay be included explicitly?
3. Is trapping/release represented, and if so what constitutive law is intended?
4. Are concentrations continuous across every interface, or are partition coefficients required?
5. Is any explicit interfacial resistance intended, especially at PyC/SiC interfaces?
6. Are (D_i) prescribed constants for the first implementation or functions of temperature, fluence, burnup, or other state variables?
7. Is (c_infty=0) an intended physical approximation or only a benchmark?
8. What supervisor repository contents/implementation are expected to populate this currently empty codebase?

## Current handoff

The continuous five-layer model is sufficiently established to justify the next mathematical task, but **not yet to claim the project foundation is complete**.

The next dependency is the explicit numerical formulation chosen from the actual repository implementation once that implementation is available. Until then, do not invent a code architecture or silently select a numerical method merely for convenience.
