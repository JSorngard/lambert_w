// Copyright 2024-2026 Johanna Sörngård
// SPDX-License-Identifier: MIT OR Apache-2.0

// Fukushima's own Fortran implementation of the method in the paper can be found at
// https://www.researchgate.net/publication/346096162_xlambwtxt_Fortran_90_test_program_package_of_sw0_swm1_dw0c_and_dwm1c_low-_and_high-precision_procedures_computing_primary_and_secondary_branch_of_Lambert_W_function_W_0z_or_W_-1z_by_piecewise_minimax_

// These Markdown ideas are taken from https://linebender.org/blog/doc-include.
//
// This style is used in the readme itself to hide specific parts of it when rendered on docs.rs.
//! <style>
//! .rustdoc-hidden { display: none; }
//! </style>
// These links take precedence over the ones in the readme since they occur first.
// This allows us to switch where the listed hyperlinks take the user depending on whether the docs are read on crates.io or docs.rs.
//! [`approx`]: https://docs.rs/approx/latest/approx/
//! [`libm`]: https://docs.rs/libm/latest/libm/
#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]
#![forbid(clippy::unwrap_used)]
#![forbid(clippy::expect_used)]
#![forbid(clippy::panic)]
#![forbid(clippy::indexing_slicing)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(any(feature = "std", feature = "libm")))]
compile_error!("at least one of the `std` or `libm` features must be enabled");

mod all_complex_branches;
mod dw0c;
mod dwm1c;
mod generic_math;
mod sw0;
mod sw0f;
mod swm1;
mod swm1f;
#[cfg(test)]
mod unit_tests;

pub use all_complex_branches::{lambert_w, lambert_wf};
pub use dw0c::lambert_w0;
pub use dwm1c::lambert_wm1;
pub use sw0::sp_lambert_w0;
pub use sw0f::lambert_w0f;
pub use swm1::sp_lambert_wm1;
pub use swm1f::lambert_wm1f;

/// The floating point value closest to the negative reciprocal of e (-1/e), the branch point of the W function.
///
/// ```text
/// floats:      f--          f-           f+           f++
/// real line: --|------------|---x--------|------------|-------> z
///                             -1/e
/// ```
/// f- is this constant and x marks the location of the true value of -1/e. f+ is the smallest value for which the W functions in this crate return a real answer.
///
/// # Example
///
/// ```
/// use lambert_w::{lambert_w0, NEG_INV_E};
///
/// assert!(lambert_w0(NEG_INV_E).is_nan());
/// assert!(!lambert_w0(NEG_INV_E.next_up()).is_nan())
/// ```
#[allow(clippy::excessive_precision)]
pub const NEG_INV_E: f64 = -0.367_879_441_171_442_321_595_523_770;

/// The distance between `NEG_INV_E` and the true value of -1/e.
const NEG_INV_E_CORRECTION: f64 = -1.2428753672788363e-17;

/// The floating point value closest to 1/sqrt(e).
#[allow(clippy::excessive_precision)]
const INV_SQRT_E: f64 = 0.606_530_659_712_633_423_603_799_535;

/// The floating point value closest to the omega constant (Ω).
///
/// Fulfills the equation Ωe^Ω = 1:
///
/// ```
/// use lambert_w::OMEGA;
/// use approx::assert_abs_diff_eq;
///
/// assert_abs_diff_eq!(OMEGA * f64::exp(OMEGA), 1.0);
/// ```
#[allow(clippy::excessive_precision)]
pub const OMEGA: f64 = 0.567_143_290_409_783_873;
