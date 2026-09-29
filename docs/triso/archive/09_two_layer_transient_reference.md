> HISTORICAL ARCHIVE: superseded working document. Do not use as the current mathematical or project-status source.

# Two-Layer Spherical Transient Reference — Semi-Analytical Solution

## Purpose

This document extends the exact two-layer steady benchmark into a transient reference solution.

The goal is simple: before trusting a stochastic WOS release curve, we need a second curve produced independently from the continuum diffusion equation.

The problem is:

- inner sphere: 0 < r < a, diffusivity D1, uniform source S0;
- outer shell: a < r < R, diffusivity D2, zero source;
- ideal interface: concentration and flux continuous;
- outer surface: perfect sink, c(R,t)=0;
- initial condition: c(r,0)=0.

This is still only a two-layer test. It is not yet the five-layer TRISO solution.

## 1. Start from the steady solution

The exact steady solution is:

$$
c_{2,ss}(r)=\\frac{S_0a^3}{3D_2}\\left(\\frac1r-\\frac1R\\right),
$$

and:

$$
c_{1,ss}(r)=
\\frac{S_0a^3}{3D_2}\\left(\\frac1a-\\frac1R\\right)
+\\frac{S_0}{6D_1}(a^2-r^2).
$$

The steady solution is what the system approaches after a long enough time.

## 2. Remove the steady part

Write the full concentration as:

$$
c_i(r,t)=c_{i,ss}(r)+v_i(r,t).
$$

The new function v_i means "how far away we are from steady state".

Because the steady solution already satisfies the source equation, v_i satisfies a source-free diffusion equation:

$$
\\frac{\\partial v_i}{\\partial t}
=
\\frac1{r^2}\\frac{\\partial}{\\partial r}
\\left(r^2D_i\\frac{\\partial v_i}{\\partial r}\\right).
$$

The boundary and interface conditions are homogeneous too:

$$
v_1'(0,t)=0,
$$

$$
v_1(a,t)=v_2(a,t),
$$

$$
D_1v_1'(a,t)=D_2v_2'(a,t),
$$

$$
v_2(R,t)=0.
$$

Because the particle initially contains no species,

$$
v_i(r,0)=-c_{i,ss}(r).
$$

## 3. Separate time and space

Look for a mode with a single decay rate:

$$
v_i(r,t)=\\phi_i(r)e^{-\\lambda t}.
$$

Here \(\\lambda>0\) has units of s^-1. It tells us how quickly this mode dies away.

Substitute this form into the source-free diffusion equation:

$$
-\\lambda\\phi_i
=
\\frac1{r^2}\\frac{d}{dr}
\\left(r^2D_i\\frac{d\\phi_i}{dr}\\right).
$$

Because D_i is constant inside each material:

$$
\\frac{d}{dr}
\\left(r^2D_i\\frac{d\\phi_i}{dr}\\right)
=
D_i\\frac{d}{dr}
\\left(r^2\\frac{d\\phi_i}{dr}\\right).
$$

Therefore:

$$
\\frac{d}{dr}
\\left(r^2\\frac{d\\phi_i}{dr}\\right)
+
\\frac{\\lambda}{D_i}r^2\\phi_i
=
0.
$$

Define the layer-specific spatial wave number:

$$
k_i^2=\\frac{\\lambda}{D_i}.
$$

Thus:

$$
[k_i]=\\mathrm{m^{-1}}.
$$

This is deliberately separate from the time-decay rate \(\\lambda\).

## 4. Remove the spherical first-derivative term

The spherical equation contains both \(\\phi_i''\) and \(2\\phi_i'/r\).

Introduce:

$$
u_i(r)=r\\phi_i(r).
$$

Then:

$$
\\phi_i(r)=\\frac{u_i(r)}r.
$$

Differentiate:

$$
\\frac{d\\phi_i}{dr}
=
\\frac{ru_i'-u_i}{r^2}.
$$

Multiply by \(r^2\):

$$
r^2\\frac{d\\phi_i}{dr}=ru_i'-u_i.
$$

Differentiate:

$$
\\frac{d}{dr}
\\left(r^2\\frac{d\\phi_i}{dr}\\right)
=
u_i'+ru_i''-u_i'.
$$

Therefore:

$$
\\frac{d}{dr}
\\left(r^2\\frac{d\\phi_i}{dr}\\right)
=
ru_i''.
$$

Substitute into the eigen-equation:

$$
ru_i''+k_i^2ru_i=0.
$$

For r>0, divide by r:

$$
\\boxed{u_i''+k_i^2u_i=0.}
$$

Now the spatial problem looks like the familiar one-dimensional sine/cosine equation.

## 5. Solutions in the two regions

At r=0 the concentration must remain finite. The cosine solution for u_1 would make \(u_1(0)\\ne0\), which would make \(\\phi_1=u_1/r\) diverge.

Therefore:

$$
u_1(r)=A\\sin(k_1r).
$$

At the outside surface, \(v_2(R,t)=0\), so \(\\phi_2(R)=0\), and therefore \(u_2(R)=0\).

A convenient form is:

$$
u_2(r)=B\\sin(k_2(R-r)).
$$

## 6. Interface concentration condition

Because \(\\phi=u/r\), continuity of concentration at r=a gives:

$$
\\phi_1(a)=\\phi_2(a).
$$

Substitute \(\\phi_i=u_i/r\):

$$
\\frac{u_1(a)}a=\\frac{u_2(a)}a.
$$

Therefore:

$$
u_1(a)=u_2(a).
$$

Let this common value be \(U\):

$$
U=A\\sin(k_1a)=B\\sin(k_2(R-a)).
$$

## 7. Interface flux condition

The concentration derivative is:

$$
\\phi_i'=\\frac{u_i'}r-\\frac{u_i}{r^2}.
$$

Flux continuity requires:

$$
D_1\\phi_1'(a)=D_2\\phi_2'(a).
$$

Substitute the derivative:

$$
D_1\\left(\\frac{u_1'(a)}a-\\frac{u_1(a)}{a^2}\\right)
=
D_2\\left(\\frac{u_2'(a)}a-\\frac{u_2(a)}{a^2}\\right).
$$

Use \(u_1(a)=u_2(a)=U\):

$$
D_1u_1'(a)-\\frac{D_1U}{a}
=
D_2u_2'(a)-\\frac{D_2U}{a}.
$$

For the inner solution:

$$
u_1'(a)=Ak_1\\cos(k_1a).
$$

Since \(A\\sin(k_1a)=U\):

$$
u_1'(a)=Uk_1\\cot(k_1a).
$$

For the outer solution:

$$
u_2'(a)=-Bk_2\\cos(k_2(R-a)).
$$

Since \(B\\sin(k_2(R-a))=U\):

$$
u_2'(a)=-Uk_2\\cot(k_2(R-a)).
$$

Substitute both derivatives:

$$
D_1Uk_1\\cot(k_1a)-\\frac{D_1U}{a}
=
-D_2Uk_2\\cot(k_2(R-a))-\\frac{D_2U}{a}.
$$

Move the outer-side terms to the left:

$$
U\\left[D_1k_1\\cot(k_1a)+D_2k_2\\cot(k_2(R-a)) +\\frac{D_2-D_1}{a}\\right]=0.
$$

Non-zero modes have U not equal to zero, so the factor in brackets must vanish:

$$
\\boxed{
D_1k_1\\cot(k_1a)
+D_2k_2\\cot(k_2(R-a))
+\\frac{D_2-D_1}{a}=0.
}
$$

This is the two-layer spherical eigenvalue equation.

Together with

$$
k_i=\\sqrt{\\frac{\\lambda}{D_i}},
$$

it gives the allowed positive decay rates \(\\lambda_n\).

## 8. Reconstructing the transient solution

Each allowed eigenvalue gives one mode:

$$
v_n(r,t)=\\phi_n(r)e^{-\\lambda_nt}.
$$

The full transient correction is an infinite sum:

$$
v(r,t)=\\sum_{n=1}^{\\infty}A_n\\phi_n(r)e^{-\\lambda_nt}.
$$

Therefore:

$$
\\boxed{
c(r,t)=c_{ss}(r)+\\sum_{n=1}^{\\infty}A_n\\phi_n(r)e^{-\\lambda_nt}.
}
$$

The coefficients are selected so that the series reproduces the initial condition \(c(r,0)=0\).

Because the spatial operator is in conservative Sturm–Liouville form, the natural radial inner product uses the spherical volume weight \(r^2\):

$$
\\langle f,g\\rangle=\\int_0^Rr^2f(r)g(r)\\,dr.
$$

Thus the initial-condition coefficients can be written:

$$
\\boxed{
A_n=
-\\frac{\\int_0^Rr^2c_{ss}(r)\\phi_n(r)\\,dr}
{\\int_0^Rr^2\\phi_n(r)^2\\,dr}.
}
$$

The minus sign appears because the initial transient correction is \(v(r,0)=-c_{ss}(r)\).

## 9. What this gives us

We now have an independently defined transient reference without using the WOS output to construct the answer.

The verification target can be:

$$
F_{WOS}(t)\\quad\\text{versus}\\quad F_{continuum}(t),
$$

or, when a spatial concentration reconstruction is available:

$$
c_{WOS}(r,t)\\quad\\text{versus}\\quad c_{continuum}(r,t).
$$

The continuum reference itself has two independent checks:

1. as \(t\\rightarrow\\infty\), the transient terms vanish and the steady solution remains;
2. setting \(D_1=D_2\) must remove the physical effect of the artificial interface.

## 10. Verification status

[VERIFIED] The steady two-layer solution is closed form.

[DERIVED] The separated two-layer transient eigenproblem follows from the conservative diffusion equation and the ideal interface conditions.

[UNVERIFIED] Numerical computation of the eigenvalues has been performed.

[UNVERIFIED] The resulting transient series has been compared with the WOS implementation.

[UNVERIFIED] The existing WOS interface rule reproduces the transient two-layer solution.

[UNVERIFIED] Finite capture epsilon and reinsertion have negligible bias.

## 11. Why this is the correct next reference

A high-school way to see the logic is:

> First find the final answer after everything settles. Then subtract that final answer from the moving solution. What remains is a collection of decaying patterns. Each pattern dies away exponentially, and the slowest pattern controls the long-time approach to equilibrium.

The mathematics above turns that idea into an exact testable reference.

## 12. Next experimental step

Once a Rust-capable execution environment is available, compute the first several roots of the eigenvalue equation, reconstruct the transient series, and compare it with WOS for the same two-layer geometry.

Do not change the production WOS interface formula before that comparison.

Do not call the model fully verified until epsilon/reinsertion and statistical errors are quantified.