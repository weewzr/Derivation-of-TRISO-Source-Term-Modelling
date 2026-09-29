# WOS Interface Rule — Markov Balance Derivation and Two-Layer Oracle

## 1. Question being tested

The supervisor implementation uses:

$$
p_{1\to2}=\frac{K D_2}{D_1+K D_2}.
$$

We need to distinguish two claims:

1. This formula gives the desired equilibrium for the particular WOS/reinsertion process.
2. The complete WOS process converges to the transient divergence-form diffusion PDE.

The first can be derived from the implemented event mechanism. The second still requires transient convergence evidence.

## 2. Why the event rate scales with D

Near an interface, the implementation uses a fixed physical capture distance epsilon and reinserts a walker a fixed multiple of epsilon from the interface.

Think of a walker immediately after reinsertion. Its next interface-free WOS sphere has a radius proportional to epsilon.

For a Brownian particle starting at the centre of a sphere of radius rho, the mean time to leave the sphere is:

$$
\mathbb E[\tau]=\frac{\rho^2}{6D}.
$$

At fixed rho, increasing D makes the time shorter.

Therefore the number of interface approaches per unit simulated time scales as:

$$
\nu\propto D.
$$

This is the specific event-rate property used by the repository's interface construction.

Status: [INFERRED FROM CODE] + [DERIVED]

## 3. Equilibrium as a flow-balance problem

Let c1 and c2 be the equilibrium concentrations immediately on the two sides of an interface.

Let nu1 and nu2 be the interface-encounter rates produced by the WOS construction.

Let p12 be the probability of crossing from side 1 to side 2.

Let p21 be the probability of crossing back.

At equilibrium, the average number of crossings from 1 to 2 must equal the average number from 2 to 1:

$$
c_1\nu_1p_{12}=c_2\nu_2p_{21}.
$$

For unit partition, the desired continuum condition is concentration continuity:

$$
c_2=c_1.
$$

Since nu_i is proportional to D_i:

$$
c_1D_1p_{12}=c_2D_2p_{21}.
$$

For a two-way interface decision:

$$
p_{21}=1-p_{12}.
$$

For K=1 this becomes:

$$
D_1p_{12}=D_2(1-p_{12}).
$$

Move the second probability term to the left:

$$
(D_1+D_2)p_{12}=D_2.
$$

Divide by D1+D2:

$$
\boxed{p_{12}=\frac{D_2}{D_1+D_2}}.
$$

## 4. Partition coefficient

Suppose the desired equilibrium condition is:

$$
K=\frac{c_2}{c_1}.
$$

Then:

$$
c_1D_1p_{12}=KD_2c_1p_{21}.
$$

Cancel c1:

$$
D_1p_{12}=KD_2p_{21}.
$$

Use p21=1-p12:

$$
D_1p_{12}=KD_2(1-p_{12}).
$$

Expand the right-hand side:

$$
D_1p_{12}=KD_2-KD_2p_{12}.
$$

Collect p12:

$$
(D_1+KD_2)p_{12}=KD_2.
$$

Therefore:

$$
\boxed{p_{12}=\frac{KD_2}{D_1+KD_2}}.
$$

Status: [DERIVED] for the stated WOS/reinsertion event-rate model.

## 5. What this derivation does not prove

This balance argument does not prove the full transient generator.

It does not by itself establish that:

- every transient concentration profile converges to the divergence-form PDE;
- finite capture epsilon has vanishing bias;
- finite reinsertion has vanishing bias;
- inverse-CDF interpolation has negligible effect;
- the same interface rule remains correct under every possible WOS implementation detail;
- the stochastic release estimator exactly equals a continuum mass-release observable at finite sample size.

Those remain [UNVERIFIED].

## 6. Why the square-root rule is not an automatic contradiction

Some heterogeneous random-walk constructions use different step-length or clock rules. If the frequency with which a process samples an interface scales differently with D, its detailed-balance probability changes.

Therefore an interface probability cannot be declared universal without specifying the stochastic process.

Published work on piecewise-constant diffusion explicitly treats transmission/interface conditions as part of the stochastic construction. See Maire and Nguyen's WOS-based stochastic methods for stratified diffusion:

https://doi.org/10.1016/j.matcom.2015.09.014

and related discontinuous-diffusion random-walk literature.

## 7. Independent two-layer continuum oracle

Use an inner sphere of radius a and an outer sphere of radius R.

Inner region:

$$
0<r<a,\qquad D=D_1,\qquad S=S_0.
$$

Outer region:

$$
a<r<R,\qquad D=D_2,\qquad S=0.
$$

Boundary conditions:

$$
c_1'(0)=0,\qquad c_1(a)=c_2(a),\qquad D_1c_1'(a)=D_2c_2'(a),\qquad c_2(R)=0.
$$

The analytical solution already derived in `05_two_layer_spherical_benchmark.md` is:

$$
c_2(r)=\frac{S_0a^3}{3D_2}\left(\frac1r-\frac1R\right),
$$

$$
c_1(r)=\frac{S_0a^3}{3D_2}\left(\frac1a-\frac1R\right)+\frac{S_0}{6D_1}(a^2-r^2).
$$

and the common interface flux is:

$$
J_I=\frac{S_0a}{3}.
$$

## 8. Recommended numerical experiment

Do not modify the production interface formula yet.

For the real supervisor code, compare WOS against the oracle while varying:

$$
D_2/D_1\in\{1,10^{-1},10^{-2},10^{-3}\},
$$

and:

$$
\epsilon/a\in\{10^{-2},10^{-3},10^{-4}\}.
$$

At each case report:

- release-time or occupancy statistic;
- Monte-Carlo standard error;
- interface observable;
- total inventory/release balance;
- capture-epsilon sensitivity.

Only after the numerical evidence is collected should the interface formula be promoted from [DERIVED FOR THE IMPLEMENTED EVENT MODEL] to a stronger [VERIFIED] status.

## 9. Verification hierarchy

$$
\text{bulk WOS first-passage law}
\rightarrow
\text{equilibrium interface balance}
\rightarrow
\text{two-layer transient benchmark}
\rightarrow
\text{epsilon/reinsertion convergence}
\rightarrow
\text{five-layer TRISO}.
$$

## 10. Readability note

Every displayed equation should be preceded or followed by a short explanation in ordinary language.

The goal is not to make the science less rigorous. The goal is to make the chain of ideas understandable to a strong high-school student while retaining the exact mathematics, units, assumptions, and uncertainty.