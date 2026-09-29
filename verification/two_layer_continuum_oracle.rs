// Independent two-layer continuum oracle.
// No external crates. This file only checks the closed-form steady solution
// and its limiting cases; it does not implement or assume the supervisor WOS code.

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

fn main() {
    let s0 = 2.0;
    let a = 2.0;
    let r_outer = 5.0;
    let d1 = 3.0;
    let d2 = 0.25;

    let ci_inner = c1(a, s0, a, d1, d2, r_outer);
    let ci_outer = c2(a, s0, a, d2, r_outer);
    let flux = interface_flux(s0, a);
    let p = transmission_probability(d1, d2, 1.0);

    assert!((ci_inner - ci_outer).abs() < 1e-12);
    assert!((p - d2 / (d1 + d2)).abs() < 1e-12);
    assert!((flux - 4.0 / 3.0).abs() < 1e-12);

    // Equal diffusivity makes the artificial interface disappear smoothly.
    let d = 1.5;
    let c_equal_inner = c1(1.0, s0, a, d, d, r_outer);
    let c_equal_outer = c2(1.0, s0, a, d, r_outer);
    assert!((c_equal_inner - c_equal_outer).abs() < 1e-12);

    println!("two-layer continuum oracle: PASS");
    println!("interface concentration = {ci_outer:.12e}");
    println!("interface flux = {flux:.12e}");
    println!("K=1 transmission probability = {p:.12e}");
}