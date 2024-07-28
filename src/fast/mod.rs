//! Calculate the Lambert W function to 24 bits of accuracy.

mod sw0;
mod swm1;

use sw0::sw0;
use swm1::swm1;

use crate::{LambertW0Error, LambertWm1Error};

/// Computes the principal branch of the Lambert W function, W_0(`z`), to 24 bits of accuracy by piecewise minimax rational function approximation and variable transformations.
///
/// Uses the [method of Toshio Fukushima](https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation).
///
/// This version is on average 15% faster than the 50 bit accurate version, [`accurate::lambert_w_0`](crate::accurate::lambert_w_0).
///
/// # Errors
///
/// Returns an error if `z` is less than -1/e.
///
/// # Example
///
/// ```
/// # use lambert_w::LambertW0Error;
/// use lambert_w::fast::lambert_w_0;
/// use approx::assert_abs_diff_eq;
/// use core::f64::consts::PI;
///
/// let w = lambert_w_0(PI)?;
///
/// assert_abs_diff_eq!(w, 1.0736581947961492, epsilon = 1e-7);
/// # Ok::<(), LambertW0Error>(())
/// ```
pub fn lambert_w_0(z: f64) -> Result<f64, LambertW0Error> {
    sw0(z)
}

/// Computes the secondary branch of the Lambert W function, W_-1(`z`), to 24 bits of accuracy by piecewise minimax rational function approximation and variable transformations.
///
/// Uses the [method of Toshio Fukushima](https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation).
///
/// This version is on average 41% faster than the 50 bit accurate version, [`accurate::lambert_w_m1`](crate::accurate::lambert_w_m1).
///
/// # Errors
///
/// Returns an error if `z` is positive or less than -1/e.
///
/// # Example
///
/// ```
/// # use lambert_w::LambertWm1Error;
/// use lambert_w::fast::lambert_w_m1;
/// use approx::assert_abs_diff_eq;
/// use core::f64::consts::PI;
///
/// let w = lambert_w_m1(-1.0/PI)?;
///
/// assert_abs_diff_eq!(w, -1.6385284199703634, epsilon = 1e-7);
/// # Ok::<(), LambertWm1Error>(())
/// ```
pub fn lambert_w_m1(z: f64) -> Result<f64, LambertWm1Error> {
    swm1(z)
}
