# Controlled Three-Layer Multistate Matrix Verification — Executed Evidence

## Provenance

- Successful run: `36815849244`
- Research commit: `9b4bc635eedbf7c581184f96a2f8607624e152fc`
- Supervisor commit: `8d31482d127e211614ebb3f66b1076e1ed6dea98`
- Artifact: `three-layer-multistate-matrix-results`
- Artifact ID: `11142115303`
- Artifact SHA-256: `e26ec7bd4bc6d153f3780a4891bf98693b83041b42e05e8ebae4543181e051e8`

Prior run `36815342086` is execution-invalid scientific evidence because the harness did not compile due to shorthand Rust float literals. The workflow correctly failed. No numerical result from it is used.

## Frozen benchmark

R=[50,75,100] um.
D=[1e-9,2e-9,5e-9] m2/s.
K=1; epsilon=100 nm; alpha=2.
Uniform-volume inner-sphere initial distribution.
N=20,000 explicit Process-B histories.
Transform s=[0,1,2,5,10,20] s^-1.
CDF times=[.02,.05,.10,.20,.40,.80] s.

## B versus C transform verification

| s | C matrix | B explicit | B SE | B-C | z | reported conditioning | residual |
|---:|---:|---:|---:|---:|---:|---:|---:|
|0|1.000000000000|1.000000000000|0|~-1.1e-15|0|392.48|1.12e-16|
|1|.610894564738|.610257673058|.00143408|-6.37e-4|-0.444|316.99|1.24e-16|
|2|.414440336362|.413544255439|.00162945|-8.96e-4|-0.550|273.42|5.59e-17|
|5|.174553542011|.173615255224|.00128713|-9.38e-4|-0.729|206.93|4.00e-17|
|10|.063769458570|.063274883544|.00075746|-4.95e-4|-0.653|159.81|5.83e-17|
|20|.015602361812|.015478161280|.00031454|-1.24e-4|-0.395|120.20|1.39e-17|

Maximum |z|=0.729.

All four transient states at s=0 equal one to ~1e-14 and Phi_init(0)=1 to floating-point precision.

Residual criterion 1e-10: PASS.

Note: the harness reports a Frobenius-norm condition estimate ||A||_F ||A^-1||_F, not an exact spectral 2-norm condition number. It is retained as a conservative conditioning diagnostic and must not be mislabeled cond_2.

## FV refinement

Fine resolved reference uses 200 cells/layer and q=0.2 timestep.

Spatial F at 50,100,200 cells/layer converges consistently. Maximum 100->200 change is below ~9.2e-6.

Fine timestep:
- q=.4 dt=4.16377e-7 s
- q=.2 dt=2.081885e-7 s
- maximum CDF change = 1.345984e-7.

Inventory residuals remain ~4.7e-15 to 4.9e-15.

Therefore FV discretization changes are much smaller than the N=20,000 stochastic uncertainty.

## Explicit B versus resolved FV CDF

| t s | B | FV | B-FV | B SE | z approx |
|---:|---:|---:|---:|---:|---:|
|.02|0|1.0571e-6|-1.06e-6|0|n/a|
|.05|.001650|.00175769|-1.08e-4|.00028699|-0.375|
|.10|.031900|.03061954|+1.280e-3|.00124263|+1.030|
|.20|.164200|.16484981|-6.498e-4|.00261953|-0.248|
|.40|.445500|.44727944|-1.779e-3|.00351447|-0.506|
|.80|.775750|.77463498|+1.115e-3|.00294925|+0.378|

RMS CDF difference = 0.0010395.
Maximum absolute difference = 0.0017794.

No nonzero-time discrepancy exceeds ~1.03 B binomial SE. No coherent signed bias is visible.

At t=.02 the explicit sample has zero releases, so the plug-in binomial SE is zero and a z-score is not meaningful; the FV probability is only ~1.06e-6, corresponding to an expected ~0.021 releases in N=20,000.

## Topology / probability audit

At s=0, the executed state solution normalizes all transient states to one. The documented four-state table includes both exits of material 2, both outcomes at each internal interface, and the direct material-3 outer release branch. No execution evidence indicates an omitted/duplicated transition or row/column orientation defect.

## Gate assessment

Controlled genuine-multistate verification PASSES at the declared precision:

- B/C transform agreement: PASS;
- normalization: PASS;
- residual: PASS;
- conditioning: measured and understood for this controlled 4x4 system;
- FV spatial/time refinement and conservation: PASS;
- explicit-B/FV release CDF compatibility: PASS;
- no unresolved multistate topology defect identified.

This is candidate evidence for independent review. It does NOT authorize the hard frozen five-layer Cs-137 matrix without that review.

R2-WOS-02 remains OPEN.
R2-B01 remains OPEN.
