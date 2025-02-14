//! This module contains elementary and rational functions used in the Lambert W function approximations.
//! They are generic over all types that implement the [`Float`] trait.

use num_traits::Float;

/// Evaluate a rational function at `x` using Horner's method.
///
/// The coefficients are organized by degree in ascending order.
#[inline(always)]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn rational_function<T, const N: usize, const D: usize>(
    x: T,
    numerator: [T; N],
    denominator: [T; D],
) -> T
where
    T: Float,
{
    numerator
        .into_iter()
        .rev()
        .fold(T::zero(), |acc, n| acc * x + n)
        / denominator
            .into_iter()
            .rev()
            .fold(T::zero(), |acc, d| acc * x + d)
}

/// Compute the square root of `x`.
#[inline(always)]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn sqrt<F: Float>(x: F) -> F {
    x.sqrt()
}

/// Compute the natural logarithm of `x`.
#[inline(always)]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn ln<F: Float>(x: F) -> F {
    x.ln()
}
