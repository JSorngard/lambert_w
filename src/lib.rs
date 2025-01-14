//! # Description
//!
//! This crate provides fast and accurate evaluation of the real valued parts of the principal and secondary branches of the [Lambert W function](https://en.wikipedia.org/wiki/Lambert_W_function)
//! with the method of Toshio Fukushima \[[1](#references)\].
//!
//! This method works by dividing the function's domain into subdomains.
//! On each one, it uses a simple transformation of the input inserted into
//! a rational function to approximate the function's value.  
//! The implementation uses conditional switches on the input value
//! to select the appropriate subdomain, followed by either a square root
//! (and possibly a division) or a logarithm. Then it performs a series of
//! additions and multiplications by constants from a look-up table,
//! and finishes the calculation with a division.
//!
//! The crate provides two approximations of each branch, one with 50 bits of accuracy
//! (implemented on 64-bit floats) and one with 24 bits (implemented on 32- and 64-bit floats).
//! The one with 50 bits of accuracy uses higher degree polynomials in the rational functions
//! compared to the one with only 24 bits, and thus more of the
//! multiplications and additions by constants.
//!
//! This crate can evaluate the approximation with 24 bits of accuracy on
//! 32-bit floats, even though it is defined on 64-bit floats in Fukushima's paper.
//! This may result in a reduction in the accuracy to less than 24 bits,
//! but this reduction has not been quantified by the author of this crate.
//!
//! The crate is `no_std` compatible, but can optionally depend on the standard library
//! through features for a potential performance gain.
//!
//! ## Examples
//!
//! Compute the value of the [omega constant](https://en.wikipedia.org/wiki/Omega_constant) with the principal branch of the Lambert W function:
//!
//! ```
//! # use approx::assert_abs_diff_eq;
//! use lambert_w::lambert_w0;
//!
//! let Ω = lambert_w0(1.0);
//!
//! assert_abs_diff_eq!(Ω, 0.5671432904097839);
//! ```
//!
//! Evaluate the secondary branch of the Lambert W function at -ln(2)/2:
//!
//! ```
//! # use approx::assert_abs_diff_eq;
//! use lambert_w::lambert_wm1;
//!
//! let mln4 = lambert_wm1(-f64::ln(2.0) / 2.0);
//!
//! assert_abs_diff_eq!(mln4, -f64::ln(4.0));
//! ```
//!
//! Do it on 32-bit floats:
//!
//!```
//!# use approx::assert_abs_diff_eq;
//! use lambert_w::{lambert_w0f, lambert_wm1f};
//!
//! let Ω = lambert_w0f(1.0);
//! let mln4 = lambert_wm1f(-f32::ln(2.0) / 2.0);
//!
//! assert_abs_diff_eq!(Ω, 0.56714329);
//! assert_abs_diff_eq!(mln4, -f32::ln(4.0));
//! ```
//!
//! The implementation can handle extreme inputs just as well:
//!
//! ```
//! # use approx::assert_relative_eq;
//! use lambert_w::{lambert_w0, lambert_wm1};
//!
//! let big = lambert_w0(f64::MAX);
//! let tiny = lambert_wm1(-1e-308);
//!
//! assert_relative_eq!(big, 703.2270331047702, max_relative = 4e-16);
//! assert_relative_eq!(tiny, -715.7695669234213, max_relative = 4e-16);
//! ```
//!
//! Importing the [`LambertW`] trait lets you call the functions with postfix notation:
//!
//! ```
//! # use approx::assert_abs_diff_eq;
//! use lambert_w::LambertW;
//!
//! let z = 2.0 * f64::ln(2.0);
//!
//! assert_abs_diff_eq!(z.lambert_w0(), f64::ln(2.0));
//! ```
//!
//! The macros are from the [`approx`](https://docs.rs/approx/latest/approx/) crate, and are used in the documentation examples of this crate.
//! The assertion passes if the two supplied values are the same to within floating point error, or within an optional epsilon or relative difference.
//!
//! ## Features
//!
//! One of the below features must be enabled:
//!
//! `libm` *(enabled by default)*: if the `std` feature is disabled, this feature uses the [`libm`]
//! crate to compute square roots and logarithms during function evaluation instead of the standard library.
//!
//! `std`: use the standard library to compute square roots and logarithms for a potential performance gain.
//! When this feature is disabled the crate is `no_std` compatible.
//!
//! ## References
//!
//! \[1\]: Toshio Fukushima.
//! **Precise and fast computation of Lambert W function by piecewise minimax rational function approximation with variable transformation**.
//! DOI: [10.13140/RG.2.2.30264.37128](https://doi.org/10.13140/RG.2.2.30264.37128).
//! November 2020.
//!
//! [⬆️ Back to top](#description).

#![no_std]
#![forbid(unsafe_code)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(any(feature = "std", feature = "libm")))]
compile_error!("at least one of the `std` or `libm` features must be enabled");

mod dw0c;
mod dwm1c;
mod elementary;
mod rational;
mod sw0;
mod sw0f;
mod swm1;
mod swm1f;
#[cfg(test)]
mod unit_tests;

/// The negative inverse of e (-1/e).
///
/// This is the smallest input value for which the Lambert W functions in this crate return a value.
//            Rounded from -0.367_879_441_171_442_322
pub const NEG_INV_E: f64 = -0.367_879_441_171_442_32;

/// 1/sqrt(e)
//         Rounded from 0.606_530_659_712_633_423
const INV_SQRT_E: f64 = 0.606_530_659_712_633_4;

/// The omega constant (Ω).
///
/// Fulfills the equation Ωe^Ω = 1.
///
/// Has been rounded to the closest available `f64` value.
//        Rounded from 0.567_143_290_409_783_87
pub const OMEGA: f64 = 0.567_143_290_409_783_8;
// If we round the last two digits (87) to 9 rustc sets the constant to
//                     0.567_143_290_409_784
// which is further away from the true value than what we get if we round them to 8.

/// The principal branch of the Lambert W function computed to 24 bits of accuracy on `f64`s.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use approx::assert_abs_diff_eq;
/// use lambert_w::sp_lambert_w0;
///
/// let Ω = sp_lambert_w0(1.0);
///
/// assert_abs_diff_eq!(Ω, 0.5671432904097839, epsilon = 1e-7);
/// ```
///
/// Arguments smaller than -1/e (≈ -0.36787944117144233) result in [`NAN`](f64::NAN):
///
/// ```
/// # use lambert_w::sp_lambert_w0;
/// assert!(sp_lambert_w0(-1.0).is_nan());
/// ```
///
/// # Reference
///
/// [Toshio Fukushima, Precise and fast computation of Lambert W function by piecewise minimax rational function approximation with variable transformation](https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation).
pub fn sp_lambert_w0(z: f64) -> f64 {
    sw0::sw0(z)
}

/// The secondary branch of the Lambert W function computed to 24 bits of accuracy on `f64`s.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use approx::assert_abs_diff_eq;
/// use lambert_w::sp_lambert_wm1;
///
/// let mln4 = sp_lambert_wm1(-f64::ln(2.0) / 2.0);
///
/// assert_abs_diff_eq!(mln4, -f64::ln(4.0), epsilon = 1e-9);
/// ```
///
/// Arguments smaller than -1/e (≈ -0.36787944117144233) or larger than 0 result in [`NAN`](f64::NAN):
///
/// ```
/// # use lambert_w::sp_lambert_wm1;
/// assert!(sp_lambert_wm1(-1.0).is_nan());
/// assert!(sp_lambert_wm1(1.0).is_nan());
/// ```
///
/// # Reference
///
/// [Toshio Fukushima, Precise and fast computation of Lambert W function by piecewise minimax rational function approximation with variable transformation](https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation).
pub fn sp_lambert_wm1(z: f64) -> f64 {
    swm1::swm1(z)
}

/// The principal branch of the Lambert W function computed to 50 bits of accuracy.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use approx::assert_abs_diff_eq;
/// use lambert_w::lambert_w0;
///
/// let Ω = lambert_w0(1.0);
///
/// assert_abs_diff_eq!(Ω, 0.5671432904097839);
/// ```
///
/// Arguments smaller than -1/e (≈ -0.36787944117144233) result in [`NAN`](f64::NAN):
///
/// ```
/// # use lambert_w::lambert_w0;
/// assert!(lambert_w0(-1.0).is_nan());
/// ```
///
/// # Reference
///
/// [Toshio Fukushima, Precise and fast computation of Lambert W function by piecewise minimax rational function approximation with variable transformation](https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation).
pub fn lambert_w0(z: f64) -> f64 {
    dw0c::dw0c(z - NEG_INV_E)
}

/// The principal branch of the Lambert W function, computed with `f32`s.
///
/// Uses the same approximation as [`sp_lambert_w0`], but computes it with 32-bit floats,
/// which may result in slightly reduced accuracy.
/// This potential accuracy reduction has not been quantified.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use approx::assert_abs_diff_eq;
/// use lambert_w::lambert_w0f;
///
/// let Ω = lambert_w0f(1.0);
///
/// assert_abs_diff_eq!(Ω, 0.56714329);
/// ```
///
/// Arguments smaller than -1/e (≈ -0.36787944) result in [`NAN`](f32::NAN):
///
/// ```
/// # use lambert_w::lambert_w0f;
/// assert!(lambert_w0f(-1.0).is_nan());
/// ```
///
/// # Reference
///
/// [Toshio Fukushima, Precise and fast computation of Lambert W function by piecewise minimax rational function approximation with variable transformation](https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation).
pub fn lambert_w0f(z: f32) -> f32 {
    sw0f::sw0f(z)
}

/// The secondary branch of the Lambert W function computed to 50 bits of accuracy.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use approx::assert_abs_diff_eq;
/// use lambert_w::lambert_wm1;
///
/// let mln4 = lambert_wm1(-f64::ln(2.0) / 2.0);
///
/// assert_abs_diff_eq!(mln4, -f64::ln(4.0), epsilon = 1e-14);
/// ```
///
/// Arguments smaller than -1/e (≈ -0.36787944117144233) or larger than 0 result in [`NAN`](f64::NAN):
///
/// ```
/// # use lambert_w::lambert_wm1;
/// assert!(lambert_wm1(-1.0).is_nan());
/// assert!(lambert_wm1(1.0).is_nan());
/// ```
///
/// # Reference
///
/// [Toshio Fukushima, Precise and fast computation of Lambert W function by piecewise minimax rational function approximation with variable transformation](https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation).
pub fn lambert_wm1(z: f64) -> f64 {
    dwm1c::dwm1c(z, z - NEG_INV_E)
}

/// The secondary branch of the Lambert W function, computed with `f32`s.
///
/// Uses the same approximation as [`sp_lambert_wm1`], but computes it with 32-bit floats,
/// which may result in slightly reduced accuracy.
/// This potential accuracy reduction has not been quantified.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use approx::assert_abs_diff_eq;
/// use lambert_w::lambert_wm1f;
///
/// let mln4 = lambert_wm1f(-f32::ln(2.0) / 2.0);
///
/// assert_abs_diff_eq!(mln4, -f32::ln(4.0));
/// ```
///
/// Arguments smaller than -1/e (≈ -0.36787944) or larger than 0 result in [`NAN`](f32::NAN):
///
/// ```
/// # use lambert_w::lambert_wm1f;
/// assert!(lambert_wm1f(-1.0).is_nan());
/// assert!(lambert_wm1f(1.0).is_nan());
/// ```
///
/// # Reference
///
/// [Toshio Fukushima, Precise and fast computation of Lambert W function by piecewise minimax rational function approximation with variable transformation](https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation).
pub fn lambert_wm1f(z: f32) -> f32 {
    swm1f::swm1f(z)
}

/// Enables evaluation of the principal and secondary branches of the Lambert W function
/// on the types that implement this trait.
pub trait LambertW {
    /// The type returned by the Lambert W functions when acting on a value of type `Self`.
    type Output;

    /// The principal branch of the Lambert W function.
    fn lambert_w0(self) -> Self::Output;

    /// The secondary branch of the Lambert W function.
    fn lambert_wm1(self) -> Self::Output;
}

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
