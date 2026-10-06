# WE Level-1 initialization-variance remediation

## Trigger

Run 37438375724 reached Level 1 and failed one predeclared comparison at t=0.1 s.

Executed values:
- direct Process A: 0.76491667, SE 0.00387104;
- analytical sphere: 0.77047874;
- WE mean: 0.81093750, independent-replicate SE 0.01635827;
- |WE-direct|=0.04602083 versus predeclared tolerance 0.04045861;
- |WE-reference|=0.04045876 versus fixed 0.04 reference tolerance.

Other three Level-1 times passed. Weight conservation max error was 8.882e-16 and weighted censoring was zero. Merge-invariance unit test passed. Level 2 was not executed.

## Diagnosis

The WE replicate began from only 40 ordinary iid initial positions. Subsequent splitting cannot recover initial-distribution information absent from those 40 parents. The observed replicate SE is correspondingly much larger than direct-MC SE.

This is consistent with the mathematical WE variance decomposition: initial-condition variance is a distinct contribution, and WE may use an initial population different from the later allocated population. It is not evidence that acceptance thresholds should be loosened.

## Predeclared remediation

Keep every scientific acceptance criterion unchanged.

Change only WE initialization from ordinary iid sampling to unbiased stratified sampling of the SAME uniform-volume initial law:

### Level 1
Five predeclared radial WE bins. Sample 8 independent initial points conditionally and uniformly in volume inside each radial bin. Each point receives weight P(bin)/8, where P([r0,r1])=(r1^3-r0^3)/R^3. Total initial N remains 40.

### Level 2
The initial law is uniform within the inner sphere. Four inner-material radial bins are initially occupied. Preserve the predeclared initial N=64 by sampling 16 independent conditional points per inner bin, each with weight P(bin)/16. Outer bins begin empty, as required by the physical initial condition. Later resampling target remains 8 per occupied bin.

This is stratification/importance allocation of the known initial distribution, not a change to Process A. It is unbiased because conditional samples within each disjoint stratum are weighted by the exact probability mass of that stratum.

No changes:
- q=20;
- target=8;
- R=12 independent replicates;
- direct N=12,000;
- physical models;
- epsilon;
- Process-A propagation;
- comparison thresholds;
- censoring rules;
- five-layer authorization boundary.

## Gate

Rerun Level 1. Only if it passes the original gate may Level 2 execute.
