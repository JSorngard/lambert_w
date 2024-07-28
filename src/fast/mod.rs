//! Calculate the Lambert W function to 24 bits of precision.

mod sw0;
mod swm1;

use sw0::sw0;
use swm1::swm1;

use crate::{LambertW0Error, LambertWm1Error};

/// The principal branch of the Lambert W function, W_0(`z`), computed to 24 bits of accuracy.
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
/// use lambert_w::fast::lambert_w0;
/// use approx::assert_abs_diff_eq;
/// use core::f64::consts::PI;
///
/// let w = lambert_w0(PI)?;
///
/// assert_abs_diff_eq!(w, 1.0736581947961492, epsilon = 1e-7);
/// # Ok::<(), LambertW0Error>(())
/// ```
pub fn lambert_w0(z: f64) -> Result<f64, LambertW0Error> {
    sw0(z)
}

/// The secondary branch of the Lambert W function, W_-1(`z`), computed to 24 bits of accuracy.
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
/// use lambert_w::fast::lambert_wm1;
/// use approx::assert_abs_diff_eq;
/// use core::f64::consts::PI;
///
/// let w = lambert_wm1(-1.0/PI)?;
///
/// assert_abs_diff_eq!(w, -1.6385284199703634, epsilon = 1e-7);
/// # Ok::<(), LambertWm1Error>(())
/// ```
pub fn lambert_wm1(z: f64) -> Result<f64, LambertWm1Error> {
    swm1(z)
}
