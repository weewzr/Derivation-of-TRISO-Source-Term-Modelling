# Independent Method-2 WOS Interface Verification Audit — Resolution

## Audit

Immutable review: `reviews/independent_method2_wos_interface_verification_audit.md`

Exact gate: **FAIL — REMEDIATION REQUIRED**

The review is accepted as the controlling Method-2 gate.

## Finding reconciliation

- **R2-WOS-01 — OPEN.** The N=2500 epsilon sequence does not demonstrate epsilon→0 convergence.
- **R2-WOS-02 — OPEN.** The controlled two-layer benchmark does not resolve five-layer censoring.
- **R2-WOS-03 — PARTIALLY RESOLVED.** Finite-epsilon transient WOS is broadly compatible with the independently refined FV reference, but convergence to the continuum limit remains unresolved.
- **R2-WOS-04 — RESOLVED.** Direct interface encounters, transmissions, reflections, crossing directions, reinsertion events and zero-dt events are now executed and reported.
- **R2-B01 — OPEN.** No successful five-layer production release-CDF verification exists.

New review findings:

- **R3-WOS-01 — OPEN / MAJOR.** Finite-epsilon convergence is not demonstrated.
- **R3-WOS-02 — OPEN / SCOPE LIMITATION.** The controlled adapter uses pinned production stochastic primitives but not the complete five-layer `step_multilayer` control path. No remediation is required for the interface-isolation experiment; the limitation remains explicit.
- **R3-WOS-03 — OPEN / MODERATE.** N=2500 lacks power to resolve a weak epsilon trend.
- **R3-WOS-04 — OPEN / MINOR INTERPRETIVE NOTE.** Near-saturated 16–32 s points are retained for completeness but must not dominate convergence interpretation.

## Higher-power remediation design

Repeat the exact frozen two-layer experiment without changing:

- geometry: a=50 um, R=100 um;
- D1=1e-10 m2/s, D2=1e-9 m2/s;
- initial inner-sphere inventory;
- K=1;
- alpha=2;
- epsilon sequence 200,100,50,25 nm;
- observation times 0.25,0.5,1,2,4,8,16,32 s;
- max-step policy.

The new sample size is selected from a precision target, not observed agreement.

Declare worst-case two-sided 95% binomial half-width <= 0.01:

[
1.96\sqrt{0.25/N}\le0.01.
]

This requires `N >= 9604`; choose `N=10000` histories per epsilon.

The scientific question remains whether increased statistical power resolves an epsilon-dependent approach to FV or instead supports an epsilon-independent plateau within a justified uncertainty bound.

No acceptance threshold is changed after observing the N=2500 results.

## Gate

Do not return to the five-layer ensemble and do not begin Method 3 before this remediation is executed and reconciled.
