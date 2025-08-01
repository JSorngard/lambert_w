// Copyright 2025 Johanna Sörngård
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This module contains elementary and rational functions used in the Lambert W function approximations.
//! They are generic over all types that implement the [`Float`] trait.

use num_traits::Float;

// The inline(always) annotations in this module could be removed.
// They are motivated by benchmarks, especially of the 50-bit functions.
// However, I have only benchmarked the functions on my own system with a CPU with large cache
// and I am not sure if the inlining is beneficial on all systems, and for all users.

/// Evaluate a rational function at `x` using Horner's method.
///
/// The coefficients are assumed to be sorted in ascending order by degree.
#[inline(always)]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn rational_function<T: Float, const N: usize, const D: usize>(
    x: T,
    numerator_coefficients: [T; N],
    denominator_coefficients: [T; D],
) -> T {
    let numerator = polynomial(x, numerator_coefficients);

    let denominator = polynomial(x, denominator_coefficients);

    numerator / denominator
}

/// Evaluate a polynomial at `x` using Horner's method.
///
/// The coefficients are assumed to be sorted in ascending order by degree.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
fn polynomial<T: Float, const N: usize>(x: T, coefficients: [T; N]) -> T {
    coefficients
        .into_iter()
        .rev()
        .fold(T::zero(), |acc, c| mul_add(acc, x, c))
}

#[cfg(not(target_feature = "fma"))]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
fn mul_add<T: Float>(acc, x, c) -> T {
    acc * x + c
}

#[cfg(target_feature = "fma")]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
fn mul_add<T: Float>(acc, x, c) -> T {
    acc.mul_add(x, c)
}

// The functions below are wrappers around the [`num-traits`] crate,
// mainly to ensure that we can always call them regardless of the presence of
// the standard library. I do not just import the trait in the files where the
// lambert w functions are defined because the standard library is available during testing,
// which means that the crate would produce warnings about the unused imports.

/// Compute the square root of `x`.
#[inline(always)]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn sqrt<T: Float>(x: T) -> T {
    Float::sqrt(x)
}

/// Compute the natural logarithm of `x`.
#[inline(always)]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn ln<T: Float>(x: T) -> T {
    Float::ln(x)
}
