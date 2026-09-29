# 21 — Frozen Production WOS Mathematical Contract

## Purpose

This is the exact mathematical contract for the **production verification benchmark** used in Review-2 preparation.

It is deliberately separate from the more general physical TRISO model and from the deterministic Part-I FTCS benchmark.

The contract is frozen from the available implementation evidence. No supervisor intent is invented where the repository does not establish it.

## 1. Production benchmark class

The production benchmark is an **initial-value pure-diffusion release problem** represented by the existing WOS implementation.

It is not, at this stage, a continuously forced source problem.

It is not the finite-transfer Robin benchmark.

It is the problem actually represented by the existing pure-diffusion WOS release path.

## 2. Geometry

The domain is a concentric five-layer TRISO particle:

- fuel kernel;
- buffer;
- IPyC;
- SiC;
- OPyC.

The particle is spherically symmetric.

Let the outer radius be R and the internal interface radii be r1, r2, r3, r4.

## 3. Unknown / stochastic representation

The continuum quantity corresponding to the walker ensemble is a probability density or concentration-like field c(x,t).

For an individual history, the numerical state is:

- position x_n;
- accumulated simulation time t_n;
- current nuclide;
- random-number state.

The production solver follows individual histories rather than storing c on a global spatial mesh.

## 4. Initial condition / source semantics

The frozen production verification case uses an **initial kernel population**.

For a uniform initial concentration in the fuel kernel, a birth point is sampled uniformly in kernel volume:

r = R_k U^(1/3)

with isotropic direction.

Thus the corresponding forward continuum benchmark is:

- c(x,0) = c0 inside the kernel;
- c(x,0) = 0 outside the kernel;
- no continuous volumetric generation after t=0.

For a normalised release-fraction calculation, the initial total inventory is the reference mass.

Therefore the production benchmark is an initial-value problem, not:

∂c/∂t = div(D grad c) + S0

with continuously active S0.

## 5. Reaction model

For the first production verification benchmark:

R_i = 0.

The pure-diffusion release path does not apply decay/transmutation during the walk.

The repository contains a separate depletion path in which reaction events are sampled as competing stochastic clocks. That is retained as a later extension, not mixed into the base verification problem.

Status: [FROZEN FOR BASE BENCHMARK]

## 6. Diffusivity

Within each material region, the benchmark uses a positive material-specific diffusivity supplied by the implementation's material-property model.

For the analytical WOS hop derivation, D_i is treated as constant during the hop.

The first verification benchmark therefore assumes:

D_i > 0,

and D_i is time-independent over the benchmark interval.

This does not freeze the eventual physical property model for all TRISO studies; it freezes the assumptions needed for the present verification problem.

## 7. Internal interface model

For the base production benchmark:

K_i = 1.

The intended continuum ideal-interface interpretation is therefore:

c_i = c_{i+1}

and

-D_i c_i' = -D_{i+1} c_{i+1}'.

The WOS implementation represents the interface using stochastic transmission/reflection with:

p_{i→j} = K_j D_j / (D_i + K_j D_j)

in the repository's two-sided interface construction.

Because K=1 in the frozen benchmark:

p_{i→j} = D_j/(D_i+D_j).

No explicit interfacial resistance or interface-storage law is included in the benchmark.

## 8. Outer boundary

The existing production WOS release driver treats arrival at the outer OPyC surface as immediate release.

Therefore the frozen continuum boundary corresponding to the production verification case is:

c(R,t)=0.

This is the **production verification boundary**.

The more general finite-transfer Robin boundary,

-D_5 c_5'(R,t)=h[c_5(R,t)-c∞],

remains part of the general continuum model and deterministic benchmark, but is not silently identified with the present WOS production boundary.

## 9. Local WOS spatial step

Let ρ_n be the minimum distance from the current position to the nearest material interface or outer stopping boundary.

The local WOS sphere is:

B_n = {x : |x-x_n| < ρ_n}.

For an ordinary hop, B_n lies inside one homogeneous layer.

The walker exits at:

x_{n+1}=x_n+ρ_n n_n,

where n_n is isotropically distributed on the unit sphere.

## 10. Local WOS time step

For the current material diffusivity D_i, define:

θ_n = D_i τ_n / ρ_n².

The production code samples θ_n from the centre-start sphere first-passage distribution and reconstructs:

τ_n = θ_n ρ_n²/D_i.

The event time is updated by:

t_{n+1}=t_n+τ_n.

There is no globally prescribed Δt.

## 11. Interface event

When the nearest interface is within the finite capture distance ε_capture, the ordinary WOS hop is replaced by an interface event.

For the frozen K=1 benchmark:

p_transmit = D_next/(D_current+D_next).

Reflection has probability:

p_reflect = 1-p_transmit.

The walker is then reinserted a distance:

δ = α ε_capture

on the selected side.

The reinsertion event carries zero simulated time in the existing implementation.

## 12. Stopping condition

A history terminates when the walker reaches the OPyC outer surface.

The release time for history m is τ_m.

The cumulative release fraction is the empirical CDF:

F_N(t) = (1/N) Σ_m 1[τ_m ≤ t].

For a uniform initial kernel inventory, this is the Monte-Carlo estimator of the released inventory fraction under the absorbing-boundary model.

## 13. Numerical approximations

The frozen mathematical contract separates the exact ideal ingredients from implementation approximations.

### Underlying mathematical ingredients

- homogeneous Brownian first-passage process in each interface-free sphere;
- isotropic exit location;
- continuous event time drawn from the exact sphere first-passage law;
- ideal stochastic interface rule for the chosen WOS construction.

### Implementation approximations

- finite first-passage inverse-CDF table and interpolation;
- finite capture distance;
- finite reinsertion distance;
- finite maximum step count;
- floating-point arithmetic;
- finite Monte-Carlo sample size.

These are numerical effects, not changes to the physical benchmark definition.

## 14. Conservation / inventory property

For the base pure-diffusion absorbing problem with no reaction:

dM/dt = -4πR² J_R.

Because the outer boundary is absorbing, all inventory that has reached the stopping boundary is counted as released.

Therefore:

released inventory(t) + inventory remaining inside(t) = initial inventory,

up to Monte-Carlo and implementation error.

This inventory balance is a primary verification property.

## 15. Error categories

The production benchmark has at least four distinct numerical-error categories:

1. Monte-Carlo sampling error;
2. first-passage sampler approximation;
3. finite interface-resolution error;
4. finite step-cap/truncation error.

Do not combine these into one undocumented 'numerical error'.

## 16. What this contract does NOT freeze

The following remain outside the base production verification benchmark:

- continuous kernel source after t=0;
- finite external mass-transfer resistance;
- radioactive decay;
- transmutation;
- trapping/release kinetics;
- species-specific partition coefficients other than K=1;
- interfacial resistance;
- time-varying diffusivity;
- non-zero coolant concentration;
- irradiated initial inventory.

## 17. Questions for supervisor

The repository does not establish the following as final physical design choices:

[QUESTION FOR SUPERVISOR] Is this initial-kernel release problem the intended production observable for the final research model, or only a verification benchmark?

[QUESTION FOR SUPERVISOR] Should the final production boundary remain absorbing, or should finite external mass-transfer resistance eventually be represented?

[QUESTION FOR SUPERVISOR] Should decay/transmutation remain a later extension or be part of the first physical production model?

[QUESTION FOR SUPERVISOR] Is K=1 intended at every final physical interface, or only for the verification benchmark?

## 18. Review-2 scope

Review 2 should audit this contract and the derivation in the companion numerical-formulation document against the actual WOS implementation.

Review 2 should not treat FTCS as the production method.

Review 2 should assess:

- whether the local WOS transition is mathematically represented correctly;
- whether interface treatment is consistent with the frozen K=1 continuum conditions;
- whether the absorbing boundary is mapped correctly;
- whether the initial-kernel ensemble estimator is correctly defined;
- whether each implementation approximation is visible and testable;
- whether conservation/inventory checks are sufficient for the proposed verification hierarchy.
