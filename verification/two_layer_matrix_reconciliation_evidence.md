# Controlled Two-Layer Finite-Capture / Exact-Interface / Matrix Reconciliation Evidence

## Provenance

- Run: `36807696742`
- Conclusion: success
- Research commit: `95ff1e99e6aa9df1567ba30a3a8072f770266d0a`
- Supervisor commit: `8d31482d127e211614ebb3f66b1076e1ed6dea98`
- Artifact: `two-layer-matrix-reconciliation-results`
- Artifact ID: `11137889673`
- Artifact SHA-256: `119357c19d4581f310aa9bff63a48b13dde8f80fab797de9472cc7ad2be69887`

## Process distinction

A = direct finite-capture production WOS.
B = explicit accelerated exact-interface renewal.
C = deterministic matrix reduction of B.

C is intended to be mathematically equivalent to B. A is compared empirically and is not claimed identical.

## Initial capture region

For uniform-volume inner births with a=50 um and epsilon=0.1 um:

P(a-epsilon <= r <= a)=0.005988008 = 0.5988008%.

This is a real finite-capture semantic difference.

## B versus C transform evidence

Predeclared s=[0,.25,.5,1,2,4] s^-1; explicit B N=20,000.

| s | C Phi_init | B MC mean | B SE | B-C | z=(B-C)/SE | cond2(I-K) | residual |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1.000000000000 | 1.000000000000 | 0 | ~1.45e-14 | 0 | 230.63 | 0 |
| .25 | .565271047887 | .565176155688 | .00172154 | -9.49e-5 | -0.055 | 195.04 | 1.12e-16 |
| .5 | .378686243232 | .378694937890 | .00182016 | +8.69e-6 | +.0048 | 173.39 | 3.47e-18 |
| 1 | .209451869140 | .209666100098 | .00153950 | +2.14e-4 | +.139 | 146.08 | 3.47e-18 |
| 2 | .090790236649 | .091358859363 | .00102223 | +5.69e-4 | +.556 | 116.44 | 2.79e-17 |
| 4 | .028729861774 | .029244528324 | .00051285 | +5.15e-4 | +1.004 | 88.75 | 4.34e-19 |

All B/C transform discrepancies are <=1.004 MC standard errors. No systematic implementation mismatch is detected.

Predeclared normalization criterion |Phi_init(0)-1|<1e-10: PASS.

Predeclared residual criterion <1e-10: PASS by many orders of magnitude.

Conditioning is explicitly quantified. No post-hoc condition-number threshold is applied.

## Absorption / spectral-radius evidence

The analytical finite-state argument is now paired with the executed Phi(0)=1 unit check.

Because D1,D2>0, interface transmission probabilities are positive; S0 has a finite positive-probability path to S1 and S1 has positive probability to exit through R. There is no closed nonabsorbing class. Therefore rho(K(0))<1. The numerical normalization check returns Phi0=Phi1=Phi_init=1 to floating-point precision.

## A versus B finite-capture discrepancy

Previously executed N=10,000 CDFs at epsilon=100 nm:

A finite-capture direct WOS:
[.0089,.0692,.2329,.5005,.7619,.9439,.9966,1.0]

B exact-interface accelerated:
[.0091,.0752,.2390,.4971,.7621,.9447,.9958,1.0]

B-A:
[+.0002,+.0060,+.0061,-.0034,+.0002,+.0008,-.0008,0]

RMS = 0.003281.
Maximum absolute discrepancy = 0.0061.

The discrepancy changes sign over time and is compatible with the previously accepted controlled MC/finite-epsilon precision. This is evidence of controlled compatibility, not identity.

## Physical-time semantic difference

A stops a region excursion on entering the epsilon capture region; B/C stop at the exact material interface. Hence A's interface-resolution stopping time is generally earlier than B/C's. Initial births in the 0.5988% capture-region mass can resolve immediately in A but receive a positive exact-interface FPT in B/C.

The present evidence quantifies the net release-CDF consequence rather than asserting this time difference is zero.

## Finding evidence

- R3-M01: candidate remediation evidence. Exact identity claim removed; A/B discrepancy quantified.
- R3-M02: candidate remediation evidence. Three equivalence levels documented and controlled reconciliation executed.
- R3-M03: candidate closure evidence. Formal absorption argument + executed Phi(0)=1.
- R3-M04: candidate closure evidence for controlled matrix. cond2 and residuals executed at all predeclared s.
- R3-m01: clarified as algebraic sanity check only.
- R3-m02: eight-state transition table persisted.
- R2-WOS-02: OPEN.
- R2-B01: OPEN.

## Gate

The 2x2 matrix is verified as a controlled representation of Process B. The finite-capture A versus exact-interface B distinction is small at the established two-layer benchmark precision but remains epistemically distinct.

Do not execute the hard five-layer matrix until independent review reconciles R3-M01 through R3-M04.
