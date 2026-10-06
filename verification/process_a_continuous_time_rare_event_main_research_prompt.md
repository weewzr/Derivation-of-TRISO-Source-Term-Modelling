# Main Research Prompt — Resolve Continuous-Time Rare-Event Bridge for Process A

Continue from CURRENT remote `main` of:

`weewzr/Derivation-of-TRISO-Source-Term-Modelling`

Recover the live repository first.

The controlling executed evidence is:

`verification/process_a_weighted_ensemble_executed_evidence.md`

Primary scientific run:

`37439263013`

Current result:

- LEVEL 1 = PASS
- LEVEL 2 = FAIL
- FIVE-LAYER WE = NOT AUTHORIZED
- INDEPENDENT WE REVIEW = NOT YET WARRANTED
- R2-WOS-02 = OPEN
- R2-B01 = OPEN
- METHOD 3 = NOT STARTED

## 1. Accept the failure

Do NOT loosen acceptance thresholds.

Do NOT tune bins.

Do NOT increase target trajectories/bin.

Do NOT increase replicate count merely to force agreement.

Do NOT execute five-layer WE.

Do NOT send the present bridge to independent review as a successful method.

The Level-2 failure is scientifically meaningful.

## 2. Current root-cause hypothesis

The current WE controller resamples after:

`q = 20 Process-A WOS kernel calls`.

This is an EVENT-COUNT resampling surface.

But Process-A WOS hops have random physical first-passage durations.

Therefore walkers after q calls generally have different accumulated physical times.

The current controller may consequently merge/resample trajectories that are not on a common physical-time surface.

The controlled two-layer transient benchmark shows large discrepancies, including approximately:

- t=0.25 s: direct ~0.00992, WE ~0.03509, FV ~0.00984
- t=0.5 s: direct ~0.06833, WE ~0.12907, FV ~0.06999
- t=1 s: direct ~0.23933, WE ~0.34640, FV ~0.23414

This is not a marginal statistical discrepancy.

## 3. Research before more code

Conduct a targeted literature and implementation review specifically on:

- continuous-time weighted ensemble;
- weighted ensemble for asynchronous stochastic trajectories;
- weighted ensemble for first-passage processes;
- resampling at fixed physical time;
- continuous-time Markov / semi-Markov weighted ensemble;
- rare-event sampling for semi-Markov processes;
- splitting methods for asynchronous first-passage trajectories;
- weighted ensemble with variable-duration transitions;
- Brownian first-passage rare-event methods.

Prioritize:

1. original papers;
2. peer-reviewed mathematical analyses;
3. established WE literature;
4. authoritative implementations such as mature WE software where relevant;
5. well-maintained scientific GitHub repositories only as implementation evidence.

Do not assume the standard fixed-lag WE prescription transfers directly to event-driven WOS.

Create/update:

`verification/process_a_weighted_ensemble_continuous_time_research.md`

## 4. Formulate Process A as a stochastic process

Write Process A explicitly as the appropriate stochastic object.

Its WOS state includes at least:

`X_n = spatial state after event n`

`T_n = accumulated physical time`

plus material/interface state as required.

Each transition produces:

`(X_n,T_n) -> (X_{n+1},T_n + tau_n)`

where tau_n is a random first-passage duration.

Determine whether the natural mathematical representation is:

- Markov renewal process;
- semi-Markov process;
- embedded Markov chain plus holding times;
- or another precise formulation.

Do not use terminology loosely.

## 5. Explain why event-index resampling failed

Analyze mathematically whether resampling at fixed event count q is unbiased for:

`F_A(t)=P(T_A<=t)`.

The previous derivation argued for unbiased weighted observables on the augmented event-index state.

Determine whether that argument actually establishes unbiasedness for the fixed-PHYSICAL-TIME release CDF.

If it does not, identify the exact missing conditioning argument.

Do not merely say "different walkers have different times."

Show mathematically where the estimator/resampling argument ceases to apply.

## 6. Common physical-time resampling

Investigate the theoretically natural alternative:

choose deterministic physical resampling times

`0 = tau_0 < tau_1 < ... < tau_M`.

At every tau_j all live weighted trajectories should represent the law of Process A at the SAME physical time.

Derive what state information is required to do this exactly.

## 7. The WOS overshoot problem

This is the central issue.

Suppose a Process-A WOS hop begins at physical time T_n and samples:

- exit location X_{n+1};
- first-passage duration Delta T_n.

If:

`T_n < tau_j < T_n + Delta T_n`,

then the desired resampling surface lies INSIDE a WOS first-passage event.

The existing Process-A primitive supplies:

- start state;
- exit state;
- total first-passage time;

but not necessarily the particle state at intermediate time tau_j conditional on survival inside the sphere.

Derive exactly what information is missing.

## 8. Do not simply truncate the hop

Do NOT implement:

`Delta T := min(Delta T, tau_j-T_n)`

or linear interpolation of position.

That would generally change the Process-A law.

Any proposed bridge across the resampling surface must be mathematically derived.

## 9. Research conditional within-sphere law

Investigate whether the required conditional law is available.

For Brownian motion starting at the centre of a sphere, conditioned on:

first-exit time > u

or, if necessary, conditioned jointly on a sampled future exit event,

research/derive the distribution of the particle position at intermediate time u.

Relevant mathematical objects may include:

- absorbing heat kernel in a sphere;
- Brownian motion killed at the sphere boundary;
- survival-conditioned transition density;
- Brownian bridge conditioned on first exit;
- Doob h-transforms;
- spectral expansion of the Dirichlet heat kernel.

These are research directions, not assumptions.

## 10. Determine whether exact pausing is practical

If a mathematically exact conditional intermediate-state sampler can be derived:

assess whether it is computationally practical.

Report:

- formula;
- normalization;
- sampling algorithm;
- numerical complexity;
- precision issues;
- verification route.

Do NOT implement it yet unless the derivation is complete and clearly tractable.

## 11. Alternative: resample only at natural events

Investigate whether an event-driven splitting/WE formulation can estimate the fixed-time release CDF without requiring common-time resampling.

Possible mathematical approaches might use:

- weighted path-space estimators;
- stopping-time splitting;
- milestone/interface splitting;
- sequential importance sampling;
- Feynman-Kac / particle methods;
- rare-event splitting based on stopping surfaces.

Do not assume these are valid.

For each candidate determine whether the target remains exactly:

`F_A(t)=P_A(T_A<=t)`.

## 12. Alternative: first-passage milestone factorization

Investigate whether Process-A release can be factorized using stopping times at physical interfaces/milestones.

For example:

kernel/Buffer milestone

Buffer/IPyC milestone

IPyC/SiC milestone

SiC/OPyC milestone

release.

Determine whether strong Markov / renewal structure permits an exact or controlled composition of:

- hitting probabilities;
- conditional passage-time distributions;
- interface-state distributions.

If this effectively becomes Process B/C rather than Process A, state that explicitly.

## 13. Alternative: importance sampling

Investigate whether importance sampling can accelerate the rare interface progression while retaining an unbiased estimator of Process A through a likelihood-ratio/Radon-Nikodym weight.

If transition probabilities are biased:

derive the path likelihood ratio exactly.

If first-passage distributions are biased:

derive the corresponding weight.

Do NOT implement heuristic biasing without a complete weight formula.

## 14. Compare candidate strategies

At minimum compare:

A. exact common-physical-time WE using conditional within-hop state sampling;

B. event/stopping-time splitting without common-time synchronization;

C. likelihood-ratio importance sampling;

D. direct Process A baseline;

E. existing Process B/C route.

For each report:

- does it preserve Process-A target law?
- proof burden;
- implementation complexity;
- expected variance reduction;
- computational cost;
- verification difficulty;
- risk of silently becoming a different process.

## 15. Define a decision matrix

Score the candidate approaches against:

- scientific exactness;
- traceability to Process A;
- mathematical derivability;
- implementation feasibility;
- expected rare-event efficiency;
- validation feasibility;
- supervisor-code invasiveness.

Do not choose a method solely because it is easiest to code.

## 16. No five-layer execution

No five-layer rare-event experiment is authorized in this pass.

The objective is to select the mathematically correct bridge first.

## 17. Small synthetic test only if needed

If a mathematical ambiguity can be resolved by a tiny synthetic numerical experiment, that is allowed.

Do NOT run another full Level-1/Level-2 WE validation until a corrected method has been fully derived and predeclared.

## 18. Status of current WE implementation

Preserve the existing implementation and executed failure evidence.

Do NOT delete it.

Classify it as:

EVENT-INDEX WE PROTOTYPE —
LEVEL 1 PASS / LEVEL 2 FAIL /
NOT VALIDATED FOR TRANSIENT PROCESS-A RELEASE CDF.

This negative result is scientifically useful.

## 19. R2-WOS-02 / R2-B01

Both remain OPEN.

Do not change their closure criteria.

## 20. Method 3

Method 3 remains NOT STARTED.

However, this pass should explicitly assess whether continued Method-2 rare-event estimator development remains scientifically worthwhile relative to proceeding to Method 3 with the Process-A limitation documented.

Do NOT begin Method 3.

Make only the recommendation.

## 21. Required decision

At the end choose exactly one:

A. CONTINUE WE —
an exact common-time WE bridge is mathematically derived and practical.

B. SWITCH RARE-EVENT METHOD —
another unbiased Process-A estimator has a stronger mathematical/engineering case.

C. METHOD-2 COMPUTATIONAL BOUNDARY —
no practical exact Process-A rare-event bridge is presently justified; document the limitation and recommend proceeding to Method 3 while keeping R2-WOS-02/R2-B01 open.

Do not choose before completing the research.

## 22. If A or B is selected

Create a PREDECLARED validation gate for the selected corrected method.

The validation ladder must again be:

homogeneous analytical benchmark

->

controlled two-layer benchmark

->

independent review

->

only then five-layer use.

Do NOT execute the corrected Level-2 experiment in the same pass unless the mathematical derivation is complete, small validation is clearly justified, and no unresolved theoretical assumption remains.

## 23. Documentation

Create at minimum:

- `verification/process_a_weighted_ensemble_continuous_time_research.md`
- `verification/process_a_rare_event_method_decision.md`

and, if A or B is selected:

- `verification/process_a_rare_event_corrected_gate.md`

Do not rewrite the full manuscript.

## End-of-pass report

Report:

1. current main tip;
2. Level-1/Level-2 failure status recovered;
3. literature reviewed;
4. precise stochastic-process classification of Process A;
5. exact mathematical reason event-index WE is insufficient or sufficient;
6. common-time resampling derivation;
7. within-WOS-hop overshoot problem;
8. conditional intermediate-state law research result;
9. practicality of exact pausing;
10. event/stopping-time splitting assessment;
11. milestone factorization assessment;
12. importance-sampling assessment;
13. candidate-method comparison;
14. decision matrix;
15. selected decision A/B/C;
16. selected method if A/B;
17. proof of Process-A target preservation if A/B;
18. proposed corrected validation ladder;
19. computational cost expectation;
20. R2-WOS-02 status;
21. R2-B01 status;
22. Method-3 status;
23. recommendation on whether continued Method-2 development is worthwhile;
24. files created;
25. commits;
26. workflow runs if any;
27. single highest-value next action.

STOP after the method decision / corrected gate.

Do NOT launch five-layer work.
