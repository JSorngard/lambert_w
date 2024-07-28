mod dw0c;
mod dwm1c;

use dw0c::dw0c;
use dwm1c::dwm1c;

use super::Z0;
use crate::error::{LambertW0Error, LambertWm1Error};

/// The principal branch of the Lambert W function, W_0(`z`), computed to 50 bits of accuracy.
///
/// Uses the [method of Toshio Fukushima](https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation).
///
/// # Errors
///
/// Returns an error if `z` is less than -1/e.
///
/// # Example
///
/// ```
/// # use lambert_w::LambertW0Error;
/// use lambert_w::accurate::lambert_w0;
/// use approx::assert_abs_diff_eq;
/// use core::f64::consts::PI;
///
/// let w = lambert_w0(PI)?;
///
/// assert_abs_diff_eq!(w, 1.0736581947961492);
/// # Ok::<(), LambertW0Error >(())
/// ```
pub fn lambert_w0(z: f64) -> Result<f64, LambertW0Error> {
    dw0c(z - Z0)
}

/// The secondary branch of the Lambert W function, W_-1(`z`), computed to 50 bits of accuracy.
///
/// Uses the [method of Toshio Fukushima](https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation).
///
/// # Errors
///
/// Returns an error if `z` is positive or less than -1/e.
///
/// # Example
///
/// ```
/// # use lambert_w::LambertWm1Error;
/// use lambert_w::accurate::lambert_wm1;
/// use approx::assert_abs_diff_eq;
/// use core::f64::consts::PI;
///
/// let w = lambert_wm1(-1.0/PI)?;
///
/// assert_abs_diff_eq!(w,  -1.6385284199703634);
/// # Ok::<(), LambertWm1Error>(())
/// ```
pub fn lambert_wm1(z: f64) -> Result<f64, LambertWm1Error> {
    dwm1c(z, z - Z0)
}
