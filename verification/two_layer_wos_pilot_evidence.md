# Two-Layer WOS Epsilon Pilot — Executed Evidence

## Role

Predeclared N=400/epsilon pilot for runtime, censoring, variance and preliminary epsilon behaviour. This is not the production verification ensemble.

## Provenance

- Run: `36723835539`
- Conclusion: success
- Research commit: `c087fe9c85c916af5742a6adfcef3b799d2f8f08`
- Supervisor commit: `8d31482d127e211614ebb3f66b1076e1ed6dea98`
- Artifact: `two-layer-wos-pilot-results`
- Artifact ID: `11101013635`
- Artifact SHA-256: `d2cc3f04cbe6e0a69bca748b43cf7ecce663175ee67095c12645b3d0b71a3c16`
- N = 400 per epsilon
- epsilon = 200, 100, 50, 25 nm
- alpha = 2; K = 1
- max steps/history = 1,000,000

## Execution gate

All four cases released 400/400 histories with zero censoring.

| eps nm | mean steps | max steps | interface events | transmit | reflect | runtime s |
|---:|---:|---:|---:|---:|---:|---:|
| 200 | 585.3 | 2,770 | 34,884 | 6,020 | 28,464 | 0.024 |
| 100 | 1,160.7 | 9,274 | 69,804 | 11,852 | 57,552 | 0.041 |
| 50 | 2,662.1 | 25,581 | 162,716 | 26,928 | 135,388 | 0.093 |
| 25 | 4,590.6 | 32,831 | 279,900 | 46,844 | 232,656 | 0.159 |

The controlled problem remains computationally feasible across the predeclared epsilon sequence.

## CDF comparison

Finest deterministic FV reference (80 cells/layer, q=0.2):

| t s | F_FV |
|---:|---:|
| 0.25 | 0.00983747 |
| 0.5 | 0.06998542 |
| 1 | 0.23413553 |
| 2 | 0.49412611 |
| 4 | 0.76267512 |
| 8 | 0.94225401 |
| 16 | 0.99648779 |
| 32 | 0.99998700 |

WOS pilot:

| t s | eps 200 | eps 100 | eps 50 | eps 25 |
|---:|---:|---:|---:|---:|
| 0.25 | .0175 | .0150 | .0050 | .0025 |
| 0.5 | .0700 | .0725 | .0725 | .0600 |
| 1 | .2650 | .2750 | .2200 | .2475 |
| 2 | .4975 | .5050 | .4750 | .5025 |
| 4 | .7450 | .7675 | .7750 | .7625 |
| 8 | .9625 | .9425 | .9525 | .9400 |
| 16 | 1.0000 | .9950 | .9950 | 1.0000 |
| 32 | 1.0000 | 1.0000 | 1.0000 | 1.0000 |

Maximum absolute pilot CDF difference from FV:

- 200 nm: 0.03086
- 100 nm: 0.04086
- 50 nm: 0.01913
- 25 nm: 0.01336

RMS CDF difference:

- 200 nm: 0.01482
- 100 nm: 0.01519
- 50 nm: 0.01033
- 25 nm: 0.00724

These pilot metrics are suggestive of smaller discrepancies at the finer epsilons but are not sufficient to claim epsilon convergence because N=400 Monte-Carlo half-width is approximately 0.049 near F=0.5.

## Statistical gate

The predeclared production target remains N=2500 per epsilon, derived before observing agreement from worst-case 95% normal half-width <=0.02.

Production reporting should use Wilson binomial intervals in addition to SE. The pilot's normal interval degenerates at empirical F=0 or 1 and must not be interpreted as zero uncertainty.

## Finding state

- R2-WOS-01: OPEN; pilot does not resolve systematic epsilon bias.
- R2-WOS-02: OPEN; five-layer censoring finding unaffected.
- R2-WOS-03: OPEN pending production-precision transient comparison.
- R2-WOS-04: direct event classification is implemented and repeatedly executed; formal closure can be considered at the next review reconciliation.
- R2-B01: OPEN.
