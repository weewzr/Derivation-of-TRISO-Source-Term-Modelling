# Equation Dependency Tree

## Continuous model
Physical conservation -> species inventory -> control volume -> outward flux -> Fick first law -> general heterogeneous diffusion equation -> spherical coordinates -> spherical symmetry -> radial conservative PDE -> five material domains -> centre condition -> interface flux conservation -> partition relation -> outer boundary -> initial condition -> complete continuous model.

## Analytical branch
Radial PDE -> steady/transient decomposition -> separation of variables -> radial eigenproblem -> substitution u=r phi -> one-dimensional Helmholtz equation -> layer solutions -> common modal decay rate -> interface equations -> boundary equations -> homogeneous coefficient system -> determinant condition -> weighted orthogonality -> modal projection -> qualified modal expansion -> analytical checks.

## Conservative finite-volume branch
Continuous conservative PDE -> spherical control volume -> time integral -> radial divergence integral -> face fluxes -> spherical face areas -> shell volume -> face gradient approximation -> Fick face flux -> discontinuous-diffusivity series resistance -> interior balance -> centre cell -> interface-adjacent cells -> outer cell -> coefficient collection -> semidiscrete matrix -> explicit time advance -> stability -> conservation -> spatial refinement -> actual-timestep temporal refinement -> executed verification.


## WOS and first-passage branch
Diffusion PDE -> Brownian diffusion -> generator -> backward equation -> first-exit stopping time -> spherical first-passage ODE -> centred-ball first-exit problem -> shell first-exit problem -> substitution v=rG -> constant-coefficient ODE -> boundary conditions -> outer joint transform -> inner joint transform -> zero-frequency exit probabilities -> conditional first-passage transforms -> conditional moments/distributions -> direct WOS -> interface transmission/reflection -> finite capture -> zero-time interface event -> reinsertion -> accelerated exact-interface renewal -> controlled verification.

## Markov-renewal branch
Post-interface side states -> enumerate each possible region exit -> multiply joint first-passage transform by interface outcome probability -> construct each K_ij(s) -> construct direct absorption vector B(s) -> define state transforms Phi_i(s) -> first-step equations -> Phi=K Phi+B -> Phi-K Phi=B -> (I-K)Phi=B -> Phi=(I-K)^-1 B -> Neumann expansion -> repeated-path interpretation -> uniform-volume initial integration -> two-state reconciliation -> four-state multistate verification -> five-layer eight-state transform -> conditioning diagnostic -> high-precision verification -> independent hard-matrix closure audit.

## Process distinction
Process A is finite-capture production WOS.
Process B is explicit accelerated exact-interface renewal.
Process C is the deterministic matrix reduction of Process B.
B and C have an exact mathematical-reduction target. A versus B/C is controlled empirical compatibility at the declared finite-epsilon precision.

## Current evidence boundary
The hard eight-state transform is independently verified with non-blocking findings. A separately predeclared controlled transform-to-time verification stage is authorized but has not yet been executed. R2-WOS-02 and R2-B01 remain open.
