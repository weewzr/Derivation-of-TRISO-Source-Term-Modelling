> HISTORICAL ARCHIVE: superseded working document. Do not use as the current mathematical or project-status source.

# Two-Layer Spherical Diffusion Benchmark — Continuum Foundation

## Purpose

This benchmark is the first controlled test of the connection between the continuous TRISO diffusion model and the repository's Walk-on-Spheres (WOS) interface treatment.

Use two concentric materials:

- inner region: 0 < r < a, diffusivity D1, uniform source S0;
- outer region: a < r < R, diffusivity D2, zero volumetric source.

The outer surface uses a perfect-sink condition c(R)=0.

This is intentionally simpler than the five-layer particle so that a mismatch can be traced to the interface treatment rather than to several interfaces at once.

## Plain-language picture

Material 1 creates the species. Material 2 is the coating it must cross. The two materials can have very different diffusivities, but material cannot suddenly appear or disappear at their shared boundary. The flux must therefore match on both sides.

## 1. Continuum equations

Inside the source region:

$$
\\frac{1}{r^2}\\frac{d}{dr}\\left(r^2D_1\\frac{dc_1}{dr}\\right)+S_0=0,
\\qquad 0<r<a.
$$

Inside the outer region:

$$
\\frac{1}{r^2}\\frac{d}{dr}\\left(r^2D_2\\frac{dc_2}{dr}\\right)=0,
\\qquad a<r<R.
$$

At the centre:

$$
\\left.\\frac{dc_1}{dr}\\right|_{r=0}=0.
$$

At the interface for K=1:

$$
c_1(a)=c_2(a),
$$

$$
-D_1c_1'(a)=-D_2c_2'(a).
$$

At the outer surface:

$$
c_2(R)=0.
$$

## 2. Inner solution

Start with:

$$
\\frac{d}{dr}\\left(r^2D_1c_1'\\right)=-S_0r^2.
$$

Because D1 is constant within this region:

$$
D_1\\frac{d}{dr}\\left(r^2c_1'\\right)=-S_0r^2.
$$

Integrate once:

$$
D_1r^2c_1'=-\\frac{S_0r^3}{3}+A.
$$

Regularity at r=0 gives:

$$
A=0.
$$

Therefore:

$$
c_1'=-\\frac{S_0r}{3D_1}.
$$

Integrating again:

$$
c_1(r)=B-\\frac{S_0r^2}{6D_1}.
$$

Define the interface concentration:

$$
c_I=c_1(a)=c_2(a).
$$

Then:

$$
\\boxed{c_1(r)=c_I+\\frac{S_0}{6D_1}(a^2-r^2)}.
$$

## 3. Outer solution

Start with:

$$
\\frac{d}{dr}\\left(r^2D_2c_2'\\right)=0.
$$

Integrate:

$$
r^2D_2c_2'=C.
$$

The total source produced in the inner region is:

$$
\\dot N_{gen}=4\\pi\\int_0^a S_0r^2\\,dr.
$$

Evaluate the integral:

$$
\\dot N_{gen}=\\frac{4\\pi S_0a^3}{3}.
$$

For any radius r>a, the same amount must cross that spherical surface each second at steady state:

$$
4\\pi r^2J_r(r)=\\dot N_{gen}.
$$

Therefore:

$$
J_r(r)=\\frac{S_0a^3}{3r^2}.
$$

Using Fick's law:

$$
J_r=-D_2c_2'.
$$

So:

$$
c_2'=-\\frac{S_0a^3}{3D_2r^2}.
$$

Integrate:

$$
c_2(r)=A_2+\\frac{S_0a^3}{3D_2r}.
$$

Apply c_2(R)=0:

$$
A_2=-\\frac{S_0a^3}{3D_2R}.
$$

Therefore:

$$
\\boxed{c_2(r)=\\frac{S_0a^3}{3D_2}\\left(\\frac1r-\\frac1R\\right)}.
$$

At the interface:

$$
\\boxed{c_I=\\frac{S_0a^3}{3D_2}\\left(\\frac1a-\\frac1R\\right)}.
$$

Substitute into the inner solution:

$$
\\boxed{
c_1(r)=
\\frac{S_0a^3}{3D_2}\\left(\\frac1a-\\frac1R\\right)
+\\frac{S_0}{6D_1}(a^2-r^2).
}
$$

## 4. Interface flux check

Differentiate the inner solution:

$$
c_1'(r)=-\\frac{S_0r}{3D_1}.
$$

At r=a:

$$
-D_1c_1'(a)=\\frac{S_0a}{3}.
$$

Differentiate the outer solution:

$$
c_2'(r)=-\\frac{S_0a^3}{3D_2r^2}.
$$

At r=a:

$$
-D_2c_2'(a)=\\frac{S_0a}{3}.
$$

Therefore:

$$
\\boxed{-D_1c_1'(a)=-D_2c_2'(a)}.
$$

Plain-language meaning: the slope changes when the material changes, but the amount crossing the interface per unit area per unit time does not jump.

## 5. What this benchmark can test

A WOS calculation can be compared with this continuum solution for quantities that are both defined mathematically and measurable from the stochastic model.

Primary targets:

1. interface flux balance;
2. concentration profile or an equivalent steady observable;
3. total generation versus total release;
4. sensitivity to capture_epsilon and reinsertion distance;
5. Monte-Carlo statistical uncertainty.

A diagnostic run can also compare the repository's D-linear interface rule with a square-root alternative, but neither should be called correct until compared with the continuum target.

## 6. Limiting cases

Equal diffusivities:

$$D_1=D_2=D.$$

The internal interface becomes mathematically artificial, and the solution must reduce to the one-material spherical source problem.

Vanishing outer layer:

$$a\\rightarrow R.$$

The problem reduces to the single-layer sphere.

Very small outer diffusivity:

$$D_2\\rightarrow0^+.$$

The interface concentration scales as

$$c_I\\propto\\frac1{D_2},$$

so the required concentration becomes very large as the outer barrier becomes harder to cross.

## 7. Relation to WOS

The continuum benchmark does not assume a particular stochastic algorithm.

The WOS implementation must show that its long-run statistics approach the continuum solution.

The existing repository uses the interface probability

$$
p_{1\\rightarrow2}=\\frac{KD_2}{D_1+KD_2}.
$$

For K=1:

$$
p_{1\\rightarrow2}=\\frac{D_2}{D_1+D_2}.
$$

This probability was derived earlier from the repository's WOS encounter-rate argument. It remains a model-specific stochastic rule until the full continuum-to-WOS correspondence is demonstrated.

## 8. Current status

[VERIFIED] The steady two-layer continuum solution is analytically derived.

[VERIFIED] The continuum solution conserves interface flux exactly.

[UNVERIFIED] Existing multilayer WOS reproduces this continuum solution.

[UNVERIFIED] Finite capture_epsilon produces negligible bias.

[UNVERIFIED] Finite reinsertion produces negligible bias.

[UNVERIFIED] The finite first-passage lookup table produces negligible bias.

[UNVERIFIED] The D-linear interface rule is the unique or universally correct stochastic representation of the chosen divergence-form PDE.

## References

- Lejay, A. and Pichot, G., work on simulating diffusion processes in discontinuous media and skew Brownian motion. citeturn463510search4turn463510search1
- Maire, S. and Nguyen, G., stochastic WOS methods for piecewise-constant diffusion with transmission conditions. citeturn463510search2
- Hwang, C.-O., Hong, S., and Kim, J., partially reflecting WOS methods for discontinuous coefficients and interfaces. citeturn463510search0