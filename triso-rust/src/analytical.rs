//! Separate analytical reference track.
//! This is intentionally not the FV solver.

use std::f64::consts::PI;

/// Dimensionless Robin-sphere eigenvalue residual for phi=sin(z r/R)/r.
/// Boundary: -D phi'(R)=h phi(R), Bi=hR/D.
/// Root equation: z cot(z) = 1 - Bi.
pub fn robin_residual(z: f64, bi: f64) -> f64 {
    z / z.tan() - (1.0 - bi)
}

/// Locate positive Robin eigenvalues by bracket scanning and bisection.
/// Poles of cot(z) are skipped by scanning finite sign changes.
pub fn robin_roots(bi: f64, count: usize) -> Vec<f64> {
    let mut roots = Vec::new();
    let eps = 1e-8;
    for n in 0..(count + 4) {
        let lo = n as f64 * PI + eps;
        let hi = (n as f64 + 1.0) * PI - eps;
        let segments = 128;
        let mut x0 = lo;
        let mut f0 = robin_residual(x0, bi);
        for k in 1..=segments {
            let x1 = lo + (hi - lo) * k as f64 / segments as f64;
            let f1 = robin_residual(x1, bi);
            if f0.is_finite() && f1.is_finite() && f0 * f1 < 0.0 {
                let mut a = x0;
                let mut b = x1;
                let mut fa = f0;
                for _ in 0..80 {
                    let m = 0.5 * (a + b);
                    let fm = robin_residual(m, bi);
                    if fa * fm <= 0.0 {
                        b = m;
                    } else {
                        a = m;
                        fa = fm;
                    }
                }
                let root = 0.5 * (a + b);
                if root > 1e-6 && roots.last().is_none_or(|x: &f64| (root - *x).abs() > 1e-6) {
                    roots.push(root);
                    if roots.len() == count {
                        return roots;
                    }
                }
            }
            x0 = x1;
            f0 = f1;
        }
    }
    roots
}

/// Two-component state [u, D u'] propagation through a homogeneous layer
/// for u''+k^2 u=0. This is the building block of a five-layer transfer matrix.
pub fn transfer_layer(u: f64, du_flux: f64, d: f64, k: f64, length: f64) -> (f64, f64) {
    let c = (k * length).cos();
    let s = (k * length).sin();
    let u2 = c * u + s * du_flux / (d * k);
    let q2 = -d * k * s * u + c * du_flux;
    (u2, q2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn robin_roots_satisfy_residual() {
        let roots = robin_roots(2.0, 4);
        assert_eq!(roots.len(), 4);
        for z in roots {
            assert!(robin_residual(z, 2.0).abs() < 1e-10);
        }
    }

    #[test]
    fn transfer_zero_length_is_identity() {
        let (u, q) = transfer_layer(2.0, -3.0, 0.5, 1.7, 0.0);
        assert!((u - 2.0).abs() < 1e-14);
        assert!((q + 3.0).abs() < 1e-14);
    }

    #[test]
    fn transfer_preserves_state_across_equal_material_subdivision() {
        let (u1, q1) = transfer_layer(1.2, 0.7, 2.0, 0.9, 0.3);
        let (u2, q2) = transfer_layer(u1, q1, 2.0, 0.9, 0.4);
        let (u, q) = transfer_layer(1.2, 0.7, 2.0, 0.9, 0.7);
        assert!((u2 - u).abs() < 1e-12);
        assert!((q2 - q).abs() < 1e-12);
    }
}
