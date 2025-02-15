//! This module contains elementary and rational functions used in the Lambert W function approximations.
//! They are generic over all types that implement the [`Real`] trait.

use num_traits::real::Real;

/// Evaluate a rational function at `x` using Horner's method.
///
/// The coefficients are organized by degree in ascending order.
// The inline(always) annotation is motivated by benchmarks, especially
// of the functions with 50 bits of accuracy.
#[inline(always)]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn rational_function<T: Real, const N: usize, const D: usize>(
    x: T,
    numerator_coefficients: [T; N],
    denominator_coefficients: [T; D],
) -> T {
    let numerator = numerator_coefficients
        .into_iter()
        .rev()
        .fold(T::zero(), |acc, n| acc * x + n);

    let denominator = denominator_coefficients
        .into_iter()
        .rev()
        .fold(T::zero(), |acc, d| acc * x + d);

    numerator / denominator
}

// The inline(always) annotation on the functions below is motivated by benchmarks.

/// Compute the square root of `x`.
#[inline(always)]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn sqrt<T: Real>(x: T) -> T {
    x.sqrt()
}

/// Compute the natural logarithm of `x`.
#[inline(always)]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn ln<T: Real>(x: T) -> T {
    x.ln()
}
