# Independent Audit — Accelerated Method-2 First-Passage Estimator

Repository: weewzr/Derivation-of-TRISO-Source-Term-Modelling
Canonical branch: main
Reviewed repository commit: 33ea4f1c462afa98e7e55ea5fc35549fcbfc833b
Supervisor repository: theodoreOnzGit/outram-park-backend
Supervisor commit used by executed runs: 8d31482d127e211614ebb3f66b1076e1ed6dea98
Scope: independent audit of the accelerated radial first-passage / Markov-renewal estimator

# Executive decision

A. VERIFIED FOR CONTROLLED TWO-LAYER USE AND CLEARED FOR BOUNDED FIVE-LAYER INVESTIGATION

The accelerated estimator has passed the controlled mathematical and numerical gates required by the current verification hierarchy.

This is not five-layer validation and does not close R2-B01.

# 1. Exact shell mathematics

For a < r < b with constant D, the backward transform for outer exit is

G_b(r,s) = E_r[ exp(-sT) 1{R_T=b} ]

with

D(G_b'' + 2G_b'/r) = s G_b,
G_b(a,s)=0, G_b(b,s)=1.

Setting v=r G_b gives v'' - lambda^2 v = 0 with lambda=sqrt(s/D). Therefore

G_b(r,s) = (b/r) sinh(lambda(r-a))/sinh(lambda(b-a)).

Likewise

G_a(r,s) = (a/r) sinh(lambda(b-r))/sinh(lambda(b-a)).

The s -> 0 limit recovers the harmonic exit probabilities. Dimensions, boundary values, normalization, and conditionalization are correct.

Verdict: PASS.

# 2. Conditional-time sampler

The implemented conditional-time construction uses q equal to the squared fractional distance from the opposite boundary and a sum of independent exponential contributions with scale (b-a)^2/(D pi^2 n^2).

Independently taking the Laplace transform of that random sum gives the exact shell conditional transform through the standard infinite-product identity for sinh. Thus the infinite-term construction is exact, not merely moment-matched.

The actual implementation truncates at 2000 terms. The repository supplies a mean-tail bound of about 9.50e-6 s for the shell benchmark, while full CDF accuracy is checked separately.

Verdict: PASS, with finite-series truncation retained as an explicit numerical approximation.

# 3. Centered-inner-ball defect and correction

Run 36745817586 at commit 93795db38d1cf58fd466ce71f0080639dfd06caa was genuinely incorrect. It treated the current distance A-r as if it were the radius of a centered inner-ball first-exit problem, producing the grossly premature CDF [0.0291, 0.1797, 0.5023, 0.8387, 0.9809, 0.9993, 1, 1].

The corrected implementation replaces this with the exact centered-ball first-exit law from radius r to A. I independently verified that the corrected ball_exit_time construction has the correct Laplace transform and therefore fixes the conceptual defect rather than just retuning the output.

Verdict: PASS.

# 4. Rapid-run provenance

Run 36745856548 used commit dddf50d71281708bc38b5493f6b31921b605a3a.
Canonical run 36746018907 used commit 8e3f295815a74462c1e3cffcdbb8907e8904f68c.

The verification source file two_layer_accelerated_coupling.rs has the same blob SHA at both corrected commits:

99756165aef32e1ae7887fa14ab89767eac1a64a.

Therefore the canonical rerun is scientifically identical to the corrected implementation.

The canonical execution reports released=10000, censored=0, interface_events=1380397, transmitted=236280, reflected=1144117, renewal_events=1390397, and mean_renewals=139.040.

Verdict: PASS.

# 5. Shell exit-side and moment verification

Run 36741901064 uses a=50 um, b=100 um, r0=75 um, D=1e-8 m2/s, N=20000.

The exact outer-exit probability is 2/3.
The exact conditional mean for both exit sides in this symmetric midpoint benchmark is 0.03125 s.

Executed accelerated results: outer probability 0.67135, outer conditional mean 0.0312289 s, inner conditional mean 0.0307283 s.

Executed direct WOS results: outer probability 0.66630, outer conditional mean 0.0312362 s, inner conditional mean 0.0309805 s.

These discrepancies are compatible with finite Monte-Carlo sampling.

Verdict: PASS.

# 6. Conditional CDF verification

Run 36743542042 uses N=20000 and evaluates both inner- and outer-conditioned time CDFs against an analytical spectral reference and direct supervisor WOS.

The outer exact / accelerated / direct values are:

0.005 s: 0.02483866 / 0.02691154 / 0.02536395
0.010 s: 0.15419952 / 0.15937031 / 0.15871229
0.020 s: 0.42224543 / 0.42608696 / 0.43036170
0.030 s: 0.61046971 / 0.60952024 / 0.60941018
0.050 s: 0.82313286 / 0.81956522 / 0.82245235
0.080 s: 0.94588852 / 0.94280360 / 0.94484466
0.120 s: 0.98884476 / 0.98793103 / 0.98806844.

Maximum accelerated-vs-analytic outer deviation is 0.00517080; maximum direct-WOS-vs-analytic outer deviation is 0.00811627. Inner-conditioned results likewise show no coherent time-dependent distortion.

This is distribution-level verification, not only first-moment verification.

Verdict: PASS.

# 7. Markov-renewal mapping

The corrected two-layer history performs:

1. exact centered-ball first exit in the inner material;
2. production interface transmission/reflection;
3. zero physical interface-event time;
4. alpha times epsilon reinsertion;
5. exact homogeneous-shell first exit in the outer material;
6. repeated interface resolution or outer absorbing release;
7. renewal until release.

For the concentric spherical benchmark, radius/material side plus accumulated time is a sufficient state because geometry and interface probabilities are radial and angularly symmetric.

The acceleration marginalises within-material diffusion excursions rather than replacing the production transmission/reflection law.

Verdict: PASS for the controlled two-layer radial benchmark.

# 8. Accelerated versus FV

Canonical accelerated CDF:
[0.0091, 0.0752, 0.2390, 0.4971, 0.7621, 0.9447, 0.9958, 1.0000].

Accepted FV reference:
[0.00983747, 0.06998542, 0.23413553, 0.49412611, 0.76267512, 0.94225401, 0.99648779, 0.999986996].

Differences Accelerated-FV:
-0.00073747, +0.00521458, +0.00486447, +0.00297389, -0.00057512, +0.00244599, -0.00068779, +0.00001300.

RMS error = 0.002895.
Maximum absolute error = 0.005215.

At N=10000, the largest discrepancy is about 1.98 accelerated-sample standard errors. The sign changes over time and do not indicate coherent systematic release-time bias.

Verdict: compatible with FV at controlled Monte-Carlo precision.

# 9. Accelerated versus direct WOS

The accepted direct-WOS result is a finite-epsilon plateau over 25–200 nm, not a privileged epsilon.

Same-100-nm direct-WOS CDF:
[0.0089, 0.0692, 0.2329, 0.5005, 0.7619, 0.9439, 0.9966, 1.0000].

Accelerated-minus-direct:
[+0.0002, +0.0060, +0.0061, -0.0034, +0.0002, +0.0008, -0.0008, 0].

RMS difference = 0.003281; maximum absolute difference = 0.0061.

One minor documentation issue was independently identified: at t=0.5 s, accelerated 0.0752 is slightly above the empirical maximum of the direct-WOS 25–200 nm values, which is 0.0749. Thus literal min-max containment at every time is not exact. The 0.0003 excess is far below the relevant statistical/finite-epsilon precision scale and does not establish systematic bias.

Verdict: PASS WITH MINOR TRACEABILITY NOTE.

# 10. Computational acceleration

Direct 100-nm WOS mean production steps/history: 1207.2.
Corrected accelerated coupling mean renewal events/history: 139.040.
Ratio: approximately 8.68x.

This is a meaningful algorithmic acceleration because the estimator analytically marginalises within-region diffusion excursions. Workflow wall time is not used because compilation dominates.

Verdict: PASS.

# 11. Censoring

Canonical corrected coupling reports released=10000 and censored=0.

Verdict: PASS for the controlled two-layer benchmark.

# 12. Findings

## CRITICAL
None.

## MAJOR
None.

## MODERATE
R3-A01 — Controlled coupling is a bounded two-layer verification, not integrated five-layer production verification. The harness directly uses production stochastic primitives but reconstructs the two-layer renewal loop. This remains a scope limitation, not a blocker to bounded five-layer investigation.

R3-A02 — The 2000-term conditional-time sampler has an explicit mean-tail bound, while full-distribution truncation error is supported by CDF verification rather than a uniform analytic CDF bound. The evidence is sufficient for the current controlled gate but remains an explicit numerical approximation.

## MINOR
R3-A03 — The persisted wording overstates literal direct-WOS empirical-band containment at t=0.5 s by 0.0003.

R3-A04 — No empirical epsilon convergence order has been established for the accelerated estimator itself; the current controlled coupling uses epsilon=100 nm. This does not block the present bounded-investigation gate.

# 13. R2-WOS-02

OPEN — five-layer direct-production censoring remains unresolved.

# 14. R2-B01

OPEN — the original integrated five-layer release-time problem remains unresolved.

# 15. Accelerated-estimator classification

A. VERIFIED FOR CONTROLLED TWO-LAYER USE AND CLEARED FOR BOUNDED FIVE-LAYER INVESTIGATION.

This classification does not authorize a full five-layer accelerated production ensemble. It authorizes only a bounded diagnostic designed to test composition across all five TRISO layers.

# 16. Single highest-value next action

Run a bounded five-layer accelerated diagnostic, not a full production ensemble.

For each renewal/history record at minimum:
- current layer/state;
- exit side;
- physical time increment and cumulative time;
- interface transmission/reflection;
- renewal count;
- termination reason;
- release time if reached.

The purpose is to determine whether the now-verified shell/interface renewal process remains computationally and statistically stable when composed across all five TRISO layers.

Any resulting five-layer release curve must be independently compared against the deterministic/FV reference where feasible before being called validation.

# 17. Final gate

# A. VERIFIED FOR CONTROLLED TWO-LAYER USE AND CLEARED FOR BOUNDED FIVE-LAYER INVESTIGATION

The accelerated estimator is sufficiently verified for a bounded five-layer investigation.

R2-B01 remains OPEN.
Method 3 should not begin from this result alone.