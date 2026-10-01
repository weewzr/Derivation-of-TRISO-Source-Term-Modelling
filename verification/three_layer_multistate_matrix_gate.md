# Controlled Three-Layer Multistate Matrix Verification Gate

## Authorization

Independent closure audit:
`reviews/independent_interface_state_markov_renewal_closure_audit.md`

Gate:
**B. REMEDIATION SUBSTANTIALLY CLOSED WITH NON-BLOCKING FINDINGS — CLEARED FOR CONTROLLED MULTISTATE MATRIX IMPLEMENTATION/VERIFICATION.**

This benchmark is deliberately NOT the hard five-layer Cs-137 case.

## Frozen controlled benchmark

Three concentric materials:

- R1 = 50 um;
- R2 = 75 um;
- R3 = 100 um absorbing outer radius.

Diffusivities:

- D1 = 1e-9 m2/s;
- D2 = 2e-9 m2/s;
- D3 = 5e-9 m2/s.

Rationale: all D are distinct; adjacent ratios 2 and 2.5 give nontrivial transmission/reflection without the pathological rare-event hierarchy of the frozen TRISO case; layer widths remain comparable and release occurs on practical controlled-verification timescales.

Other parameters:

- K=1;
- epsilon=100 nm;
- alpha=2;
- initial distribution uniform in volume of inner sphere;
- exact-interface Process B/C semantics;
- absorbing R3.

Process A finite-capture production WOS is optional supplementary evidence only and is not the primary exact test.

## Minimal state space

Four transient post-interface states:

S0 = material 1 side of I0=R1, r=R1-delta.
S1 = material 2 side of I0, r=R1+delta.
S2 = material 2 side of I1=R2, r=R2-delta.
S3 = material 3 side of I1, r=R2+delta.

A = absorbing release at R3.

## Nonzero transition table

| Source | Region exit | Interface | Outcome | Destination | factor |
|---|---|---|---|---|---|
| S0 | outer ball | I0 | reflect M1 | S0 | H1 p11 |
| S0 | outer ball | I0 | transmit M2 | S1 | H1 p12 |
| S1 | inner shell | I0 | transmit M1 | S0 | G2^- p21 |
| S1 | inner shell | I0 | reflect M2 | S1 | G2^- p22(I0) |
| S1 | outer shell | I1 | reflect M2 | S2 | G2^+ p22(I1) |
| S1 | outer shell | I1 | transmit M3 | S3 | G2^+ p23 |
| S2 | inner shell | I0 | transmit M1 | S0 | G2^- p21 |
| S2 | inner shell | I0 | reflect M2 | S1 | G2^- p22(I0) |
| S2 | outer shell | I1 | reflect M2 | S2 | G2^+ p22(I1) |
| S2 | outer shell | I1 | transmit M3 | S3 | G2^+ p23 |
| S3 | inner shell | I1 | transmit M2 | S2 | G3^- p32 |
| S3 | inner shell | I1 | reflect M3 | S3 | G3^- p33 |
| S3 | outer shell | R3 | absorb | A | G3^+ |

All transforms are evaluated at the deterministic reinsertion radius of the source state.

## Predeclared transform points

s = [0, 1, 2, 5, 10, 20] s^-1.

## Explicit Process-B sample

N=20,000 histories.

MC precision is reported from the actual bounded transform variable exp(-sT). No result-dependent N selection.

## Observation times

t = [0.02,0.05,0.10,0.20,0.40,0.80] s.

These are frozen before execution for explicit-renewal/FV CDF comparison.

## Matrix numerical checks

Normalization:
- max_i |Phi_i(0)-1| < 1e-10;
- |Phi_init(0)-1| < 1e-10.

Linear-system relative residual:
<1e-10 at every s.

Record kappa_F^est(I-K(s)) = ||I-K||_F ||(I-K)^-1||_F at every s. No post-hoc condition-number pass threshold.

At s=0 audit:
- each shell exit probability sums to one;
- each interface outcome sums to one;
- row transient probability + direct absorption = one.

Primary B/C transform metric:
- signed difference;
- MC SE;
- z-score;
- maximum |z| reported without inventing a post-hoc threshold.

## FV reference

Use the same conservative spherical FV face-resistance formulation already independently validated.

Controlled reference requirements:

Spatial:
- aligned meshes with 50, 100, 200 cells per 25-um layer.

Temporal:
- explicit Euler with dt selected from the existing spherical FV stability bound;
- fine reference dt and one halved-dt check on the finest mesh.

Initial condition:
- normalized uniform concentration in material 1, zero outside.

Interfaces:
- K=1 concentration/flux continuity through the verified face-resistance treatment.

Outer boundary:
- absorbing c(R3,t)=0.

Evidence:
- F_FV(t) at all predeclared times;
- spatial refinement differences;
- temporal refinement difference;
- inventory conservation residual.

The fine FV reference should have discretization change materially smaller than the N=20,000 explicit-renewal MC uncertainty at the observation times before being used for comparison.

## Distribution comparison

Primary matrix validation is transform-space B versus C.

Independently compare explicit B CDF against resolved FV at the predeclared times using:
- signed difference;
- RMS difference;
- maximum absolute difference;
- B binomial SE.

No inverse Laplace is introduced in this stage.

## Gate

Success requires:
- B/C transform compatibility at MC scale;
- normalization criterion passes;
- residual criterion passes;
- conditioning understood/reported;
- probability topology audit passes;
- FV refinement/conservation adequate;
- explicit-B/FV CDF compatible at the controlled precision with no unresolved systematic topology defect.

Even success does NOT authorize the hard five-layer matrix without independent review.
