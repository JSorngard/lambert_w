//! This module contains elementary and rational functions used in the Lambert W function approximations.
//! They are generic over all types that implement the [`Real`] trait.

use num_traits::real::Real;

// The inline(always) annotations in this module could be removed.
// I have only benchmarked the functions on my own system with a CPU with large cache
// and I am not sure if the inlining is beneficial on all systems, and for all users.

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

// The functions below are wrappers around the [`num-traits`] crate,
// mainly to ensure that we can always call them regardless of the presence of
// the standard library. I do not just import the trait in the files where the
// lambert w functions are defined because the standard library is available during testing,
// which means that the crate would produce warnings about the unused imports.

// The inline(always) annotation on the functions below is motivated by benchmarks.

/// Compute the square root of `x`.
#[inline(always)]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn sqrt<T: Real>(x: T) -> T {
    Real::sqrt(x)
}

/// Compute the natural logarithm of `x`.
#[inline(always)]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn ln<T: Real>(x: T) -> T {
    Real::ln(x)
}
