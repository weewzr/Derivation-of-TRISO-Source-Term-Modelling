# Two-Layer WOS Capture-Epsilon Smoke — Executed Evidence

## Role

Execution/feasibility diagnostic only. This is **not** transient WOS↔FV validation and does not close R2-WOS-01, R2-WOS-03, or R2-B01.

## Provenance

- Workflow: `Two-layer WOS epsilon smoke`
- Run ID: `36694765968`
- Conclusion: success
- Event: push
- Research commit: `89b27c54779e40e5ce37b8e3b4137a5a02dee05a`
- Supervisor repository: `theodoreOnzGit/outram-park-backend`
- Supervisor commit: `8d31482d127e211614ebb3f66b1076e1ed6dea98`
- Rust: `rustc 1.98.1 (48a229cea 2026-09-01)`
- Artifact: `two-layer-wos-smoke-results`
- Artifact ID: `11088295276`
- Artifact SHA-256: `f169f295fbee39d908d7222bfabab81d2d83f38f0b26d3495a5a8415a296bb04`

## Frozen problem

- interface radius: 50 um
- outer absorbing radius: 100 um
- D1 = 1e-10 m2/s
- D2 = 1e-9 m2/s
- K = 1
- alpha = 2
- epsilon sequence = 200, 100, 50, 25 nm
- N = 32 per epsilon
- max steps/history = 1,000,000
- initial distribution = uniform inner sphere

## Results

| epsilon (nm) | released | censored | mean steps | max steps | interface encounters | transmitted | reflected | inner→outer | outer→inner | runtime (s) |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 200 | 32 | 0 | 589.2 | 2,695 | 2,838 | 470 | 2,336 | 251 | 219 | 0.006 |
| 100 | 32 | 0 | 1,085.8 | 3,457 | 5,049 | 886 | 4,131 | 459 | 427 | 0.003 |
| 50 | 32 | 0 | 3,253.1 | 13,962 | 15,881 | 2,696 | 13,153 | 1,364 | 1,332 | 0.009 |
| 25 | 32 | 0 | 4,125.2 | 17,188 | 20,143 | 3,240 | 16,871 | 1,636 | 1,604 | 0.012 |

Every reported interface encounter is a zero-simulated-time event, consistent with the frozen finite-capture interface semantics.

Release-time ranges (s):

- 200 nm: 0.2040 to 8.6914, mean 2.4133
- 100 nm: 0.1644 to 22.8136, mean 3.6898
- 50 nm: 0.3090 to 9.2290, mean 3.4498
- 25 nm: 0.3000 to 8.4692, mean 2.7723

With N=32 these release-time differences are not interpreted as an epsilon-convergence trend.

## Gate assessment

The smoke prerequisites for the predeclared N=400 pilot are satisfied:

1. all four epsilon cases executed;
2. censoring is zero and therefore understood for this controlled benchmark at the smoke scale;
3. runtime is comfortably feasible;
4. direct interface encounter/transmit/reflect/crossing accounting is present;
5. no max-step or semantic pathology is evident.

R2-WOS-04 has its requested direct event classification implemented and executed in this controlled adapter, but formal finding closure is deferred until the transient comparison evidence is reconciled.

R2-WOS-01 remains OPEN.
R2-WOS-02 remains OPEN (five-layer finding).
R2-WOS-03 remains OPEN.
R2-B01 remains OPEN.
