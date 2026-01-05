// Copyright 2026 Johanna Sörngård
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This module contains the general implementation of the Lambert W function.
//! This implementation is capable of computing the function at any point in the complex plane on any branch.

use num_complex::{Complex, ComplexFloat};
use num_traits::{Float, Signed};

use crate::NEG_INV_E;
use core::{
    f64::consts::{E, PI},
    ops::{Add, Mul, Sub, SubAssign},
};

const MAX_ITERS: u8 = 255;

/// Branch `k` of the complex valued Lambert W function computed
/// on 64-bit floats with Halley's method.
///
/// The return value is a tuple where the first element is the
/// real part and the second element is the imaginary part.
///
/// The function iterates until it either reaches the requested error tolerance (relative difference between two iterations),
/// or has iterated a maximum number of times.
///
/// This function may be slightly less accurate close to the branch cut at -1/e,
/// as well as close to zero on branches other than k=0.
///
/// If you know you want the principal or secondary branches where they are real-valued,
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
/// let w = lambert_w(2, 1.0, 2.0, f64::EPSILON);
///
/// assert_eq!(w, (-1.6869138779375397, 11.962631435322813));
/// ```
///
/// Returns [`NAN`](f64::NAN)s if any of the inputs are infinite:
///
/// ```
/// # use lambert_w::lambert_w;
/// let w = lambert_w(-13, f64::INFINITY, 0.0, f64::EPSILON);
///
/// assert!(w.0.is_nan() && w.1.is_nan());
/// ```
///
/// or `NAN`:
///
/// ```
/// # use lambert_w::lambert_w;
/// let w = lambert_w(1_000, 0.0, f64::NAN, f64::EPSILON);
///
/// assert!(w.0.is_nan() && w.1.is_nan());
/// ```
#[must_use = "this is a pure function that only returns a value and has no side effects"]
pub fn lambert_w(k: i32, z_re: f64, z_im: f64, error_tolerance: f64) -> (f64, f64) {
    let w = lambert_w_generic(k, num_complex::Complex64::new(z_re, z_im), error_tolerance);
    (w.re, w.im)
}

/// Branch `k` of the complex valued Lambert W function computed
/// on 32-bit floats with Halley's method.
///
/// The return value is a tuple where the first element is the
/// real part and the second element is the imaginary part.
///
/// The function iterates until it either reaches the requested error tolerance (relative difference between two iterations),
/// or has iterated a maximum number of times.
///
/// This function may be slightly less accurate close to the branch cut at -1/e,
/// as well as close to zero on branches other than k=0.
///
/// If you know you want the principal or secondary branches where they are real-valued,
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
/// let w = lambert_wf(2, 1.0, 2.0, f32::EPSILON);
///
/// assert_eq!(w, (-1.6869138, 11.962631));
/// ```
///
/// Returns [`NAN`](f32::NAN)s if any of the inputs are infinite:
///
/// ```
/// # use lambert_w::lambert_wf;
/// let w = lambert_wf(-13, f32::INFINITY, 0.0, f32::EPSILON);
///
/// assert!(w.0.is_nan() && w.1.is_nan());
/// ```
///
/// or `NAN`:
///
/// ```
/// # use lambert_w::lambert_wf;
/// let w = lambert_wf(1_000, 0.0, f32::NAN, f32::EPSILON);
///
/// assert!(w.0.is_nan() && w.1.is_nan());
/// ```
#[must_use = "this is a pure function that only returns a value and has no side effects"]
pub fn lambert_wf(k: i16, z_re: f32, z_im: f32, error_tolerance: f32) -> (f32, f32) {
    let w = lambert_w_generic(k, num_complex::Complex32::new(z_re, z_im), error_tolerance);
    (w.re, w.im)
}

/// This is a generic implementation of the Lambert W function.
/// It is capable of computing the function at any point in the complex plane on any branch.
///
/// It performs a maximum of 255 iterations of Halley's method, and looks for a relative error
/// of less than floating point epsilon.
fn lambert_w_generic<T, U>(k: U, z: Complex<T>, error_tolerance: T) -> Complex<T>
where
    U: Signed + Copy,
    T: Float
        + F64AsT
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
    // Early return if we know we can not compute an answer.
    if !z.is_finite() {
        return Complex::<T>::new(T::nan(), T::nan());
    }

    if error_tolerance.is_nan() {
        return Complex::<T>::new(T::nan(), T::nan());
    }

    // region: construct constants of the generic type

    let i_zero = U::zero();
    let i_one = U::one();

    let d_zero = T::zero();
    let d_one = T::one();
    let d_two = d_one + d_one;
    let d_e: T = T::f64_as_t(E);
    let d_neg_inv_e: T = T::f64_as_t(NEG_INV_E);

    let z_zero = Complex::<T>::from(d_zero);
    let z_one = Complex::<T>::from(d_one);

    // This value is only constructed to help the compiler see that
    // it is the same type as what Complex<T>::abs() returns.
    let z_tolerance_abs = Complex::<T>::from(error_tolerance).abs();

    // endregion: construct constants of the generic type

    // region: special cases

    if z == z_zero {
        return if k == i_zero {
            z_zero
        } else {
            T::neg_infinity().into()
        };
    }
    if z == d_neg_inv_e.into() && (k == i_zero || k == -i_one) {
        return -z_one;
    }
    if z == d_e.into() && k == i_zero {
        return z_one;
    }

    // endregion: special cases

    let mut w = determine_start_point(k, z);

    // region: Halley iteration

    let mut w_prev_prev = None;
    let mut iter = 0;
    loop {
        let w_prev = w;
        let ew = w.exp();
        // Simplified form of 2*((w*e^w - z)*(e^w + w*e^w))/(2*(e^w + w*e^w)^2 - (w*e^w - z)*(2e^w + w*e^w)).
        w -= d_two * (w + d_one) * (w * ew - z)
            / (ew * (w * w + d_two * w + d_two) + (w + d_two) * z);

        iter += 1;

        if Some(w) == w_prev_prev {
            // If we are stuck in a loop of two values we return the previous one,
            // since the current one is a step back.
            return w_prev;
        }

        if ((w - w_prev) / w).abs() <= z_tolerance_abs || iter == MAX_ITERS || !w.is_finite() {
            return w;
        }

        w_prev_prev = Some(w);
    }

    // endregion: Halley iteration
}

/// Carefully determines the initial search point for Halley's method.
fn determine_start_point<T, U>(k: U, z: Complex<T>) -> Complex<T>
where
    U: Signed + Copy,
    T: Float
        + F64AsT
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
    let i_zero = U::zero();
    let i_one = U::one();

    let d_zero = T::zero();
    let d_one = T::one();
    let d_two = d_one + d_one;
    let d_half = d_one / d_two;
    let d_e: T = T::f64_as_t(E);
    let d_pi: T = T::f64_as_t(PI);
    let d_neg_inv_e: T = T::f64_as_t(NEG_INV_E);

    let i = Complex::<T>::i();
    let z_one = Complex::<T>::from(d_one);
    let z_two = z_one + z_one;

    let z_neg_inv_e = Complex::<T>::from(d_neg_inv_e);
    let z_half = z_one / z_two;

    // These values are only constructed to help the compiler see that
    // they are the same type as what Complex<T>::abs() returns.
    let abs_one = z_one.abs();
    let abs_half = z_half.abs();

    let two_pi_k_i = d_two * d_pi * <T as From<U>>::from(k) * i;
    let mut initial_point = z.ln() + two_pi_k_i - (z.ln() + two_pi_k_i).ln();

    // Choose the initial point more carefully when we are close to the branch cut.
    if (z - z_neg_inv_e).abs() <= abs_one {
        let p = (d_two * (d_e * z + d_one)).sqrt();
        let p2 = T::f64_as_t(1.0 / 3.0) * p * p;
        let p3 = T::f64_as_t(11.0 / 72.0) * p * p * p;
        if k == i_zero {
            initial_point = -d_one + p - p2 + p3;
        } else if (k == i_one && z.im < d_zero) || (k == -i_one && z.im > d_zero) {
            initial_point = -d_one - p - p2 - p3;
        }
    }

    if k == i_zero && (z - d_half).abs() <= abs_half {
        // Order (1,1) Padé approximant for the principal branch
        initial_point = (T::f64_as_t(0.351_733_71)
            * (T::f64_as_t(0.123_716_6) + T::f64_as_t(7.061_302_897) * z))
            / (d_two + T::f64_as_t(0.827_184) * (d_one + d_two * z));
    }

    if k == -i_one && (z - d_half).abs() <= abs_half {
        // Order (1,1) Padé approximant for the secondary branch
        initial_point = -(((T::f64_as_t(2.259_158_898_5) + T::f64_as_t(4.220_96) * i)
            * ((T::f64_as_t(-14.073_271) - T::f64_as_t(33.767_687_754) * i) * z
                - (T::f64_as_t(12.712_7) - T::f64_as_t(19.071_643) * i) * (d_one + d_two * z)))
            / (d_two
                - (T::f64_as_t(17.231_03) - T::f64_as_t(10.629_721) * i) * (d_one + d_two * z)));
    }

    initial_point
}

/// A type that can be converted lossily from an [`f64`].
/// This works like an `as`-cast conversion:
/// an effort is made to represent the `f64`'s value
/// in the new type, but it is allowed to be lossy,
/// like when converting a `f64` to a [`f32`].
trait F64AsT {
    fn f64_as_t(x: f64) -> Self;
}

impl F64AsT for f32 {
    #[inline]
    fn f64_as_t(x: f64) -> f32 {
        x as f32
    }
}

impl F64AsT for f64 {
    #[inline]
    fn f64_as_t(x: f64) -> f64 {
        x
    }
}
