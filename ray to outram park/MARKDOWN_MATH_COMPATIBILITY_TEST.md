# GitHub Markdown Mathematics Compatibility Test

Purpose: establish the exact Markdown mathematics syntax permitted for the TRISO manuscript before full regeneration. This file is a rendering fixture, not scientific evidence.

## A. Simple scalar equation

**Equation TEST-SCALAR-001**

$$
D = D_i
$$

## B. Fraction

**Equation TEST-FRAC-001**

$$
q = \frac{a}{b}
$$

## C. Derivative

**Equation TEST-DERIV-001**

$$
\frac{dc}{dr}=0
$$

## D. Partial derivative

**Equation TEST-PARTIAL-001**

$$
\frac{\partial c}{\partial t}
=
D\frac{\partial^2 c}{\partial r^2}
$$

## E. Vector

**Equation TEST-VECTOR-001**

$$
\mathbf{J}=-D\nabla c
$$

## F. Integral

**Equation TEST-INTEGRAL-001**

$$
N_V(t)=\int_V c(\mathbf{x},t)\,dV
$$

## G. Matrix

**Equation TEST-MATRIX-001**

$$
\mathbf{K}
=
\begin{bmatrix}
K_{00} & K_{01} \\
K_{10} & K_{11}
\end{bmatrix}
$$

## H. Aligned multi-line derivation

**Equation TEST-ALIGNED-001**

$$
\begin{aligned}
\Phi &= K\Phi+B,\\
\Phi-K\Phi &= B,\\
(I-K)\Phi &= B,\\
\Phi &= (I-K)^{-1}B.
\end{aligned}
$$

## I. Cases / piecewise equation

**Equation TEST-CASES-001**

$$
S_{i,\mathrm{gen}}
=
\begin{cases}
S_0, & 0 \le r < r_1,\\
0, & r_1 < r < R.
\end{cases}
$$

## J. Superscript units

**Equation TEST-UNITS-DISPLAY-001**

$$
[c]=\mathrm{mol\,m^{-3}}
$$

Inline unit test: $\mathrm{m^2\,s^{-1}}$.

Inline compound unit test: $\mathrm{mol\,m^{-3}\,s^{-1}}$.

## K. Subscripts containing roman text

**Equation TEST-ROMAN-SUBSCRIPT-001**

$$
S_{i,\mathrm{gen}}=S_0
$$

## L. Greek symbols

**Equation TEST-GREEK-001**

$$
\lambda_d>0,
\qquad
\epsilon>0,
\qquad
\Phi(s)=\mathbb{E}[e^{-sT}]
$$

## M. Stable equation identifier strategy

The Markdown target does **not** require LaTeX `\tag{...}` for traceability. The stable identifier is ordinary Markdown immediately above the atomic math block:

**Equation TRISO-GOV-020**

$$
N_V(t)=\int_V c(\mathbf{x},t)\,dV
$$

This is the proposed production strategy unless rendered GitHub evidence demonstrates that another syntax is more robust.

## N. Long equation

**Equation TEST-LONG-001**

$$
\frac{\partial c_i}{\partial t}
=
\frac{1}{r^2}
\frac{\partial}{\partial r}
\left(
r^2D_i\frac{\partial c_i}{\partial r}
\right)
+
S_{i,\mathrm{gen}}
+
S_{i,\mathrm{other}}
+
S_{i,\mathrm{release}}
-
S_{i,\mathrm{decay}}
-
S_{i,\mathrm{trap}}
$$

## O. Nested parentheses and braces

**Equation TEST-NESTED-001**

$$
\Phi_{\mathrm{init}}(s)
=
\int_0^{R_1}
\left[
\frac{3r^2}{R_1^3}
H_K(r,s)
\left(
p_{K\rightarrow K}\Phi_0(s)
+
p_{K\rightarrow B}\Phi_1(s)
\right)
\right]dr
$$

## Acceptance tests

Rendered GitHub output must show all A-O expressions as mathematics. In particular it must not show literal `$$`, literal `mol m$^{-3}$`, raw `[S_{i,\mathrm{gen}}]` fragments, vertically stacked equation characters, raw `\mathrm{...}` outside math, or a `Missing \end{cases}` error.
