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
