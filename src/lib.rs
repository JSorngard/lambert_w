// Copyright 2025 Johanna Sörngård
// SPDX-License-Identifier: MIT OR Apache-2.0

#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]

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

// This crate uses a build script to check for an environment variable and sets
// the `assert_no_panic` attribute if that variable is set to a specific value.
// We then check for that attribute when testing and if so we statically ensure that no
// function in the crate can panic using the `no-panic` crate.
// This is the source of all the `#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]` attributes.

/// The negative inverse of e (-1/e).
///
/// This is the branch point of the Lambert W function.
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

/// The principal branch of the Lambert W function computed to 50 bits of accuracy on 64-bit floats with Fukushima's method.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use lambert_w::lambert_w0;
///
/// let Ω = lambert_w0(1.0);
///
/// assert_eq!(Ω, 0.5671432904097839);
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
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
#[must_use = "this is a pure function that only returns a value and has no side effects"]
pub fn lambert_w0(z: f64) -> f64 {
    dw0c::dw0c(z - NEG_INV_E)
}

/// The principal branch of the Lambert W function, computed on 32-bit floats with Fukushima's method.
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
/// use lambert_w::lambert_w0f;
/// use approx::assert_abs_diff_eq;
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
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
#[must_use = "this is a pure function that only returns a value and has no side effects"]
pub fn lambert_w0f(z: f32) -> f32 {
    sw0f::sw0f(z)
}

/// The principal branch of the Lambert W function computed to 24 bits of accuracy on 64-bit floats with Fukushima's method.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use lambert_w::sp_lambert_w0;
/// use approx::assert_abs_diff_eq;
///
/// let Ω = sp_lambert_w0(1.0);
///
/// assert_abs_diff_eq!(
///     Ω,
///     0.5671432904097839,
///     epsilon = f64::from(f32::EPSILON) / 4.5
/// );
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
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
#[must_use = "this is a pure function that only returns a value and has no side effects"]
pub fn sp_lambert_w0(z: f64) -> f64 {
    sw0::sw0(z)
}

/// The secondary branch of the Lambert W function computed to 50 bits of accuracy on 64-bit floats with Fukushima's method.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use lambert_w::lambert_wm1;
///
/// let mln4 = lambert_wm1(-f64::ln(2.0) / 2.0);
///
/// assert_eq!(mln4, -f64::ln(4.0));
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
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
#[must_use = "this is a pure function that only returns a value and has no side effects"]
pub fn lambert_wm1(z: f64) -> f64 {
    dwm1c::dwm1c(z, z - NEG_INV_E)
}

/// The secondary branch of the Lambert W function, computed on 32-bit floats with Fukushima's method.
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
/// use lambert_w::lambert_wm1f;
///
/// let mln4 = lambert_wm1f(-f32::ln(2.0) / 2.0);
///
/// assert_eq!(mln4, -f32::ln(4.0));
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
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
#[must_use = "this is a pure function that only returns a value and has no side effects"]
pub fn lambert_wm1f(z: f32) -> f32 {
    swm1f::swm1f(z)
}

/// The secondary branch of the Lambert W function computed to 24 bits of accuracy on 64-bit floats with Fukushima's method.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use lambert_w::sp_lambert_wm1;
/// use approx::assert_abs_diff_eq;
///
/// let mln4 = sp_lambert_wm1(-f64::ln(2.0) / 2.0);
///
/// assert_abs_diff_eq!(
///     mln4,
///     -f64::ln(4.0),
///     epsilon = f64::from(f32::EPSILON) / 165.0
/// );
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
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
#[must_use = "this is a pure function that only returns a value and has no side effects"]
pub fn sp_lambert_wm1(z: f64) -> f64 {
    swm1::swm1(z)
}

/// Branch `k` of the complex valued Lambert W function computed
/// on 64-bit floats with Halley's method.
///
/// The return value is a tuple where the first element is the
/// real part and the second element is the imaginary part.
///
/// Close to the branch cut at -1/e this function may be slightly less accurate.
///
/// If you know you want the principal or secondary branches where they are real valued,
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
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
#[must_use = "this is a pure function that only returns a value and has no side effects"]
pub fn lambert_w(k: i32, z_re: f64, z_im: f64) -> (f64, f64) {
    let w = all_complex_branches::lambert_w_generic(k, num_complex::Complex64::new(z_re, z_im));
    (w.re, w.im)
}

/// Branch `k` of the complex valued Lambert W function computed
/// on 32-bit floats with Halley's method.
///
/// The return value is a tuple where the first element is the
/// real part and the second element is the imaginary part.
///
/// Close to the branch cut at -1/e this function may be slightly less accurate.
///
/// If you know you want the principal or secondary branches where they are real valued,
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
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
#[must_use = "this is a pure function that only returns a value and has no side effects"]
pub fn lambert_wf(k: i16, z_re: f32, z_im: f32) -> (f32, f32) {
    let w = all_complex_branches::lambert_w_generic(k, num_complex::Complex32::new(z_re, z_im));
    (w.re, w.im)
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
