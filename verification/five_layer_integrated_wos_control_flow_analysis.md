# Five-Layer Integrated WOS Control-Flow Analysis

## Scope

This analysis follows executed integrated diagnostic run `36735378843`. It does not alter the supervisor implementation and does not close R2-B01.

The controlled two-layer finite-epsilon interface result remains closed. The question here is why the complete five-layer production walker makes negligible outward progress.

## Executed observation

At 1,000,000 production steps/history:

- 8/8 histories are CensoredMaxSteps;
- all terminate in Buffer;
- deepest layer reached is IPyC;
- no history enters SiC or OPyC;
- approximately 153,000 zero-time interface events occur per history;
- essentially all layer-classified steps occur in Buffer.

## Frozen transmission probabilities

For K=1 the production rule is

[
p_{i\to j}=\frac{D_j}{D_i+D_j}.
]

Using the executed Cs-137 diffusivities:

[
D_B=1.0\times10^{-8},\quad
D_I=4.0622991\times10^{-14},\quad
D_S=9.2277732\times10^{-17}\;m^2/s.
]

Therefore

[
p_{B\to I}\approx4.0623\times10^{-6},
]

or roughly one outward transmission per 246,000 Buffer→IPyC interface decisions.

The reverse probability is

[
p_{I\to B}\approx0.99999594.
]

At the IPyC→SiC interface,

[
p_{I\to S}\approx2.266\times10^{-3}.
]

These are consequences of the frozen verified production interface rule and physical diffusivity contrast, not newly introduced probabilities.

## Reinsertion and probability of traversing IPyC before returning

After a successful Buffer→IPyC transmission the production walker is reinserted

[
\delta=\alpha\epsilon=20\;nm
]

outside the Buffer radius.

IPyC spans

[
a=312.5\;um
\quad\text{to}\quad
b=352.5\;um.
]

For ordinary isotropic diffusion in a spherical shell, the probability of hitting the outer sphere b before the inner sphere a from radius r is the harmonic hitting probability

[
P_b(r)=\frac{1/a-1/r}{1/a-1/b}.
]

For `r=a+20 nm` this is approximately

[
P_b\approx5.64\times10^{-4}.
]

This calculation is a first-principles diagnostic of spatial progression between the two spherical interfaces; it is not a replacement for the production WOS simulation.

Thus a successful outward Buffer→IPyC transmission usually returns to the Buffer boundary before ever reaching the IPyC/SiC interface.

A crude single-attempt scale for the sequence

Buffer→IPyC transmission
→ reach IPyC outer boundary before returning
→ transmit IPyC→SiC

is

[
(4.06\times10^{-6})(5.64\times10^{-4})(2.27\times10^{-3})
\approx5.2\times10^{-12}.
]

This is not the full eventual passage probability of the recurrent stochastic process and must not be interpreted as one. It is an order-of-magnitude explanation of why step-by-step integrated progression can require an enormous number of interface-resolution attempts.

## Interpretation

The executed million-step failure is consistent with the intended frozen stochastic law combined with:

1. extreme Buffer/IPyC diffusivity contrast;
2. reinsertion only 20 nm into a 40 um IPyC shell;
3. near-unity reverse transmission from IPyC back into Buffer;
4. a further low-probability IPyC→SiC transmission;
5. a hard step-count censoring policy.

The evidence therefore does not require an implementation defect to explain the observed lack of progression.

Physical time and computational step count remain distinct. The small SiC diffusivity implies a long physical residence time after SiC entry, but the present integrated histories are censored before SiC entry. The immediate computational barrier is the rare-event sequence required to penetrate from Buffer through IPyC into SiC.

## Consequence for R2-B01

The current step-by-step five-layer estimator is not computationally usable for a release-CDF ensemble under the frozen 10 nm configuration and present step-cap design.

Blindly increasing N or max_steps is not a scientifically efficient remedy.

## Smallest next Method-2 design question

Investigate an **equivalent accelerated interface/shell first-passage representation** that preserves the already-verified physical stochastic law while analytically or statistically marginalising repeated local interface excursions.

Examples of a design class to investigate, not yet adopted:

- exact/verified shell-to-shell first-passage kernels;
- interface-state Markov renewal kernels;
- rare-event acceleration with unbiased weighting.

Any accelerated estimator would be a new numerical implementation. Before it could replace the current production driver it would require:

1. derivation from the same diffusion/interface process;
2. homogeneous and two-layer analytical/FV verification;
3. comparison against direct production WOS in regimes where direct WOS is computationally feasible;
4. explicit proof/check that release-time statistics are preserved;
5. independent review.

No accelerated estimator is implemented in this pass.
