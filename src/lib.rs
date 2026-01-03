// Copyright 2025 Johanna Sörngård
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

pub use all_complex_branches::{lambert_w, lambert_wf, ErrorTolerance};
pub use dw0c::lambert_w0;
pub use dwm1c::lambert_wm1;
pub use sw0::sp_lambert_w0;
pub use sw0f::lambert_w0f;
pub use swm1::sp_lambert_wm1;
pub use swm1f::lambert_wm1f;

/// The negative inverse of e (-1/e).
///
/// This is the branch point of the Lambert W function.
///
/// ```
/// use lambert_w::{lambert_w0, NEG_INV_E};
/// use approx::assert_abs_diff_eq;
///
/// assert_abs_diff_eq!(lambert_w0(NEG_INV_E), -1.0);
/// assert!(lambert_w0(NEG_INV_E.next_down()).is_nan());
/// ```
//            Rounded from -0.367_879_441_171_442_322
pub const NEG_INV_E: f64 = -0.367_879_441_171_442_32;

/// 1/sqrt(e)
//         Rounded from 0.606_530_659_712_633_423
const INV_SQRT_E: f64 = 0.606_530_659_712_633_4;

/// The omega constant (Ω).
///
/// Fulfills the equation Ωe^Ω = 1:
///
/// ```
/// use lambert_w::OMEGA;
/// use approx::assert_abs_diff_eq;
///
/// assert_abs_diff_eq!(OMEGA * f64::exp(OMEGA), 1.0);
/// ```
// We include more digits than fit in an f64 because if we write
// 0.567_143_290_409_783_8 (clippy's suggestion without excessive precision)
// it looks as if we have rounded it incorrectly,
// since the correctly rounded value to that many digits would be
// 0.567_143_290_409_783_9.
// However, if we write the correctly rounded value the compiler rounds it to
// 0.567_143_290_409_784, which is further from the true value than
// 0.567_143_290_409_783_8.
// To avoid all this confusion for any potential readers of the docs
// we just add more digits so that the compiler rounds it correctly and then
// allow the clippy lint.
#[allow(clippy::excessive_precision)]
pub const OMEGA: f64 = 0.567_143_290_409_783_873;
