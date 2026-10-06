# Main Research Execution Prompt — Process-A Stopping-Time Splitting Validation

Continue from the CURRENT remote `main` of:

`weewzr/Derivation-of-TRISO-Source-Term-Modelling`

Recover the live repository before doing anything else.

The controlling scientific documents are:

- `verification/process_a_weighted_ensemble_continuous_time_research.md`
- `verification/process_a_rare_event_method_decision.md`
- `verification/process_a_rare_event_corrected_gate.md`
- `verification/process_a_weighted_ensemble_executed_evidence.md`

The method decision is already frozen:

**B. SWITCH RARE-EVENT METHOD**

Selected method:

**stopping-time weighted splitting with unbiased Russian roulette around unchanged Process-A propagation.**

This pass is the IMPLEMENTATION AND CONTROLLED EXECUTION of the already-predeclared corrected gate.

Do NOT redesign the method unless execution reveals a genuine implementation defect.
Do NOT change acceptance criteria after observing results.
Do NOT execute a five-layer rare-event calculation.
Do NOT begin Method 3.

## 1. Preserve the failed WE evidence

The previous event-index WE prototype remains immutable evidence:

- Level 1 PASS;
- Level 2 FAIL;
- not validated for transient Process-A release CDF.

Do not delete, rewrite, or reinterpret that failure.

The corrected method is a distinct rare-event estimator.

## 2. Freeze the corrected gate before implementation

Read `verification/process_a_rare_event_corrected_gate.md` in full.

Recover and explicitly restate the frozen:

- supervisor commit;
- Process-A dynamics;
- Level-0 tests;
- Level-1 geometry;
- Level-1 times;
- Level-1 direct sample size;
- Level-1 splitting milestones;
- Level-1 branch factors;
- Level-1 acceptance criteria;
- Level-2 geometry;
- Level-2 diffusivities;
- Level-2 interface parameters;
- Level-2 observation times;
- Level-2 FV reference;
- Level-2 direct sample size;
- Level-2 stopping milestones;
- Level-2 branch factors;
- Level-2 acceptance criteria;
- review boundary.

Do not modify these merely to improve the result.

If the live gate differs materially from this prompt, the live committed gate is authoritative unless it contains an obvious contradiction. Report any contradiction before execution.

## 3. Supervisor safety

Supervisor repository:

`theodoreOnzGit/outram-park-backend`

Pinned supervisor commit:

`8d31482d127e211614ebb3f66b1076e1ed6dea98`

Treat the supervisor repository as READ-ONLY.

Do not push to it.

Implement the rare-event controller entirely in the research repository while reusing/pinning the unchanged Process-A propagation primitives as already established by the project.

## 4. Mathematical implementation contract

Every propagated trajectory must remain a genuine Process-A trajectory between branching events.

Do NOT alter:

- WOS spatial transition law;
- first-passage-time distribution;
- physical clock;
- interface transmission probability;
- finite capture epsilon;
- reinsertion rule;
- absorbing release rule;
- initial physical distribution.

Population control may alter only the statistical representation through weights and independent future copies.

## 5. Stopping-time split operation

At an eligible predeclared Process-A stopping event, a parent trajectory with weight w split into b children must produce:

`w_child = w / b`

for every child.

All children inherit exactly the parent's:

- physical position/state;
- material/interface state;
- accumulated physical time;
- relevant history/milestone flags.

Only their future random streams become independent.

Require deterministic split-weight conservation to the gate tolerance.

## 6. Russian roulette implementation

Implement the general roulette primitive even if Level 1 and Level 2 do not require roulette.

For predeclared survival probability q:

- survive with probability q and assign weight w/q;
- terminate statistically with probability 1-q and assign zero continuing weight.

The roulette operation must never be confused with physical release or physical censoring.

Track roulette termination separately from Process-A physical outcomes.

Do not use roulette in Level 1 or Level 2 if the frozen gate says none is required.

## 7. Independent RNG streams

When splitting a trajectory, ensure descendants receive genuinely independent future random streams.

Do not clone RNG state identically across children.

Record the RNG/seeding strategy.

The physical state/time is cloned; future randomness is not.

## 8. Milestone eligibility

Each lineage may trigger only the milestone events allowed by the frozen gate.

Preserve explicit milestone flags in lineage state.

A trajectory must not repeatedly trigger the same "first attainment" split simply because it later recrosses the same radial/interface surface.

Test this explicitly.

## 9. Released trajectories

Released trajectories are absorbing.

Once physically released:

- their release time is frozen;
- their weight contributes to the weighted release estimator at all later observation times;
- they are never split again;
- they are never roulette-pruned as a live trajectory;
- they never re-enter the simulation.

## 10. Physical censoring

Maintain explicit distinction between:

- physically released;
- still live within budget;
- CensoredMaxSteps or equivalent physical-computation censoring;
- roulette-pruned statistical branches.

Do NOT classify roulette-pruned weight as physically censored.

Do NOT classify physical censoring as unreleased probability.

For Level 1 and Level 2, the frozen gate requires zero weighted physical censoring.

## 11. Weighted CDF estimator

For each observation time t compute the rare-event estimate of:

`F_A(t)=P_A(T_A<=t)`

from the sum of weights of physically released descendants whose release time is <= t, with the estimator constructed according to the derived stopping-time splitting argument.

Persist per-replicate estimates before computing the replicate mean.

Do not use ordinary Bernoulli SE for the weighted estimator.

## 12. Statistical uncertainty

Use the predeclared independent-replicate design.

The replicate is the statistical unit for the rare-event estimator.

Report:

- each replicate estimate at every observation time;
- replicate mean;
- replicate standard deviation;
- replicate standard error;
- interval/statistic used by the frozen gate.

For direct Process A, retain the appropriate direct Monte-Carlo uncertainty calculation.

## 13. Level 0 — synthetic resampling gate

Execute Level 0 FIRST.

Use artificial stopping-event populations with known exact weighted observables.

At minimum test:

- deterministic split conservation;
- several roulette q values;
- arbitrary nontrivial observables;
- at least 1,000,000 roulette trials as frozen in the gate;
- empirical weighted mean against exact expectation;
- RNG independence sanity checks;
- milestone one-shot eligibility logic.

Hard requirements include:

- split exact weight error <= 1e-15;
- roulette empirical mean within 5 estimated standard errors of exact expectation.

If Level 0 fails:

STOP.

Do not run Level 1.

Diagnose implementation versus mathematical-contract failure.

## 14. Level 1 — homogeneous absorbing sphere

Only if Level 0 passes, execute the frozen Level-1 benchmark.

Frozen model:

- R = 100 um;
- D = 1e-8 m2/s;
- uniform-volume start;
- epsilon = 100 nm;
- observation times = 0.05, 0.1, 0.2, 0.4 s;
- direct Process A N = 12,000;
- rare-event estimator = 12 independent replicates;
- exact-volume-stratified initialization as already predeclared.

Frozen splitting milestones:

- first outward crossing of 0.4R: split b=2;
- first outward crossing of 0.6R: split b=2;
- first outward crossing of 0.8R: split b=2.

No roulette in Level 1.

Compare:

1. direct Process A;
2. stopping-time splitting Process A;
3. analytical homogeneous-sphere reference.

Use the acceptance criteria exactly as committed in the gate.

If Level 1 fails:

STOP.

Do not run Level 2.

Do not tune milestones or branch factors.

## 15. Level 2 — frozen two-layer benchmark

Only if Level 1 passes, execute Level 2.

Frozen model:

- interface a = 50 um;
- outer R = 100 um;
- D1 = 1e-10 m2/s;
- D2 = 1e-9 m2/s;
- K = 1;
- alpha = 2;
- epsilon = 100 nm;
- uniform inner-sphere start;
- observation times = 0.25, 0.5, 1, 2, 4, 8, 16, 32 s;
- direct Process A N = 12,000;
- rare-event estimator = 12 independent replicates.

Frozen FV reference:

`[0.00983747, 0.06998542, 0.23413553, 0.49412611, 0.76267512, 0.94225401, 0.99648779, 0.99998700]`

Frozen splitting milestones:

1. first successful inner -> outer interface transmission: split b=4;
2. first attainment of outer-material radial fraction >=0.5: split b=2;
3. first attainment of outer-material radial fraction >=0.8: split b=2.

Each lineage is eligible for each milestone only once.

Maximum descendants per initial lineage must therefore remain bounded as predeclared.

No roulette in Level 2.

## 16. Level-2 acceptance

Apply the committed gate exactly.

Require:

1. zero weighted physical censoring;
2. rare-event versus direct agreement at every time according to the frozen uncertainty criterion;
3. max |RE-FV| <= 0.025;
4. nonzero expected interface transmission/reflection traffic;
5. no systematic signed rare-event/direct discrepancy beyond uncertainty;
6. exact split-weight conservation <= 1e-14.

Do not loosen these thresholds.

## 17. Direct Process-A control

The direct Process-A control is essential.

Confirm that the direct Level-2 result remains compatible with the already accepted FV benchmark.

If direct Process A itself unexpectedly fails materially against the established benchmark, do not blame the rare-event estimator automatically.

Diagnose the control first.

## 18. Weight accounting

Persist detailed weight accounting.

At appropriate checkpoints report:

- initial total weight;
- live total weight;
- released total weight;
- physically censored total weight;
- roulette-pruned expected/statistical accounting if roulette is exercised in Level 0;
- total represented weight.

For deterministic splitting Levels 1/2, unexplained weight loss/gain is a hard implementation failure.

## 19. Genealogy diagnostics

Record enough genealogy information to verify:

- number of splits by milestone;
- descendant counts;
- maximum lineage depth;
- maximum descendants from one initial parent;
- effective weight distribution;
- whether a tiny number of ancestors dominates the estimator.

These are diagnostics, not post-hoc acceptance criteria.

## 20. Cost evidence

Report:

- Process-A kernel calls;
- direct runtime;
- rare-event runtime;
- number of propagated descendants;
- splitting overhead;
- peak live population;
- memory where practical.

Compare with the predeclared cost expectations.

## 21. GitHub Actions workflow

Create a dedicated workflow for this corrected validation.

Use a clear name such as:

`Process-A stopping-time splitting validation`

The workflow should execute only the research-repository validation harness and pinned dependencies.

Persist machine-readable result artifacts.

Do NOT combine this with manuscript compilation.

## 22. Machine-readable evidence

Create a structured result file containing at minimum:

- provenance;
- commit SHA;
- pinned supervisor SHA;
- Level-0 results;
- Level-1 direct/reference/rare-event values;
- Level-1 uncertainties;
- Level-2 direct/FV/rare-event values;
- Level-2 uncertainties;
- milestone counts;
- weight errors;
- censoring;
- runtime/cost;
- final gate result.

Do not leave the only evidence inside workflow logs.

## 23. Human-readable executed evidence

Create:

`verification/process_a_stopping_time_splitting_executed_evidence.md`

or an equivalently explicit path.

Record the exact workflow run ID and artifact provenance.

Do not overwrite the earlier failed WE evidence.

## 24. Gate outcomes

After execution classify exactly:

A. LEVEL 0/1/2 PASS — BRIDGE READY FOR INDEPENDENT REVIEW

B. CONTROLLED VALIDATION FAIL — REMEDIATION OR METHOD-2 BOUNDARY DECISION REQUIRED

C. EXECUTION INCONCLUSIVE — SPECIFIC INFRASTRUCTURE/EVIDENCE ISSUE

Do not classify before execution.

## 25. If Level 2 passes

If and ONLY if Level 0, Level 1 and Level 2 all pass:

STOP.

Create a dedicated independent-review request for the stopping-time splitting bridge.

Do NOT execute five-layer Process A.

The independent reviewer must assess:

- target-law preservation;
- stopping-time unbiasedness;
- split implementation;
- RNG independence;
- milestone semantics;
- weighted CDF estimator;
- uncertainty treatment;
- Level-1 evidence;
- Level-2 evidence;
- whether five-layer use is scientifically authorized.

## 26. If Level 2 fails

If the corrected stopping-time method fails Level 2:

do NOT invent another immediate parameter-tuning cycle.

Reconcile the failure.

Determine whether it is:

- implementation defect;
- estimator-theory defect;
- milestone-design limitation;
- statistical-power limitation;
- direct-control failure;
- or evidence that Method 2 has reached a practical computational boundary.

Given that event-index WE has already failed, a second properly derived rare-event method failing the controlled two-layer benchmark should substantially increase the burden of proof for continued Method-2 estimator development.

Explicitly reassess whether to proceed to Method 3 while preserving open Process-A limitations.

## 27. R2-WOS-02

Remain OPEN throughout this pass.

Even a Level-2 PASS does not close it.

Five-layer evidence is required for reconsideration.

## 28. R2-B01

Remain OPEN throughout this pass.

A successful rare-event bridge must first pass independent review and then a separately predeclared five-layer gate.

## 29. Method 3

Remain NOT STARTED during execution.

Do not begin Method 3 in this pass.

## 30. Manuscript

Do not perform full manuscript editing.

Only update scientific status/traceability required to preserve executed evidence.

## End-of-pass report

Report:

1. recovered main tip;
2. frozen corrected gate confirmed;
3. implementation files;
4. workflow file;
5. pinned supervisor SHA;
6. Level-0 result;
7. split conservation result;
8. roulette result;
9. RNG independence result;
10. Level-1 direct values;
11. Level-1 rare-event values;
12. Level-1 analytical values;
13. Level-1 uncertainty comparison;
14. Level-1 gate;
15. Level-2 direct values;
16. Level-2 rare-event values;
17. Level-2 FV values;
18. Level-2 uncertainty comparison;
19. Level-2 gate;
20. interface-event diagnostics;
21. milestone split counts;
22. physical censoring;
23. weight-accounting result;
24. genealogy diagnostics;
25. runtime/cost;
26. machine-readable evidence path;
27. human-readable evidence path;
28. workflow run ID;
29. artifact ID/digest;
30. final classification A/B/C;
31. R2-WOS-02 status;
32. R2-B01 status;
33. Method-3 status;
34. whether independent review is warranted;
35. whether five-layer execution is authorized;
36. resulting commits;
37. single highest-value next action.

STOP after this controlled validation/reconciliation.

DO NOT execute five-layer Process A in this pass.
