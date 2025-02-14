//! Rational functions evaluated with Horner's method.

use core::ops::{Add, Div, Mul};

pub trait Zero {
    fn zero() -> Self;
}

impl Zero for f32 {
    #[inline(always)]
    fn zero() -> Self {
        0.0
    }
}

impl Zero for f64 {
    #[inline(always)]
    fn zero() -> Self {
        0.0
    }
}

/// Evaluate a rational function at `x` using Horner's method.
#[inline(always)]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn rational_function<T, const N: usize, const D: usize>(
    x: T,
    numerator: [T; N],
    denominator: [T; D],
) -> T
where
    T: Add<Output = T> + Mul<Output = T> + Div<Output = T> + Zero + Copy,
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
