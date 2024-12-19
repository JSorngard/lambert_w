//! This module contains elementary math functions that exist in both the standard library and the [`libm`] crate.
//! Uses the standard library versions if the `std` feature is enabled, otherwise uses the `libm` versions if the
//! `std` feature is disabled and the `libm` feature is enabled. If both are disabled these functions panic.

// #[inline(always)] is motivated by the fact that these functions are trivial, so just placing the call to the
// correct sqrt/ln at the caller does not add extra code, but skips an extra indirection.

#[inline(always)]
pub fn sqrt(x: f64) -> f64 {
    #[cfg(feature = "std")]
    {
        x.sqrt()
    }

    #[cfg(all(not(feature = "std"), feature = "libm"))]
    {
        libm::sqrt(x)
    }

    #[cfg(all(not(feature = "std"), not(feature = "libm")))]
    {
        panic!("computing sqrt({x}) needs at least one of the `std` or `libm` feature flags to be enabled");
    }
}

#[inline(always)]
pub fn sqrtf(x: f32) -> f32 {
    #[cfg(feature = "std")]
    {
        x.sqrt()
    }

    #[cfg(all(not(feature = "std"), feature = "libm"))]
    {
        libm::sqrtf(x)
    }

    #[cfg(all(not(feature = "std"), not(feature = "libm")))]
    {
        panic!("computing sqrtf({x}) needs at least one of the `std` or `libm` feature flags to be enabled");
    }
}

#[inline(always)]
pub fn ln(x: f64) -> f64 {
    #[cfg(feature = "std")]
    {
        x.ln()
    }

    #[cfg(all(not(feature = "std"), feature = "libm"))]
    {
        libm::log(x)
    }

    #[cfg(all(not(feature = "std"), not(feature = "libm")))]
    {
        panic!("computing ln({x}) needs at least one of the `std` or `libm` feature flags to be enabled");
    }
}

#[inline(always)]
pub fn lnf(x: f32) -> f32 {
    #[cfg(feature = "std")]
    {
        x.ln()
    }

    #[cfg(all(not(feature = "std"), feature = "libm"))]
    {
        libm::logf(x)
    }

    #[cfg(all(not(feature = "std"), not(feature = "libm")))]
    {
        panic!("computing lnf({x}) needs at least one of the `std` or `libm` feature flags to be enabled");
    }
}

#[cfg(test)]
mod test {
    use approx::{assert_abs_diff_eq, assert_relative_eq};

    use super::*;

    #[test]
    fn sanity_check_log() {
        assert!(ln(-1.0).is_nan());
        assert_abs_diff_eq!(ln(1.0), 0.0);
        assert_abs_diff_eq!(ln(2.0), core::f64::consts::LN_2);
        assert_abs_diff_eq!(ln(f64::MAX), 709.782712893384);
    }

    #[test]
    fn sanity_check_logf() {
        assert!(lnf(-1.0).is_nan());
        assert_abs_diff_eq!(lnf(1.0), 0.0);
        assert_abs_diff_eq!(lnf(2.0), core::f32::consts::LN_2);
        assert_abs_diff_eq!(lnf(f32::MAX), 88.722839);
    }

    #[test]
    fn sanity_check_sqrt() {
        assert!(sqrt(-1.0).is_nan());
        assert_abs_diff_eq!(sqrt(0.0), 0.0);
        assert_abs_diff_eq!(sqrt(1.0), 1.0);
        assert_abs_diff_eq!(sqrt(2.0), 1.4142135623730951);
        assert_abs_diff_eq!(sqrt(4.0), 2.0);
        assert_abs_diff_eq!(sqrt(f64::MAX), 1.3407807929942596e154);
    }

    #[test]
    fn sanity_check_sqrtf() {
        assert!(sqrtf(-1.0).is_nan());
        assert_abs_diff_eq!(sqrtf(0.0), 0.0);
        assert_abs_diff_eq!(sqrtf(1.0), 1.0);
        assert_abs_diff_eq!(sqrtf(2.0), 1.4142135);
        assert_abs_diff_eq!(sqrtf(4.0), 2.0);
        assert_relative_eq!(sqrtf(f32::MAX), 1.8446744e19);
    }
}
