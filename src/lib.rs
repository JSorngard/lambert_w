// Copyright 2026 Johanna Sörngård
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

// Use the semver trick to ease refactoring of dependents when they with to upgrade.
// https://github.com/dtolnay/semver-trick
pub use lambert_w2::lambert_w0;
pub use lambert_w2::lambert_w0f;
pub use lambert_w2::lambert_wm1;
pub use lambert_w2::lambert_wm1f;
pub use lambert_w2::sp_lambert_w0;
pub use lambert_w2::sp_lambert_wm1;
pub use lambert_w2::NEG_INV_E;
pub use lambert_w2::OMEGA;

// Remember to change the docstring of `lambert_w_generic` if you change the above value.

/// Branch `k` of the complex valued Lambert W function computed
/// on 64-bit floats with Halley's method.
///
/// The return value is a tuple where the first element is the
/// real part and the second element is the imaginary part.
///
/// This function may be slightly less accurate close to the branch cut at -1/e,
/// as well as close to zero on branches other than k=0.
///
/// If you know you want the principal or secondary branches where they are real-valued,
/// take a look at the [`lambert_w0`] or [`lambert_wm1`] functions instead.
/// They can be up to two orders of magnitude faster.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use lambert_w::lambert_w;
///
/// // W_2(1 + 2i)
/// let w = lambert_w(2, 1.0, 2.0);
///
/// assert_eq!(w, (-1.6869138779375397, 11.962631435322813));
/// ```
///
/// Returns [`NAN`](f64::NAN)s if any of the inputs are infinite:
///
/// ```
/// # use lambert_w::lambert_w;
/// let w = lambert_w(-13, f64::INFINITY, 0.0);
///
/// assert!(w.0.is_nan() && w.1.is_nan());
/// ```
///
/// or `NAN`:
///
/// ```
/// # use lambert_w::lambert_w;
/// let w = lambert_w(1_000, 0.0, f64::NAN);
///
/// assert!(w.0.is_nan() && w.1.is_nan());
/// ```
#[must_use = "this is a pure function that only returns a value and has no side effects"]
#[inline]
pub fn lambert_w(k: i32, z_re: f64, z_im: f64) -> (f64, f64) {
    lambert_w2::lambert_w(k, z_re, z_im, f64::EPSILON)
}

/// Branch `k` of the complex valued Lambert W function computed
/// on 32-bit floats with Halley's method.
///
/// The return value is a tuple where the first element is the
/// real part and the second element is the imaginary part.
///
/// This function may be slightly less accurate close to the branch cut at -1/e,
/// as well as close to zero on branches other than k=0.
///
/// If you know you want the principal or secondary branches where they are real-valued,
/// take a look at the [`lambert_w0f`] or [`lambert_wm1f`] functions instead.
/// They can be up to two orders of magnitude faster.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use lambert_w::lambert_wf;
///
/// // W_2(1 + 2i)
/// let w = lambert_wf(2, 1.0, 2.0);
///
/// assert_eq!(w, (-1.6869138, 11.962631));
/// ```
///
/// Returns [`NAN`](f32::NAN)s if any of the inputs are infinite:
///
/// ```
/// # use lambert_w::lambert_wf;
/// let w = lambert_wf(-13, f32::INFINITY, 0.0);
///
/// assert!(w.0.is_nan() && w.1.is_nan());
/// ```
///
/// or `NAN`:
///
/// ```
/// # use lambert_w::lambert_wf;
/// let w = lambert_wf(1_000, 0.0, f32::NAN);
///
/// assert!(w.0.is_nan() && w.1.is_nan());
/// ```
#[must_use = "this is a pure function that only returns a value and has no side effects"]
#[inline]
pub fn lambert_wf(k: i16, z_re: f32, z_im: f32) -> (f32, f32) {
    lambert_w2::lambert_wf(k, z_re, z_im, f32::EPSILON)
}

/// Enables evaluation of the principal and secondary branches of the Lambert W function
/// on the types that implement this trait.
#[deprecated(
    since = "1.1.0",
    note = "use the functions directly or create your own trait, the `lambert_w` crate is not the place for making such API decisions for others."
)]
pub trait LambertW {
    /// The type returned by the Lambert W functions when acting on a value of type `Self`.
    type Output;

    /// The principal branch of the Lambert W function.
    fn lambert_w0(self) -> Self::Output;

    /// The secondary branch of the Lambert W function.
    fn lambert_wm1(self) -> Self::Output;
}

#[allow(deprecated)]
impl LambertW for f32 {
    type Output = Self;
    /// The principal branch of the Lambert W function.
    ///
    /// Evaluated with the approximation with 24-bits of accuracy from the paper, but on 32-bit floats.
    ///
    /// Arguments smaller than -1/e (≈ -0.36787944) result in [`NAN`](f32::NAN).
    ///
    /// Delegates to the [`lambert_w0f`] function.
    #[inline]
    fn lambert_w0(self) -> Self::Output {
        lambert_w0f(self)
    }
    /// The secondary branch of the Lambert W function.
    ///
    /// Evaluated with the approximation with 24-bits of accuracy from the paper, but on 32-bit floats.
    ///
    /// Arguments smaller than -1/e (≈ -0.36787944) or larger than 0 result in [`NAN`](f32::NAN).
    ///
    /// Delegates to the [`lambert_wm1f`] function.
    #[inline]
    fn lambert_wm1(self) -> Self::Output {
        lambert_wm1f(self)
    }
}

#[allow(deprecated)]
impl LambertW for f64 {
    type Output = Self;
    /// The principal branch of the Lambert W function evaluated to 50 bits of accuracy.
    ///
    /// Arguments smaller than -1/e (≈ -0.36787944117144233) result in [`NAN`](f64::NAN).
    ///
    /// Delegates to the [`lambert_w0`] function.
    #[inline]
    fn lambert_w0(self) -> Self::Output {
        lambert_w0(self)
    }
    /// The secondary branch of the Lambert W function evaluated to 50 bits of accuracy.
    ///
    /// Arguments smaller than -1/e (≈ -0.36787944117144233) or larger than 0 result in [`NAN`](f64::NAN).
    ///
    /// Delegates to the [`lambert_wm1`] function.
    #[inline]
    fn lambert_wm1(self) -> Self::Output {
        lambert_wm1(self)
    }
}
