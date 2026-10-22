//! Gives the examples and tests in this folder access to the delivered code files.
//! `main.rs` already contains codes B and C as modules (declared with `mod`).

#![allow(dead_code, clippy::all)]

#[path = "../../../deliverables/main.rs"]
pub mod delivered;

pub use delivered::{
    analytical_sturm_liouville, compare, config, discrete_finite_volume_tridiagonal, plots,
};
