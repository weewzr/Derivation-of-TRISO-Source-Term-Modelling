/// Adaptive two-layer transient continuum reference generator.
///
/// This program intentionally contains no WOS code. It only evaluates the
/// independently derived two-layer continuum eigenmodes so that later WOS
/// results can be compared against a reference that does not depend on the
/// implementation being tested.

#[(derive(Clone, Copy, Debug))]
struct Params {
    a: f64,
    r_outer: f64,
    d1: f64,
    d2: f64,
    s0: f64,
}

fn characteristic(lambda: f64, p: Params) -> f64 {
    let k1 = (lambda / p.d1).sqrt();
    let k2 = (lambda / p.d2).sqrt();
    d1k1_cos_sin(k1, k2, p.a, p.r_outer, p.d1, p.d2)
}

fn d1k1_cos_sin(k1: f64, k2: f64, a: f64, r_outer: f64, d1: f64, d2: f64) -> f64 {
    let x = k1 * a;
    let y = k2 * (r_outer - a);
    d1 * k1 * x.cos() * y.sin()
        + d2 * k2 * x.sin() * y.cos()
        + (d2 - d1) / a * x.sin() * y.sin()
}

fn steady(r: f64, p: Params) -> f64 {
    if r <= p.a {
        p.s0 * p.a.powi(3) / (3.0 * p.d2) * (1.0 / p.a - 1.0 / p.r_outer)
            + p.s0 / (6.0 * p.d1) * (p.a * p.a - r * r)
    } else {
        p.s0 * p.a.powi(3) / (3.0 * p.d2) * (1.0 / r - 1.0 / p.r_outer)
    }
}

fn mode_shape(r: f64, lambda: f64, p: Params) -> f64 {
    let k1 = (lambda / p.d1).sqrt();
    let k2 = (lambda / p.d2).sqrt();
    let raw = if r <= p.a {
        (k1 * r).sin()
    } else {
        let denom = (k2 * (p.r_outer - p.a)).sin();
        let amp = (k1 * p.a).sin() / denom;
        amp * (k2 * (p.r_outer - r)).sin()
    };
    if r.abs() < 1e-12 { k1 } else { raw / r }
}

// The normalization coefficient cancels in the ratio defining the projection.
// We use numerical quadrature on each material interval.
fn coefficient(lambda: f64, p: Params, nquad: usize) -> f64 {
    let mut num = 0.0;
    let mut den = 0.0;
    let h1 = p.a / nquad as f64;
    for j in 0..nquad {
        let r = (j as f64 + 0.5) * h1;
        let phi = mode_shape(r, lambda, p);
        let css = steady(r, p);
        num += r * r * css * phi * h1;
        den += r * r * phi * phi * h1;
    }
    let h2 = (p.r_outer - p.a) / nquad as f64;
    for j in 0..nquad {
        let r = p.a + (j as f64 + 0.5) * h2;
        let phi = mode_shape(r, lambda, p);
        let css = steady(r, p);
        num += r * r * css * phi * h2;
        den += r * r * phi * phi * h2;
    }
    -num / den
}

fn bracket_root(f: impl Fn(f64) -> f64, lo: f64, hi: f64, n: usize) -> Option<(f64, f64)> {
    let mut x0 = lo;
    let mut f0 = f(x0);
    for i in 1..=n {
        let x1 = lo + (hi - lo) * i as f64 / n as f64;
        let f1 = f(x1);
        if f0 == 0.0 { return Some((x0, x0)); }
        if f0.is_finite() && f1.is_finite() && f0.signum() != f1.signum() {
            return Some((x0, x1));
        }
        x0 = x1;
        f0 = f1;
    }
    None
}

fn bisect(f: impl Fn(f64) -> f64, mut lo: f64, mut hi: f64) -> f64 {
    let mut flo = f(lo);
    for _ in 0..100 {
        let mid = 0.5 * (lo + hi);
        let fm = f(mid);
        if fm.abs() < 1e-13 { return mid; }
        if flo.signum() == fm.signum() {
            lo = mid;
            flo = fm;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

fn eigenvalues(p: Params, count: usize) -> Vec<f64> {
    let mut roots = Vec::with_capacity(count);
    let mut x = 1e-10_f64;
    let mut fx = characteristic(x, p);
    let mut step = 1e-4_f64;
    while roots.len() < count && x < 100.0 {
        let next = x + step;
        let fnxt = characteristic(next, p);
        if fx.is_finite() && fnxt.is_finite() && fx.signum() != fnxt.signum() {
            let root = bisect(|z| characteristic(z, p), x, next);
            if roots.last().map_or(true, |r| (root - r).abs() > 1e-7) {
                roots.push(root);
            }
        }
        x = next;
        fx = fnxt;
        step = (step * 1.001).min(0.05);
    }
    roots
}

fn transient(r: f64, t: f64, p: Params, lambdas: &[f64], nquad: usize) -> f64 {
    let mut sum = 0.0;
    for &lambda in lambdas {
        let a_n = coefficient(lambda, p, nquad);
        sum += a_n * mode_shape(r, lambda, p) * (-lambda * t).exp();
    }
    steady(r, p) + sum
}

fn main() {
    let p = Params { a: 2.0, r_outer: 5.0, d1: 3.0, d2: 0.25, s0: 2.0 };
    let lambdas = eigenvalues(p, 60);
    assert!(lambdas.len() >= 12, "only found {} modes", lambdas.len());

    println!("lambda_1 = {:.15e}", lambdas[0]);
    for n in 1..=6 {
        let x = lambdas[n - 1];
        println!("mode {:>2}: lambda={:.15e}, F={:.3e}", n, x, characteristic(x, p));
    }

    let points = [(0.0,0.1),(2.0,0.1),(3.0,0.1),(4.0,0.1),(0.0,0.5),(2.0,0.5),(3.0,0.5),(4.0,0.5),(0.0,1.0),(2.0,1.0),(3.0,1.0),(4.0,1.0)];
    for (r,t) in points {
        let c12 = transient(r,t,p,&lambdas[0..12],4000);
        let c24 = transient(r,t,p,&lambdas[0..24],4000);
        let c48 = transient(r,t,p,&lambdas[0..48],4000);
        let e = (c48-c24).abs().max((c24-c12).abs());
        println!("r={r:.3}, t={t:.3}, c12={c12:.12e}, c24={c24:.12e}, c48={c48:.12e}, err={e:.3e}");
    }
}