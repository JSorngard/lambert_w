//! This module contains elementary math functions that exist in both the standard library and the [`libm`] crate.
//! Uses the standard library versions if the `std` feature is enabled, otherwise uses the `libm` versions if the
//! `std` feature is disabled and the `libm` feature is enabled. If both are disabled these functions panic.
//!
//! The panics in this module can never be triggered by using the crate since it's a compile error to not
//! have at least one of the `std` and `libm` features enabled.
//! The panic code will therefore never actually be inserted into any binary.

// #[inline(always)] is motivated by the fact that these functions are trivial, so just placing the call to the
// correct sqrt/ln at the caller does not add extra code, but skips an extra indirection.

use num_traits::Float;

#[inline(always)]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn sqrt<F: Float>(x: F) -> F {
    x.sqrt()
}

#[inline(always)]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn ln<F: Float>(x: F) -> F {
    x.ln()
}
