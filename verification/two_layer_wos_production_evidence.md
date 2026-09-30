# Two-Layer WOS Epsilon Production Verification — Executed Evidence

## Provenance

- Workflow: `Two-layer WOS epsilon production verification`
- Run: `36726533800`
- Conclusion: success
- Research commit: `3dd3fcfafb79e776055c67790e5cac2c189e4fd8`
- Supervisor commit: `8d31482d127e211614ebb3f66b1076e1ed6dea98`
- Artifact: `two-layer-wos-production-results`
- Artifact ID: `11103137350`
- Artifact SHA-256: `208be11cf6a7c8b350d0626eba90bd075b5d105c79a6612dc0df838775ea244a`

Frozen design: N=2500 per epsilon; eps=200,100,50,25 nm; alpha=2; K=1; cap=1,000,000; D1=1e-10 m2/s; D2=1e-9 m2/s; a=50 um; R=100 um.

## Execution

All 10,000 histories released. Censoring was zero at every epsilon.

| eps nm | mean steps | max steps | interface encounters | transmitted | reflected | inner→outer | outer→inner | runtime s |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 200 | 615.1 | 4,674 | 230,682 | 39,536 | 188,646 | 21,018 | 18,518 | 0.131 |
| 100 | 1,187.3 | 9,274 | 447,661 | 75,650 | 369,511 | 39,075 | 36,575 | 0.246 |
| 50 | 2,564.7 | 25,581 | 975,663 | 162,662 | 810,501 | 82,581 | 80,081 | 0.529 |
| 25 | 4,875.5 | 43,976 | 1,859,404 | 308,946 | 1,547,958 | 155,723 | 153,223 | 1.004 |

Direct interface classification therefore satisfies the diagnostic evidence requested by R2-WOS-04.

## Continuum comparison

Resolved FV reference:
F_FV=[0.00983747,0.06998542,0.23413553,0.49412611,0.76267512,0.94225401,0.99648779,0.99998700]
at t=[0.25,0.5,1,2,4,8,16,32] s.

Production WOS:

| t s | FV | 200 nm | 100 nm | 50 nm | 25 nm |
|---:|---:|---:|---:|---:|---:|
| .25 | .00984 | .0140 | .0068 | .0064 | .0068 |
| .5 | .06999 | .0776 | .0700 | .0700 | .0708 |
| 1 | .23414 | .2444 | .2284 | .2204 | .2400 |
| 2 | .49413 | .4984 | .5008 | .4776 | .5092 |
| 4 | .76268 | .7620 | .7616 | .7580 | .7696 |
| 8 | .94225 | .9524 | .9416 | .9356 | .9464 |
| 16 | .99649 | .9964 | .9964 | .9964 | .9960 |
| 32 | .99999 | 1.0000 | 1.0000 | 1.0000 | .9996 |

Delta F = F_WOS-F_FV:

| t s | 200 nm | 100 nm | 50 nm | 25 nm |
|---:|---:|---:|---:|---:|
| .25 | +.00416 | -.00304 | -.00344 | -.00304 |
| .5 | +.00761 | +.00001 | +.00001 | +.00081 |
| 1 | +.01026 | -.00574 | -.01374 | +.00586 |
| 2 | +.00427 | +.00667 | -.01653 | +.01507 |
| 4 | -.00068 | -.00108 | -.00468 | +.00692 |
| 8 | +.01015 | -.00065 | -.00665 | +.00415 |
| 16 | -.00009 | -.00009 | -.00009 | -.00049 |
| 32 | +.00001 | +.00001 | +.00001 | -.00039 |

Maximum absolute CDF difference:

- 200 nm: 0.01026
- 100 nm: 0.00667
- 50 nm: 0.01653
- 25 nm: 0.01507

RMS CDF difference:

- 200 nm: approximately 0.00598
- 100 nm: approximately 0.00334
- 50 nm: approximately 0.00803
- 25 nm: approximately 0.00659

## Statistical interpretation

The predeclared N=2500 gives worst-case MC SE about 0.01 and 95% half-width about 0.0196 near F=0.5. Wilson intervals were reported for every point.

The observed discrepancies are generally on the scale of Monte-Carlo uncertainty. However, the epsilon sequence does not show a consistent monotone approach to the FV CDF. At t=2 s, for example, the differences are +0.00427, +0.00667, -0.01653, +0.01507 as epsilon decreases.

Therefore the evidence supports:

- transient two-layer WOS results are broadly compatible in scale with the resolved continuum reference at the tested finite epsilons;
- zero censoring and direct interface semantics are demonstrated;
- a systematic finite-epsilon bias is not clearly detected at N=2500;
- but the required epsilon-convergence trend toward FV is NOT established.

It is not scientifically justified to claim that epsilon→0 convergence has been demonstrated.

## Finding reconciliation

- R2-WOS-01 MAJOR: **OPEN**. Required epsilon-refined convergence is not established.
- R2-WOS-02 MODERATE: **OPEN**. This remains the five-layer censoring/release-CDF finding.
- R2-WOS-03 MODERATE: transient continuum comparison has now been executed and is broadly compatible within MC scale, but because the finite-epsilon convergence question remains unresolved, retain OPEN pending independent review/reconciliation.
- R2-WOS-04 MINOR: direct event classification is implemented and executed for smoke, pilot and production. Evidence supports closure, subject to independent review.
- R2-B01: **OPEN**.

## Gate

Do not return automatically to the full five-layer ensemble.

The production two-layer benchmark has reached an independent-review milestone because it directly tests the major reviewer's requested transient interface experiment and produces a nontrivial result: broad continuum compatibility but no resolved epsilon-convergence trend.
