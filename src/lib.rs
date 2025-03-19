#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]

#[cfg(feature = "std")]
extern crate std;

use core::f64::consts::{E, PI};
use num_complex::{Complex64, ComplexFloat};

#[cfg(not(any(feature = "std", feature = "libm")))]
compile_error!("at least one of the `std` or `libm` features must be enabled");

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

/// Computes branch `k` of the complex valued Lambert W function
/// using Halley's method.
///
/// The return value is a tuple where the first element is the
/// real part and the second element is the imaginary part.
///
/// Close to the branch cut at -1/e this function may be slightly less accurate.
///
/// If you know you want the principal or secondary branches where they are real valued,
/// take a look at the [`lambert_w0`] or [`lambert_wm1`] functions and their 32-bit equivalents instead.
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
    const MAX_ITER: usize = 30;
    /// If the absolute difference between two consecutive iterations is less than this value,
    /// the iteration stops.
    const PREC: f64 = 1e-30;
    const I: Complex64 = Complex64::I;

    let z = Complex64::new(z_re, z_im);

    // region: special cases

    if z == Complex64::ZERO {
        if k == 0 {
            return (0.0, 0.0);
        } else {
            return (f64::NEG_INFINITY, 0.0);
        }
    }
    if z.re == NEG_INV_E && z.im == 0.0 && (k == 0 || k == -1) {
        return (-1.0, 0.0);
    }
    if z.re == E && z.im == 0.0 && k == 0 {
        return (1.0, 0.0);
    }

    // endregion: special cases

    // region: determine initial search point

    let two_pi_k_i = 2.0 * PI * f64::from(k) * I;
    let mut w = z.ln() + two_pi_k_i - (z.ln() + two_pi_k_i).ln();

    // Choose the initial point more carefully when we are close to the branch cut.
    if (z - NEG_INV_E).abs() <= 1.0 {
        let p = (2.0 * (E * z + 1.0)).sqrt();
        let p2 = -1.0 + p - 1.0 / 3.0 * p * p;
        let p3 = 11.0 / 72.0 * p * p * p;
        if k == 0 {
            w = p2 + p3;
        } else if (k == 1 && z.im < 0.0) || (k == -1 && z.im > 0.0) {
            w = p2 - p3;
        }
    }

    if k == 0 && (z - 0.5).abs() <= 0.5 {
        // Order (1,1) Padé approximant for W(0,a)
        w = (0.35173371 * (0.1237166 + 7.061302897 * z)) / (2.0 + 0.827184 * (1.0 + 2.0 * z));
    }

    if k == -1 && (z - 0.5).abs() <= 0.5 {
        // Order (1,1) Padé approximant for W(-1,a)
        w = -(((2.2591588985 + 4.22096 * I)
            * ((-14.073271 - 33.767687754 * I) * z - (12.7127 - 19.071643 * I) * (1.0 + 2.0 * z)))
            / (2.0 - (17.23103 - 10.629721 * I) * (1.0 + 2.0 * z)));
    }

    // endregion: determine initial search point

    // region: Halley iteration

    let mut iter = 0;
    loop {
        let w_prev = w;
        let ew = w.exp();
        let wew = w * ew;
        let wew_d = ew + wew;
        let wew_dd = ew + wew_d;
        w -= 2.0 * ((wew - z) * wew_d) / (2.0 * wew_d * wew_d - (wew - z) * wew_dd);

        iter += 1;

        if ((w - w_prev).abs() <= PREC) || iter >= MAX_ITER {
            return (w.re, w.im);
        }
    }

    // endregion: Halley iteration
}

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
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
#[must_use = "this is a pure function that only returns a value and has no side effects"]
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
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
#[must_use = "this is a pure function that only returns a value and has no side effects"]
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
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
#[must_use = "this is a pure function that only returns a value and has no side effects"]
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
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
#[must_use = "this is a pure function that only returns a value and has no side effects"]
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
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
#[must_use = "this is a pure function that only returns a value and has no side effects"]
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
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
#[must_use = "this is a pure function that only returns a value and has no side effects"]
pub fn lambert_wm1f(z: f32) -> f32 {
    swm1f::swm1f(z)
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
