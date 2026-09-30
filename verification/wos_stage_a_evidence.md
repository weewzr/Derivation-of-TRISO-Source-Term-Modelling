# WOS Stage-A Smoke / Contract Execution Evidence

## Status

**Diagnostic execution evidence only — NOT WOS validation.**

R2-B01 remains **OPEN**.

## Provenance

- Repository: `weewzr/Derivation-of-TRISO-Source-Term-Modelling`
- Branch: `main`
- Workflow: `WOS Stage A smoke contract`
- Run ID: `36669658813`
- Workflow conclusion: `success`
- Research commit: `857160c7d83d42ad271f1db4276897bd86d1d710`
- Supervisor repository: `theodoreOnzGit/outram-park-backend`
- Supervisor commit: `8d31482d127e211614ebb3f66b1076e1ed6dea98`
- Rust: `rustc 1.98.1 (48a229cea 2026-09-01)`
- Artifact: `wos-stage-a-results`
- Artifact ID: `11077881079`
- Artifact SHA-256: `3d72f7af2056a2fb0d30668d1499536ad96cc6b1dadaeacde9dc14c48e5c716a`

## Frozen Stage-A parameters

- histories: `N = 24`
- max steps/history: `5,000,000`
- capture epsilon: `10 nm`
- reinsertion factor: `2`
- partition coefficient: `K = 1`
- nuclide: Cs-137
- initial distribution: uniform in kernel volume
- outer boundary: absorbing production OPyC release boundary

Radii (m), excluding the centre:

`[2.125e-4, 3.125e-4, 3.525e-4, 3.875e-4, 4.275e-4]`

Layer diffusivities (m^2/s):

`[1.2502982636347968e-13, 1.0e-8, 4.062299125614697e-14, 9.227773168241615e-17, 4.062299125614697e-14]`

## Executed result

- released: `0`
- CensoredMaxSteps: `24`
- censor fraction: `1.00000000`
- WOS execution wall time: `14.366 s`
- mean steps/history: `5,000,000`
- maximum steps observed: `5,000,000`

Every history exhausted the complete step budget.

## Interpretation

This run proves that the concrete production WOS path can be compiled and executed through the verification adapter and that release and max-step censoring are kept distinct.

It does **not** provide a five-layer release CDF and does **not** validate WOS against FV. No censored history is treated as released.

The 100% censoring result is a diagnostic finding requiring localisation of where production steps are spent before Stage B.

## Characteristic layer diffusion scales

Using only the order-of-magnitude diagnostic scaling `tau ~ L^2/D`:

- kernel, L=212.5 um: approximately `3.61e5 s` (~4.18 d);
- buffer, L=100 um: `1.00 s`;
- IPyC, L=40 um: approximately `3.94e4 s` (~10.9 h);
- SiC, L=35 um: approximately `1.33e7 s` (~154 d);
- OPyC, L=40 um: approximately `3.94e4 s` (~10.9 h).

These are characteristic scales, not exact first-passage times. In particular, very small SiC diffusivity strongly increases physical residence time but does not by itself require millions of geometric WOS/interface steps.

## Next diagnostic gate

Before Stage B, instrument a very small verification ensemble around the unchanged production `step_multilayer` transition to measure layer residence in step count, interface encounter/crossing behaviour, radius extrema, zero-time interface events and accumulated physical time. Separately execute homogeneous absorbing-sphere and controlled interface semantic tests.

R2-B01 remains OPEN.
