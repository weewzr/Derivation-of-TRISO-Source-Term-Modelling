# Process-A Weighted-Ensemble Controlled Validation Gate

## Frozen upstream

Supervisor repository: theodoreOnzGit/outram-park-backend
Pinned commit: 8d31482d127e211614ebb3f66b1076e1ed6dea98
Supervisor source is read-only.

## Common WE controls

- resampling every q=20 unchanged Process-A kernel calls;
- target M=8 trajectories per occupied live bin;
- independent WE replicates R=12;
- released trajectories absorbing;
- per-lineage cap 1,000,000 Process-A kernel calls;
- weight-conservation tolerance 5e-13 absolute per resampling event;
- replicate-level Student-t uncertainty; no Bernoulli SE for WE.

## Resampling unit tests

Hard PASS:
- split weights sum exactly to parent weight within 1e-15;
- probabilistic merge expected arbitrary test observable agrees with pre-merge weighted observable to Monte-Carlo tolerance 5 standard errors over >=100,000 artificial merges;
- deterministic weight conservation <=5e-13.

## Level 1 — homogeneous absorbing sphere

- R=100 um; D=1e-8 m2/s; uniform volume initial distribution;
- capture epsilon=100 nm, matching the existing analytical semantic benchmark scale;
- observation times 0.05,0.1,0.2,0.4 s;
- direct Process A N=12,000;
- WE: 12 independent replicates, initial N=40 equal-weight trajectories, 5 radial bins, target M=8/bin;
- q=20; cap=1,000,000 calls/lineage.

Acceptance:
1. zero weighted censoring;
2. weight conservation hard PASS;
3. for every time, |WE-direct| <= max(0.025, 2*(direct binomial SE + WE replicate SE));
4. both direct and WE absolute error versus analytical sphere reference <=0.04 (same conservative reference tolerance used by existing project semantic test);
5. no systematic one-signed WE-direct discrepancy exceeding uncertainty at all times.

## Level 2 — existing controlled two-layer benchmark

Frozen geometry/reference:
- interface a=50 um; outer R=100 um;
- D1=1e-10, D2=1e-9 m2/s;
- K=1; alpha=2;
- epsilon=100 nm (one predeclared representative point from accepted 25–200 nm plateau);
- initial uniform inner sphere;
- times 0.25,0.5,1,2,4,8,16,32 s;
- refined FV reference [0.00983747,0.06998542,0.23413553,0.49412611,0.76267512,0.94225401,0.99648779,0.99998700].

Direct Process A: N=12,000.
WE: 12 independent replicates, initial N=64, 8 radial/material bins, target M=8/bin, q=20.

Acceptance:
1. zero weighted censoring;
2. weight conservation hard PASS;
3. |WE-direct| <= max(0.025,2*(direct binomial SE + WE replicate SE)) at every time;
4. max |WE-FV| <=0.025;
5. interface transmission/reflection counters nonzero in both directions where physically expected;
6. no systematic WE-direct signed discrepancy beyond uncertainty.

## Cost declaration

Existing Rust diagnostics achieve ~8.3e6 Process-A steps/s in five-layer instrumentation; controlled two-layer costs are much lower (accepted 10k histories at epsilon=100 nm averaged ~1207 steps/history).

Expected Level 1/2 work is O(10^7–10^8) kernel calls, comfortably bounded to minutes on CI. Memory O(number of live WE trajectories), <10^3 states/replicate.

## Five-layer authorization

Even if Levels 1 and 2 pass: **STOP and request independent review**.

Five-layer WE remains NOT AUTHORIZED until the bridge derivation, implementation and controlled results are independently reviewed.

R2-WOS-02 OPEN.
R2-B01 OPEN.
Method 3 NOT STARTED.
