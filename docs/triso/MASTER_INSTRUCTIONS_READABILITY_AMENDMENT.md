# Master Instructions Amendment — High-School-Readable Mathematics

All major derivations and explanations must be understandable to a strong high-school student with algebra, basic calculus, graphs, and introductory physics, while remaining rigorous enough for engineering/research review.

This is a communication requirement, not a simplification of the science.

## Required presentation

1. Explain the physical idea before a substantial equation.
2. Define each new important symbol, its physical meaning, units, and purpose.
3. Define specialist terms in plain language on first use.
4. Keep approximately one meaningful mathematical operation per displayed step.
5. After important steps, briefly explain what changed and why.
6. Separate, where useful:
   - Physical meaning
   - Mathematics
   - Implementation meaning
7. Use small numerical examples when they materially aid understanding.
8. Never gain accessibility by deleting assumptions, units, derivation steps, or uncertainty, or by presenting an approximation as exact.
9. Before marking a major derivation complete, ask whether a strong high-school student can follow what each major equation means, why it is written, and how the next step follows.

## Stochastic-interface caution

When a stochastic method represents a discontinuous-coefficient diffusion equation, do not assume the interface transmission probability is universal. Different stochastic constructions can produce different interface parameters.

Explicitly identify:
- the continuum PDE and interface conditions;
- the stochastic process represented;
- the local encounter/waiting-time mechanism;
- the resulting transmission probability;
- the assumptions under which that probability is valid.

If literature gives a different interface parameter, record the disagreement and resolve it through derivation and benchmark testing rather than silently choosing a formula.
