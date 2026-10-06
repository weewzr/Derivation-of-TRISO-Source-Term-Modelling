# Continuous-Time Rare-Event Research for Process A

## Executed starting point

The event-index WE prototype is preserved as negative evidence:
**LEVEL 1 PASS / LEVEL 2 FAIL / NOT VALIDATED FOR TRANSIENT PROCESS-A RELEASE CDF**.
Primary run: 37439263013.

## 1. Process-A stochastic classification

Let J_n denote the full post-event spatial/material/interface state after the nth production WOS event and let H_n>=0 be the sampled physical holding/first-passage time of that event. Let

T_0=0,  T_{n+1}=T_n+H_n.

The pair (J_n,T_n) is naturally a **Markov-renewal process** when the joint kernel

Q(x,dy,dh)=P(J_{n+1} in dy, H_n in dh | J_n=x)

depends only on the current complete state x. The continuous-time interpolation is a semi-Markov process only after specifying what state means during a holding interval. For Process A, a WOS holding interval represents an unobserved killed Brownian excursion inside the maximal sphere; the production primitive stores only its endpoint and total duration. Therefore the embedded Markov-renewal description is precise, while a fully observed continuous-time semi-Markov state requires an additional within-hop conditional law.

The target is the physical-time first-release CDF F_A(t)=P(T_A<=t), not an event-count observable.

## 2. Literature reviewed

### Weighted ensemble
Huber & Kim (1996), Biophysical Journal 70:97–110, DOI 10.1016/S0006-3495(96)79552-8: original WE Brownian-dynamics rare-event method.

Zhang, Jasnow & Zuckerman (2010), JCP 132:054107, DOI 10.1063/1.3306345: WE as statistically exact path-space resampling; standard construction propagates trajectories for a fixed time interval tau and resamples at common physical times.

Aristoff (2018), MMS 16:1222–1246: mathematical WE framework; conditional unbiased selection/mutation and variance decomposition.

WESTPA literature/software: mature fixed-time-iteration WE implementation; useful engineering evidence, not a theorem for asynchronous WOS event indices.

### Splitting / stopping surfaces
Kahn-Harris/Rosenbluth-style multilevel splitting and modern adaptive multilevel splitting estimate rare-event probabilities by branching at nested score levels/stopping events. Exactness relies on correct stopping-time/path conditioning and, for time-dependent events, retaining the full relevant state including time. These methods are natural for hitting probabilities but do not automatically produce a fixed-time FPT CDF from asynchronous WOS without conditioning on physical time.

### Sequential Monte Carlo / Feynman-Kac
Particle systems can be unbiased for unnormalised Feynman-Kac functionals under conditionally unbiased resampling, but the mutation/state model must represent the target process at the selection times. Event-index selection on the embedded chain can estimate embedded-chain path functionals; translating that into a physical-time snapshot requires the holding-time/within-event state information.

### Importance sampling
Likelihood-ratio importance sampling is unbiased when every biased transition has known density/probability relative to the original path law. For Process A this is straightforward in principle for discrete interface Bernoulli transmission decisions, but the dominant rare progression also contains spatial shell excursions and random first-passage times. Biasing those requires Radon-Nikodym factors for the full joint exit-location/exit-time kernels. No complete low-variance change of measure is currently derived.

### Killed Brownian motion / Dirichlet heat kernel
For Brownian motion killed on exiting a ball B, the killed transition density p_B(t,x,y) solves the Dirichlet heat equation. Survival probability is S_x(t)=integral_B p_B(t,x,y)dy. Therefore

P_x(X_t in dy | tau_B>t)=p_B(t,x,y)dy/S_x(t).

For a WOS sphere the start is its centre, so rotational symmetry simplifies this to a radial density proportional to r^2 p_B(t,0,r), normalised by survival. The Dirichlet heat kernel has spectral/Bessel expansions.

If one additionally conditions on a pre-sampled future exit time/location, the required bridge density is more complicated: by the Markov property it is proportional to the killed kernel from start to intermediate y multiplied by the boundary first-exit density from y to the specified future exit event. The current Process-A primitive does not provide or sample from this bridge.

## 3. Why the event-index WE proof is insufficient for the implemented CDF

There is an important distinction.

If the *entire Markov-renewal state* (J_n,T_n) is retained and a weighted particle system performs conditionally unbiased resampling at deterministic event indices n, then for any functional g(J_0,H_0,...,J_n,T_n) measurable at that event index, the standard tower-property argument does preserve its expectation.

For an eventually completed path, g_t=1{T_A<=t} is indeed a path functional. Thus **different accumulated times alone do not mathematically prove bias**.

The actual prototype, however, bins/merges trajectories primarily by spatial/material coordinate while allowing different T_n. Probabilistic merging preserves an arbitrary observable only if the survivor selection/weighting is applied to the full path/state required for the future conditional law. The code carries current time, so the elementary merge identity still holds; consequently the Level-2 discrepancy cannot honestly be declared a proven time-asynchrony bias solely from standard WE fixed-lag literature.

What is missing is a theorem/verification that this particular asynchronous resampling + stopping rule + horizon bookkeeping is an unbiased particle approximation of the *eventually completed physical-time release indicator*. The prototype also removes a live trajectory once its accumulated event-end time exceeds t_max. If the final WOS event straddles an observation time, the endpoint representation cannot reconstruct the within-hop state, although release itself occurs only at event endpoints in the production algorithm. For release-by-t, an overshooting non-release hop can safely be classified as not released by t, but it cannot be resumed from the exact state at t for common-time WE.

Conclusion: the original event-index unbiasedness argument is plausible for completed path indicators but insufficiently tied to the exact implemented truncation/resampling algorithm, and the Level-2 failure is empirical evidence that the implementation/gate is not validated. It is not rigorous evidence that all event-index particle methods are invalid.

## 4. Exact common-physical-time WE

Choose deterministic physical times 0=tau_0<...<tau_M. At tau_j each live trajectory must represent the conditional Process-A state at that exact time.

For a WOS hop beginning at (x,T_n) with sampled first exit (Y,T_n+H):
- if T_n+H<=tau_j, apply the event and continue;
- if T_n<tau_j<T_n+H, the production endpoint pair is insufficient.
One needs the law of X_{tau_j} conditional on survival of the current sphere to u=tau_j-T_n, plus enough residual conditional state to continue without changing the joint path law.

### Exact pause without pre-sampling the future exit
At a common-time boundary encountered inside a fresh WOS sphere, discard any not-yet-realised future endpoint sample and sample

Y ~ p_B(u,x,y)/S_x(u).

Then, by the strong Markov property of killed Brownian motion conditioned on survival to u, restart the *physical Brownian diffusion* from Y at tau_j.

But restarting the **production WOS algorithm** from Y changes the event partition while representing the same underlying Brownian law only if the production WOS sphere construction/first-passage sampler is itself exact for the local diffusion and no finite-capture/interface approximation is crossed during the paused interval. In homogeneous interior spheres this is defensible. Near finite-capture/interface logic the maximal WOS sphere remains inside one material, so an interior pause also does not cross the interface. Nevertheless this adds a new conditional Brownian primitive absent from the supervisor and therefore creates a new numerical representation requiring its own proof/verification.

### Conditional radial law at sphere centre
For ball radius R, diffusivity D, start 0, the killed heat kernel can be represented spectrally. The conditional radial density is

f(r|tau_R>u) = 4 pi r^2 p_B(u,0,r) / S_0(u), 0<r<R.

A convenient radial spectral form follows from the l=0 Dirichlet eigenfunctions sin(n pi r/R)/r. Both numerator and survival become alternating/exponentially weighted series with rates D n^2 pi^2/R^2.

Sampling options:
1. tabulate high-accuracy conditional radial CDF and invert;
2. rejection/sample from a positive representation if derived;
3. eigenfunction expansion with rigorous truncation/error control.

Numerical difficulty is greatest at very small u, where many modes are required and cancellation occurs; asymptotic/free-Gaussian representations would be needed for robust production use. At large u the first eigenmode dominates and sampling is easy.

### Joint bridge to pre-sampled exit
Conditioning simultaneously on future exit time/location requires a killed Brownian bridge to a boundary flux event. This is mathematically derivable from p_B and the boundary normal derivative/first-exit density, but is substantially more complex and unnecessary if future endpoint samples are not committed before crossing a resampling time.

## 5. Practicality of exact pausing

Scientific exactness: high for homogeneous Brownian interior motion if the conditional killed-kernel sampler is exact/numerically controlled.

Engineering burden: high.
- new special-function/spectral sampler;
- small-u stability;
- validation against survival law and direct Brownian/WOS endpoint distributions;
- careful handling of finite-capture neighborhoods;
- new stateful scheduler;
- potentially many pauses per long WOS hop.

It is feasible research, but not a small correction to the existing WE controller.

## 6. Event/stopping-time splitting

A stopping-time splitting method can branch when trajectories hit nested milestones. If cloning occurs at an actual Process-A stopping event, children can continue independently with divided weights. This avoids merging states at different physical times.

However, **splitting alone does not control population growth**. Pruning/merging trajectories at asynchronous milestones requires a valid particle/resampling construction. One safe construction is fixed branching with Russian roulette: at selected stopping events, split successful progressions and apply unbiased roulette elsewhere, with weights chosen so expected path contribution is preserved. This is a classical unbiased splitting class and does not require common-time states.

For F_A(t), every child retains its physical clock; released child weight contributes iff release time<=t. This directly targets the physical-time path indicator. The proof burden is local split/roulette expectation preservation at stopping times plus optional-stopping/integrability conditions.

This is a stronger fit to WOS than common-time WE because WOS naturally exposes interface/capture/milestone stopping events.

## 7. Milestone factorisation

Strong Markov structure permits exact factorisation only if each milestone state retains all variables required for future evolution. Pure scalar products of layer-to-layer hitting probabilities and independent passage-time distributions are generally insufficient because exit location, side/interface state and passage time can be correlated.

A full interface-state renewal kernel recovers the required information. That is essentially the Process-B/C direction already present in this project. Therefore using exact shell/interface first-passage kernels as the primary acceleration would cease to be a direct estimator built from unchanged finite-capture Process-A steps, even if it targets the same intended physical diffusion model.

Milestone splitting that merely clones the actual Process-A trajectory when it reaches a milestone remains Process A; analytically replacing the intervening Process-A dynamics with shell kernels moves toward Process B/C.

## 8. Importance sampling

For an interface Bernoulli decision with original p and biased q, a realised transmission contributes likelihood factor p/q and reflection (1-p)/(1-q). Products of such factors preserve expectations provided support is maintained.

But the principal Buffer->IPyC->SiC rarity includes:
- rare transmission;
- rare traversal of IPyC before return;
- another rare transmission.

Biasing only Bernoulli interface choices leaves the spatial traversal bottleneck. Biasing WOS exit direction/time requires density ratios for the joint first-passage law under a specified changed measure. A rigorous Doob-transform/importance process could be derived from committor functions, but that is a substantial new method and risks converging on Process B/C analytical machinery.

## 9. Candidate comparison

### A — common-time WE with killed-Brownian pause sampler
Preserves target: potentially yes.
Proof burden: high.
Implementation: high.
Variance reduction: potentially high.
Validation: difficult but possible.
Risk of becoming different process: moderate unless pause primitive is proven equivalent.
Supervisor invasiveness: can remain research wrapper but requires new propagation primitive.

### B — stopping-time weighted splitting + Russian roulette on unchanged Process-A events
Preserves target: yes under local unbiased branching/roulette and unchanged propagation.
Proof burden: moderate.
Implementation: moderate.
Variance reduction: expected high if milestones track the known penetration bottleneck.
Validation: straightforward on homogeneous/two-layer cases.
Risk of becoming different process: low: between branch/roulette decisions every child uses unchanged Process A.
Supervisor invasiveness: low; wrapper/controller only.

### C — likelihood-ratio importance sampling
Preserves target: yes if full RN derivative is known.
Proof burden: high for spatial first-passage bias.
Implementation: high.
Variance reduction: potentially high.
Validation: difficult.
Risk of different process: high if any likelihood factor is omitted.
Supervisor invasiveness: moderate.

### D — direct Process A
Exact target by definition.
No proof burden.
Implementation exists.
Rare-event efficiency: unacceptable for five layers.
Validation: established in feasible regimes.
Invasiveness: none.

### E — Process B/C
Strong existing numerical efficiency and accepted benchmark.
Does not constitute direct production Process-A estimator; equivalence remains unclosed.
Risk of relabelling: high if used to close Process-A findings.

## 10. Decision

The strongest next Process-A estimator is **stopping-time weighted splitting with unbiased Russian roulette**, not common-time WE.

Reason: it aligns resampling with natural WOS stopping events, never requires an intermediate within-hop state, leaves every propagated child on the unchanged Process-A kernel, and has a simpler local unbiasedness proof than either killed-Brownian pausing or full likelihood-ratio biasing.

This is a method switch within rare-event Monte Carlo, not validation of the failed event-index WE prototype.
