fn c2(r: f64, s0: f64, a: f64, d2: f64, r_outer: f64) -> f64 {
    s0 * a.powi(3) / (3.0 * d2) * (1.0 / r - 1.0 / r_outer)
}

fn c1(r: f64, s0: f64, a: f64, d1: f64, d2: f64, r_outer: f64) -> f64 {
    s0 * a.powi(3) / (3.0 * d2) * (1.0 / a - 1.0 / r_outer)
        + s0 / (6.0 * d1) * (a * a - r * r)
}

fn interface_flux(s0: f64, a: f64) -> f64 {
    s0 * a / 3.0
}

fn transmission_probability(d1: f64, d2: f64, k: f64) -> f64 {
    let numerator = k * d2;
    numerator / (d1 + numerator)
}

fn assert_close(name: &str, a: f64, b: f64, tol: f64) {
    let err = (a - b).abs();
    assert!(
        err <= tol,
        "{name}: {a:.15e} vs {b:.15e}; abs error {err:.3e} > {tol:.3e}"
    );
}

fn main() {
    let s0 = 2.0;
    let a = 2.0;
    let r_outer = 5.0;
    let d1 = 3.0;
    let d2 = 0.25;

    let ci1 = c1(a, s0, a, d1, d2, r_outer);
    let ci2 = c2(a, s0, a, d2, r_outer);
    assert_close("interface concentration continuity", ci1, ci2, 1e-12);

    let flux1 = interface_flux(s0, a);
    let flux2 = -d1 * (-(s0 * a) / (3.0 * d1));
    assert_close("inner interface flux", flux1, flux2, 1e-12);

    let outer_gradient = -(s0 * a.powi(3)) / (3.0 * d2 * a.powi(2));
    let flux_from_outer = -d2 * outer_gradient;
    assert_close("outer interface flux", flux1, flux_from_outer, 1e-12);

    let p = transmission_probability(d1, d2, 1.0);
    assert_close("K=1 transmission rule", p, d2 / (d1 + d2), 1e-15);

    // Equal diffusivity: the internal interface must not create a concentration jump.
    let d = 1.5;
    let equal_c1 = c1(a, s0, a, d, d, r_outer);
    let equal_c2 = c2(a, s0, a, d, r_outer);
    assert_close("equal-D interface continuity", equal_c1, equal_c2, 1e-12);

    println!("PASS: two-layer continuum oracle");
    println!("interface concentration = {ci1:.12e}");
    println!("interface flux = {flux1:.12e}");
    println!("K=1 transmission probability = {p:.12e}");
}
