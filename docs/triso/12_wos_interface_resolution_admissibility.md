# WOS Interface-Resolution Admissibility

## Why this check matters

After an interface decision, the supervisor implementation moves the walker a fixed distance away from the interface:

$$
\delta = \alpha\epsilon,
$$

where `epsilon` is `capture_eps` and `alpha` is `reinsert_factor`.

The point is supposed to land safely inside one of the two neighbouring materials.

If `delta` becomes larger than the available thickness of a neighbouring layer, the reinserted point can jump across another interface. The next diffusion coefficient would then be taken from the wrong material.

That is a numerical geometry error, not a physical effect.

## Admissibility condition

Let `L_left` be the distance from an interface to the next interface on the side being left, and `L_right` the corresponding distance on the side being entered.

A sufficient local condition is:

$$
\delta < \min(L_{left},L_{right}).
$$

Since

$$
\delta=\alpha\epsilon,
$$

we require:

$$
\boxed{\alpha\epsilon < \min(L_{left},L_{right}).}
$$

A convenient global condition for a five-layer particle is:

$$
\boxed{\alpha\epsilon < L_{min}},
$$

where `L_min` is the thinnest layer thickness.

An even more conservative capture-region condition is:

$$
\epsilon < \frac12 L_{min},
$$

so the capture regions around adjacent interfaces cannot overlap.

## CRP-6 numbers

The repository's CRP-6 geometry uses:

- kernel radius = 212.5 µm;
- buffer thickness = 100 µm;
- IPyC thickness = 40 µm;
- SiC thickness = 35 µm;
- OPyC thickness = 40 µm.

Thus:

$$
L_{min}=35\,\mu\mathrm m.
$$

With the default `alpha = 2` and `epsilon = 10 nm`,

$$
\delta=20\,\mathrm{nm}.
$$

Therefore:

$$
\frac{\delta}{L_{min}}
=\frac{0.020\,\mu\mathrm m}{35\,\mu\mathrm m}
\approx5.7\times10^{-4}.
$$

The default reinsertion distance is therefore far smaller than the thinnest layer.

## Important consequence

This does not prove that 10 nm is accurate.

It only proves that the default reinsertion geometry is comfortably inside the neighbouring material for the nominal CRP-6 layer thicknesses.

Accuracy still requires a convergence study as epsilon is reduced.

## Interaction with interface tolerance

The supervisor geometry classifier also uses an independent geometric tolerance of `1e-12 m` when assigning a point to a TRISO region.

Therefore the final mathematical model contains at least three length scales near an interface:

1. the physical material-layer thickness;
2. `capture_eps`;
3. the region-classification tolerance.

These should not be conflated.

## Recommended implementation guard

Before a future production WOS run, validate:

$$
\alpha>1,
$$

$$
\epsilon>0,
$$

$$
\alpha\epsilon < L_{min}.
$$

and preferably:

$$
\epsilon < \frac12L_{min}.
$$

If the user requests a larger epsilon, the implementation should reject the configuration or explicitly mark it as outside the validated regime.

## Verification status

[VERIFIED] The default CRP-6 parameter values satisfy the geometric admissibility inequality.

[UNVERIFIED] The resulting reinsertion bias is negligible.

[UNVERIFIED] The same condition remains sufficient under all custom geometries supported by `TrisoCell::new`.

[UNVERIFIED] The region-classification tolerance is negligible compared with `capture_eps`.

## Readability note

A simple picture is: `capture_eps` defines how close we allow the walker to approach the wall before resolving the wall explicitly. The reinsertion distance then moves it back into a material. That move must be smaller than the material thickness, or the walker could accidentally skip another wall.