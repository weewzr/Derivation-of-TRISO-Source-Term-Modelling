# Controlled Two-Layer Finite-Capture Reconciliation Gate

## Independent-review basis

Audit: `reviews/independent_interface_state_markov_renewal_audit.md`
Gate: **C. REMEDIATION REQUIRED BEFORE IMPLEMENTATION**.

Purpose: reconcile finite-capture production WOS (A), exact-interface accelerated renewal (B), and a deterministic matrix-transform reduction of B (C). No hard five-layer matrix is authorized.

## Frozen benchmark

- inner interface a=50 um;
- outer absorbing radius R=100 um;
- D1=1e-10 m2/s;
- D2=1e-9 m2/s;
- K=1;
- alpha=2;
- epsilon=100 nm;
- initial inventory uniform in inner-sphere volume;
- no continuing source;
- absorbing outer boundary.

## Process definitions

### A — finite-capture production WOS

Ordinary WOS hops occur until distance to an interface <= epsilon. The interface event is resolved at that capture-region location with zero interface-event time; production transmission/reflection is applied; reinsertion is alpha*epsilon on the selected side. Births initially within epsilon of the inner interface may therefore enter interface resolution without first reaching the physical interface.

### B — accelerated exact-interface renewal

Exact centered-ball/shell kernels propagate to the physical interface radius and accumulate the corresponding physical first-passage time. At the exact interface, the same production transmission/reflection probability is applied with zero event time, followed by alpha*epsilon reinsertion.

### C — matrix reduction of B

A finite-state first-step transform system uses the same exact-interface region transforms and interface probabilities as B. C is intended to be algebraically equivalent to B, not to A.

A differs from B/C only in finite-capture stopping surface/time semantics.

## Initial capture-region mass

For uniform-volume inner births,

P(a-epsilon <= r <= a)=1-(1-epsilon/a)^3.

At a=50 um and epsilon=0.1 um this is approximately 0.005988008, or 0.5988%.

This contribution is explicitly retained in A/B comparison and is not assumed zero.

## Controlled two-layer matrix

Transient post-interface states:

S0 = inner-material side of interface, reinsertion radius a-delta.
S1 = outer-material side of interface, reinsertion radius a+delta.
A = released at R.

Let H(s) be the exact centered-ball exit transform from a-delta to a in D1.
Let G^-(s),G^+(s) be the exact outer-shell joint transforms from a+delta to inner interface / absorbing R in D2.

With p10 = D1/(D1+D2), p11=D2/(D1+D2), p01=D2/(D1+D2), p00=D1/(D1+D2):

Phi0 = H [p00 Phi0 + p01 Phi1]
Phi1 = G^- [p10 Phi0 + p11 Phi1] + G^+.

Thus

K(s)=[[H p00, H p01],
      [G^- p10, G^- p11]],

B(s)=[0,G^+]^T,

Phi=(I-K)^-1 B.

The initial transform integrates the uniform-volume birth distribution:
Phi_init(s)= integral_0^a 3r^2/a^3 H(r,s)[p00 Phi0+p01 Phi1] dr.

## Predeclared transform comparison

Evaluate s in s^-1:

[0, 0.25, 0.5, 1, 2, 4].

For B, estimate E[e^-sT] by N=20,000 explicit accelerated histories, fixed before results. Report MC SE of the bounded transform variable.

For C, evaluate deterministically.

Primary B/C metrics:
- signed transform discrepancy;
- discrepancy / MC SE;
- probability normalization at s=0;
- first release-time moment where numerically stable.

## Numerical criteria

- C normalization: |Phi_init(0)-1| < 1e-10.
- linear-system relative residual ||(I-K)Phi-B||_2 / max(||B||_2,1) < 1e-10 at every s.
- report 2-norm condition number of I-K at every s; no post-hoc pass threshold is imposed.
- B/C deterministic-vs-MC agreement is assessed against the predeclared MC uncertainty; do not invent a tolerance after execution.

## A versus B finite-capture discrepancy

Use the independently accepted N=10,000 direct finite-capture WOS evidence at epsilon=100 nm and the canonical explicit exact-interface accelerated evidence at the same benchmark for CDF-level reconciliation.

Already executed CDFs:
A direct 100-nm WOS = [.0089,.0692,.2329,.5005,.7619,.9439,.9966,1.0].
B exact-interface accelerated = [.0091,.0752,.2390,.4971,.7621,.9447,.9958,1.0].

The reconciliation will report signed, RMS and max discrepancies and their relation to existing MC precision. These data are not re-run merely for completeness.

## R3-M03 absorption argument

At s=0, all interface probabilities are strictly between zero and one because D1,D2>0. S0 has positive probability to transmit to S1; S1 has positive probability G^+(0)>0 to release at R. Hence every transient state has a finite positive-probability path to A and there is no closed transient class. For the finite substochastic K(0), rho(K(0))<1 and Phi(0)=1.

The implementation must verify normalization numerically.

## Conditioning

Record cond_2(I-K(s)) and residuals at every predeclared s. This controlled study does not extrapolate conditioning conclusions to the hard five-layer matrix.

## Gate

If C agrees with B within predeclared numerical/MC precision, matrix implementation is verified for the controlled exact-interface process.

A versus B quantifies compatibility only; it cannot establish identity.

Do not implement or execute the hard five-layer matrix in this pass.
