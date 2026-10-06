# Process-A Rare-Event Method Decision

## Decision

**B. SWITCH RARE-EVENT METHOD.**

Selected corrected method: **stopping-time weighted splitting with unbiased Russian roulette around unchanged Process-A propagation**.

The event-index WE prototype remains preserved as:
**LEVEL 1 PASS / LEVEL 2 FAIL / NOT VALIDATED FOR TRANSIENT PROCESS-A RELEASE CDF.**

## Why not continue common-time WE now?

An exact common-time construction is mathematically possible in principle by sampling the survival-conditioned killed-Brownian position inside a WOS sphere at each physical resampling time. The required density is the Dirichlet heat kernel normalised by survival probability. But robust sampling requires a new spectral/asymptotic numerical primitive and a new scheduler. This is disproportionate when Process A already exposes natural stopping events suitable for unbiased branching.

## Selected estimator

Let a Process-A path carry weight w. Define predeclared stopping milestones M_0,...,M_L using states/events already observable in Process A. At a qualifying progression stopping event, replace the trajectory by b children at the identical full Process-A state/history/time, each weight w/b, with independent future RNG streams.

To control population/cost, apply Russian roulette only at predeclared stopping events. For a trajectory of weight w selected for roulette with survival probability q in (0,1], keep it with probability q and assign weight w/q; otherwise kill it with weight zero.

For arbitrary future path functional G,

E[sum_children (w/b)G_i | H] = w E[G|H],

and

E[1{survive}(w/q)G | H] = q(w/q)E[G|H]=w E[G|H].

By iterated conditional expectation at stopping times, repeated splitting/roulette preserves the expectation of the physical release indicator G_t=1{T_A<=t}, assuming finite/integrable total weighted work and independent future random streams.

No transition probability, first-passage-time law, interface probability, or physical clock is modified.

## Candidate milestones

Validation milestones are geometry-specific and predeclared before execution.

Homogeneous sphere: radial stopping surfaces at r/R = 0.4,0.6,0.8 and capture/release neighborhood.

Two-layer frozen benchmark: actual Process-A events associated with approach/transmission across the material interface plus radial progress in outer material. No analytical shell replacement.

Future five-layer candidate milestones, NOT AUTHORIZED:
Fuel/Buffer progression; successful Buffer->IPyC transmission; first attainment of deeper IPyC radial progress; IPyC/SiC progression; SiC/OPyC progression; release.

Milestones may influence branching/roulette only. They never replace propagation.

## Status

R2-WOS-02 OPEN.
R2-B01 OPEN.
Method 3 NOT STARTED.

Continued Method-2 development remains scientifically worthwhile for one bounded corrected-method validation cycle because the selected method has a materially lower proof/engineering burden than exact common-time pausing and directly addresses the observed penetration rare event. If the corrected method fails the controlled two-layer gate, the recommendation should change to Method-2 computational boundary rather than indefinite tuning.
