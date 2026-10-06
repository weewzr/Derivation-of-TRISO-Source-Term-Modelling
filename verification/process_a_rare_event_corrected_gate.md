# Corrected Process-A Rare-Event Validation Gate

## Method

Stopping-time weighted splitting + unbiased Russian roulette around unchanged Process-A production propagation.

No event-index merging. No common-time interpolation. No Process-B/C shell substitution. No biased transition probabilities.

## Mathematical hard gate before implementation

1. Every branch occurs at an observable Process-A stopping event/state.
2. Split: b children, weight w/b.
3. Roulette: survival probability q fixed in advance for each eligible event class; survivor weight w/q.
4. Released trajectories absorbing.
5. Physical clock copied exactly at split/roulette event.
6. Between population-control events each child uses the unchanged pinned Process-A kernel.
7. Expected weighted contribution of arbitrary test observables verified by unit tests.
8. Weight is conserved in expectation; deterministic splitting conserves exactly. Roulette variance/weight excursions are reported, not hidden.

## Frozen supervisor

theodoreOnzGit/outram-park-backend @ 8d31482d127e211614ebb3f66b1076e1ed6dea98.
Supervisor is read-only.

## Level 0 synthetic resampling

Artificial stopping-event population with known observables.
At least 1,000,000 roulette trials across several q values.
Require empirical weighted mean within 5 estimated standard errors of exact expectation.
Split exact weight error <=1e-15.

## Level 1 homogeneous sphere

Use the same homogeneous analytical benchmark as the failed-WE research:
R=100 um, D=1e-8 m2/s, uniform-volume start, epsilon=100 nm, times .05,.1,.2,.4 s.

Direct Process A N=12,000.

Rare-event estimator: 12 independent replicates. Initial sampling uses the already-predeclared exact-volume stratification. Branch only on first outward crossing of radial milestone surfaces 0.4R,0.6R,0.8R; branch factor b=2 at each first crossing. Apply no roulette in Level 1 so the split proof is isolated.

Acceptance criteria remain the previous gate:
- zero weighted censoring;
- |RE-direct| <= max(.025,2*(direct Bernoulli SE + replicate SE));
- |RE-analytical| <= .04 at every time;
- no systematic signed discrepancy beyond uncertainty.

## Level 2 frozen two-layer benchmark

Geometry unchanged:
a=50 um, R=100 um, D1=1e-10, D2=1e-9 m2/s, K=1, alpha=2, epsilon=100 nm, uniform inner start, times .25,.5,1,2,4,8,16,32 s.
FV reference remains [0.00983747,0.06998542,0.23413553,0.49412611,0.76267512,0.94225401,0.99648779,0.99998700].
Direct Process A N=12,000.

Rare-event estimator: 12 independent replicates.

Predeclared stopping rules:
- first successful inner->outer interface transmission: split b=4;
- first attainment of outer-material radial fraction >=0.5: split b=2;
- first attainment >=0.8: split b=2.
No trajectory is merged with another.
To bound population, each lineage is eligible for each split milestone only once. No roulette is needed in Level 2 under this bounded branching design (maximum 16 descendants per initial lineage). This intentionally validates stopping-time splitting before adding roulette to a harder problem.

Initial N per replicate: 64 exact-volume-stratified inner starts.

Acceptance:
- zero weighted censoring;
- |RE-direct| <= max(.025,2*(direct Bernoulli SE + replicate SE)) every time;
- max |RE-FV| <=.025;
- interface transmission/reflection nonzero in both expected directions;
- no systematic signed discrepancy beyond uncertainty;
- exact split-weight conservation <=1e-14.

## Review boundary

If Level 0, Level 1 and Level 2 pass, STOP and request independent review of the stopping-time splitting bridge.

Do NOT execute five-layer Process A in the same pass.

## Future five-layer rule

Not authorized by this gate. A separate post-review gate must predeclare five-layer milestones, branching/roulette probabilities, replicate count, budget, censoring bounds and comparison times without tuning to Process C.

R2-WOS-02 OPEN.
R2-B01 OPEN.
Method 3 NOT STARTED.
