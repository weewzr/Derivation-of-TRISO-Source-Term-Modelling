# 20 — Production WOS Stochastic Formulation (Foundation Note)

This document is retained as the original Review-2 stochastic formulation. The exact production verification contract is now frozen in `docs/triso/21_frozen_production_wos_contract.md` and the derived numerical formulation is in `docs/triso/22_review2_numerical_formulation.md`.

The key conclusion remains:

- the production method is event-driven Walk-on-Spheres;
- it is not an FTCS mesh discretisation;
- its local bulk hop is based on homogeneous Brownian first-passage theory;
- its interface, boundary, initial-condition and approximation semantics are now fixed for the Review-2 verification benchmark.

No production WOS code was changed in this foundation stage.