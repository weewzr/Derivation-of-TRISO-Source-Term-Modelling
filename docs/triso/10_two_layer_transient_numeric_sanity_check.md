# Two-Layer Transient Reference — Numerical Sanity Check

## Execution limitation

The Ray execution container has no Rust compiler or Cargo. The numerical root check below was therefore performed with a tiny independent C program containing only the algebraic characteristic equation already derived in `09_two_layer_transient_reference.md`.

This is a **mathematical sanity check**, not a production Rust test and not a WOS simulation.

## Test parameters

Use the dimensioned example from the derivation with numerical values:

- inner diffusivity: `D1 = 3.0` in the chosen diffusion units;
- outer diffusivity: `D2 = 0.25` in the same units;
- interface radius: `a = 2.0`;
- outer radius: `R = 5.0`.

The transient eigenvalue equation was evaluated in its sine/cosine form so that artificial poles from cotangent notation were avoided:

$$
F(\\lambda)=
D_1k_1\\cos(k_1a)\\sin(k_2(R-a))
+D_2k_2\\sin(k_1a)\\cos(k_2(R-a))
+\\frac{D_2-D_1}{a}\\sin(k_1a)\\sin(k_2(R-a)).
$$

with

$$
k_i=\\sqrt{\\frac{\\lambda}{D_i}}.
$$

## Positive roots found

| mode | lambda | k1 | k2 | residual F(lambda) |
|---:|---:|---:|---:|---:|
| 1 | 0.101587738207 | 0.184017877942 | 0.637456628192 | 6.8e-14 |
| 2 | 0.482120346199 | 0.400882504066 | 1.38869772982 | 2.3e-14 |
| 3 | 1.30911274235 | 0.660583767675 | 2.28832929654 | 7.0e-15 |
| 4 | 2.65055850530 | 0.939957180816 | 3.25610718822 | 9.9e-14 |
| 5 | 4.51286757866 | 1.22649467707 | 4.24870219181 | 9.4e-14 |
| 6 | 6.88143638986 | 1.51453363447 | 5.24649840936 | 4.3e-15 |

The first twelve positive roots were found in the exploratory scan; the table records the first six because these are the most useful for constructing an initial transient reference.

The residual is the value of the characteristic equation at the numerical root. Values near zero confirm that the root-finding calculation solved the equation it was given.

## Equal-diffusivity limiting check

Set

$$
D_1=D_2=D.
$$

The characteristic equation becomes:

$$
Dk[\\cos(ka)\\sin(k(R-a))+\\sin(ka)\\cos(k(R-a))]=0.
$$

Use the angle-addition identity:

$$
\\cos(ka)\\sin(k(R-a))+\\sin(ka)\\cos(k(R-a))=\\sin(kR).
$$

Therefore the artificial interface disappears and:

$$
\\sin(kR)=0.
$$

Positive modes satisfy:

$$
k_n=\\frac{n\\pi}{R}.
$$

The corresponding decay rates are:

$$
\\lambda_n=D\\left(\\frac{n\\pi}{R}\\right)^2.
$$

For the sanity-check values `D=1.5`, `R=5`, the first five rates are:

| n | k_n | lambda_n |
|---:|---:|---:|
| 1 | 0.628318530718 | 0.592176264065 |
| 2 | 1.256637061436 | 2.368705056261 |
| 3 | 1.884955592154 | 5.329586376588 |
| 4 | 2.513274122872 | 9.474820225046 |
| 5 | 3.141592653590 | 14.804406601634 |

These values are the expected spherical Dirichlet eigenvalues for the one-material limiting case.

## Interpretation

The heterogeneous example has a much smaller first decay rate than the equal-D example because the outer material has a much smaller diffusivity (`D2/D1 = 1/12`).

In ordinary language: the slow outer layer makes the whole system take longer to approach its final state.

This is a qualitative sanity check, not yet an accuracy statement about the WOS implementation.

## What has now been established

[VERIFIED] The derived two-layer characteristic equation has positive numerical roots for a controlled parameter set.

[VERIFIED] The characteristic equation reduces algebraically to the familiar one-material spherical condition when `D1=D2`.

[UNVERIFIED] The truncated eigenfunction series has been numerically evaluated against a WOS trajectory ensemble.

[UNVERIFIED] The WOS multilayer interface algorithm converges to this transient continuum solution.

[UNVERIFIED] Finite capture and reinsertion parameters produce negligible transient bias.

## Next gate

The next executable research step is to evaluate the eigenfunction series itself at selected radii and times, then compare those values against WOS statistics.

That requires either:

1. a local Rust-capable checkout, or
2. a successfully executed GitHub Actions run that exposes the Rust results.

Do not modify the production interface formula before this comparison.