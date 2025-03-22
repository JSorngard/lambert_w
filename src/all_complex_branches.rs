//! This module contains the general implementation of the Lambert W function.
//! This implementation is capable of computing the function at any point in the complex plane on any branch.

use num_complex::{Complex, Complex32, Complex64, ComplexFloat};
use num_traits::{Float, FromPrimitive, Signed};

use core::{
    f32::NEG_INFINITY,
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
/// take a look at the [`lambert_w0`](crate::lambert_w0) or [`lambert_wm1`](crate::lambert_wm1) functions and their 32-bit equivalents instead.
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

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
#[must_use = "this is a pure function that only returns a value and has no side effects"]
pub fn lambert_wf(k: i16, z_re: f32, z_im: f32) -> (f32, f32) {
    let w = lambert_w_generic(k, Complex32::new(z_re, z_im));
    (w.re, w.im)
}

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
    const PREC: f32 = 1e-30;

    let i_zero = U::zero();
    let i_one = U::one();
    let i_neg_one = -i_one;

    let d_zero = T::zero();
    let d_one = T::one();
    let d_neg_one = -d_one;
    let d_two = d_one + d_one;
    let d_e = T::from_f64(E).unwrap_or(T::from_f32(E as f32).unwrap());
    let d_pi = T::from_f64(PI).unwrap_or(T::from_f32(PI as f32).unwrap());

    let i = Complex::<T>::i();
    let z_zero = Complex::<T>::from(d_zero);
    let z_one = Complex::<T>::from(d_one);
    let z_neg_one = Complex::<T>::from(d_neg_one);
    let z_two = z_one + z_one;

    let z_e = Complex::<T>::from(d_e);
    let z_neg_inv_e = Complex::<T>::from(
        T::from_f64(NEG_INV_E).unwrap_or(T::from_f32(NEG_INFINITY as f32).unwrap()),
    );
    let z_half = z_one / z_two;

    let abs_one = z_one.abs();
    let abs_half = z_half.abs();

    // region: special cases

    if z == z_zero {
        if k == i_zero {
            return z_zero;
        } else {
            return Complex::<T>::new(T::neg_infinity(), d_zero);
        }
    }
    if z == z_neg_inv_e && (k == i_zero || k == i_one) {
        return Complex::<T>::new(d_neg_one, d_zero);
    }
    if z == z_e && k == i_zero {
        return z_one;
    }

    // endregion: special cases

    // region: determine initial search point

    let two_pi_k_i = z_two * d_pi * Complex::<T>::from(<T as From<U>>::from(k)) * i;
    let mut w = z.ln() + two_pi_k_i - (z.ln() + two_pi_k_i).ln();

    // Choose the initial point more carefully when we are close to the branch cut.
    if (z - z_neg_inv_e).abs() <= abs_one {
        let p = (d_two * (d_e * z + d_one)).sqrt();
        let p2 = z_neg_one + p - z_one / Complex::<T>::from(T::from_f32(3.0).unwrap()) * p * p;
        let p3 = Complex::<T>::from(T::from_f32(11.0).unwrap())
            / Complex::<T>::from(T::from_f32(72.0).unwrap())
            * p
            * p
            * p;
        if k == i_zero {
            w = p2 + p3;
        } else if (k == i_one && z.im < d_zero) || (k == i_neg_one && z.im > d_zero) {
            w = p2 - p3;
        }
    }

    if k == i_zero && (z - z_half).abs() <= abs_half {
        // Order (1,1) Padé approximant for the principal branch
        w = (T::from_f32(0.35173371).unwrap()
            * (T::from_f32(0.1237166).unwrap() + T::from_f32(7.061302897).unwrap() * z))
            / (d_two + T::from_f32(0.827184).unwrap() * (d_one + d_two * z));
    }

    if k == i_neg_one && (z - z_half).abs() <= abs_half {
        // Order (1,1) Padé approximant for the secondary branch
        w = -(((T::from_f32(2.2591588985).unwrap() + T::from_f32(4.22096).unwrap() * i)
            * ((T::from_f32(-14.073271).unwrap() - T::from_f32(33.767687754).unwrap() * i) * z
                - (T::from_f32(12.7127).unwrap() - T::from_f32(19.071643).unwrap() * i)
                    * (d_one + d_two * z)))
            / (d_two
                - (T::from_f32(17.23103).unwrap() - T::from_f32(10.629721).unwrap() * i)
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
        w -= z_two * ((wew - z) * wew_d) / (z_two * wew_d * wew_d - (wew - z) * wew_dd);

        iter += 1;

        if (w - w_prev).abs() <= Complex::<T>::from(T::from_f32(PREC).unwrap()).abs()
            || iter >= MAX_ITER
        {
            return w;
        }
    }

    // endregion: Halley iteration
}
