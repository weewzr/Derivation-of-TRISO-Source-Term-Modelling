# Five-Layer Integrated WOS Usability Diagnostic Gate

## Purpose

This gate returns to the actual frozen five-layer production WOS path after independent closure of the controlled two-layer interface milestone.

It does not change the physical model, capture epsilon, reinsertion factor, partition coefficient, diffusivities, geometry, or supervisor source.

## Established premise

The controlled two-layer audit `reviews/independent_method2_wos_interface_closure_audit.md` closes the finite-epsilon interface milestone through an epsilon-independent plateau at declared statistical precision. This does not establish integrated five-layer correctness.

## Frozen five-layer problem

- CRP-6 concentric five-layer geometry;
- Cs-137;
- initial births uniform in kernel volume;
- no continuing source;
- production material-property diffusivities at frozen benchmark state;
- K=1;
- capture epsilon=10 nm;
- reinsertion factor alpha=2;
- absorbing OPyC outer surface;
- actual supervisor `WoSWalker::step_multilayer`;
- Released and CensoredMaxSteps remain distinct.

Characteristic L^2/D scales from executed Stage-A properties are retained only as physical diagnostics:
kernel ~3.61e5 s; Buffer ~1 s; IPyC ~3.94e4 s; SiC ~1.33e7 s; OPyC ~3.94e4 s.

## Bounded diagnostic

N=8 histories, maximum 1,000,000 production steps/history.

The verification harness records:
- total steps and physical time;
- steps by layer;
- interface encounters by interface;
- harness-classified transmit/reflection and direction;
- zero-time versus positive-time steps;
- displacement statistics;
- maximum radius and deepest layer reached;
- current layer at termination;
- Released versus CensoredMaxSteps.

No censored history is reclassified as unreleased.

## Computational-usability decision criterion

A full five-layer release ensemble is NOT justified merely by observing some release.

Proceed to a staged release ensemble only if this bounded diagnostic demonstrates all of:

1. no abnormal termination other than explicit Released/CensoredMaxSteps;
2. measurable integrated progression beyond Buffer into the deeper stack, including SiC/OPyC in at least some histories, or actual release;
3. step/interface statistics show histories are not universally trapped in a single near-interface cycle at the 1,000,000-step scale;
4. observed step throughput and progression permit a bounded next-stage ensemble whose censoring can be measured without simply raising max_steps blindly.

If all histories remain confined to Fuel/Buffer/IPyC with interface events dominating and no SiC penetration, the path is classified computationally unusable at the frozen 10-nm configuration for a release-CDF ensemble, and the next action is estimator/integration-design analysis rather than a larger ensemble.

This is a computational-usability gate, not WOS validation and not R2-B01 closure.
