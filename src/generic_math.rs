//! This module contains elementary and rational functions used in the Lambert W function approximations.
//! They are generic over all types that implement the [`Float`] trait.

use num_traits::Float;

/// Evaluate a rational function at `x` using Horner's method.
///
/// The coefficients are organized by degree in ascending order.
// The inline(always) annotation is motivated by benchmarks, especially
// of the functions with 50 bits of accuracy.
#[inline(always)]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn rational_function<T: Float, const N: usize, const D: usize>(
    x: T,
    numerator: [T; N],
    denominator: [T; D],
) -> T {
    numerator
        .into_iter()
        .rev()
        .fold(T::zero(), |acc, n| acc * x + n)
        / denominator
            .into_iter()
            .rev()
            .fold(T::zero(), |acc, d| acc * x + d)
}

// The inline(always) annotation on the functions below is motivated by benchmarks.

/// Compute the square root of `x`.
#[inline(always)]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn sqrt<T: Float>(x: T) -> T {
    x.sqrt()
}

/// Compute the natural logarithm of `x`.
#[inline(always)]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn ln<T: Float>(x: T) -> T {
    x.ln()
}
