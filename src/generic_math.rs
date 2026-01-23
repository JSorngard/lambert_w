// Copyright 2024-2026 Johanna Sörngård
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This module contains elementary and rational functions used in the Lambert W function approximations.
//! They are generic over all types that implement the [`Float`] trait.

use num_traits::Float;

// The `inline` annotations are motivated by benchmarks on an AMD 5800X3D processor.
// The `lambert_w0` function is sped up by around 50% when the inline annotations are present.
// The reason it's not `inline(always)` is to let the compiler have more of a say on platforms where
// it might decide that inlining is not a good idea.

/// Evaluate a rational function at `x` using [Horner's method](https://en.wikipedia.org/wiki/Horner%27s_method).
///
/// The coefficients are assumed to be sorted in ascending order by the degree of the associated x-term.
#[inline]
pub(crate) fn rational_function<T: Float, const N: usize, const D: usize>(
    x: T,
    numerator_coefficients: [T; N],
    denominator_coefficients: [T; D],
) -> T {
    let numerator = polynomial(x, numerator_coefficients);

    let denominator = polynomial(x, denominator_coefficients);

    numerator / denominator
}

/// Evaluate a polynomial at `x` using [Horner's method](https://en.wikipedia.org/wiki/Horner%27s_method).
///
/// The coefficients are assumed to be sorted in ascending order by the degree of the associated x-term.
#[inline]
fn polynomial<T: Float, const N: usize>(x: T, coefficients: [T; N]) -> T {
    coefficients
        .into_iter()
        .rev()
        .fold(T::zero(), |acc, c| acc * x + c)
}

// The functions below are wrappers around the [`num-traits`] crate,
// mainly to ensure that we can always call them regardless of the presence of
// the standard library. I do not just import the trait in the files where the
// lambert w functions are defined because the standard library is available during testing,
// which means that the crate would produce warnings about the unused imports.

/// Compute the square root of `x`.
///
/// Just wraps the [`sqrt`](Float::sqrt) function from [`num_traits`].
#[inline]
pub(crate) fn sqrt<T: Float>(x: T) -> T {
    Float::sqrt(x)
}

/// Compute the natural logarithm of `x`.
///
/// Just wraps the [`ln`](Float::ln) function from [`num_traits`].
#[inline]
pub(crate) fn ln<T: Float>(x: T) -> T {
    Float::ln(x)
}
