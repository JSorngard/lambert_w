//! Calculate the Lambert W function to 50 bits of accuracy.

mod dw0c;
mod dwm1c;

use dw0c::dw0c;
use dwm1c::dwm1c;

use super::Z0;

/// Computes the principal branch of the Lambert W function, W_0(`z`) to 50 bits of accuracy by piecewise minimax rational function approximation and variable transformations.
///
/// Uses the [method of Toshio Fukushima](https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation).
///
/// Returns [`None`] if `z` is less than -1/e.
///
/// # Example
///
/// ```
/// use lambert_w::accurate::lambert_w_0;
/// use approx::assert_abs_diff_eq;
/// use core::f64::consts::PI;
///
/// let w = lambert_w_0(PI).unwrap();
///
/// assert_abs_diff_eq!(w, 1.0736581947961492);
/// ```
pub fn lambert_w_0(z: f64) -> Option<f64> {
    dw0c(z - Z0)
}

/// Computes the secondary branch of the Lambert W function, W_-1(`z`), to 50 bits of accuracy by piecewise minimax rational function approximation and variable transformations.
///
/// Uses the [method of Toshio Fukushima](https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation).
///
/// Returns [`None`] if `z` is positive or less than -1/e.
///
/// # Example
///
/// ```
/// use lambert_w::accurate::lambert_w_m1;
/// use approx::assert_abs_diff_eq;
/// use core::f64::consts::PI;
///
/// let w = lambert_w_m1(-1.0/PI).unwrap();
///
/// assert_abs_diff_eq!(w,  -1.6385284199703634);
/// ```
pub fn lambert_w_m1(z: f64) -> Option<f64> {
    dwm1c(z, z - Z0)
}
