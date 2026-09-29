# TRISO Fuel Particle Mathematical Derivation & Numerical Implementation

This repository develops the TRISO fuel-particle source-term mathematical model from first principles and connects the continuous equations to a reproducible numerical implementation.

## Project status

The supervisor-provided repository is authoritative. Active development is isolated on the feature branch `ray/triso-foundation`; the default branch is `main`.

The current branch is being used for the research foundation. Scientific implementation changes should follow the first-principles derivation and verification workflow.

## Research evidence streams

1. Existing research notebook / converted derivation notes
2. Independent first-principles mathematical derivation
3. Authoritative literature and documentation
4. Repository implementation
5. Analytical and numerical verification

Disagreements between these streams must be investigated and documented.

## Intended trace

physical principles -> geometry -> assumptions -> conservation laws -> constitutive relations -> governing equations -> spherical formulation -> boundary/interface conditions -> continuous derivation -> spatial discretisation -> temporal discretisation -> discrete algebraic system -> algorithm -> implementation -> verification

## Documentation

The rigorous derivation will be developed under `docs/triso/` as the repository architecture becomes established.

## Important rule

Do not treat exploratory notes or an existing implementation as mathematical ground truth.