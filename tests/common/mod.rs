// Copyright 2024-2026 Johanna Sörngård
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Some common functionality and definitions for the two different integration test crates.

#[macro_export]
/// Delegates to the [`assert_ulps_eq!`] macro while setting `epsilon` to 0.0.
macro_rules! assert_only_ulps_eq {
    ($left:expr, $right:expr $(,)?) => {
        assert_ulps_eq!($left, $right, epsilon = 0.0);
    };
    ($left:expr, $right:expr, max_ulps = $max_ulps:expr $(,)?) => {
        assert_ulps_eq!($left, $right, epsilon = 0.0, max_ulps = $max_ulps);
    };
}

/// The number of bits that are accurate in the 32-bit functions.
/// Raising this any higher causes some tests to fail.
///
/// The tests of the 32-bit functions near the branch point use their own constant.
pub const WF_ACCURACY_BITS: u32 = 20;
/// The number of ulps that the functions on 32-bit floats should get from the true answer.
pub const WF_MAX_ULPS: u32 = u32::pow(2, f32::MANTISSA_DIGITS - WF_ACCURACY_BITS);

/// The number of bits that are accurate in the high precision 64-bit functions.
/// Raising this any higher causes some tests to fail.
pub const W_ACCURACY_BITS: u32 = 50;
/// The number of ulps that the 50-bit functions should get from the true answer.
pub const W_MAX_ULPS: u32 = u32::pow(2, f64::MANTISSA_DIGITS - W_ACCURACY_BITS);

/// The number of bits that are accurate in the low precision 64-bit functions.
/// Raising this any higher causes some tests to fail.
pub const SP_W_ACCURACY_BITS: u32 = 24;
/// The number of ulps that the 24-bit functions should get from the true answer.
pub const SP_W_MAX_ULPS: u32 = u32::pow(2, f64::MANTISSA_DIGITS - SP_W_ACCURACY_BITS);

/// Returns the distance between `got` and `exact` in ulps.
/// Sometimes useful when debugging the tests.
#[allow(dead_code)]
pub const fn ulp_err64(got: f64, exact: f64) -> f64 {
    if got == exact {
        return 0.0;
    }
    let ulp = {
        let b = exact.abs().to_bits();
        f64::from_bits(b + 1) - exact.abs()
    };
    ((got - exact) / ulp).abs()
}

/// Same as [`ulp_err64`] but for 32-bit floats.
#[allow(dead_code)]
pub const fn ulp_err32(got: f32, exact: f32) -> f32 {
    if got == exact {
        return 0.0;
    }
    let ulp = {
        let b = exact.abs().to_bits();
        f32::from_bits(b + 1) - exact.abs()
    };
    ((got - exact) / ulp).abs()
}
