# First-Principles Derivation and Verification of Diffusive Release from Multilayer TRISO Fuel

**Canonical research manuscript — Method 2 remains open**

## Abstract

This work derives and verifies diffusion-based source-term models for a concentric five-layer TRISO particle. Method 1 develops the continuous spherical diffusion equations and analytical/modal structure. Method 2 develops an independently verified conservative finite-volume (FV) reference, the production walk-on-spheres (WOS) formulation, exact spherical first-passage kernels, an accelerated exact-interface renewal process, and a finite-state Markov-renewal transform reduction. Controlled two-layer and three-layer verification establishes the numerical building blocks. The frozen five-layer Cs-137 matrix has been constructed and diagnosed, but ordinary double precision is strongly affected by near-recurrent interface dynamics; a predeclared high-precision cross-check is ongoing. No final five-layer release CDF is claimed, and R2-WOS-02 and R2-B01 remain open.

## 1. Geometry and scope

The particle consists of concentric regions

\[
0=R_0<R_1<R_2<R_3<R_4<R_5=R,
\]

corresponding to kernel, buffer, IPyC, SiC and OPyC. For the frozen five-layer Cs-137 verification benchmark,

\[
(R_1,\ldots,R_5)
=(212.5,312.5,352.5,387.5,427.5)\;\mu{\rm m}.
\]

The frozen diffusivities are

\[
D=(1.2502983\times10^{-13},10^{-8},4.0622991\times10^{-14},
9.2277732\times10^{-17},4.0622991\times10^{-14})\ {\rm m^2\,s^{-1}}.
\]

The project contains exactly three active methods: (1) first-principles analytical diffusion; (2) first-principles computational diffusion using FV and WOS/first-passage formulations; and (3) a later semi-empirical mechanistic/reduced-order method. This manuscript presently develops Methods 1 and 2 only.

## 2. Governing problem

For piecewise constant \(D(r)\), the conservative spherical diffusion equation is

\[
\frac{\partial c}{\partial t}
=
\frac{1}{r^2}\frac{\partial}{\partial r}
\left(r^2D(r)\frac{\partial c}{\partial r}\right)+S-R(c).
\tag{1}
\]

The centre is regular,

\[
\left.\frac{\partial c}{\partial r}\right|_{r=0}=0.
\tag{2}
\]

For the ideal \(K=1\) interfaces used in the verification benchmarks,

\[
c_i(R_i,t)=c_{i+1}(R_i,t),
\tag{3}
\]

\[
D_i\left.\frac{\partial c_i}{\partial r}\right|_{R_i^-}
=
D_{i+1}\left.\frac{\partial c_{i+1}}{\partial r}\right|_{R_i^+}.
\tag{4}
\]

The general continuum model permits a Robin exterior,

\[
-D_5 c_r(R,t)=h[c(R,t)-c_\infty],
\tag{5}
\]

whereas the frozen production-WOS verification benchmark uses the absorbing boundary

\[
c(R,t)=0.
\tag{6}
\]

For that release benchmark the initial inventory is uniform in kernel volume, zero in the coatings, and there is no continuing source after \(t=0\):

\[
c(r,0)=
\begin{cases}
c_0,&0\le r<R_1,\\
0,&R_1<r<R.
\end{cases}
\tag{7}
\]

## 3. Method 1 — analytical diffusion

Within a homogeneous layer,

\[
\frac{\partial c}{\partial t}
=D\left(c_{rr}+\frac{2}{r}c_r\right).
\tag{8}
\]

For a source-driven homogeneous benchmark, write \(c=w+v\), where \(w\) is the steady solution and \(v\) satisfies the homogeneous transient equation. Separation \(v=\phi(r)T(t)\) gives

\[
\frac{1}{r^2}\frac{d}{dr}\left(r^2\frac{d\phi}{dr}\right)+k^2\phi=0,
\qquad
T(t)=e^{-Dk^2t}.
\tag{9}
\]

With \(u=r\phi\),

\[
u''+k^2u=0.
\tag{10}
\]

The multilayer extension uses a common modal decay rate \(\Lambda\) and layer wave numbers

\[
k_i^2=\frac{\Lambda}{D_i}.
\tag{11}
\]

Interface and boundary conditions assemble

\[
\mathbf M(\Lambda)\mathbf a=0,
\qquad
\det\mathbf M(\Lambda)=0.
\tag{12}
\]

The repository directly establishes piecewise self-adjoint boundary/interface cancellation and weighted orthogonality. Full singular piecewise-transmission completeness and numerical modal truncation remain qualified rather than assumed.

## 4. Method 2A — conservative deterministic FV reference

Integrating over spherical control volume \(i\),

\[
V_i\frac{dc_i}{dt}
=
G_{i-\frac12}(c_{i-1}-c_i)
+
G_{i+\frac12}(c_{i+1}-c_i),
\tag{13}
\]

with

\[
V_i=\frac{4\pi}{3}(r_{i+\frac12}^3-r_{i-\frac12}^3).
\tag{14}
\]

For a face at \(r_f\),

\[
G_f=
\frac{4\pi r_f^2}
{\dfrac{r_f-r_L}{D_L}+\dfrac{r_R-r_f}{D_R}}.
\tag{15}
\]

Forward Euler gives

\[
c_i^{n+1}
=
c_i^n+\frac{\Delta t}{V_i}
\left[
G_{i-\frac12}(c_{i-1}^n-c_i^n)
+
G_{i+\frac12}(c_{i+1}^n-c_i^n)
\right].
\tag{16}
\]

Independent audit found approximately second-order spatial convergence for the aligned five-layer benchmark, first-order temporal self-convergence for Forward Euler, and inventory closure to floating-point scale. This is benchmark-specific verification.

## 5. Method 2B — production WOS contract

A production history begins uniformly in kernel volume,

\[
r=R_1U^{1/3}.
\tag{17}
\]

Inside one homogeneous material, ordinary WOS uses the largest interface-free sphere of radius \(\rho\). Physical first-passage time obeys

\[
\tau=\theta\frac{\rho^2}{D}.
\tag{18}
\]

At finite capture distance \(\epsilon\),

\[
p_{i\rightarrow j}
=
\frac{D_j}{D_i+D_j}
\qquad(K=1),
\tag{19}
\]

with zero interface-event time and

\[
\delta=\alpha\epsilon.
\tag{20}
\]

Arrival at the OPyC exterior is a released history with release time \(t\). Exhausting a numerical step budget is explicit censoring and is never interpreted as release or known non-release.

Controlled WOS/FV verification established a finite-\(\epsilon\) statistical plateau over the accepted range. It does not establish an empirical convergence order in \(\epsilon\).

## 6. Exact spherical first-passage kernels

For shell \(a<r<b\), \(T\) is first exit and \(\lambda=\sqrt{s/D}\). The joint outer-exit/time transform is

\[
G^+(r,s)
=
\mathbb E_r[e^{-sT}\mathbf 1_{\{R_T=b\}}]
=
\frac{b}{r}
\frac{\sinh[\lambda(r-a)]}{\sinh[\lambda(b-a)]},
\tag{21}
\]

and

\[
G^-(r,s)
=
\frac{a}{r}
\frac{\sinh[\lambda(b-r)]}{\sinh[\lambda(b-a)]}.
\tag{22}
\]

At \(s\to0\),

\[
P(R_T=b)=\frac{b(r-a)}{r(b-a)},
\qquad
P(R_T=a)=\frac{a(b-r)}{r(b-a)}.
\tag{23}
\]

For a centred ball,

\[
H(r,s)
=
\frac{b}{r}
\frac{\sinh(r\sqrt{s/D})}{\sinh(b\sqrt{s/D})},
\tag{24}
\]

with the regular \(r\to0\) limit. Exit probabilities, conditional moments and conditional CDFs were checked against analytical references and direct supervisor-WOS sampling before interface coupling.

## 7. Exact-interface accelerated renewal

Define **Process A** as finite-capture production WOS and **Process B** as accelerated exact-interface renewal. B replaces repeated homogeneous-region WOS wandering by Eqs. (21)–(24), but retains Eq. (19), zero interface-event time and Eq. (20).

A and B are not mathematically identical. In the controlled two-layer benchmark,

\[
1-(1-\epsilon/a)^3=0.005988008
\tag{25}
\]

of uniform-volume births lie in the finite capture shell for \(a=50\,\mu{\rm m}\), \(\epsilon=0.1\,\mu{\rm m}\). Their executed CDF discrepancy is statistically compatible at the declared controlled precision, but this is empirical compatibility rather than identity.

The corrected two-layer accelerated implementation agrees with FV at Monte-Carlo scale and reduces stochastic event count by approximately \(8.68\times\) relative to direct 100-nm WOS.

## 8. Direct five-layer computational pathology

The direct five-layer production diagnostic censored all tested histories at the numerical step cap. A bounded exact-shell accelerated diagnostic likewise produced 0/16 releases and 16/16 censored histories; almost all 1.6 million renewals occurred in Buffer, only two histories reached IPyC, and none reached SiC/OPyC.

Thus homogeneous-region wandering is not the remaining dominant bottleneck. The hard problem is repeated rare interface-state recurrence generated by extreme diffusivity contrasts.

## 9. Interface-state Markov-renewal reduction

For four internal interfaces define eight post-interface states

\[
S_0,\ldots,S_7,
\tag{26}
\]

plus absorbing release. Let

\[
\Phi_i(s)=\mathbb E_i[e^{-sT_{\rm release}}].
\tag{27}
\]

Each \(K_{ij}(s)\) is an exact region first-passage transform multiplied by the appropriate zero-time interface transition probability:

\[
\boldsymbol\Phi(s)
=
\mathbf K(s)\boldsymbol\Phi(s)+\mathbf B(s),
\tag{28}
\]

so

\[
\boxed{
\boldsymbol\Phi(s)
=
[\mathbf I-\mathbf K(s)]^{-1}\mathbf B(s)
}.
\tag{29}
\]

**Process C** is this deterministic matrix reduction of B. The epistemic hierarchy is

\[
B=C\quad\text{(mathematical reduction target)},
\tag{30}
\]

\[
A\approx B/C\quad\text{(controlled empirical compatibility only)}.
\tag{31}
\]

For uniform kernel births,

\[
\Phi_{\rm init}(s)
=
\int_0^{R_1}\frac{3r^2}{R_1^3}
H(r,s)
\left[p_{K\to K}\Phi_0(s)+p_{K\to B}\Phi_1(s)\right]dr.
\tag{32}
\]

## 10. Controlled matrix verification

The controlled \(2\times2\) matrix reproduced explicit Process B at all predeclared transform points within approximately 1.004 Monte-Carlo standard errors.

A genuine four-state benchmark with

\[
R=(50,75,100)\,\mu{\rm m},
\qquad
D=(1,2,5)\times10^{-9}\ {\rm m^2/s}
\tag{33}
\]

was then tested against N=20,000 explicit B histories and a refined FV reference. Maximum B/C transform discrepancy was \(0.729\) MC standard errors. B/FV CDF RMS difference was \(1.0395\times10^{-3}\), with maximum absolute difference \(1.7794\times10^{-3}\). FV refinement and conservation errors were much smaller than stochastic uncertainty. Independent audit cleared the bounded hard-matrix diagnostic.

## 11. Hard five-layer transform diagnostic — ONGOING / NOT YET CLOSED

The frozen hard \(8\times8\) matrix was evaluated at

\[
s=(0,10^{-9},10^{-8},10^{-7},10^{-6},10^{-5},10^{-4})\ {\rm s^{-1}}.
\tag{34}
\]

At \(s=0\),

\[
\rho[\mathbf K(0)]
\approx0.999999999949649,
\qquad
1-\rho\approx5.04\times10^{-11}.
\tag{35}
\]

Ordinary f64 arithmetic produced

\[
\Phi_{\rm init}(0)\approx0.999999887846684,
\tag{36}
\]

missing the predeclared \(10^{-9}\) normalization criterion. The \(s=10^{-5}\) residual was \(1.138\times10^{-10}\), narrowly above the \(10^{-10}\) criterion, and the f64 Frobenius condition estimate is of order \(10^{11}\) at \(s=0\).

These failures are not waived. The predeclared 50/80/120-decimal-digit high-precision cross-check has now executed successfully. At the 80-digit reference, \\(\\Phi_{\\rm init}(0)=1\\), the relative residuals are of order \\(10^{-85}\\), \\(\\rho[K(0)]=0.9999999999499492888\\), and robust SVD gives \\(\\kappa_2(I-K)\\approx5.20\\times10^{10}\\). The 50→80 and 80→120 results converge far beyond the original criteria. This establishes candidate evidence that the f64 failures were conditioning error. **Independent review is still required before any inverse-Laplace work.**

No inverse-Laplace recovery has been verified. No final five-layer \(F(t)\) is reported.

## 12. Limitations and open findings

1. **R2-WOS-02 — OPEN.**
2. **R2-B01 — OPEN.**
3. Process A finite-capture WOS is not pathwise identical to Process B/C.
4. Hard-matrix high-precision remediation passes its predeclared numerical gates but is pending independent review.
5. No verified inverse-Laplace recovery or final five-layer release CDF exists.
6. Method 3 has not begun.
7. Multilayer analytical modal completeness and numerical modal truncation remain separately qualified.

## 13. Verification hierarchy

\[
\text{continuous analytical model}
\rightarrow
\text{deterministic FV reference}
\rightarrow
\text{direct WOS verification}
\rightarrow
\text{exact first-passage kernels}
\rightarrow
\text{accelerated renewal}
\rightarrow
\text{matrix reduction}.
\tag{37}
\]

Each arrow is a distinct evidence layer.

## 14. Traceability

Major claims are mapped in paper/TRACEABILITY.md. The historical raw LaTeX under notes/raw remains preserved and is not the canonical manuscript.
