# Controlled inverse-Laplace method selection

## Scope

Frozen Process-C hard five-layer transform only. Process A/B equivalence is not claimed. R2-WOS-02 and R2-B01 remain open.

## Transform identities

For a nonnegative release time T,
Phi(s)=E[exp(-sT)]=Laplace{f_T}(s).

Hence:
- PDF: L{f_T}(s)=Phi(s).
- survival S(t)=P(T>t): L{S}(s)=(1-Phi(s))/s when eventual release probability is one.
- CDF F(t)=P(T<=t): L{F}(s)=Phi(s)/s for t>=0 with F(0)=0 in the present continuous first-passage model.

The controlled calculation will invert Phi(s)/s for the CDF and (1-Phi(s))/s for survival; it will not label the inverse of Phi(s) itself as a CDF.

## Literature comparison

### Talbot contour

Talbot (1979) deforms the Bromwich contour and applies trapezoidal quadrature on a contour chosen to improve convergence. It requires complex-s transform evaluations and knowledge/avoidance of singularities. It is attractive for high accuracy but sensitive to analytic continuation and contour placement.

### de Hoog–Knight–Stokes

de Hoog, Knight & Stokes (1982) accelerate the Fourier-series representation of the Bromwich integral using an epsilon/Padé/continued-fraction construction. It requires complex-s evaluations along a vertical line. It is widely used and generally robust, but cancellation and truncation/acceleration parameters still require convergence checks.

### Gaver–Stehfest

Gaver–Stehfest evaluates the transform only at positive real s=k ln(2)/t. This is immediately compatible with the existing frozen real-s transform. Its alternating large coefficients cause severe cancellation; arbitrary precision is therefore mandatory here. It is useful as an independent real-axis method, not as the sole production inversion.

### Cohen/Fourier acceleration

Modern accelerated Fourier/Bromwich approaches, including Cohen's treatment, require complex transform evaluations and can be effective over broad classes. They remain candidates after complex-s continuation is verified.

## Selection

Method 1: high-precision Gaver–Stehfest, because it uses only positive real s and therefore exercises the frozen transform without unverified analytic continuation.

Method 2: de Hoog/Fourier inversion after explicit complex-s continuation. It is genuinely distinct from Gaver–Stehfest and has strong numerical-analysis provenance.

Talbot is retained as an optional third cross-check after complex-s support is established. It is not selected as the primary complex method because contour deformation adds another analytic/singularity-placement dependency.

## Complex-s continuation requirement

The frozen formulas use lambda=sqrt(s/D), sinh ratios, centered-ball transforms, shell first-exit transforms, K(s), B(s), and the matrix inverse. Mathematically these formulas extend to complex s away from transform singularities using the principal square-root branch. For Re(s)>0 the principal root has Re(sqrt(s/D))>0, which is compatible with Laplace-domain decay. The implementation must verify:
- Phi(conj(s))=conj(Phi(s));
- agreement with the real implementation on positive real s;
- no branch discontinuity along the de Hoog evaluation line;
- stable complex sinh-ratio evaluation without real-only comparisons.

Until these tests pass, physical de Hoog/Talbot inversion is blocked.

## Predeclared synthetic benchmarks

1. Exponential: Phi(s)=lambda/(s+lambda), CDF=1-exp(-lambda t).
2. Two-exponential mixture: Phi(s)=w*l1/(s+l1)+(1-w)*l2/(s+l2).
3. Diffusion/first-passage: centered 3-D ball exit transform Phi(s)=z/sinh(z), z=R sqrt(s/D), with trusted time-domain reference obtained independently from its eigenfunction/residue series.
4. Long-timescale exponential with lambda=1e-8 s^-1.

Parameters are fixed before numerical inversion.

## Predeclared physical grid and precision

Physical CDF grid: logarithmic 10^2 to 10^10 s, 81 points (0.1 decade spacing). This spans far below the ~1 s Buffer scale through well beyond the ~1.33e7 s SiC L^2/D scale.

Precision convergence: 50, 80, 120 decimal digits. Primary comparison uses 80 digits; 120 digits is the convergence reference.

Method-to-method comparison will report absolute and relative differences without retroactively choosing an acceptance threshold.
