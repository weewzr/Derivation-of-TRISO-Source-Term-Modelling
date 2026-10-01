# Interface-State Renewal Minimal Analytical Verification

## Status

**ALGEBRAIC SANITY CHECK ONLY — NOT STOCHASTIC VALIDATION.**

## Purpose

Verify the algebraic marginalisation identity before any hard five-layer transform implementation.

Consider one transient state X. One renewal:
- releases with probability p;
- returns to X with probability q=1-p;
- physical waiting-time transform for the renewal is g(s).

Then

Phi_X(s)=g(s)[p+q Phi_X(s)].

Solving,

Phi_X(s)= p g(s)/(1-q g(s)).

The explicit path expansion is

Phi_X(s)=p g(s) [1+q g(s)+q^2 g(s)^2+...]

which is the transform of any number of failed return cycles followed by success.

At s=0, g(0)=1, so

Phi_X(0)=p/(1-q)=1.

Thus probability normalization is preserved exactly and the matrix inverse is the finite-state generalization of the geometric renewal sum.

For multiple states, path expansion of

[I-K]^{-1}=I+K+K^2+...

sums every possible finite sequence of transient interface-state renewals, with each path product carrying:
- all shell/ball physical-time transforms;
- all interface transition probabilities.

The absorbing vector B terminates paths at release.

This verifies the mathematical interpretation of the matrix construction. It does not verify numerical conditioning, initial-radius quadrature, or inverse Laplace recovery; those remain future controlled gates.

No workflow is required for this algebraic identity.
