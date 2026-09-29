> HISTORICAL ARCHIVE: superseded working document. Do not use as the current project source.

# TRISO Continuum-to-WOS Equivalence — Research Note

## Purpose

Map the existing Walk-on-Spheres implementation to the continuum five-layer diffusion model and identify what must be demonstrated before the multilayer algorithm is called verified.

## Continuum model

Inside layer i:

$$
\frac{\partial c_i}{\partial t}
=
\nabla\cdot(D_i\nabla c_i)+S_i-R_i.
$$

Under spherical symmetry:

$$
\frac{\partial c_i}{\partial t}
=
\frac{1}{r^2}\frac{\partial}{\partial r}
\left(r^2 D_i\frac{\partial c_i}{\partial r}\right)+S_i-R_i.
$$

The current ideal-interface model uses concentration continuity and flux continuity.

## Existing WOS construction

The implementation:
1. finds the nearest interface;
2. constructs an interface-free sphere around the walker;
3. samples the sphere first-exit time;
4. samples an isotropic exit direction;
5. resolves an interface by transmission/reflection when within a finite capture distance;
6. reinserts the walker a finite distance from the interface;
7. repeats until release.

For one homogeneous WOS sphere,

$$
\theta=\frac{D\tau}{R^2},
\qquad
E[\tau]=\frac{R^2}{6D}.
$$

## Repository interface rule

The implementation uses

$$
p_{1\to2}=\frac{K D_2}{D_1+K D_2}.
$$

For K=1,

$$
p_{1\to2}=\frac{D_2}{D_1+D_2}.
$$

The repository derives this from its stated WOS encounter-rate argument. Near an interface at distance epsilon,

$$
\tau_i\sim\frac{\epsilon^2}{D_i},
$$

so

$$
\nu_i\sim\frac{D_i}{\epsilon^2}.
$$

Detailed balance at equilibrium is written

$$
c_1\nu_1p_{1\to2}=c_2\nu_2p_{2\to1}.
$$

With

$$
K=\frac{c_2}{c_1},
\qquad
p_{2\to1}=1-p_{1\to2},
$$

this gives

$$
D_1p_{1\to2}=KD_2(1-p_{1\to2}),
$$

and therefore

$$
\boxed{p_{1\to2}=\frac{KD_2}{D_1+KD_2}}.
$$

### Scientific status

This is a derivation of the repository's particular encounter-rate construction.

It is **not yet a complete proof** that the entire WOS process converges to the desired divergence-form diffusion equation for arbitrary transient multilayer problems.

## Literature distinction

Discontinuous-diffusion literature also contains skew-Brownian/random-walk constructions with square-root diffusivity parameters. For example,

$$
\alpha^*
=
\frac{\sqrt{D_+}}{\sqrt{D_+}+\sqrt{D_-}}.
$$

This does not by itself invalidate the repository's D-linear rule, because the stochastic process and clock/encounter construction may be different.

It does mean the project must not call the D-linear formula a universal Brownian interface probability without a derivation tied to the exact implemented process.

## Approximation inventory

The actual implementation contains at least:

- piecewise/state-dependent layer diffusivity;
- finite capture epsilon;
- finite reinsertion offset;
- finite maximum step count;
- finite inverse-CDF table and interpolation;
- Monte-Carlo sampling uncertainty;
- finite numerical precision;
- modelling assumptions about partition/interface physics;
- separate stochastic decay/transmutation coupling.

These must remain separate from the exact continuum equations.

## Verification ladder

The next validation hierarchy is:

$$
\text{homogeneous sphere}
\rightarrow
\text{single interface}
\rightarrow
\text{two-layer sphere}
\rightarrow
\text{five-layer TRISO}.
$$

The two-layer case should compare WOS with an independently derived continuum solution and test:

- total inventory versus time;
- outward release flux;
- interface concentration relation;
- interface flux continuity;
- capture-epsilon convergence;
- reinsertion-distance convergence;
- Monte-Carlo uncertainty;
- D-linear versus square-root diagnostic interface rules.

The existing CRP-6 WOS-versus-Crank test validates the single-layer case only; it does not validate the multilayer interface mechanism.

## Current conclusion

Do not replace WOS with FTCS simply because the original notebook uses FTCS.

Do not call the multilayer WOS solver fully verified yet.

The next substantive step is a controlled two-layer continuum-versus-WOS benchmark that directly tests the interface treatment.
