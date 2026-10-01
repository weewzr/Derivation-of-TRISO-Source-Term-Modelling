# Hard Five-Layer 8x8 Matrix Diagnostic Gate

## Authorization

Independent audit: `reviews/independent_three_layer_multistate_matrix_audit.md`.

Authorized stage: **BOUNDED HARD FIVE-LAYER MATRIX DIAGNOSTIC** only.

No inverse Laplace, release CDF, production release claim, R2-B01 closure, R2-WOS-02 closure, or Method 3 is authorized.

## Frozen Process-B/C benchmark

Cs-137 exact-interface accelerated process / matrix reduction.

Radii m:
R=[2.125e-4,3.125e-4,3.525e-4,3.875e-4,4.275e-4].

Diffusivities m2/s:
D=[1.2502982636347968e-13,1.0e-8,4.062299125614697e-14,9.227773168241615e-17,4.062299125614697e-14].

K=1.
epsilon=100 nm.
alpha=2; delta=200 nm.
Uniform-in-volume kernel birth.
Absorbing OPyC exterior.

Process A finite-capture production WOS remains distinct from Process B/C.

## Eight transient states

S0 Kernel side I0.
S1 Buffer side I0.
S2 Buffer side I1.
S3 IPyC side I1.
S4 IPyC side I2.
S5 SiC side I2.
S6 SiC side I3.
S7 OPyC side I3.
A=Released.

Nonzero topology is exactly `verification/five_layer_interface_state_transition_table.md`.

## Interface probabilities

For current material i and neighbor j:
p(i->j)=Dj/(Di+Dj), reflection=Di/(Di+Dj).

Both directions at I0..I3 will be printed at full scientific precision. No tiny probability is rounded to zero.

## Predeclared real-s grid

s=[0,1e-9,1e-8,1e-7,1e-6,1e-5,1e-4] s^-1.

Heuristic rationale only:
s~1/t indicates which physical-time scales influence the Laplace transform. SiC has an L^2/D scale ~1.33e7 s, motivating 1e-8–1e-7 s^-1. Kernel/IPyC/OPyC scales motivate higher points. This is not an exact time mapping.

## Stable transform evaluation

For shell ratios sinh(x)/sinh(y), 0<=x<=y:
- small y: use direct sinh ratio / limiting ratio;
- moderate y: direct sinh ratio;
- large y: use exp(x-y)*(1-exp(-2x))/(1-exp(-2y)).

For centered-ball z/sinh(z):
- small z: stable direct/series behavior;
- large z: 2 z exp(-z)/(1-exp(-2z)).

No unprincipled clamping to zero/one.

The diagnostic includes small/moderate/large test arguments and finite/bound checks.

## Matrix diagnostics

At every s:
- build K and B mechanically from the reviewed transition table;
- solve (I-K)Phi=B;
- report all Phi_i and Phi_init;
- report relative residual;
- report Frobenius condition estimate kappa_F^est=||A||F||A^-1||F;
- report an estimated smallest singular value using an internal symmetric-eigen/Jacobi calculation of A^T A;
- report spectral radius rho(K) at s=0 using power iteration/eigen diagnostic;
- check finite values and 0<=Phi<=1 within tolerance;
- check monotonicity across ordered s.

## Numerical gates

Normalization at s=0:
max_i |Phi_i-1| < 1e-9;
|Phi_init-1| < 1e-9.

Relative residual:
||A Phi-B||2 / max(||B||2,1) < 1e-10.

Spectral radius:
rho(K(0)) < 1, reported with enough digits to show the gap.

Bounds tolerance:
[-1e-10,1+1e-10].

Monotonicity tolerance:
Phi(s_next) <= Phi(s_prev)+1e-10.

No condition-number pass threshold is invented. If conditioning materially compromises the gates, stop and diagnose.

## Probability topology audit

At s=0:
- every shell G-+G+=1 within 1e-12;
- every interface transmit+reflect=1 within machine precision;
- each K row plus direct absorption probability sums to one within 1e-12;
- no missing/duplicate transitions.

## Initial-radius quadrature

Midpoint quadrature resolutions:
Nq=[1,000,10,000,100,000].

Report Phi_init at every s and max difference 1k->10k and 10k->100k.
Quadrature convergence is diagnostic; no single arbitrary resolution is trusted.

## Execution boundary

This workflow outputs transform diagnostics only.
It MUST NOT perform inverse Laplace or construct F(t).
