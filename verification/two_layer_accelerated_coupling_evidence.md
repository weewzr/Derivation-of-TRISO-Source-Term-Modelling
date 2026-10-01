# Accelerated Two-Layer Shell/Interface Coupling — Reconciliation Evidence

## Canonical run

- Run: `36746018907`
- Conclusion: success
- Research commit: `8e3f295815a74462c1e3cffcdbb8907e8904f68c`
- Supervisor commit: `8d31482d127e211614ebb3f66b1076e1ed6dea98`
- Artifact: `two-layer-accelerated-coupling-results`
- Artifact ID: `11112224532`
- Artifact SHA-256: `08dfed747323fb4ac782e98c55cf42093061332958c5fb98c9e4a257c5d3a5d6`

N=10,000; epsilon=100 nm; alpha=2; K=1; a=50 um; R=100 um; D1=1e-10; D2=1e-9 m2/s.

## Rapid-run provenance

### 36745817586 — superseded implementation

Commit `93795db...`.

This run used an incorrect inner-region acceleration: it treated a local sphere of radius A-r as if it were the centered inner-ball exit. Its release CDF was grossly too fast:
[.0291,.1797,.5023,.8387,.9809,.9993,1,1].

This is a real implementation defect and its numerical evidence is rejected for validation.

### 36745856548 — corrected implementation

Commit `ddddf50d...`.

The inner region was corrected to the exact centered-ball first-exit kernel derived from the backward equation. Result:
[.0091,.0752,.2390,.4971,.7621,.9447,.9958,1].

### 36746018907 — canonical corrected rerun

Commit `8e3f295...`.

Workflow-only trigger touch; scientific harness unchanged from `ddddf50d...`. It reproduces the corrected run bit-for-bit, including:
- released 10,000;
- censored 0;
- interface events 1,380,397;
- transmitted 236,280;
- reflected 1,144,117;
- renewal events 1,390,397;
- mean renewals/history 139.040;
- identical CDF.

Therefore the corrected result is reproducible and canonical.

## Mathematical mapping

Each accelerated history consists of:

1. exact centered-ball first-exit side/time in the inner material;
2. interface encounter at r=a;
3. production `does_transmit(D_current,D_next,K)`;
4. zero physical time for interface resolution, matching production semantics;
5. reinsertion at alpha*epsilon on the selected side;
6. exact homogeneous shell first-exit side/time in the outer material;
7. another production interface resolution if the inner shell boundary is reached, or release if R is reached;
8. renewal until release.

The acceleration marginalises only within-region diffusion excursions. It does not add physical time at an interface and does not replace the verified transmission/reflection law.

## Shell-kernel prerequisite chain

The kernel evidence chain is complete:

exact shell mathematics
→ exit-side and conditional-moment verification (run 36741901064)
→ conditional CDF verification (run 36743542042)
→ corrected two-layer coupling.

## Accelerated vs FV

| t s | F_ACCEL | F_FV | Delta |
|---:|---:|---:|---:|
| .25 | .0091 | .00983747 | -.00073747 |
| .5 | .0752 | .06998542 | +.00521458 |
| 1 | .2390 | .23413553 | +.00486447 |
| 2 | .4971 | .49412611 | +.00297389 |
| 4 | .7621 | .76267512 | -.00057512 |
| 8 | .9447 | .94225401 | +.00244599 |
| 16 | .9958 | .99648779 | -.00068779 |
| 32 | 1.0000 | .999986996 | +.00001300 |

RMS error = 0.002895.
Maximum absolute error = 0.005215.

At N=10,000, accelerated MC SE ranges from ~0.00065 to 0.0050 away from saturation. The largest FV discrepancy (at .5 s) is ~1.98 accelerated-sample SE. No coherent signed bias is present across time.

## Accelerated vs accepted direct WOS

The independently accepted direct-WOS conclusion is a finite-epsilon plateau over 25–200 nm, not a privileged single epsilon and not an asymptotic epsilon→0 claim.

F_ACCEL is statistically compatible with the executed N=10,000 direct-WOS 25–200 nm plateau at the declared precision. Literal empirical min-max containment is not exact at t=0.5 s: F_ACCEL=0.0752 versus observed direct-WOS maximum 0.0749; the 0.0003 excess is negligible relative to the relevant MC/finite-epsilon scale.

For the same 100-nm direct-WOS realization:
F_direct=[.0089,.0692,.2329,.5005,.7619,.9439,.9966,1.0].

Accelerated-minus-direct:
[+.0002,+.0060,+.0061,-.0034,+.0002,+.0008,-.0008,0].

RMS difference = 0.003281; maximum absolute difference = 0.0061.

These differences are compatible with the combined Monte-Carlo and accepted finite-epsilon scale. No systematic release-time bias is detected at the declared controlled-benchmark precision.

## Computational acceleration

The direct high-power 100-nm WOS run required mean 1,207.2 production steps/history.

The corrected accelerated coupling requires mean 139.040 renewal events/history.

Event-count reduction = approximately 8.68x.

This is the meaningful algorithmic acceleration metric; GitHub wall time is not used because checkout/compilation dominates.

## Classification

**A. VERIFIED FOR CONTROLLED TWO-LAYER USE.**

Evidence:
- exact shell mathematics derived;
- exit-side/moment verification passed;
- conditional CDF verification passed;
- real coupling defect exposed and corrected;
- corrected result reproduced bit-for-bit;
- accelerated CDF agrees with FV at MC scale;
- accelerated CDF is statistically compatible with the independently accepted direct-WOS plateau at the declared precision (with the documented 0.0003 empirical-band excess at t=0.5 s);
- zero censoring;
- substantial event-count reduction.

This classification does NOT authorize five-layer use without independent audit.

## Findings

- R2-WOS-02: OPEN (five-layer direct-production censoring).
- R2-B01: OPEN.
- accelerated-estimator independent review: WARRANTED before five-layer use.
