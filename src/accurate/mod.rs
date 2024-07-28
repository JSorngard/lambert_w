//! Calculate the Lambert W function to 50 bits of accuracy.

mod dw0c;
mod dwm1c;

use dw0c::dw0c;
use dwm1c::dwm1c;

use super::Z0;
use crate::error::{LambertW0Error, LambertWm1Error};

/// Computes the principal branch of the Lambert W function, W_0(`z`) to 50 bits of accuracy by piecewise minimax rational function approximation.
///
/// Uses the [method of Toshio Fukushima](https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation).
///
/// The 24-bit version, [`fast::lambert_w_0`](crate::fast::lambert_w_0), is on average 15% faster than this one.
///
/// # Errors
///
/// Returns an error if `z` is less than -1/e.
///
/// # Example
///
/// ```
/// # use lambert_w::LambertW0Error;
/// use lambert_w::accurate::lambert_w_0;
/// use approx::assert_abs_diff_eq;
/// use core::f64::consts::PI;
///
/// let w = lambert_w_0(PI)?;
///
/// assert_abs_diff_eq!(w, 1.0736581947961492);
/// # Ok::<(), LambertW0Error >(())
/// ```
pub fn lambert_w_0(z: f64) -> Result<f64, LambertW0Error> {
    dw0c(z - Z0)
}

/// Computes the secondary branch of the Lambert W function, W_-1(`z`), to 50 bits of accuracy by piecewise minimax rational function approximation.
///
/// Uses the [method of Toshio Fukushima](https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation).
///
/// The 24-bit version, [`fast::lambert_w_m1`](crate::fast::lambert_w_m1), is on everage 41% faster than this one.
///
/// # Errors
///
/// Returns an error if `z` is positive or less than -1/e.
///
/// # Example
///
/// ```
/// # use lambert_w::LambertWm1Error;
/// use lambert_w::accurate::lambert_w_m1;
/// use approx::assert_abs_diff_eq;
/// use core::f64::consts::PI;
///
/// let w = lambert_w_m1(-1.0/PI)?;
///
/// assert_abs_diff_eq!(w,  -1.6385284199703634);
/// # Ok::<(), LambertWm1Error>(())
/// ```
pub fn lambert_w_m1(z: f64) -> Result<f64, LambertWm1Error> {
    dwm1c(z, z - Z0)
}
