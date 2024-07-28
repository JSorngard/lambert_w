mod dw0c;
mod dwm1c;
mod sw0;
mod swm1;

// -1/e
pub(crate) const Z0: f64 = -0.367_879_441_171_442_33;

// 1/sqrt(e)
pub(crate) const X0: f64 = 0.606_530_659_712_633_4;

pub use dw0c::dw0c;
pub use dwm1c::dwm1c;
pub use sw0::sw0;
pub use swm1::swm1;
