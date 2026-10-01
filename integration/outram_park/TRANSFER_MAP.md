# Outram Park / Supervisor Integration Transfer Map

## Policy

Canonical development/integration branch for this research is `weewzr/Derivation-of-TRISO-Source-Term-Modelling:main`.

Supervisor repository `theodoreOnzGit/outram-park-backend` is READ-ONLY unless explicit authorization is given.

Pinned supervisor base used throughout the current verification chain:

`8d31482d127e211614ebb3f66b1076e1ed6dea98`.

No supervisor patch is currently authorized for transfer because the hard five-layer Method-2 matrix/high-precision gate and R2-B01 remain open.

## Existing production source interfaces used by the project

The verification harness imports supervisor code from the `boon_lay::lagrangian_decay_simulator::lagrangian_diffusion` namespace, including:

- `first_passage::walk_on_spheres::{WalkParams, WoSWalker, sample_uniform_in_ball}`;
- `first_passage::interface::does_transmit`;
- `first_passage::sphere_fpt` first-passage primitives;
- `single_particle_simulator::constructive_solid_geometry::TrisoCell`.

The exact concrete supervisor filesystem paths must be re-resolved against the pinned supervisor commit before a transfer patch is produced; this document does not invent paths not established by current repository evidence.

## Candidate eventual integration

If the independently reviewed final Method-2 improvement is the deterministic interface-state transform/macro-renewal accelerator, the transferable change will need to preserve:

1. existing production WOS as the baseline/reference path;
2. finite-capture semantics as a distinct Process-A option unless explicitly superseded;
3. exact shell/ball first-passage mathematics for Process B;
4. the existing `does_transmit` interface law and partition semantics;
5. explicit censoring/release distinctions;
6. provenance of geometry/material diffusivities;
7. a selectable/isolated accelerated path rather than silently changing existing production results.

## Proposed patch location in this repository

Until independent closure, transferable supervisor-facing work belongs under:

- `integration/outram_park/` for patch/mapping records;
- `verification/` for executable verification harnesses;
- `docs/triso/` for mathematical contracts.

No code is pushed to the supervisor repository.

## Transfer record required before authorization

A future patch record must contain:

| Field | Required content |
|---|---|
| Supervisor base | exact commit SHA |
| Supervisor source file | exact path at that SHA |
| Original behaviour | current finite-capture/direct-WOS semantics |
| Proposed behaviour | independently verified Method-2 improvement |
| Mathematical justification | exact first-passage / renewal equations |
| Verification evidence | controlled 2-layer, 3-layer, hard-matrix and final closure evidence |
| Compatibility | Process A vs B/C distinction and API behaviour |
| Tests | unit, controlled FV/WOS, regression, provenance |
| Transfer commit/patch | clean commit or patch generated from canonical research `main` |

## Current status

**PREPARATORY ONLY — NO TRANSFER PATCH YET.**

The high-precision hard-matrix cross-check is pending and R2-B01 remains OPEN. Creating a supervisor-facing implementation now would risk encoding an unfinished numerical representation.
