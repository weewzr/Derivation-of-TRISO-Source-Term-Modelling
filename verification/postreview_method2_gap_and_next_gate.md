# Post-review Method-2 gap and next-gate analysis

## Process distinctions

- **Process A:** original finite-capture production WOS using the pinned production stochastic primitives/control path.
- **Process B:** exact-interface accelerated stochastic process.
- **Process C:** deterministic Markov-renewal/matrix reduction of Process B.

The accepted Process-C CDF is a quantitative benchmark. It is not Process-A validation.

## Established evidence

### Process A

- homogeneous absorbing-sphere release agrees with the analytical sphere solution at tested parameters;
- absorbing outer-surface release semantics are explicit;
- production interface rule/capture/reinsertion semantics are mapped and exercised;
- controlled two-layer transient production primitives agree with refined FV to the declared statistical precision over epsilon=25–200 nm; high-power run N=10,000/epsilon, 40,000 total histories, zero censoring, max |Delta F|=0.008774;
- integrated five-layer production path is directly executed;
- Stage-A five-layer run: 24/24 histories censored at 5,000,000 steps;
- integrated diagnostic: 8/8 censored at 1,000,000 steps, all ending in Buffer, deepest layer IPyC, no SiC/OPyC entry;
- ~153k zero-time interface events/history; almost all classified steps in Buffer;
- 8 million diagnostic steps execute in ~0.965 s, so raw CPU throughput is high.

### Process B

Exact-interface stochastic primitives and accelerated first-passage/interface representation have been separately derived/verified at their stated scope. Process B is not renamed Process A.

### Process C

The hard five-layer 8-state matrix/transform has passed independent high-precision review. Its controlled release CDF has now passed independent inverse-Laplace review with non-blocking finding R4-m01. Representative accepted values are F(1e6)=5.849128692e-4, F(1e7)=0.1594036656, F(1e8)=0.8872792421, F(1e9)=0.999999999788.

## Exact remaining Process-A gap

The production five-layer finite-capture WOS has no usable release-time ensemble. Existing direct histories are step-censored before deep-stack progression. The immediate rare-event bottleneck is penetration from Buffer through IPyC toward SiC, not CPU cost per step and not observed SiC residence.

The diagnostic single-attempt scale for Buffer transmission -> traverse IPyC -> transmit to SiC is ~5.2e-12. This is an explanatory scale, not an eventual-passage probability.

## Finding closure requirements

### R2-WOS-02

The controlling diagnostic audit requires an uncensored or explicitly censored **five-layer Process-A release-time ensemble sufficient for the intended observable**, with explicit treatment of remaining censoring. A Process-C curve or milestone-only probability does not close it.

### R2-B01

Requires a scientifically supported five-layer production release estimator/verification. The accepted Process-C CDF narrows the target but does not close the production blocker. If an accelerated estimator is used, its relationship to Process A must be proven rather than assumed.

## Candidate next experiments considered

1. Larger direct Process-A release ensemble / larger max_steps — rejected as low information: existing 24x5e6 and 8x1e6 runs are 100% censored and progression scale is extraordinarily small.
2. Milestone-only direct Process-A statistics — useful diagnostically, but cannot by itself satisfy the release-CDF findings.
3. Process-B/C substitution — rejected: changes the process/representation and cannot be relabelled Process A.
4. **Weighted-ensemble / splitting estimator that propagates the unchanged Process-A dynamics between resampling times** — selected for method-development investigation. Rigorous WE literature shows splitting/merging can preserve the underlying trajectory distribution and unbiased observables when weights are handled correctly. This is a candidate bridge, not yet accepted for this project.

## Cost / information-gain decision

Direct brute force is computationally fast per step but scientifically inefficient. At ~8.3 million steps/s observed harness throughput, even very large step budgets do not address the rare progression probability efficiently; increasing N or max_steps is expected mainly to reproduce censoring.

A weighted/resampled estimator could concentrate trajectories in progressively deeper radial/layer milestones while continuing to use the exact unchanged Process-A step dynamics. But unbiasedness for the desired finite-time release CDF, treatment of weighted censoring, binning/resampling schedule, variance estimator, and validation against direct Process A in feasible geometries must be derived and predeclared before implementation.

## Decision

**C. ADDITIONAL METHOD DEVELOPMENT REQUIRED.**

No new production experiment is launched in this pass.

## Next gate: Process-A weighted-ensemble bridge derivation

Scientific question: can trajectory resampling be applied around the unchanged Process-A production dynamics to estimate the five-layer finite-time release CDF without changing its target law?

Required derivation before code:
1. define weighted path ensemble and release indicator estimator for F_A(t)=P_A(T<=t);
2. prove conditional split/merge operations preserve expected weighted observables;
3. specify progress coordinate/bins using only state variables available in Process A (candidate: layer + radial progress);
4. define resampling times/schedule without changing physical propagation;
5. define treatment of released and step-censored trajectories;
6. derive variance/uncertainty estimator;
7. predeclare computational budget and stopping rules;
8. verify on homogeneous and two-layer problems against both direct Process A and accepted analytical/FV references;
9. only then attempt a bounded five-layer comparison to the accepted Process-C benchmark.

No claim is made yet that this bridge closes R2-WOS-02 or R2-B01.


## Weighted-ensemble bridge execution outcome

The first bounded bridge-development cycle has now been executed through its controlled gate.

- Level 1 homogeneous validation: PASS after predeclared stratified-initialization remediation.
- Level 2 frozen two-layer validation: FAIL at 0.25, 0.5, 1 and 4 s.
- weight conservation: PASS;
- weighted censoring: zero;
- interface event coverage: PASS;
- five-layer WE: NOT AUTHORIZED.

The current weighted-ensemble bridge therefore does not yet satisfy the required validation chain. Decision C remains in force: additional method development is required before a useful Process-A rare-event estimator exists.

The next question is narrower than generic WE tuning: derive a rigorous common-physical-time resampling bridge for WOS first-passage hops, or prove an alternative asynchronous/event-index resampling theorem sufficient for the physical release-time CDF. Do not tune bins against Process C before that issue is resolved.
