// Copyright 2025 Johanna Sörngård
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This module contains the general implementation of the Lambert W function.
//! This implementation is capable of computing the function at any point in the complex plane on any branch.

use num_complex::{Complex, ComplexFloat};
use num_traits::{Float, FromPrimitive, Signed};

use core::{
    f64::consts::{E, PI},
    ops::{Add, Mul, Sub, SubAssign},
};

use crate::NEG_INV_E;

const MAX_ITER: u8 = u8::MAX;

// Remember to change the docstring of `lambert_w_generic` if you change the above value.

/// This is a generic implementation of the Lambert W function.
/// It is capable of computing the function at any point in the complex plane on any branch.
///
/// It performs a maximum of 255 iterations of Halley's method, and looks for a relative error
/// of less than floating point epsilon.
///
/// # Panics
///
/// Panics if `T` can not be losslessly created from either an `f64` or an `f32`.
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
    // Early return if we know we can not compute an answer.
    if z.is_nan() || z.is_infinite() {
        return Complex::<T>::new(T::nan(), T::nan());
    }

    // region: construct constants of the generic type

    let i_zero = U::zero();
    let i_one = U::one();

    let d_zero = T::zero();
    let d_one = T::one();
    let d_two = d_one + d_one;
    let d_e: T = t_from_f64_or_f32(E);
    let d_neg_inv_e: T = t_from_f64_or_f32(NEG_INV_E);

    let z_zero = Complex::<T>::from(d_zero);
    let z_one = Complex::<T>::from(d_one);

    // This value is only constructed to help the compliler see that
    // it is the same type as what Complex<T>::abs() returns.
    let epsilon = Complex::<T>::from(T::epsilon()).abs();

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

        if ((w - w_prev) / w).abs() <= epsilon || iter == MAX_ITER {
            return w;
        }

        w_prev_prev = Some(w);
    }

    // endregion: Halley iteration
}

/// Carefully determines the initial search point for Halley's method.
///
/// # Panics
///
/// Panics if `T` can not be losslessly created from either an `f64` or an `f32`.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
fn determine_start_point<T, U>(k: U, z: Complex<T>) -> Complex<T>
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
    let z_one = Complex::<T>::from(d_one);
    let z_two = z_one + z_one;

    let z_neg_inv_e = Complex::<T>::from(d_neg_inv_e);
    let z_half = z_one / z_two;

    // These values are only constructed to help the compliler see that
    // they are the same type as what Complex<T>::abs() returns.
    let abs_one = z_one.abs();
    let abs_half = z_half.abs();

    let two_pi_k_i = d_two * d_pi * <T as From<U>>::from(k) * i;
    let mut initial_point = z.ln() + two_pi_k_i - (z.ln() + two_pi_k_i).ln();

    // Choose the initial point more carefully when we are close to the branch cut.
    if (z - z_neg_inv_e).abs() <= abs_one {
        let p = (d_two * (d_e * z + d_one)).sqrt();
        let p2 = t_from_f64_or_f32::<T>(1.0 / 3.0) * p * p;
        let p3 = t_from_f64_or_f32::<T>(11.0 / 72.0) * p * p * p;
        if k == i_zero {
            initial_point = -d_one + p - p2 + p3;
        } else if (k == i_one && z.im < d_zero) || (k == -i_one && z.im > d_zero) {
            initial_point = -d_one - p - p2 - p3;
        }
    }

    if k == i_zero && (z - d_half).abs() <= abs_half {
        // Order (1,1) Padé approximant for the principal branch
        initial_point = (t_from_f64_or_f32::<T>(0.351_733_71)
            * (t_from_f64_or_f32::<T>(0.123_716_6) + t_from_f64_or_f32::<T>(7.061_302_897) * z))
            / (d_two + t_from_f64_or_f32::<T>(0.827_184) * (d_one + d_two * z));
    }

    if k == -i_one && (z - d_half).abs() <= abs_half {
        // Order (1,1) Padé approximant for the secondary branch
        initial_point = -(((t_from_f64_or_f32::<T>(2.259_158_898_5)
            + t_from_f64_or_f32::<T>(4.220_96) * i)
            * ((t_from_f64_or_f32::<T>(-14.073_271)
                - t_from_f64_or_f32::<T>(33.767_687_754) * i)
                * z
                - (t_from_f64_or_f32::<T>(12.712_7) - t_from_f64_or_f32::<T>(19.071_643) * i)
                    * (d_one + d_two * z)))
            / (d_two
                - (t_from_f64_or_f32::<T>(17.231_03) - t_from_f64_or_f32::<T>(10.629_721) * i)
                    * (d_one + d_two * z)));
    }

    initial_point
}

/// Attempts to convert a `f64` to a `T`.
///
/// If that fails, it tries to convert the `f64` to a `f32` with `as` and then to a `T`.
///
/// # Panics
///
/// Panics if a `T` cannot be created from a `f32`.
fn t_from_f64_or_f32<T>(x: f64) -> T
where
    T: FromPrimitive,
{
    T::from_f64(x).unwrap_or_else(|| T::from_f32(x as f32).unwrap())
}
