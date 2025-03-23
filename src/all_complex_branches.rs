//! This module contains the general implementation of the Lambert W function.
//! This implementation is capable of computing the function at any point in the complex plane on any branch.

use num_complex::{Complex, Complex32, Complex64, ComplexFloat};
use num_traits::{Float, FromPrimitive, Signed};

use core::{
    f64::consts::{E, PI},
    ops::{Add, Mul, Sub, SubAssign},
};

use crate::NEG_INV_E;

/// Computes branch `k` of the complex valued Lambert W function
/// using Halley's method.
///
/// The return value is a tuple where the first element is the
/// real part and the second element is the imaginary part.
///
/// Close to the branch cut at -1/e this function may be slightly less accurate.
///
/// If you know you want the principal or secondary branches where they are real valued,
/// take a look at the [`lambert_w0`](crate::lambert_w0) or [`lambert_wm1`](crate::lambert_wm1) functions instead.
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
    let w = lambert_w_generic(k, Complex64::new(z_re, z_im));
    (w.re, w.im)
}

/// Computes branch `k` of the complex valued Lambert W function
/// using Halley's method.
///
/// The return value is a tuple where the first element is the
/// real part and the second element is the imaginary part.
///
/// Close to the branch cut at -1/e this function may be slightly less accurate.
///
/// If you know you want the principal or secondary branches where they are real valued,
/// take a look at the [`lambert_w0f`](crate::lambert_w0f) or [`lambert_wm1f`](crate::lambert_wm1f) functions instead.
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
    let w = lambert_w_generic(k, Complex32::new(z_re, z_im));
    (w.re, w.im)
}

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn lambert_w_generic<T, U>(k: U, z: Complex<T>) -> Complex<T>
where
    U: Signed + Copy,
    T: Float
        + FromPrimitive
        + From<U>
        + Mul<Complex<T>, Output = Complex<T>>
        + Add<Complex<T>, Output = Complex<T>>
        + Sub<Complex<T>, Output = Complex<T>>,
    Complex<T>: ComplexFloat
        + SubAssign
        + Mul<T, Output = Complex<T>>
        + Add<T, Output = Complex<T>>
        + Sub<T, Output = Complex<T>>,
{
    const MAX_ITER: usize = 30;
    /// If the absolute difference between two consecutive iterations is less than this value,
    /// the iteration stops.
    const PREC: f64 = 1e-30;

    // region: construct constants of the generic type

    let i_zero = U::zero();
    let i_one = U::one();

    let d_zero = T::zero();
    let d_one = T::one();
    let d_two = d_one + d_one;
    let d_half = d_one / d_two;
    let d_e: T = t_from_f64_or_f32(E);
    let d_pi: T = t_from_f64_or_f32(PI);
    let d_neg_inv_e: T = t_from_f64_or_f32(NEG_INV_E);

    let i = Complex::<T>::i();
    let z_zero = Complex::<T>::from(d_zero);
    let z_one = Complex::<T>::from(d_one);
    let z_two = z_one + z_one;

    let z_neg_inv_e = Complex::<T>::from(d_neg_inv_e);
    let z_half = z_one / z_two;

    // These values are only constructed to help the compliler see that
    // they are the same type as what Complex<T>::abs() returns.
    let abs_one = z_one.abs();
    let abs_half = z_half.abs();
    let abs_prec = Complex::<T>::from(t_from_f64_or_f32::<T>(PREC)).abs();

    // endregion: construct constants of the generic type

    // region: special cases

    if z == z_zero {
        if k == i_zero {
            return z_zero;
        } else {
            return T::neg_infinity().into();
        }
    }
    if z == d_neg_inv_e.into() && (k == i_zero || k == -i_one) {
        return -z_one;
    }
    if z == d_e.into() && k == i_zero {
        return z_one;
    }

    // endregion: special cases

    // region: determine initial search point

    let two_pi_k_i = z_two * d_pi * <T as From<U>>::from(k) * i;
    let mut w = z.ln() + two_pi_k_i - (z.ln() + two_pi_k_i).ln();

    // Choose the initial point more carefully when we are close to the branch cut.
    if (z - z_neg_inv_e).abs() <= abs_one {
        let p = (d_two * (d_e * z + d_one)).sqrt();
        let p2 = -z_one + p - t_from_f64_or_f32::<T>(1.0 / 3.0) * p * p;
        let p3 = t_from_f64_or_f32::<T>(11.0 / 72.0) * p * p * p;
        if k == i_zero {
            w = p2 + p3;
        } else if (k == i_one && z.im < d_zero) || (k == -i_one && z.im > d_zero) {
            w = p2 - p3;
        }
    }

    if k == i_zero && (z - d_half).abs() <= abs_half {
        // Order (1,1) Padé approximant for the principal branch
        w = (t_from_f64_or_f32::<T>(0.35173371)
            * (t_from_f64_or_f32::<T>(0.1237166) + t_from_f64_or_f32::<T>(7.061302897) * z))
            / (d_two + t_from_f64_or_f32::<T>(0.827184) * (d_one + d_two * z));
    }

    if k == -i_one && (z - d_half).abs() <= abs_half {
        // Order (1,1) Padé approximant for the secondary branch
        w = -(((t_from_f64_or_f32::<T>(2.2591588985) + t_from_f64_or_f32::<T>(4.22096) * i)
            * ((t_from_f64_or_f32::<T>(-14.073271) - t_from_f64_or_f32::<T>(33.767687754) * i)
                * z
                - (t_from_f64_or_f32::<T>(12.7127) - t_from_f64_or_f32::<T>(19.071643) * i)
                    * (d_one + d_two * z)))
            / (d_two
                - (t_from_f64_or_f32::<T>(17.23103) - t_from_f64_or_f32::<T>(10.629721) * i)
                    * (d_one + d_two * z)));
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
        w -= d_two * ((wew - z) * wew_d) / (d_two * wew_d * wew_d - (wew - z) * wew_dd);

        iter += 1;

        if (w - w_prev).abs() <= abs_prec || iter >= MAX_ITER {
            return w;
        }
    }

    // endregion: Halley iteration
}

/// Attempts to convert a `f64` to a `T`.
///
/// If that fails, it tries to convert the `f64` to a `f32` with `as` and then to a `T`.
///
/// # Panics
///
/// Panics if a `T` cannot be created from a `f32`.
#[inline(always)]
fn t_from_f64_or_f32<T>(x: f64) -> T
where
    T: FromPrimitive,
{
    T::from_f64(x).unwrap_or(T::from_f32(x as f32).unwrap())
}
