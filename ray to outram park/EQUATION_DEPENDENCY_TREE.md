# Equation Dependency Tree

## Continuous model
Physical conservation -> species inventory -> control volume -> outward flux -> Fick first law -> general heterogeneous diffusion equation -> spherical coordinates -> spherical symmetry -> radial conservative PDE -> five material domains -> centre condition -> interface flux conservation -> partition relation -> outer boundary -> initial condition -> complete continuous model.

## Analytical branch
Radial PDE -> steady/transient decomposition -> separation of variables -> radial eigenproblem -> substitution u=r phi -> one-dimensional Helmholtz equation -> layer solutions -> common modal decay rate -> interface equations -> boundary equations -> homogeneous coefficient system -> determinant condition -> weighted orthogonality -> modal projection -> qualified modal expansion -> analytical checks.

## Conservative finite-volume branch
Continuous conservative PDE -> spherical control volume -> time integral -> radial divergence integral -> face fluxes -> spherical face areas -> shell volume -> face gradient approximation -> Fick face flux -> discontinuous-diffusivity series resistance -> interior balance -> centre cell -> interface-adjacent cells -> outer cell -> coefficient collection -> semidiscrete matrix -> explicit time advance -> stability -> conservation -> spatial refinement -> actual-timestep temporal refinement -> executed verification.
