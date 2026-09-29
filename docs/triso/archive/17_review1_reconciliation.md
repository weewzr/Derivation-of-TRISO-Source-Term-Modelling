> HISTORICAL ARCHIVE: superseded working document.\n\n# 17 — Review-1 Reconciliation and Numerical-Stage Handoff

## Review-1 result

Independent Review 1 determination:

PASS WITH NON-BLOCKING FINDINGS.

Review scope was the continuous mathematical foundation.

## Finding R1-m01 — stable equation mapping

Status: RESOLVED.

The 33 original LaTeX equations now have one authoritative mapping in `docs/triso/13_canonical_equation_register.md`.

The rule is that new work must use this register rather than recreate source-to-ID mappings independently.

## Finding R1-M01 — separation-of-variables scope

Status: RESOLVED FOR THE BENCHMARK.

The general PDE remains general.

The separated five-layer eigenbenchmark is explicitly restricted to:

- time-independent positive D_i;
- reaction-free model R_i=0;
- time-independent interface parameters;
- constant h;
- c_infty=0 benchmark condition;
- zero initial concentration.

This is documented in `docs/triso/16_frozen_continuous_benchmark.md`.

The notation k_i²=Lambda/D_i is therefore only used inside this restricted analytical eigenbenchmark.

## Finding R1-M03 — source/reaction closure

Status: RESOLVED FOR THE BASE NUMERICAL BENCHMARK; PHYSICAL EXTENSIONS REMAIN CONDITIONAL.

The numerical foundation is frozen to:

R_i=0.

Generation is steady and kernel-confined.

Decay, trapping and other reaction models remain `[QUESTION FOR SUPERVISOR]` items rather than being silently inserted later.

## Finding R1-m04 — coolant/interface choices

Status: RESOLVED FOR THE BASE BENCHMARK; PHYSICAL ALTERNATIVES REMAIN CONDITIONAL.

Base benchmark uses:

- concentration-continuous interfaces;
- flux-continuous interfaces;
- K_i=1;
- no interfacial resistance;
- c_infty=0.

The general Robin law retains c_infty explicitly.

## Finding R1-M05 — literature provenance

Status: PARTIALLY RESOLVED.

The core TRISO diffusion-model provenance is established, but parameter-level references for specific material correlations remain a later requirement.

Repository documents must use durable bibliographic references/URLs/DOIs rather than transient ChatGPT citation tokens.

## Finding R1-M06 — stale repository state

Status: RESOLVED.

Old repository-state statements have been replaced with current pointers or explicitly marked as historical snapshots.

Current Ray repository state is authoritative at the branch tip recorded below.

## Finding R1-m07 — original LaTeX preservation

Status: RESOLVED.

The original notebook is preserved verbatim at:

notes/raw/TRISO Fuel Derivation Ray V1.tex

SHA-256:

5ce5f77e5148a234a7e7339a485923df982b54edfc79ff96796d26e0673158ce

## Current Ray repository state

Repository:

weewzr/Derivation-of-TRISO-Source-Term-Modelling

Feature branch:

ray/triso-foundation

Current branch tip is maintained by the repository's current Ray branch and should be obtained from the branch ref rather than this historical handoff note.

Default branch:

main

Current main tip:

bc325f6f3cb3f187b6ff9bd696130423d3ff8c85

At this handoff the Ray branch is ahead of main and remains isolated from main.

## Frozen continuous model for numerical derivation

The numerical stage must discretise:

partial c_i/partial t = (1/r²) partial/partial r [r² D_i partial c_i/partial r] + S_i(r),

with:

R_i=0;

S_i=S0 in the kernel and 0 elsewhere;

D_i positive, piecewise constant in space and constant in time;

c_k=c_{k+1} at every interface;

-D_k c_k'=-D_{k+1}c_{k+1}' at every interface;

K_k=1;

no interfacial resistance;

-D_5 c_5'(R)=h[c_5(R)-c_infty] with the benchmark choice c_infty=0;

c_i(r,0)=0.

## Numerical method status

FTCS is retained as a transparent deterministic Part-I benchmark.

The supervisor production implementation is a Lagrangian first-passage Walk-on-Spheres method.

The next stage must derive the actual production discrete formulation from the repository implementation. No assumption is made that the production method is FTCS.

## Highest-value next mathematical task

Derive the selected production numerical formulation directly from the frozen conservative five-layer PDE and the actual supervisor WOS algorithm, starting with the precise stochastic/first-passage discrete state and its correspondence to the continuum control-volume conservation statement.

Before changing production code, establish the equation-to-algorithm map and identify the exact discrete quantities the code computes.

Review 1 is closed. The production WOS benchmark is now frozen in docs/triso/21_frozen_production_wos_contract.md, and Review 2 preparation is documented in docs/triso/22_review2_numerical_formulation.md.