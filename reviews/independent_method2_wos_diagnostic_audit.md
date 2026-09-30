# Independent Audit — Current Method-2 WOS Verification Evidence

**Repository:** `weewzr/Derivation-of-TRISO-Source-Term-Modelling`  
**Canonical branch:** `main`  
**Reviewed repository commit:** `b9e1f3e00244afe5349c3c86f816f64bd6899630`  
**Supervisor repository:** `theodoreOnzGit/outram-park-backend`  
**Reviewed supervisor commit:** `8d31482d127e211614ebb3f66b1076e1ed6dea98`  
**Scope:** Method-2 WOS diagnostic evidence only  
**Production supervisor repository modified:** No

## Final conclusion

The recent diagnostics provide **credible localization evidence** for the five-layer WOS pathology, but they do not constitute full WOS verification and they do not close R2-B01.

The strongest independent conclusion is:

\[
\boxed{
\text{the observed computational bottleneck is dominated by repeated interface-resolution events and finite-capture/reinsertion behaviour, rather than direct traversal of SiC}
}
\]

for the exact trajectories executed.

That conclusion is justified because the targeted trajectories spend more than 99.97% of their capped transitions in Buffer, accumulate roughly 3.1e4 interface events per 2e5 transitions, record zero simulated time on every reported interface event, make only a handful of Buffer-to-IPyC crossings, and make **zero** IPyC-to-SiC or SiC/OPyC transitions.

However, the evidence does **not** establish that the interface algorithm is mathematically wrong. It establishes that the current finite-epsilon capture/reinsertion mechanism is computationally dominant in the observed histories and that the current max-step configuration produces complete censoring.

The correct root-cause classification is:

\[
\boxed{\textbf{E. MIXED}}
\]

with the strongest evidence pointing to interface-resolution behaviour as the dominant computational bottleneck, while physical diffusivity contrast and max-step truncation remain relevant contributors.

---

# 1. Evidence inspected

## Stage-A smoke / contract

Workflow:

`WOS Stage A smoke contract`

Run:

`36669658813`

Run conclusion: **success**.

Research commit:

`857160c7d83d42ad271f1db4276897bd86d1d710`

Supervisor commit:

`8d31482d127e211614ebb3f66b1076e1ed6dea98`

Frozen parameters included:

- N = 24;
- max_steps/history = 5,000,000;
- capture epsilon = 10 nm;
- reinsert factor = 2;
- K = 1;
- Cs-137;
- uniform kernel-volume initial sampling;
- absorbing OPyC release boundary.

Executed result:

\[
N=24,
\qquad
N_{released}=0,
\qquad
N_{censored}=24,
\]

therefore:

\[
\boxed{f_{censor}=1.0}.
\]

Every history exhausted all 5,000,000 permitted steps.

The persisted Stage-A evidence also records:

[
D_{Fuel}\approx1.2503\times10^{-13},
\]

[
D_{Buffer}=1.0\times10^{-8},
\]

[
D_{IPyC}\approx4.0623\times10^{-14},
\]

[
D_{SiC}\approx9.2278\times10^{-17},
\]

[
D_{OPyC}\approx4.0623\times10^{-14}
\]

in m^2/s for the executed benchmark.

The diagnostic record correctly labels this run as execution/censoring evidence, not WOS validation.

## Targeted step diagnostic

Workflow:

`WOS targeted step diagnostic`

Run:

`36670865310`

Head research commit:

`b9e1f3e00244afe5349c3c86f816f64bd6899630`

Supervisor commit:

`8d31482d127e211614ebb3f66b1076e1ed6dea98`

Three histories were followed for 200,000 transitions each.

Observed:

Trajectory 0:

\[
[26,199963,11,0,0]
\]

for

[
[Fuel,Buffer,IPyC,SiC,OPyC].
]

Interface events:

[
30,836
]

Zero-delta-t interface events:

[
30,836.
]

Trajectory 1:

\[
[27,199971,2,0,0].
\]

Interface events:

[
31,033.
]

Trajectory 2:

\[
[53,199944,3,0,0].
\]

Interface events:

[
30,787.
]

No trajectory entered SiC.

The transition matrices show only:

- Fuel -> Buffer;
- Buffer -> IPyC;
- IPyC -> Buffer.

There are no transitions into SiC.

The reported physical accumulated times at 200,000 transitions were approximately:

- trajectory 0: (1.87\times10^4) s;
- trajectory 1: (3.56\times10^4) s;
- trajectory 2: (1.21\times10^5) s.

These are hours-to-day scale physical times, not months.

## Semantic diagnostics

Workflow:

`WOS semantic contract diagnostics`

Run:

`36670875328`

Supervisor commit:

`8d31482d127e211614ebb3f66b1076e1ed6dea98`

All three requested tests passed:

1. `absorbing_sphere_release_matches_crank`
2. `hop_releases_at_outer_surface`
3. `interface_rule_gives_uniform_equilibrium_density`

---

# 2. Production mathematics ↔ supervisor code

The frozen production contract identifies:

- piecewise-constant positive (D_i);
- an interface-free local WOS sphere;
- centre-start first-passage time sampling;
- isotropic exit direction;
- finite capture epsilon;
- stochastic transmission/reflection at an interface;
- (K=1) base case;
- reinsertion distance (delta=\alpha\epsilon);
- zero physical time on the interface event;
- absorbing outer OPyC boundary;
- release-time censoring after max_steps.

The supervisor implementation at commit `8d31482d...` corresponds to this contract.

## Sphere-radius selection

`step_multilayer` obtains inner and outer shell bounds and chooses the smaller radial distance to the two boundaries.

Thus an ordinary WOS sphere is contained within the current spherical layer.

This mapping is correct.

## First-passage time

`sample_first_passage_time` evaluates:

[
\tau=\theta\frac{\rho^2}{D}.
]

The dimensionless (	heta) distribution is built from the analytical centre-start absorbing-sphere survival function.

The scaling is correct.

The numerical inversion is approximate because the production sampler uses a finite table and interpolation.

## Interface detection

The code tests whether:

[
\rho\le\epsilon_{capture}.
]

That replaces the mathematically limiting sequence of shrinking WOS spheres with a finite interface-resolution rule.

## Transmission

The actual implementation uses:

[
\boxed{
p_{1\rightarrow2}
=
\frac{KD_2}{D_1+KD_2}
}
]

so for (K=1):

[
\boxed{
p_{1\rightarrow2}
=
\frac{D_2}{D_1+D_2}.
}
]

This agrees with the frozen contract.

## Reflection and reinsertion

On an interior interface the code either moves the walker to the neighbouring side or returns it to the current side.

The reinsertion distance is:

[
\delta=\alpha\epsilon.
]

No simulated time is added during this interface resolution.

That matches the documented production process.

## Layer classification

`TrisoCell::get_triso_region` assigns layer membership by radial coordinate, with a geometric tolerance.

The shell-bound routine then uses the corresponding material boundaries.

This mapping is internally consistent for the concentric-sphere geometry.

## Outer release

At the OPyC outer radius, `step_multilayer` returns `Released`.

Thus the implementation has a concrete absorbing release event.

## Censoring

`walk_until_released` returns a release time if the walker is released and `None` when the max-step budget is exhausted.

The diagnostic adapter preserves the distinction between `Released(Time)` and `CensoredMaxSteps`.

Therefore the current adapter no longer silently converts censoring into known non-release.

---

# 3. Homogeneous WOS analytical check

## Verdict: meaningful but limited verification

The test `absorbing_sphere_release_matches_crank` uses:

- a homogeneous sphere;
- (R=100,\mu m);
- constant (D=10^{-8}\,m^2/s);
- a uniform initial volume distribution;
- a perfect absorbing surface;
- 4,000 histories;
- capture epsilon (=10^{-3}R);
- observation times 0.05, 0.1, 0.2, and 0.4 s;
- absolute MC/reference tolerance 0.04.

The reference function is the standard analytical sphere-release series for uniform initial concentration and a zero-concentration surface.

This is a real analytical verification of the **homogeneous** WOS release path.

It is not a trivial test: the Monte Carlo sample is actually compared to multiple nonzero analytical release fractions at several times.

The 4,000-history statistical standard error for a Bernoulli release fraction is at most:

[
\frac{1}{2\sqrt{4000}}
\approx0.0079.
]

The tolerance 0.04 is therefore substantially wider than pure sampling error. It accommodates numerical effects such as finite capture epsilon, but it also makes the test relatively permissive.

The test also uses the finite capture condition rather than the ideal exact-boundary limit.

### What it establishes

It supports:

[
\boxed{
\text{homogeneous WOS release statistics are compatible with the analytical sphere solution at tested parameters}.
}
\]

### What it does not establish

It does not verify:

- multilayer interface transmission;
- five-layer release;
- epsilon convergence;
- reinsertion convergence;
- transient equivalence of the multilayer process to the divergence-form PDE;
- absence of systematic bias in the first-passage table.

Therefore this is **useful Stage-B-style homogeneous evidence**, not full Method-2 validation.

---

# 4. Outer release semantics

## Verdict: semantic correctness established

The test `hop_releases_at_outer_surface` initializes a walker at approximately:

[
427.4,\mu m
]

for a particle with outer radius approximately:

[
427.5,\mu m,
]

and uses:

[
\epsilon=0.5,\mu m.
]

Since the walker is already within the capture region, the next multilayer resolution classifies the outer OPyC surface as release.

This proves that:

[
\boxed{
\text{the production code has an explicit absorbing OPyC release event}.
}
]

It does not prove correct release-time statistics or negligible boundary-capture bias.

---

# 5. Interface-equilibrium diagnostic

## Verdict: useful semantic diagnostic, insufficient as transport validation

The test geometry has:

[
a=50,\mu m,
qquad
b=100,\mu m.
]

The correct volume fraction of the inner region is:

[
\left(\frac ab\right)^3
=
0.125.
]

Thus the expected reference is mathematically correct for a uniform equilibrium concentration under (K=1).

The measured inner time fraction was:

[
0.1368,
]

with absolute deviation:

[
0.0118.
]

The test's tolerance is:

[
0.02,
]

so it passes.

This establishes that the implemented interface rule is **compatible with approximately uniform equilibrium density for this one finite-epsilon reflecting two-region experiment**.

It does not establish transient equivalence with:

[
\frac{\partial c}{\partial t}
=
\nabla\cdot(D\nabla c).
]

It also does not establish that the finite capture/reinsertion bias vanishes.

A single equilibrium observable cannot rule out compensating transient/interface errors.

Thus:

[
\boxed{
\text{equilibrium evidence only}
}
]

not:

[
\boxed{
\text{proof of the multilayer interface model}.
}
]

---

# 6. 100% censoring root cause

## Independent classification: E — MIXED

This is the central diagnostic conclusion.

### A. Physical-timescale dominated?

**Partly yes.**

The diffusivity hierarchy is extreme.

In particular:

[
D_{SiC}\approx9.23\times10^{-17}\,m^2/s
]

is about five orders of magnitude lower than the IPyC diffusivity and roughly eight orders below the Buffer value.

Therefore once a history enters SiC, its physical residence time can become extremely large.

So diffusivity contrast is unquestionably physically relevant.

### B. Interface-algorithm dominated?

**Strongly supported as a computational-step-count explanation.**

The targeted histories spend approximately:

[
\frac{199963}{200000}
=
0.999815
]

of trajectory 0's transitions in Buffer.

Similarly:

[
\frac{199971}{200000}
=
0.999855
]

for trajectory 1.

Trajectory 2 spends:

[
\frac{199944}{200000}
=
0.99972
]

in Buffer.

Meanwhile there are roughly (3.1\times10^4) interface events per history.

So the computational effort is dominated by repeated near-interface resolution.

### C. Implementation defect?

**Not established.**

The evidence shows what the algorithm does, but does not establish that the interface rule or geometry logic violates its intended mathematical process.

A large step count alone is not a proof of a coding error.

### D. Max-step / estimator-design issue?

**Definitely contributing.**

Stage A gives:

[
24/24
]

histories censored.

Thus the current estimator cannot produce a complete release CDF under the tested step cap.

### E. Mixed?

**Yes.**

The observed pathology contains:

- physical diffusivity contrast;
- interface-event computational overhead;
- finite-capture/reinsertion approximation;
- hard step-cap censoring.

### F. Unresolved?

The remaining mathematical question is whether the interface/capture mechanism merely causes inefficiency or also introduces a significant systematic transport bias.

That has not yet been answered.

---

# 7. Why small SiC diffusivity alone cannot explain the observed trajectories

This point is particularly important.

The trajectories report:

[
N_{SiC}=0
]

for all three targeted histories.

There were also:

[
N_{IPyC\rightarrow SiC}=0.
]

Thus the observed 200,000-step computational path does not contain any SiC residence at all.

A physical SiC residence time cannot be the direct cause of those specific 200,000-step trajectories because the histories never enter SiC.

Small SiC (D) can influence the probability of eventually entering/crossing the barrier, because the interface probability depends on (D), but that is a different mechanism.

Therefore the statement:

> “the million-step pathology is simply because SiC has an extremely small diffusivity”

is not supported by the targeted evidence.

The more defensible statement is:

> **The present step-count pathology is dominated by repeated interface resolution before meaningful penetration into the deeper layers, with diffusivity contrast influencing crossing probabilities and eventual physical residence times.**

---

# 8. Capture → interface decision → reinsertion cycle

The implementation executes:

[
\text{capture}
\rightarrow
\text{interface decision}
\rightarrow
\text{reinsertion}
\rightarrow
\text{new WOS hop}.
]

The observed equality:

[
N_{interface}=N_{zero\text{-}dt}
]

is exactly what the implementation semantics predict.

With:

[
\epsilon=10\,nm,
qquad
\alpha=2,
]

the reinsertion displacement is:

[
\delta=20\,nm.
]

The Buffer thickness is:

[
100\,\mu m.
]

Therefore:

[
\frac{100\,\mu m}{20\,nm}=5000.
]

The next ordinary WOS sphere after reinsertion can consequently begin on a length scale vastly smaller than the full layer thickness.

Repeated interface encounters are therefore mathematically plausible.

This supports the observed computational bottleneck.

It does **not** by itself show that the scheme is biased.

The correct classification is:

[
\boxed{
\text{mathematically defined finite-}\epsilon\text{ approximation with potentially large computational overhead; bias unresolved}
}
]

No production-code modification is justified by these diagnostics alone.

---

# 9. R2-B01 status

## **R2-B01 remains OPEN**

The earlier blocker concerned whether the claimed production five-layer release estimator is actually supported by an executed, uncensored five-layer release-time ensemble.

The new diagnostic work improves the evidence substantially:

- `walk_until_released` returns a concrete release time;
- max-step censoring is explicitly represented;
- the five-layer production path is exercised;
- the current computational bottleneck is localized.

But the Stage-A ensemble remains:

[
24/24
]

censored.

The targeted histories also remain uncensored only in the sense that they intentionally terminate at 200,000 steps for diagnosis; none reaches release.

Therefore there is still no successful five-layer release-time ensemble from which a production release CDF can be compared against a continuum reference.

R2-B01 therefore **cannot be closed**.

---

# 10. Important findings

## R2-WOS-01 — MAJOR

### Interface-resolved computation is strongly implicated as the dominant step-count bottleneck, but the evidence does not establish mathematical bias.

**Evidence:** approximately 30,800 interface events in each 200,000-step targeted trajectory; nearly all transitions remain in Buffer; zero SiC entry.

**Consequence:** the current five-layer WOS path is computationally dominated by interface resolution.

**Required remediation:** controlled interface benchmark with capture-epsilon refinement.

**Closure evidence:** observable convergence as epsilon decreases against an independent continuum reference.

---

## R2-WOS-02 — MODERATE

### 100% Stage-A censoring prevents release-CDF verification.

**Evidence:** (24/24) histories reach 5,000,000 steps without release.

**Consequence:** the present production ensemble cannot estimate a complete release-time distribution.

**Required remediation:** obtain an uncensored or explicitly censored release-time ensemble sufficient for the intended observable.

**Closure evidence:** a five-layer run with explicit censor classification and a defined treatment of any remaining censored histories.

---

## R2-WOS-03 — MODERATE

### Interface equilibrium test is too narrow to establish transient PDE equivalence.

**Evidence:** one reflecting two-region geometry, one equilibrium observable, one finite epsilon, one tolerance criterion.

**Consequence:** the test validates only an equilibrium semantic property.

**Required remediation:** retain it as a diagnostic and add a non-equilibrium transient two-layer continuum comparison.

**Closure evidence:** release/transient concentration comparison under epsilon refinement.

---

## R2-WOS-04 — MINOR

### Diagnostic does not directly report the interface-event outcome breakdown.

The transition matrix allows reconstruction that most interface events were reflections, but the diagnostic does not print:

- transmitted events;
- reflected events;
- interface-by-interface counts.

**Consequence:** slightly weaker diagnostic traceability.

**Required remediation:** future diagnostic should print direct event classification.

**Closure stage:** implementation/diagnostic refinement; not required to interpret the current result.

---

# 11. One highest-value next experiment

## Controlled two-layer transient interface benchmark with capture-epsilon refinement

This is the single highest-value next experiment because it isolates the unresolved question without immediately repeating the large five-layer ensemble.

Use a two-material concentric sphere with:

[
D_1\neq D_2,
]

a known interface radius, and a deterministic FV continuum reference.

Choose a non-equilibrium initial condition so that transient interface transport matters.

Run the **unchanged WOS production algorithm** over:

[
\epsilon,
\quad
\epsilon/2,
\quad
\epsilon/4,
\quad \ldots
]

while controlling the reinsertion factor.

Measure:

- transient concentration/release observable;
- interface crossing counts;
- censoring;
- Monte-Carlo uncertainty;
- epsilon dependence;
- reinsertion dependence.

The key question is:

[
\boxed{
\text{Does the WOS solution converge toward the deterministic interface problem as }\epsilon\rightarrow0?
}
]

That experiment separates:

[
\text{computational overhead}
]

from

[
\text{systematic transport bias}.
]

It is more informative than immediately increasing the five-layer history count.

---

# 12. What should NOT be concluded from the current diagnostics

The following claims are **not supported**:

- “WOS is mathematically wrong.”
- “The SiC diffusivity alone causes the million-step failure.”
- “The interface rule has been validated.”
- “The five-layer release curve has been validated.”
- “100% censoring proves the implementation is defective.”
- “The equilibrium test proves transient multilayer correctness.”
- “The WOS model agrees with FV.”

The evidence supports only narrower statements.

---

# 13. Branch/repository safety

The audit was performed against project `main`.

The supervisor repository was only read at pinned commit:

`8d31482d127e211614ebb3f66b1076e1ed6dea98`.

No supervisor source was modified.

No production implementation was altered during the audit.

---

# 14. Final independent assessment

### Homogeneous WOS

**Meaningful limited verification:** PASS as a homogeneous semantic/analytical check.

### Outer release semantics

**Verified:** the implementation has an explicit absorbing OPyC release event.

### Interface equilibrium

**Supported:** approximate equilibrium consistency in the tested two-region case.

**Not established:** transient PDE equivalence.

### Five-layer censoring

**Strongly localized:** computational effort is dominated by repeated interface handling before SiC traversal.

### Root cause

[
\boxed{\textbf{E — MIXED}}
]

with interface-resolution behaviour the strongest explanation for the observed **computational step-count pathology**, physical diffusivity contrast a legitimate contributor to crossing/residence physics, and the step cap a direct cause of estimator censoring.

### R2-B01

[
\boxed{\textbf{OPEN}}
]

No evidence justifies closing it.

### Highest-value next experiment

[
\boxed{
\textbf{two-layer non-equilibrium transient WOS-vs-FV benchmark with capture-epsilon refinement}
}
]

### Gate

[
\boxed{
\textbf{DIAGNOSTIC EVIDENCE SUFFICIENT TO LOCALISE THE PATHOLOGY, BUT FULL METHOD-2 VERIFICATION IS NOT YET ESTABLISHED}
}
]

This audit does not authorize a broad five-layer production claim and does not justify starting Method 3.
