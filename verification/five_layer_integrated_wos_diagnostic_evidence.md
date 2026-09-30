# Five-Layer Integrated Production-WOS Diagnostic — Executed Evidence

## Role

This is an integrated production-path computational-usability diagnostic for R2-B01. It is not a release-CDF validation and does not close R2-B01.

## Provenance

- Workflow: `Five-layer integrated WOS diagnostic`
- Run: `36735378843`
- Conclusion: success
- Research commit: `74c9d11a94bba0abf73055941076b6df01b7973b`
- Supervisor commit: `8d31482d127e211614ebb3f66b1076e1ed6dea98`
- Artifact: `five-layer-integrated-wos-diagnostic-results`
- Artifact ID: `11107067525`
- Artifact SHA-256: `2e00a22f5abeaf38c4bba04a37fb04187acbb23122ad5b08d387c37afd4f1bca`

Frozen production parameters: Cs-137; CRP-6 five-layer geometry; capture epsilon 10 nm; alpha=2; K=1; N=8; cap=1,000,000 steps/history; initial births uniform in kernel volume; actual supervisor `WoSWalker::step_multilayer`.

## Result

All 8 histories terminate as `CensoredMaxSteps`. No history releases.

Every history terminates in Buffer and the deepest layer reached is IPyC. No history reaches SiC or OPyC.

Layer-step counts are dominated by Buffer:

- h0 [28,999958,14,0,0,0]
- h1 [24,999967,9,0,0,0]
- h2 [15,999969,16,0,0,0]
- h3 [40,999954,6,0,0,0]
- h4 [38,999951,11,0,0,0]
- h5 [48,999942,10,0,0,0]
- h6 [22,999959,19,0,0,0]
- h7 [18,999974,8,0,0,0]

Interface-event counts per history are approximately 153,000–154,000. Every such event has zero simulated time, consistent with production semantics.

Only history 6 records one outward Buffer→IPyC crossing; the other IPyC excursions return to Buffer. No IPyC→SiC transmission occurs.

Accumulated physical time at the step cap ranges from approximately 1.15e3 s to 2.75e4 s. This is far below the diagnostic SiC L^2/D scale (~1.33e7 s), but the walkers never enter SiC, so SiC residence time cannot be the direct cause of this observed computational termination.

Mean spatial displacement per production step is about 0.098–0.108 um, with minimum positive displacement approximately 10 nm, consistent with finite capture/reinsertion resolution dominating a large fraction of computational work.

The eight histories complete in about 0.965 s of harness execution after compilation; raw CPU throughput is therefore high. The blocker is not wall-clock cost per million steps by itself but extremely poor progress through the physical layer stack per step budget.

## Classification

The integrated five-layer path at the frozen 10-nm configuration is **NOT COMPUTATIONALLY USABLE FOR A RELEASE-CDF ENSEMBLE UNDER THE CURRENT STEP-based estimator design**.

Evidence:
1. 8/8 censored at one million steps;
2. 8/8 terminate in Buffer;
3. zero SiC/OPyC entry;
4. approximately 15.3% of all steps are zero-time interface events;
5. nearly all remaining steps are ordinary Buffer hops;
6. physical time does advance, but deeper-stack penetration is negligible.

This does not establish a defect in the already-verified controlled interface primitives. It establishes an integrated five-layer efficiency/estimator-design limitation under the frozen production geometry and diffusivity hierarchy.

## Usability gate

The predeclared gate in `verification/five_layer_integrated_wos_diagnostic_gate.md` is FAILED.

Therefore a larger five-layer history ensemble or blind max-step increase is not scientifically justified next.

## R2-B01

R2-B01 remains OPEN.

The smallest next scientific task is integrated estimator/control-flow analysis aimed at preserving the frozen physical process while determining whether the repeated finite-interface-resolution sequence can be accelerated or represented equivalently without changing the verified stochastic interface law. Any such change must be treated as a new numerical implementation requiring its own verification before use.
