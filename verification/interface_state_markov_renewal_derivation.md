# Reduced Five-Layer Interface-State Markov-Renewal Derivation

## Status

DERIVATION / DESIGN ONLY. No five-layer release calculation is performed here.

## 1. Why a second reduction is justified

Executed run 36801138769 analytically marginalised all homogeneous ball/shell wandering but still exhausted 100,000 renewals/history before SiC entry. Hence the remaining explicit stochastic work is repeated interface encounter -> transmit/reflect -> reinsert -> shell exit.

Because the benchmark is concentric, radial, time-homogeneous and K=1 with fixed D, the post-interface reinsertion radius is deterministic given interface and selected side. Thus a finite post-interface state is sufficient.

## 2. Minimal transient state space

Let interfaces I0=Kernel/Buffer, I1=Buffer/IPyC, I2=IPyC/SiC, I3=SiC/OPyC.

Define post-interface states by the material side into which the walker is reinserted:

S0 = Kernel side of I0.
S1 = Buffer side of I0.
S2 = Buffer side of I1.
S3 = IPyC side of I1.
S4 = IPyC side of I2.
S5 = SiC side of I2.
S6 = SiC side of I3.
S7 = OPyC side of I3.

A = Released at outer OPyC radius (absorbing).

The initial kernel birth distribution is a separate initial state/source J rather than a recurrent interface state.

There are eight transient post-interface states plus one absorbing release state.

## 3. Region first-passage transforms

For a shell material m bounded by radii a_m,b_m and start radius r, define

G_m^-(r,s) = E[e^{-sT} 1{exit inner}],
G_m^+(r,s) = E[e^{-sT} 1{exit outer}].

The exact derived formulas are

G_m^+(r,s)=(b_m/r) sinh(lambda_m(r-a_m))/sinh(lambda_m(b_m-a_m)),

G_m^-(r,s)=(a_m/r) sinh(lambda_m(b_m-r))/sinh(lambda_m(b_m-a_m)),

lambda_m=sqrt(s/D_m).

For the kernel centered ball, define H_K(r,s)=E[e^{-sT_K}], the exact centered-ball exit transform.

Every post-interface state has a fixed reinsertion radius r_i = interface radius +/- delta, delta=alpha epsilon.

## 4. Interface probabilities

At interface Ik separating material l and r,

p_{l->r}=D_r/(D_l+D_r),
p_{l->l}=D_l/(D_l+D_r),

and conversely

p_{r->l}=D_l/(D_r+D_l),
p_{r->r}=D_r/(D_r+D_l).

The interface decision has zero physical time.

## 5. Transition construction

A state does not jump directly merely because it is adjacent to an interface. It first undergoes a region first-passage event to one of that material's bounding interfaces. The appropriate G^-/G^+ transform weights both exit probability and physical time. At the reached interface, the zero-time transmission/reflection probability maps to a post-interface reinsertion state.

Examples:

From S1 (Buffer side of I0, r=R0+delta):
- hit I0 first with transform G_B^-(r,s); then transmit to Kernel -> S0 or reflect -> S1;
- hit I1 first with transform G_B^+(r,s); then transmit to IPyC -> S3 or reflect -> S2.

Thus
K_{1,0}=G_B^- p_{B->K},
K_{1,1}=G_B^- p_{B->B},
K_{1,3}=G_B^+ p_{B->I},
K_{1,2}=G_B^+ p_{B->B at I1}.

From S2 (Buffer side of I1, r=R1-delta), the same four destinations occur but G_B^-/G_B^+ are evaluated at its different fixed radius.

From S3/S4 in IPyC, exits map through I1 or I2 to Buffer/IPyC or IPyC/SiC post-interface states.

From S5/S6 in SiC, exits map through I2 or I3.

From S7 in OPyC:
- inner exit I3 maps to S6 or S7 after interface resolution;
- outer exit contributes directly to absorbing release.

From S0 in Kernel, centered-ball exit reaches I0 and then maps to S0 on reflection or S1 on transmission.

## 6. Matrix renewal equation

Let Phi_i(s)=E_i[e^{-s T_release}] for i=0..7.

For each transient state,

Phi_i(s) = sum_j K_ij(s) Phi_j(s) + B_i(s),

where K_ij contains exact probability-weighted physical first-passage transforms followed by zero-time interface probabilities.

Only OPyC states have a direct release term. For S7,

B_7(s)=G_OPyC^+(r_7,s).

All other B_i=0.

Therefore in vector form

Phi(s)=K(s)Phi(s)+B(s),

so, whenever I-K(s) is nonsingular,

Phi(s)=[I-K(s)]^{-1} B(s).

This exactly sums arbitrarily many repeated reflection/return cycles in transform space instead of sampling them one by one.

## 7. Initial kernel distribution

For a uniform-in-volume initial kernel birth,

f_R(r)=3r^2/R0^3, 0<=r<=R0.

The initial release-time transform is

Phi_init(s)= integral_0^R0 [3r^2/R0^3] H_K(r,s)
             [p_{K->K} Phi_0(s)+p_{K->B} Phi_1(s)] dr.

The kernel first-passage physical time is therefore retained; births are not collapsed to the interface.

## 8. Probability normalization

At s=0, every G^-+G^+=1 and each interface transmit+reflect=1.

For a finite absorbing system whose release is reached with probability one, the solution must satisfy Phi_i(0)=1 and Phi_init(0)=1.

This is a mandatory analytical/unit verification.

## 9. Connection to the original process and PDE

The forward continuum process is radial diffusion

partial_t c = r^-2 partial_r[r^2 D(r) partial_r c]

with K=1 concentration continuity, flux continuity and absorbing outer boundary.

The shell transforms G are solutions of the corresponding backward generator equation D(u''+2u'/r)=s u inside each homogeneous material.

By the strong Markov property, stopping the diffusion at each interface first-passage time and restarting from the production reinsertion state gives the same finite-epsilon stochastic process as the verified shell/interface renewal implementation.

The matrix equation performs only algebraic marginalisation of repeated renewal cycles. It does not alter D, interface probabilities, reinsertion radius or physical first-passage time law.

Important limitation: this establishes equivalence to the frozen finite-epsilon stochastic renewal process, not automatically to the ideal epsilon->0 continuum PDE. The controlled two-layer audit supplies the existing finite-epsilon compatibility evidence.

## 10. Release-time recovery

Phi_init(s)=E[e^{-sT_release}] is the Laplace-Stieltjes transform of the release-time distribution.

For CDF F(t)=P(T_release<=t),

Laplace{F}(s)=Phi_init(s)/s.

Preferred next numerical route should be chosen only after verification. Candidate routes:
1. controlled numerical inverse Laplace transform of Phi_init(s)/s;
2. exact sampling from a reduced macro-renewal kernel derived from the matrix;
3. moment/quantile recovery as unit checks.

No inversion method is selected in this pass.

## 11. Numerical error sources

A future implementation would have distinct errors from:
- finite 2000-term approximation if stochastic shell sampling remains involved;
- numerical evaluation of hyperbolic transforms for extreme s/D scales;
- conditioning/linear solve of I-K(s);
- quadrature of the initial-radius integral;
- inverse-Laplace error if inversion is used;
- floating-point error.

The transform equations themselves introduce no Monte-Carlo error.

## 12. Validation hierarchy

Before frozen five-layer release use:

A. analytical two/three-state toy chain with closed-form geometric/renewal sum;
B. matrix-transform result vs explicit accelerated renewal simulation in a feasible controlled regime;
C. controlled two-layer FV comparison;
D. controlled three-layer or reduced-contrast multilayer comparison;
E. Phi(0)=1 probability normalization;
F. release-time moment/distribution verification;
G. independent review.

Only after these gates may the hard five-layer release transform/CDF be treated as a candidate Method-2 result.

## 13. Gate

This derivation is sufficiently concrete for a small analytical/unit verification, but it is a new reduced numerical representation. Do not execute the hard five-layer release calculation before the validation hierarchy and independent review gate are satisfied.
