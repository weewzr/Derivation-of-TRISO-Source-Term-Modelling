# Final Derivation-Quality Audit

Baseline audited: successful render run 36875538607, commit 2151da0696555c7eeddd3b84bd49c64f74ec1cdf, followed only by targeted missing-step remediation.

Audit standard: physical principle -> continuous equation -> material intermediate manipulations -> final continuous equation -> discretisation -> algorithm -> implementation -> verification.

| Section | Status | Equation range / transition | Original-source relationship | Code mapping | Verification mapping | Remediation performed |
|---|---|---|---|---|---|---|
| Conservation -> Fick -> spherical PDE | PASS | TRISO-GOV-020..040; TRISO-SPH-020..041 | EXPANDED MISSING STEPS from Ray notebook | continuum equations feed FV/WOS implementations | independent continuous-mathematics audit | none |
| Centre boundary | PASS | TRISO-BC-100..109 | EXPANDED MISSING STEPS | centre FV row in verification/fv_convergence.rs | continuous + discrete audits | none |
| Material interfaces | PASS | TRISO-INT-100..118 | CORRECTED/EXPANDED: conservation-derived flux continuity separated from partition/resistance assumptions | FV face resistance; WOS contract kept epistemically separate | continuous/discrete audits | none |
| Analytical radial transformation/eigenproblem | PASS | TRISO-ANA families in manuscript | UNCHANGED LOGIC where correct; EXPANDED MISSING STEPS; demonstrated boundary/source distinctions corrected | analytical reference/oracle evidence | continuous-mathematics audit | none |
| Conservative FV derivation | PASS | TRISO-FV-100..257 and stability/convergence families | NEW VERIFIED EXTENSION of original FTCS path | verification/fv_convergence.rs; two_layer_fv_refinement.rs | accuracy/convergence audit | none |
| WOS PDE -> generator -> backward problem -> algorithm | PASS | TRISO-FPT-001..010; TRISO-WOS-001..010 | NEW VERIFIED EXTENSION | frozen supervisor WOS contract; verification/r2_b01_supervisor_integration.rs; WOS harnesses | controlled WOS/FV audits | none |
| Centred-ball first passage | PASS after remediation | TRISO-FPT-011..019E | NEW VERIFIED EXTENSION | accelerated coupling / shell-kernel verification | accelerated first-passage evidence | Added backward ODE -> v=rH -> regularity -> boundary -> H derivation |
| Spherical-shell first passage | PASS | TRISO-FPT-020..065 | NEW VERIFIED EXTENSION | spherical_shell_kernel_verification.rs; spherical_shell_cdf_verification.rs | shell probability/moment/CDF evidence | none |
| Interface transmission/reflection | PASS WITH SCOPE QUALIFICATION | TRISO-WOS-005..010 | NEW VERIFIED EXTENSION; production law is INFERRED FROM CODE, not claimed as continuum theorem | pinned production interface implementation and verification adapters | controlled two-layer finite-epsilon verification | Reflection complement derived explicitly; transmission law retained as implementation contract rather than inventing unsupported continuum derivation |
| Accelerated exact-interface renewal | PASS | TRISO-WOS-020..033 plus FPT kernels | NEW VERIFIED EXTENSION | two_layer_accelerated_coupling.rs; spherical-shell verification | independent accelerated-first-passage audit | none |
| Markov-renewal state construction | PASS after remediation | TRISO-MR-001..050 | NEW VERIFIED EXTENSION | interface_state_markov_renewal_derivation.md; five_layer_interface_state_transition_table.md | two-layer/three-layer matrix audits | Added explicit S2,S3,S4,S5,S6,S7 first-step equations and every nonzero K entry |
| Matrix solve | PASS | TRISO-MR-022..027 | NEW VERIFIED EXTENSION | two_layer_matrix_reconciliation.rs; hard_five_layer_matrix.rs | independent matrix audits | none |
| Neumann-series/path interpretation | PASS | TRISO-MR-028..030 | NEW VERIFIED EXTENSION | deterministic matrix reduction | matrix closure audits | none |
| Uniform-volume source integration | PASS | TRISO-WOS-001..004; TRISO-MR-031..033 | EXPANDED MISSING STEPS / NEW VERIFIED EXTENSION | hard_five_layer_high_precision.py and matrix harnesses | hard-matrix audit | none |
| Two-layer matrix reconciliation | PASS after remediation | TRISO-VER-300..303 | NEW VERIFIED EXTENSION | two_layer_matrix_reconciliation.rs | two-layer reconciliation evidence | Added N, standardized discrepancy and normalization equations |
| Three-layer multistate verification | PASS after remediation | TRISO-VER-304..308 | NEW VERIFIED EXTENSION | controlled three-layer multistate harness/evidence | independent three-layer audit | Added geometry, diffusivities, B/C z metric and FV discrepancy equations |
| Hard five-layer 8-state matrix | PASS | TRISO-MR rows + hard benchmark evidence | NEW VERIFIED EXTENSION | hard_five_layer_matrix.rs | independent hard-matrix closure audit | full row construction completed above |
| High-precision conditioning verification | PASS after remediation | TRISO-HARD-001..015 | NEW VERIFIED EXTENSION | hard_five_layer_high_precision.py | run 36869649217 + independent hard-matrix closure audit | Added f64 normalization error, spectral gap, singular-value condition number and high-precision residual chain |

## Findings

### PASS sections
All 17 requested mathematical sections pass the reproducible derivation standard after the targeted additions recorded above.

### Minor gaps
No remaining mathematical intermediate-step gap blocks reproduction.

One scope qualification remains: the production transmission probability p(i->j)=D_j/(D_i+D_j) is a pinned production-algorithm contract inferred from code and verified empirically. The current project sources do not establish it as a theorem derived from the ideal continuum interface conditions. The manuscript states that distinction explicitly.

### Major gaps
None remain in the audited derivation chain.

### Incorrect equations
No demonstrated incorrect equation was found in this final pass.

## Code traceability

- FV face flux / discontinuous diffusivity -> TRISO-FV conductance equations -> verification/fv_convergence.rs and verification/two_layer_fv_refinement.rs -> deterministic accuracy/convergence evidence.
- WOS first passage -> TRISO-FPT / TRISO-WOS equations -> frozen supervisor contract plus repository WOS verification harnesses -> controlled WOS/FV evidence.
- Shell kernel -> TRISO-FPT-020..065 -> verification/spherical_shell_kernel_verification.rs and spherical_shell_cdf_verification.rs -> executed shell evidence.
- Interface transmission -> TRISO-WOS-005..010 -> pinned production implementation/adapters -> controlled finite-epsilon verification.
- Accelerated renewal -> exact ball/shell kernels -> verification/two_layer_accelerated_coupling.rs -> accelerated-coupling evidence/audit.
- K(s) construction -> TRISO-MR-001..050 -> verification/interface_state_markov_renewal_derivation.md and five_layer_interface_state_transition_table.md -> two-/three-layer matrix verification.
- 8x8 solve -> TRISO-MR-022..033 -> verification/hard_five_layer_matrix.rs -> hard transform evidence.
- High precision -> TRISO-HARD-001..015 -> verification/hard_five_layer_high_precision.py -> run 36869649217 and independent hard-matrix closure audit.

## Original-source classification

Ray's original notebook is preserved unchanged under notes/raw.

- Conservation, spherical PDE, homogeneous analytical solution, separation/eigenfunction path, and FTCS intent: UNCHANGED LOGIC where correct, with EXPANDED MISSING STEPS.
- Interface treatment and conservative discontinuous-D finite volume: CORRECTED DEMONSTRATED ERROR / ambiguity where the raw derivation was insufficient, with traceable canonical replacement.
- WOS, exact shell first passage, accelerated renewal, and Markov-renewal matrix: NEW VERIFIED EXTENSION.
- No correct original reasoning was silently replaced merely for brevity.

## Parity and rendering

Markdown and LaTeX are generated from the same scientific content. EQUATION_PARITY.md is refreshed after remediation. A final render audit must pass after these targeted additions before the baseline is frozen.

## Evidence boundary

The hard eight-state transform is independently verified with non-blocking findings. Controlled transform-to-time recovery is authorized but has not been executed.

R2-WOS-02 remains OPEN.
R2-B01 remains OPEN.

No inverse-Laplace result is part of this audit.
