# 22 — Review-2 Numerical Formulation from the Frozen WOS Contract

## 1. Target continuous problem

The production verification benchmark is an initial-value problem, not a continuously forced source problem.

In layer i:

∂c_i/∂t = (1/r²) ∂/∂r [r² D_i ∂c_i/∂r].

Initial condition:

c_1(r,0)=c_0 in the fuel kernel, and c_i(r,0)=0 outside it.

Outer boundary:

c_5(R,t)=0.

At each internal interface:

c_i(r_i,t)=c_{i+1}(r_i,t),

-D_i c_i'(r_i,t) = -D_{i+1} c_{i+1}'(r_i,t).

## 2. Stochastic state

For history m, define:

X_n^(m) = walker position after event n,

T_n^(m) = accumulated simulated time after event n.

The numerical state transition is:

(X_n,T_n) -> (X_{n+1},T_{n+1}).

## 3. Spatial WOS transition

Let ρ_n be the minimum distance from X_n to the nearest material interface or the absorbing outer surface.

Construct the local ball:

B_n = {x : ||x-X_n|| < ρ_n}.

An ordinary WOS hop remains in one homogeneous material, so its diffusivity is the current layer value D_i.

The exit point is:

X_{n+1}=X_n+ρ_n N_n,

where N_n is a uniformly distributed unit vector.

## 4. First-passage time

Define:

θ_n=D_i τ_n/ρ_n².

The exact centre-start mean is:

E[τ_n]=ρ_n²/(6D_i).

The implementation samples θ_n from the spherical first-passage distribution and reconstructs:

τ_n=θ_n ρ_n²/D_i.

Then:

T_{n+1}=T_n+τ_n.

There is no globally imposed Δt.

## 4A. Diffusivity treatment

The production property lookup is not intrinsically a fixed constant. The supervisor code evaluates the Jiang diffusivity correlation using the layer material, nuclide, region temperature, and gamma-neutron fluence.

Thus the underlying material model may be written schematically as:

D_i = D_i(T_i, Phi, nuclide).

For the frozen Review-2 benchmark, T_i, Phi, and nuclide are held fixed, so each resulting D_i is a constant during the benchmark.

This fixed-property snapshot is what makes the local first-passage scaling tau proportional to rho squared divided by D_i and keeps the benchmark mathematically time-homogeneous.

[INFERRED FROM CODE] functional dependence on material, nuclide, temperature and fluence.

[ASSUMPTION] benchmark holds those inputs fixed.
## 5. Why this represents the local diffusion operator

The local ball contains no material interface, so D_i is constant in it.

The local PDE is therefore the homogeneous diffusion equation:

∂u/∂t=D_i∇²u.

For a stationary source-free problem, the corresponding first-exit expectation is harmonic:

D_i∇²u=0.

This explains why WOS can replace a spatial finite-difference stencil with boundary-first-passage sampling.

The local correspondence does not, by itself, prove the full multilayer forward concentration equation.

## 6. Interface event

When:

ρ_n≤ε_capture,

the implementation resolves an interface event rather than taking an ordinary WOS hop.

For the frozen benchmark K=1:

p_{i→j}=D_j/(D_i+D_j).

Reflection has probability:

p_{i→i}=1-p_{i→j}.

Afterwards the walker is reinserted at:

δ=α ε_capture.

The implementation assigns zero physical time to this interface/reinsertion event.

## 7. Outer boundary

When the nearest stopping surface is the OPyC outer radius R, the history terminates as released.

The continuum condition represented by this benchmark is:

c(R,t)=0.

This is different from the Robin condition retained in the general continuum model and FTCS benchmark.

## 8. Initial population

For a uniform initial kernel concentration, the radial birth distribution must follow volume.

The probability of being within radius x is:

P(r≤x)=x³/r_1³.

Let U be uniform on [0,1).

Then:

r=r_1 U^(1/3).

An isotropic direction completes the three-dimensional initial point.

Thus the current release-fraction path represents an initially loaded kernel.

## 9. Observable

For history m define release time τ_m.

The release indicator by time t is:

I_m(t)=1 if τ_m≤t, otherwise 0.

For N histories:

F_hat_N(t)=(1/N) sum_m I_m(t).

This is the cumulative released fraction of the initial represented population.

The complementary remaining fraction is:

M_hat_N(t)=1-F_hat_N(t).

Therefore:

F_hat_N(t)+M_hat_N(t)=1.

This identity is exact for the estimator.

## 10. Statistical uncertainty

At fixed t each release indicator is Bernoulli-valued.

For independent histories with release probability F(t):

Var(F_hat_N)=F(t)[1-F(t)]/N.

Using the measured fraction gives the usual empirical estimate:

SE ≈ sqrt(F_hat_N[1-F_hat_N]/N).

This is Monte-Carlo sampling uncertainty, not discretisation error.

## 11. Numerical approximations

First-passage inversion:

θ_hat=F_hat^{-1}(U),

where the production code uses a finite table and interpolation rather than an analytic inverse.

Interface capture:

ρ≤ε_capture,

replaces the limiting interface approach with a finite tolerance.

Reinsertion:

δ=α ε_capture,

moves the walker away from the interface without advancing physical time.

Step limit:

a history may stop because max_steps is reached before release.

Floating-point arithmetic and finite N add further numerical error.

## 12. Conservation properties

Because the frozen production benchmark has no reaction and no continuous source, the initial inventory can only leave through the absorbing outer boundary.

Therefore the continuum balance is:

remaining inventory + released inventory = initial inventory.

The history estimator enforces the analogous identity:

F_hat_N(t)+M_hat_N(t)=1.

Release fraction must also be monotone:

t_2>t_1 implies F_hat_N(t_2)≥F_hat_N(t_1).

These checks are necessary but not sufficient for PDE verification.

## 13. Limiting cases

Homogeneous limit:

D_1=D_2=D_3=D_4=D_5.

Internal interfaces then have no physical diffusivity contrast.

Equal-D interface probability:

p_{i→j}=1/2.

Strong barrier limit for D_j much smaller than D_i:

p_{i→j}≈D_j/D_i.

These are diagnostic limits, not complete verification.

## 14. Implementation mapping

Geometry and region selection → constructive_solid_geometry/mod.rs.

Layer diffusivity selection → TrisoCell::try_get_diffusion_coefficient.

Hop radius → walk_on_spheres.rs: nearest_interface_distance and shell_bounds.

First-passage sampling → sphere_fpt.rs: sample_first_passage_time.

Exit direction → sphere_fpt.rs: sample_uniform_direction.

Interface decision → interface.rs: does_transmit.

Capture and reinsertion → walk_on_spheres.rs: step_multilayer.

Absorbing OPyC release → walk_on_spheres.rs: step_multilayer and walk_until_released.

Uniform kernel birth distribution → walk_on_spheres.rs: sample_uniform_in_ball.

Release ensemble → the repository's release-fraction ensemble driver.

## 15. What this formulation establishes

It establishes a complete mathematical description of the current production WOS verification problem and its numerical approximations.

It does not yet prove that the complete multilayer stochastic process converges to the forward piecewise diffusion equation.

It also does not prove that finite capture, reinsertion, inverse-CDF interpolation, or max_steps are negligible.

## 16. Review-2 readiness

The numerical formulation is now mature enough for an independent Review-2 audit because:

- the continuous target is frozen;
- the stochastic state is defined;
- the local WOS transition is defined;
- interface and outer-boundary events are defined;
- the initial distribution is defined;
- the observable is defined;
- numerical approximations are separately identified;
- conservation and monotonicity checks are explicit;
- implementation locations are mapped.

The next audit should test mathematical fidelity to the supervisor implementation and identify any missing stochastic condition before production-code changes.