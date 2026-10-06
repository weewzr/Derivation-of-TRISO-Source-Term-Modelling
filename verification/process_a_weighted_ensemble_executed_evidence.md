# Process-A Weighted-Ensemble Controlled Validation — Executed Evidence

## Provenance

Primary scientific run: **37439263013**.
Research commit: **2605cd815d860b36d835d07632f5c94705691ac1**.
Pinned supervisor: **8d31482d127e211614ebb3f66b1076e1ed6dea98**.
Artifact: `process-a-weighted-ensemble-controlled-results`, ID **11400413307**, digest `sha256:9fea9da43f781a2d79121283f2315367180abe8c680e7503549f8dec710debcf`.

Earlier run 37437547382 failed before scientific validation because the merge-invariance unit-test inequality was accidentally reversed. Run 37438375724 then reached Level 1; one Level-1 point failed, motivating the predeclared stratified-initialization remediation. No scientific acceptance criterion was changed.

## Resampling invariant

Merge test:
- exact weighted observable: 2.3;
- Monte-Carlo merge estimate: 2.29674;
- estimated SE: 0.0070781;
- split weight error: 0.

PASS.

## Level 1 — homogeneous sphere

- direct N=12,000;
- WE R=12 independent replicates;
- q=20 Process-A kernel calls;
- target 8/bin;
- stratified exact-mass initialization;
- maximum weight-conservation error: 7.772e-16;
- weighted censoring: 0.

All four predeclared times PASS the original gate.

| t s | direct | WE | analytical | |WE-direct| | |WE-ref| |
|---:|---:|---:|---:|---:|---:|
| .05 | .60408333 | .61441667 | .60693976 | .01033333 | .00747691 |
| .1 | .76491667 | .77025716 | .77047874 | .00534049 | .00022158 |
| .2 | .91291667 | .89678906 | .91549557 | .01612760 | .01870650 |
| .4 | .98875000 | .97975586 | .98826923 | .00899414 | .00851337 |

**Level 1: PASS.**

## Level 2 — frozen controlled two-layer benchmark

- direct N=12,000;
- WE R=12;
- q=20;
- target 8/bin;
- stratified exact-mass initialization;
- maximum weight-conservation error: 1.044e-14;
- weighted censoring: 0;
- interface events occur with nonzero transmission/reflection and both directions in direct and WE.

| t s | direct | WE | FV | |WE-direct| | |WE-FV| | gate |
|---:|---:|---:|---:|---:|---:|---|
| .25 | .00991667 | .03509307 | .00983747 | .02517640 | .02525560 | FAIL |
| .5 | .06833333 | .12907107 | .06998542 | .06073774 | .05908565 | FAIL |
| 1 | .23933333 | .34639808 | .23413553 | .10706474 | .11226255 | FAIL |
| 2 | .49666667 | .51508442 | .49412611 | .01841775 | .02095831 | PASS |
| 4 | .76283333 | .71402117 | .76267512 | .04881217 | .04865395 | FAIL |
| 8 | .94200000 | .91888753 | .94225401 | .02311247 | .02336648 | PASS |
| 16 | .99658333 | .98376352 | .99648779 | .01281981 | .01272427 | PASS |
| 32 | 1.0 | .99999705 | .99998700 | .00000295 | .00001005 | PASS |

**Level 2: FAIL.**

## Diagnosis and scope

The failure is not caused by lost weight, weighted censoring, absent interface traffic, or direct-Process-A mismatch with FV. The direct N=12,000 curve remains close to the accepted FV benchmark.

The current controller resamples after a fixed number of WOS transition-kernel calls. Because WOS calls carry random first-passage time increments, trajectories being merged can have different accumulated physical times. The original derivation treated physical time as part of the augmented state and argued that event-index path-space resampling is conditionally unbiased. However, the primary WE literature used for this project explicitly formulates standard WE resampling among trajectories at the same physical time / fixed physical lag. Therefore the present implementation is **not sufficiently justified as a standard continuous-time WE wrapper for the release-time CDF**, and the Level-2 failure prevents empirical validation of the proposed bridge.

It would be inappropriate to tune spatial/time bins or acceptance thresholds against the known FV/Process-C benchmark until the synchronization issue is resolved mathematically.

## Gate

- Level 1: PASS.
- Level 2: FAIL.
- Five-layer WE: **NOT AUTHORIZED**.
- Independent closure review requested: **NO**, because the prompt requires Levels 1 and 2 to pass first.
- R2-WOS-02: OPEN.
- R2-B01: OPEN.
- Method 3: NOT STARTED.

## Required next method-development question

Can Process-A WOS be embedded in a WE/resampling scheme with a rigorously common physical-time resampling surface **without changing the Process-A release-time law**?

Because a WOS hop samples an entire sphere first-passage event, pausing at an arbitrary physical time would require additional conditional within-sphere information not supplied by the existing Process-A primitive. That bridge must be derived before another WE validation run.
