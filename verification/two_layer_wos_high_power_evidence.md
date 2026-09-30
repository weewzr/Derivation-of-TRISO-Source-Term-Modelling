# Two-Layer WOS High-Power Epsilon Remediation — Executed Evidence

## Provenance

- Workflow: `Two-layer WOS high-power epsilon verification`
- Run: `36730522197`
- Conclusion: success
- Research commit: `47f1638d94c89ecb350180a063a0b11d63166ef6`
- Supervisor commit: `8d31482d127e211614ebb3f66b1076e1ed6dea98`
- Artifact: `two-layer-wos-high-power-results`
- Artifact ID: `11105626485`
- Artifact SHA-256: `11c266e360b0a5d7a49e8c533898a81a491b0807e63cc014f9f33baeeda05fa6`
- N = 10,000 per epsilon
- predeclared worst-case 95% half-width target <= 0.01
- epsilon = 200,100,50,25 nm
- alpha=2; K=1
- geometry, diffusivities and observation times unchanged from the frozen benchmark.

## Execution

All 40,000 histories released with zero censoring.

Mean steps/history:
- 200 nm: 623.9
- 100 nm: 1207.2
- 50 nm: 2485.8
- 25 nm: 4834.0

Direct interface encounters:
- 200 nm: 934,102
- 100 nm: 1,822,890
- 50 nm: 3,780,360
- 25 nm: 7,372,585

The expected approximately inverse-epsilon computational scaling remains visible.

## High-power CDF comparison

Resolved FV:
`[0.00983747,0.06998542,0.23413553,0.49412611,0.76267512,0.94225401,0.99648779,0.99998700]`

WOS:

| t s | FV | 200 nm | 100 nm | 50 nm | 25 nm |
|---:|---:|---:|---:|---:|---:|
| .25 | .00984 | .0107 | .0089 | .0084 | .0100 |
| .5 | .06999 | .0734 | .0692 | .0710 | .0749 |
| 1 | .23414 | .2337 | .2329 | .2312 | .2423 |
| 2 | .49413 | .4947 | .5005 | .4931 | .5029 |
| 4 | .76268 | .7606 | .7619 | .7597 | .7661 |
| 8 | .94225 | .9435 | .9439 | .9402 | .9435 |
| 16 | .99649 | .9949 | .9966 | .9960 | .9964 |
| 32 | .99999 | 1.0000 | 1.0000 | 1.0000 | .9999 |

Maximum absolute |Delta F|:
- 200 nm: 0.00341
- 100 nm: 0.00637
- 50 nm: 0.00298
- 25 nm: 0.00877

RMS |Delta F|:
- 200 nm: 0.00163
- 100 nm: 0.00242
- 50 nm: 0.00181
- 25 nm: 0.00476

Across-epsilon CDF spread at each time:
- .25 s: .0023
- .5 s: .0057
- 1 s: .0111
- 2 s: .0098
- 4 s: .0064
- 8 s: .0037
- 16 s: .0017
- 32 s: .0001

## Interpretation

The increased-power experiment still does not show a monotone epsilon-dependent approach to FV. Signed discrepancies fluctuate with epsilon.

However, all four epsilon curves occupy a narrow band around the independently resolved FV reference. The largest single |Delta F| across all 32 comparisons is 0.00877, below the predeclared worst-case 95% precision target of 0.01. The maximum across-epsilon spread is 0.0111 at t=1 s, while individual MC standard errors there are about 0.0042.

This is evidence for an approximately epsilon-independent finite-epsilon plateau over 25–200 nm at the declared precision, rather than evidence for a resolved monotone convergence rate.

The independent audit explicitly allowed R3-WOS-01 closure through either:
1. statistically resolved epsilon-dependent convergence toward FV; or
2. a robust epsilon-independent plateau with an independently justified error bound.

The present run is therefore a candidate for the second closure path, but that closure must be independently reviewed rather than self-certified.

## Finding state before independent review

- R3-WOS-01: OPEN, candidate plateau closure evidence available.
- R3-WOS-03: remediation executed; statistical-power deficiency materially reduced.
- R3-WOS-02: scope limitation unchanged.
- R2-WOS-03: partially resolved; candidate stronger closure evidence available.
- R2-WOS-04: resolved.
- R2-WOS-02: open.
- R2-B01: open.

## Gate

Independent review is warranted before any reinsertion-factor sensitivity or return to the five-layer ensemble.
