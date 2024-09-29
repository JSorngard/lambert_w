#[cfg(feature = "50bits")]
use super::{lambert_w0, lambert_wm1};
#[cfg(feature = "24bits")]
use super::{lambert_w0f, lambert_wm1f};
use num_traits::Float;

/// Compute the principal branch of the Lambert W function.
pub trait LambertW0: Float {
    /// The principal branch of the Lambert W funciton.
    fn lambert_w0(self) -> Self;
}

/// Compute the secondary branch of the Lambert W function.
pub trait LambertWm1: Float {
    /// The secondary branch of the Lambert W funciton.
    fn lambert_wm1(self) -> Self;
}

#[cfg(feature = "24bits")]
impl LambertW0 for f32 {
    #[inline]
    fn lambert_w0(self) -> Self {
        lambert_w0f(self)
    }
}

#[cfg(feature = "24bits")]
impl LambertWm1 for f32 {
    #[inline]
    fn lambert_wm1(self) -> Self {
        lambert_wm1f(self)
    }
}

#[cfg(feature = "50bits")]
impl LambertW0 for f64 {
    #[inline]
    fn lambert_w0(self) -> Self {
        lambert_w0(self)
    }
}

#[cfg(feature = "50bits")]
impl LambertWm1 for f64 {
    #[inline]
    fn lambert_wm1(self) -> Self {
        lambert_wm1(self)
    }
}
