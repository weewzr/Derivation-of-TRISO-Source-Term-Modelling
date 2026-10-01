# Five-Layer Accelerated Bounded Diagnostic — Executed Evidence

## Provenance

- Run: `36801138769`
- Workflow: `Five-layer accelerated bounded diagnostic`
- Conclusion: success (workflow only)
- Research commit: `c878db1b9342498bc81ab13431bffff673100a65`
- Supervisor commit: `8d31482d127e211614ebb3f66b1076e1ed6dea98`
- Artifact: `five-layer-accelerated-diagnostic-results`
- Artifact ID: `11136436088`
- Artifact SHA-256: `41348848f99c3ebf7e319165c81d732b32334580bf31b4f0031615ff82565e3e`

Frozen: Cs-137; N=16; max renewals=100,000/history; 2000-term region kernels; epsilon=100 nm; alpha=2; K=1; radii [212.5,312.5,352.5,387.5,427.5] um; diffusivities [1.2502983e-13,1e-8,4.0622991e-14,9.2277732e-17,4.0622991e-14] m2/s.

## Executed result

- released: 0
- CensoredMaxRenewals: 16
- errors: 0
- mean renewals/history: 100,000
- renewals by layer: [24, 1,599,974, 2, 0, 0]
- interface encounters: [534,495, 1,065,505, 0, 0]
- transmitted: [32, 4, 0, 0]
- reflected: [534,463, 1,065,501, 0, 0]
- deepest distribution [Kernel,Buffer,IPyC,SiC,OPyC]: [0,14,2,0,0]
- harness execution: 7.988 s after compilation
- physical accumulated times range from ~1.03e2 s to ~6.42e4 s.

No history enters SiC or OPyC. No history releases.

## Gate decision

**FAILED — NOT COMPUTATIONALLY USABLE FOR A FIVE-LAYER RELEASE ENSEMBLE.**

The exact shell/ball acceleration successfully removes ordinary within-material wandering, but the complete five-layer process remains dominated by repeated rare interface-state renewals.

This is stronger localization than direct run 36735378843:
- direct: 8/8 censored at 1,000,000 production steps, all ending Buffer, deepest IPyC;
- accelerated: 16/16 censored at 100,000 renewals, all ending Buffer, deepest IPyC.

Therefore homogeneous-region WOS wandering is not the fundamental remaining bottleneck. The rare interface renewal process is.

R3-A01 remains OPEN: integrated five-layer composition has now been directly tested and shown computationally unusable under explicit renewal simulation. This is evidence about the limitation, not closure of five-layer verification.

R2-WOS-02 remains OPEN.
R2-B01 remains OPEN.
