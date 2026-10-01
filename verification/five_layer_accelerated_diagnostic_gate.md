# Bounded Five-Layer Accelerated Markov-Renewal Diagnostic Gate

## Independent authorization

Controlling immutable audit: `reviews/independent_accelerated_first_passage_audit.md`.

Gate: **A. VERIFIED FOR CONTROLLED TWO-LAYER USE AND CLEARED FOR BOUNDED FIVE-LAYER INVESTIGATION.**

This document authorizes only a bounded five-layer diagnostic, not production release-CDF validation.

## Frozen physical benchmark

- nuclide: Cs-137;
- initial births: uniform in kernel volume, r=Rk U^(1/3);
- radii (m): [2.125e-4, 3.125e-4, 3.525e-4, 3.875e-4, 4.275e-4];
- diffusivities (m2/s): [1.2502982636347968e-13, 1.0e-8, 4.062299125614697e-14, 9.227773168241615e-17, 4.062299125614697e-14];
- K=1 at all internal interfaces;
- production transmission probability p(i->j)=Dj/(Di+Dj);
- capture epsilon represented by renewal interface state: 100 nm;
- reinsertion factor alpha=2, so delta=200 nm;
- absorbing OPyC exterior;
- no continuing source and no reaction.

Epsilon=100 nm is fixed before execution because it lies inside the independently accepted controlled direct-WOS plateau. No accelerated epsilon convergence order is claimed.

## Renewal states

State is (layer, radius, accumulated physical time, RNG state).

### Kernel

Region 0 <= r < Rk.

Use exact centered-ball first-exit-time kernel to Rk. The next event is necessarily Kernel/Buffer interface encounter.

### Buffer

Shell Rk < r < Rb.

Use exact joint shell kernel to sample:
- inner exit to Kernel/Buffer interface; or
- outer exit to Buffer/IPyC interface;
- associated physical first-passage time.

### IPyC

Shell Rb < r < Ripyc.

Use exact joint shell kernel to Kernel-facing Buffer/IPyC or outward IPyC/SiC interface plus physical time.

### SiC

Shell Ripyc < r < Rsic.

Use exact joint shell kernel to IPyC/SiC or SiC/OPyC plus physical time.

### OPyC

Shell Rsic < r < R.

Use exact joint shell kernel to SiC/OPyC or absorbing outer radius R plus physical time.

Outer exit from OPyC is `Released(t)`.

## Interface renewal

At internal interface i/j:

1. shell/ball first-passage time has already been accumulated;
2. call the pinned production `does_transmit(D_current,D_next,K=1)`;
3. interface event adds zero physical time;
4. on transmission, renew at alpha*epsilon inside the next layer;
5. on reflection, renew at alpha*epsilon inside the current layer;
6. continue with the corresponding exact region kernel.

This marginalises homogeneous-region wandering only. It retains the verified stochastic interface law and finite reinsertion convention.

## Series approximation

Each conditional region-time sampler uses 2000 terms.

Claim retained:
- explicit mean-tail control exists;
- distribution-level accuracy is supported by executed homogeneous-shell CDF verification;
- no uniform analytical CDF truncation-error bound is claimed.

## Bounded diagnostic design

Predeclare N=16 histories.

Rationale: this is an execution/semantic/computational diagnostic, not a release-fraction estimate. Sixteen histories are sufficient to expose state-machine failures, censoring, layer penetration and order-of-magnitude renewal cost while remaining deliberately non-production.

Maximum renewals/history: 100,000.

This is a computational safeguard only. Exhaustion is `CensoredMaxRenewals`, never silently interpreted as physical non-release.

For each history record:
- initial radius;
- renewal count;
- layer/state before renewal;
- exit side;
- physical time increment;
- cumulative physical time;
- interface;
- transmission/reflection;
- transition direction;
- reinsertion layer/radius;
- deepest layer;
- Released(time), CensoredMaxRenewals, or explicit error.

Aggregate:
- released/censored/errors;
- renewals/history and by layer;
- encounters/transmit/reflect by interface;
- deepest-layer distribution;
- release-time range;
- execution time.

## Predeclared computational-usability gate

The accelerated five-layer composition is computationally usable for a subsequent staged ensemble only if ALL hold:

1. no abnormal/error termination;
2. explicit censoring accounting;
3. complete-stack progression is demonstrated: at least one history enters SiC and at least one enters OPyC or releases;
4. renewal counts do not universally approach the 100,000 cap;
5. physical time increments remain nonnegative and cumulative time advances only through region first-passage events, not interface events;
6. histories entering low-D SiC exhibit physical-time accumulation consistent in order of magnitude with its long diffusion scale rather than zero-time collapse;
7. observed renewal cost is sufficiently bounded that a later staged ensemble can be sized without blind cap inflation.

Success means computational usability only. It does not validate the five-layer release-time CDF and does not close R2-B01.

## Direct-production comparison

Direct run 36735378843:
- 8/8 censored at 1,000,000 production steps/history;
- all terminate in Buffer;
- deepest layer IPyC;
- no SiC/OPyC penetration.

The bounded accelerated diagnostic will compare renewal count and layer progression against that direct result. Wall-clock workflow duration is not the primary metric.
