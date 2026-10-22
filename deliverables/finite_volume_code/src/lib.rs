//! Code map.
//!
//! - Code A, main file: `src/main.rs`. Reads the input file, runs B and C on it and writes the results.
//! - Code B, discrete approach (finite volume, tridiagonal solve): `src/discrete_finite_volume_tridiagonal.rs`.
//! - Code C, analytical approach (Sturm-Liouville eigenfunction series): `src/analytical_sturm_liouville.rs`.
//!
//! Supporting modules: `config` (input file), `compare` (B against C), `plots` (figures).

pub mod analytical_sturm_liouville;
pub mod compare;
pub mod config;
pub mod discrete_finite_volume_tridiagonal;
pub mod plots;
