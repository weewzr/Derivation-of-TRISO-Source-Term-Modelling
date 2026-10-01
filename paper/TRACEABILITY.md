# Manuscript Traceability

This table maps the canonical manuscript to repository derivation, executed evidence and independent review. Pending results are explicitly marked.

| Manuscript section / result | Canonical derivation / contract | Verification evidence | Workflow run | Independent review / status |
|---|---|---|---|---|
| Geometry, PDE, centre/interface/outer BCs | docs/triso/13_canonical_equation_register.md | canonical equation register | — | continuous foundation passed |
| Method 1 homogeneous and multilayer modal structure | docs/triso/13_canonical_equation_register.md | canonical analytical derivations registered there | — | verified/qualified per equation status |
| Conservative spherical FV | canonical DIS/ACC equations in equation register | verification/fv_convergence_results.txt and accuracy/convergence resolution | post-remediation FV run 36661249962 | independent accuracy/convergence audit passed for deterministic reference |
| Frozen production WOS contract | docs/triso/21_frozen_production_wos_contract.md | verification/r2_b01_supervisor_integration.rs and WOS diagnostics | multiple; see evidence files | R2-B01 OPEN |
| Controlled WOS/FV finite-epsilon verification | controlled two-layer benchmark/evidence | verification/two_layer_wos_high_power_evidence.md | canonical runs recorded in evidence | independent interface closure audit passed with non-blocking findings |
| Exact shell first-passage transforms | verification/spherical_shell_joint_first_passage_derivation.md | verification/spherical_shell_kernel_evidence.md; verification/spherical_shell_cdf_evidence.md | 36741901064; 36743542042 | prerequisite chain accepted |
| Accelerated two-layer renewal | exact ball/shell derivation + production interface contract | verification/two_layer_accelerated_coupling_evidence.md | 36746018907 canonical corrected run | independently verified for controlled two-layer use |
| Five-layer explicit accelerated pathology | verification/five_layer_accelerated_diagnostic_gate.md | verification/five_layer_accelerated_diagnostic_evidence.md | 36801138769 | computational-usability gate failed; R2-WOS-02/R2-B01 OPEN |
| Interface-state Markov-renewal equation Phi=(I-K)^-1 B | verification/interface_state_markov_renewal_derivation.md | verification/two_layer_matrix_reconciliation_evidence.md | 36807696742 | independent closure audit substantially closed controlled remediation |
| Genuine three-layer multistate matrix | verification/three_layer_multistate_matrix_gate.md | verification/three_layer_multistate_matrix_evidence.md | 36815849244 | independent three-layer audit passed; hard diagnostic authorized |
| Hard five-layer 8x8 transform | verification/hard_five_layer_matrix_gate.md | verification/hard_five_layer_matrix_evidence.md | 36866849598 | f64 numerical gate NOT closed |
| High-precision hard-matrix remediation | verification/hard_five_layer_high_precision_gate.md | **PENDING** | 36869649217 currently queued/pending at manuscript creation | ONGOING / NOT YET CLOSED |
| Final five-layer release CDF | — | **DOES NOT EXIST** | — | not authorized; R2-B01 OPEN |
