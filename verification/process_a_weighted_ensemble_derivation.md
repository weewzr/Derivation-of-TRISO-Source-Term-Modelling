# Process-A Weighted-Ensemble Bridge — Derivation

## 1. Target stochastic process

Let Z_j=(X_j,	au_j,A_j) be the augmented Process-A state after j unchanged production WOS transition-kernel calls: position X_j, accumulated physical time tau_j, and absorbing release flag A_j. The kernel P_A is exactly the pinned production Process-A kernel.

Define T_A=inf{tau_j: A_j=Released}. The target is F_A(t)=P_A(T_A<=t).

WE changes only how a finite weighted population represents the law of Z_j. It does not modify P_A.

## 2. Weighted population and conservation

At resampling generation n, live trajectories are (Z_n^k,w_n^k), w_n^k>=0. Released weight R_n(t) is stored separately by release time. Censored weight C_n is also separate.

L_n=sum_{k live} w_n^k.

For total initial weight one,

L_n + R_n(infinity) + C_n = 1

to floating-point tolerance. Released/censored weights never return to live resampling.

## 3. CDF estimator

For a complete non-censored transient ensemble define

Fhat_WE(t)=sum over released trajectories/events a of w_a 1{T_a<=t}.

Equivalently, augment every path with g_t(path)=1{T_A<=t}. Conditional-unbiased resampling preserves weighted expectations, and unchanged mutation uses P_A. Therefore

E[Fhat_WE(t)]=E_A[g_t]=P_A(T_A<=t)=F_A(t).

This is a direct transient weighted-indicator estimator, not a steady-state Markov reconstruction of the FPT distribution.

If weighted censoring C>0 remains, released weight gives the rigorous lower bound

R(t) <= F_A(t).

Without additional assumptions, censored paths could all release by t after their censoring point only when censoring occurs before t; a conservative upper bound is

F_A(t) <= R(t)+C_{<=t},

where C_{<=t} is total weight censored before or at t. Censoring after t cannot affect F_A(t). A simpler all-time bound is R(t)<=F_A(t)<=R(t)+C_total. Censored weight is never counted as known non-release.

## 4. Splitting

Parent state z with weight w is replaced by m children at identical current state/history, each weight w/m. For arbitrary future path observable g, conditionally on current history H,

E[sum_{l=1}^m (w/m) g(Y_l)|H]
= sum_l (w/m) E[g(Y)|H]
= w E[g(Y)|H].

Thus splitting preserves expected weighted contribution. Independent future RNG streams are required after cloning.

## 5. Merging/pruning

Given m trajectories z_i with weights w_i and W=sum_i w_i, choose survivor I with

P(I=i | population)=w_i/W

and assign survivor weight W.

For arbitrary current/future conditional value h(z),

E[W h(z_I)|population]
= W sum_i (w_i/W) h(z_i)
= sum_i w_i h(z_i).

Thus probabilistic merge preserves expected weighted contribution. Deterministic "keep the largest" merging is not allowed.

## 6. Repeated resampling

Let Q_n be the conditional resampling operator and P_A^q the unchanged Process-A propagation over q kernel calls. If Q_n preserves weighted expectations conditionally, then by tower property

E[<mu_{n+1},g>]
=E[E[<Q_n(mu_n) P_A^q,g>|mu_n]]
=E[<mu_n,P_A^q g>].

Induction gives equality to the underlying Process-A expectation at every finite generation. This is finite-population unbiasedness of the weighted observable under the stated resampling rule; consistency/variance behavior is a separate issue.

## 7. Resampling schedule

Physical-time synchronization is awkward because one WOS hop samples an entire first-passage event and cannot be paused at an arbitrary physical time without introducing a bridge law.

Chosen controlled scheme: resample after a fixed q unchanged Process-A transition-kernel calls, with accumulated physical time included in each state/history. This is mathematically justified because it is deterministic-generation resampling of the augmented Markov chain Z_j; it does not pretend trajectories share physical time.

For validation q=20 kernel calls. Five-layer q remains frozen at the review boundary and must not be tuned against Process C without a new declaration.

## 8. Progress coordinate and bins

Resampling coordinate only:

(layer, radial fraction within layer).

It does not enter P_A.

Level 1 homogeneous sphere: 5 equal radial-fraction bins [0,.2),...,[.8,1).
Level 2 controlled two-layer benchmark: 4 equal radial bins inside each material (8 live bins total), plus Released and Censored bookkeeping.
Candidate future five-layer bins: Fuel 2, Buffer 4, IPyC 4, SiC 4, OPyC 4 radial subdivisions, plus Released/Censored. This is predeclared but NOT executed before independent review.

## 9. Population control

Each occupied live bin targets M trajectories.

If n<M: split selected trajectories, choosing the largest-weight trajectory iteratively; a selected parent is replaced by two equal-weight children until M. Splitting choice affects variance only; expectation is preserved.

If n>M: repeatedly choose a pair (implementation uses the two smallest weights for variance control), merge them with survivor selected proportional to their weights and survivor weight equal to their sum.

All children get independent future RNG streams.

## 10. Released trajectories

On Process-A release, record (release_time,weight). Do not resample released trajectories. At requested t, sum weights of release events with time<=t.

## 11. Censoring

A trajectory that exhausts the declared per-lineage kernel-call cap is moved to CensoredMaxSteps with its full current weight. It is not treated as unreleased probability. Report lower/upper CDF bounds as above.

## 12. Uncertainty

Within-run children are correlated. The statistical unit is an independently seeded complete WE replicate.

For R independent replicates with estimates Fhat_r(t),

mean = R^{-1} sum_r Fhat_r(t),
s^2 = (R-1)^{-1} sum_r (Fhat_r-mean)^2,
SE_rep=s/sqrt(R).

For controlled validation we report the replicate mean and Student-t 95% interval. This is a pragmatic replicate-level uncertainty estimate; it does not assert independent Bernoulli trajectory errors.

## 13. Validation logic

Level 1 and Level 2 compare direct Process A and weighted Process A under identical physical models. Agreement is assessed using predeclared combined uncertainty plus deterministic reference tolerance. Weight conservation and exact resampling-invariance tests are hard gates.

No five-layer WE execution is authorized until independent review of this bridge.
