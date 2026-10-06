# Process-A Weighted-Ensemble Literature Review

## Scope

Target: preserve the original finite-capture Process-A WOS transition kernel and use weighted resampling only to improve rare-event sampling. This is not Process B/C.

## Primary and authoritative sources

### Huber & Kim (1996), Biophysical Journal 70, 97–110
DOI: 10.1016/S0006-3495(96)79552-8.
Original weighted-ensemble Brownian-dynamics paper. Supports splitting probability packets to enhance rare-event sampling while retaining probability weights. Relevance: direct historical precedent for applying WE to Brownian first-passage dynamics. Limitation: does not by itself prove this project's specific resampling/censoring implementation.

### Zhang, Jasnow & Zuckerman (2010), J. Chem. Phys. 132, 054107
DOI: 10.1063/1.3306345; PMCID PMC2830257.
Shows WE is statistically exact for a broad class of Markovian and non-Markovian stochastic dynamics by interpreting WE as resampling in path space. Arbitrary/adaptive bins can guide resampling without changing correctness when weights are handled correctly. Relevance: principal justification that binning affects efficiency rather than the underlying Process-A law.

### Aristoff (2018), Multiscale Modeling & Simulation 16, 1222–1246
"Analysis and optimization of weighted ensemble sampling."
Provides a mathematical WE framework and proves unbiasedness in a general setting. The weighted estimator sum_j omega_j f(xi_j) has expectation equal to the underlying-process expectation. Also decomposes selection/mutation variance. Relevance: finite-generation unbiasedness argument used here.

### Aristoff et al. (2023), J. Chem. Phys. 158, 014108
DOI: 10.1063/5.0110873.
Reviews recent mathematical WE developments, variance reduction and trajectory allocation. Relevance: modern mathematical context; bin/population allocation is an efficiency choice, not a license to alter dynamics.

### Zwier et al. (2015), J. Chem. Theory Comput. 11, 800–809
DOI: 10.1021/ct5010615; PMCID PMC4573570.
WESTPA implementation paper. Describes trajectories carrying statistical weights, periodic replication/pruning, flexible progress coordinates/bins, and rigorous kinetics when weights are managed correctly. Relevance: implementation architecture reference.

### WESTPA repository/documentation
https://github.com/westpa/westpa
Established open-source WE implementation. Relevance: software architecture reference only; this project will not substitute WESTPA dynamics for Process A.

### WESTPA tutorials / error-analysis guidance
The WESTPA tutorial literature recommends multiple fully independent WE simulations because trajectories within one WE run are genealogically correlated. Relevance: independent replicate is the statistical unit in this project's controlled validation. Limitation: replicate t-intervals are an empirical uncertainty device, not a proof of exact finite-sample coverage for arbitrary rare-event distributions.

### Suárez et al. (2016), Protein Science 25, 67–78
DOI: 10.1002/pro.2738; PMCID PMC4815309.
Discusses FPT distributions from WE and explicitly states split children share parent weight while merged survivor state/history is selected proportional to incoming weights. Their post-analysis FPT distribution is approximate in the steady-state/non-Markovian analysis setting. Relevance: supports resampling mechanics and cautions against overclaiming FPT post-analysis. Our transient absorbing estimator differs: accumulated released weight is directly an indicator expectation, not their approximate transition-matrix FPT reconstruction.

## Project-specific conclusions

1. Resampling must act on weighted trajectory/path states; it must not alter Process-A propagation.
2. Splitting a weight w into m identical current states gives child weight w/m; children then receive independent future random streams.
3. Merging weights w_i uses survivor i with probability w_i/W, W=sum_i w_i, and survivor weight W.
4. Released trajectories are absorbing and their weight is removed from live resampling.
5. Bins/progress coordinates influence allocation only.
6. Naive sqrt(F(1-F)/N) is invalid after genealogical resampling. Controlled validation uses independent WE replicates.
7. Fixed *WOS-step-generation* resampling is mathematically admissible if time is included in the augmented state: Process A itself is a discrete transition kernel on (position, accumulated physical time, released flag). Selection after a deterministic number of unchanged kernel calls is adapted resampling of that chain. This does not imply physical-time synchronization. The target remains the physical release-time indicator.
8. For the five-layer application, the progress coordinate is layer plus radial fraction. It changes only resampling allocation, never transition probabilities.
