//! This module contains elementary math functions that exist in both the standard library and the [`libm`] crate.
//! Uses the standard library versions if the `std` feature is enabled, otherwise uses the `libm` versions if the
//! `std` feature is disabled and the `libm` feature is enabled. If both are disabled these functions panic.
//!
//! The panics in this module can never be triggered by using the crate since it's a compile error to not
//! have at least one of the `std` and `libm` features enabled.
//! The panic code will therefore never actually be inserted into any binary.

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn sqrt(x: f64) -> f64 {
    #[cfg(feature = "std")]
    {
        x.sqrt()
    }

    #[cfg(all(not(feature = "std"), feature = "libm"))]
    {
        libm::sqrt(x)
    }

    #[cfg(not(any(feature = "std", feature = "libm")))]
    {
        panic!("computing sqrt({x}) needs at least one of the `std` or `libm` feature flags to be enabled");
    }
}

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn sqrtf(x: f32) -> f32 {
    #[cfg(feature = "std")]
    {
        x.sqrt()
    }

    #[cfg(all(not(feature = "std"), feature = "libm"))]
    {
        libm::sqrtf(x)
    }

    #[cfg(not(any(feature = "std", feature = "libm")))]
    {
        panic!("computing sqrtf({x}) needs at least one of the `std` or `libm` feature flags to be enabled");
    }
}

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
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

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
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
