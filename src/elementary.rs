//! This module contains elementary math functions that exist in both the standard library and the [`libm`] crate.
//! Uses the standard library versions if the `std` feature is enabled, and uses the `libm` versions if the
//! `std` feature is disabled and the `libm` feature is enabled. If both are disabled these functions panic.

pub(crate) fn sqrt(x: f64) -> f64 {
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

pub(crate) fn ln(x: f64) -> f64 {
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
