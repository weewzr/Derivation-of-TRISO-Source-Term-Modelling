# Main Research Prompt — Process-A Weighted-Ensemble Bridge

Continue from CURRENT remote `main` of:

`weewzr/Derivation-of-TRISO-Source-Term-Modelling`

Recover the live repository first.

The controlling next-gate document is:

`verification/postreview_method2_gap_and_next_gate.md`

Its decision is:

**C. ADDITIONAL METHOD DEVELOPMENT REQUIRED.**

The selected candidate is a weighted-ensemble / splitting estimator operating on the UNCHANGED Process-A production dynamics.

This pass is a MATHEMATICAL / ALGORITHMIC DERIVATION AND CONTROLLED VALIDATION pass.

Do NOT jump directly to a five-layer weighted-ensemble production run.
Do NOT begin Method 3.

## 1. Preserve the project's first-principles standard

The required chain remains:

physical/stochastic principle
→ probability law
→ estimator derivation
→ unbiasedness / consistency argument
→ resampling algorithm
→ implementation
→ controlled verification
→ five-layer use.

Do NOT jump from "weighted ensemble is known to work" directly to code.

## 2. Research the method properly

Conduct targeted research on weighted-ensemble / splitting / rare-event Monte Carlo methods applicable to first-passage probabilities and first-passage-time distributions.

Prioritize:

1. original weighted-ensemble papers;
2. peer-reviewed mathematical/statistical analyses;
3. established rare-event Monte-Carlo literature;
4. authoritative implementations;
5. well-maintained scientific GitHub repositories as implementation references.

Research at minimum:

- weighted ensemble;
- splitting / cloning;
- merging / pruning;
- sequential Monte Carlo relationships where relevant;
- rare-event first-passage estimation;
- weighted absorbing trajectories;
- variance estimation;
- unbiasedness under resampling.

Do NOT rely primarily on generic tutorials.

Create:

`verification/process_a_weighted_ensemble_literature.md`

Record references, exact result each source supports, assumptions, relevance to Process A, and limitations.

## 3. Define the target without changing Process A

The target remains the ORIGINAL finite-capture production WOS Process A.

Define:

`T_A = Process-A release time.`

Target observable:

`F_A(t) = P_A(T_A <= t).`

Weighted ensemble must NOT alter the underlying transition kernel.

Between resampling events, every trajectory must propagate using the exact unchanged Process-A dynamics.

Explicitly distinguish trajectory dynamics from population resampling.

## 4. Define the weighted ensemble

At resampling generation n define weighted trajectories:

`{(X_n^(k), T_n^(k), w_n^(k))}_{k=1}^{N_n}`.

Require:

`w_n^(k) >= 0`

and:

`sum_k w_n^(k) = 1`

apart from explicitly separated absorbed/released weight bookkeeping.

Define precisely live weight, released weight, censored weight, and total weight.

Derive the conservation identity.

## 5. Derive the release-CDF estimator

For each requested physical time t derive the weighted estimator for:

`F_A(t)=P_A(T_A<=t)`.

Derive whether the appropriate estimator is accumulated released weight by t, or another mathematically equivalent weighted indicator.

Do not assume the estimator.

Derive `E[F_hat_WE(t)]` and state conditions under which:

`E[F_hat_WE(t)] = F_A(t)`.

## 6. Splitting derivation

Suppose one trajectory of weight w is replaced by m conditionally identical children.

Derive the child weights.

Show explicitly that for an arbitrary observable g:

expected weighted contribution before splitting
=
expected weighted contribution after splitting.

Do not hide this behind prose.

## 7. Merging / pruning derivation

Derive a valid merge/prune operation.

For example, if trajectories with weights w1,...,wm are merged, derive the probability used to select the surviving state and its resulting weight.

Show explicitly that the expected weighted contribution of an arbitrary observable is preserved.

This proof is essential.

## 8. Multiple resampling steps

Extend the single-resampling argument recursively.

Establish why repeated conditionally unbiased resampling does not change the target Process-A expectation.

Clearly state assumptions.

Distinguish unbiased finite-N estimator from asymptotic consistency if the literature requires that distinction.

## 9. Released trajectories

Released trajectories are absorbing.

Define what happens to their weights.

Do NOT allow released weight to be accidentally resampled back into the live population.

Derive:

released_weight(t)
+
live_weight(t)
+
other explicitly classified weight
=
1.

## 10. Censoring

The existing production problem has severe step censoring.

Therefore censoring cannot be an afterthought.

Define separately:

Released

Live

CensoredMaxSteps

or equivalent states.

Do NOT count censored weight as unreleased physical probability.

Derive what can and cannot be inferred about `F_A(t)` when nonzero weighted censoring remains.

Where possible derive rigorous lower/upper bounds.

## 11. Physical-time synchronization

Process A advances in random first-passage time increments.

Therefore trajectories do NOT naturally share a common event count or physical time.

Derive a valid resampling schedule.

Investigate alternatives such as:

- fixed physical-time resampling;
- milestone-triggered resampling;
- hybrid schemes.

Do NOT resample at equal WOS step count merely because it is easy unless it is mathematically justified for the target observable.

## 12. Progress coordinate

Design the progress coordinate using only information available in Process A.

Candidate:

`(layer, radial progress)`

but research alternatives.

The coordinate must influence ONLY population allocation/resampling.

It must NOT alter Process-A transition probabilities.

Explain why.

## 13. Bin design

Predeclare a candidate binning scheme.

It should resolve the known bottleneck:

Fuel
→ Buffer
→ IPyC
→ SiC
→ OPyC
→ Released.

Consider additional radial subdivision near difficult interfaces only if justified.

Do NOT tune bins after observing five-layer results without declaring a new experiment.

## 14. Target population per bin

Define how many weighted trajectories each occupied bin attempts to maintain.

Derive split rule, merge rule, and weight redistribution.

Preserve total probability weight exactly to numerical tolerance.

## 15. Variance / uncertainty

This is critical.

Weighted trajectories after resampling are genealogically correlated.

Therefore the ordinary independent Bernoulli standard error:

`sqrt(F(1-F)/N)`

is NOT automatically valid.

Research and derive an appropriate uncertainty estimator.

Possible approaches may involve:

- independent WE replicates;
- block/replicate variance;
- genealogical variance methods;
- other literature-supported estimators.

Prefer a robust method that can be independently verified.

## 16. Use independent replicates

Unless literature strongly supports a better approach, design the controlled verification around multiple independently seeded weighted-ensemble replicates.

The replicate should be the statistical unit for uncertainty.

Predeclare replicate count before the five-layer experiment.

## 17. Implementation safety

Do NOT modify the supervisor repository.

Implement the weighted-ensemble controller in the research repository around the pinned Process-A primitives/control path.

The underlying Process-A propagation routine must remain unchanged.

Record the exact upstream supervisor commit used.

## 18. Controlled validation ladder

Do NOT go directly to five layers.

The required ladder is:

LEVEL 1 — homogeneous absorbing sphere.

Compare:

direct Process A;
weighted Process A;
analytical reference.

LEVEL 2 — controlled two-layer benchmark.

Compare:

direct Process A;
weighted Process A;
refined FV reference;
existing accepted two-layer evidence.

LEVEL 3 — only after Levels 1 and 2 pass:

bounded five-layer weighted Process-A experiment.

## 19. Level-1 acceptance

Predeclare geometry, diffusivity, observation times, direct sample size, WE bins, WE population, replicates, resampling schedule, and uncertainty calculation.

Require weighted and direct Process-A estimates to agree within justified statistical uncertainty and both to agree with the analytical reference.

## 20. Level-2 acceptance

Use the already controlled two-layer geometry rather than inventing a new easy case.

Compare against the existing deterministic FV reference and direct Process-A evidence.

Require:

- no systematic weighted-estimator bias detectable at declared precision;
- weight conservation;
- correct interface behaviour;
- correct release observable.

## 21. Weight-conservation tests

At every resampling event verify numerically:

total weight before = total weight after

within a strict floating-point tolerance.

Track live, released, censored, and total weight.

Any unexplained weight creation/loss is a hard failure.

## 22. Resampling-invariance unit tests

Construct artificial small weighted populations for which split/merge expected values can be computed exactly.

Test the implementation statistically and, where possible, deterministically.

## 23. Cost model

Before five-layer execution estimate:

- trajectories propagated;
- expected Process-A WOS steps;
- resampling overhead;
- replicate count;
- runtime;
- memory;
- expected effective rare-event sampling gain.

Use existing observed Process-A throughput.

## 24. Five-layer gate

Only if Levels 1 and 2 pass may a bounded five-layer experiment be executed.

The five-layer comparison target is the independently accepted Process-C CDF.

However, agreement does not automatically prove A=B=C.

Disagreement must not be hidden.

Compare at predeclared physical times drawn from the accepted CDF grid, including approximately the release-onset, intermediate and near-complete regions.

## 25. Five-layer censoring

Explicitly report weighted censoring.

If substantial probability weight remains censored, do NOT report a complete Process-A CDF without bounds/qualification.

## 26. Do not tune to Process C

Process-C values may be used to choose scientifically informative observation times.

They must NOT be used to tune weights, bins, resampling rules, or acceptance thresholds until Process A agrees.

Predeclare these before observing the five-layer comparison.

## 27. R2-WOS-02

Do NOT close R2-WOS-02 during method derivation.

Only reconsider it after an executed five-layer weighted Process-A ensemble demonstrates a usable release-time observable with explicit censoring treatment.

## 28. R2-B01

Do NOT close R2-B01 during method derivation.

A successful weighted estimator must first be independently reviewed as a valid estimator of Process A.

## 29. Method 3

Method 3 remains NOT STARTED.

Do not begin it during this pass.

## 30. Deliverables

Create at minimum:

- `verification/process_a_weighted_ensemble_literature.md`
- `verification/process_a_weighted_ensemble_derivation.md`
- `verification/process_a_weighted_ensemble_gate.md`

and appropriate implementation/test files only after the derivation is complete.

Persist machine-readable results for any executed validation.

## 31. Independent review boundary

If:

- the estimator is derived;
- implementation exists;
- Level 1 passes;
- Level 2 passes;

STOP.

Prepare a dedicated independent-review request for the weighted-ensemble bridge BEFORE executing the five-layer production comparison.

The new estimator must be independently reviewed before it is allowed to generate evidence that could close R2-WOS-02 or R2-B01.

## End-of-pass report

Report:

1. recovered main tip;
2. literature reviewed;
3. target Process-A observable;
4. weighted-ensemble mathematical definition;
5. splitting derivation;
6. merging derivation;
7. repeated-resampling unbiasedness argument;
8. released-weight treatment;
9. censoring treatment;
10. physical-time resampling scheme;
11. progress coordinate;
12. bins;
13. target population rule;
14. uncertainty estimator;
15. independent-replicate design;
16. implementation architecture;
17. supervisor commit pinned;
18. Level-1 benchmark;
19. Level-1 result;
20. Level-2 benchmark;
21. Level-2 result;
22. weight-conservation result;
23. resampling-invariance result;
24. computational cost estimate;
25. whether five-layer execution is scientifically authorized;
26. R2-WOS-02 status;
27. R2-B01 status;
28. Method-3 status;
29. files created;
30. commits;
31. workflow runs;
32. whether independent weighted-ensemble review is now warranted;
33. single highest-value next action.

STOP before five-layer weighted-ensemble execution if independent review is warranted.
