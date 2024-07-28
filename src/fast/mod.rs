//! Calculate the Lambert W function to 24 bits of accuracy.

mod sw0;
mod swm1;

use sw0::sw0;
use swm1::swm1;

/// Computes the principal branch of the Lambert W function, W_0(`z`), to 24 bits of accuracy, if `z` > -1/e.
///
/// Uses the piecewise minimax rational function approximation method of Toshio Fukushima.
///
/// # Examples
///
/// Basic usage:
/// ```
/// use lambert_w::fast::lambert_w_0;
///
/// use approx::assert_abs_diff_eq;
/// use core::f64::consts::PI;
///
/// let w = lambert_w_0(PI).unwrap();
///
/// assert_abs_diff_eq!(w, 1.0736581947961492, epsilon = 1e-7);
/// ```
/// Too small arguments result in `None`:
/// ```
/// # use lambert_w::fast::lambert_w_0;
/// assert_eq!(lambert_w_0(-1.0), None);
/// ```
pub fn lambert_w_0(z: f64) -> Option<f64> {
    sw0(z)
}

/// Computes the secondary branch of the Lambert W function, W_-1(`z`), to 24 bits of accuracy, if -1/e < `z` <= 0.
///
/// Uses the piecewise minimax rational function approximation method of Toshio Fukushima.
///
/// # Examples
///
/// Basic usage:
/// ```
/// use lambert_w::fast::lambert_w_m1;
///
/// use approx::assert_abs_diff_eq;
/// use core::f64::consts::PI;
///
/// let w = lambert_w_m1(-1.0/PI).unwrap();
///
/// assert_abs_diff_eq!(w, -1.6385284199703634, epsilon = 1e-7);
/// ```
/// Too small or positive arguments result in `None`:
/// ```
/// # use lambert_w::fast::lambert_w_m1;
/// assert_eq!(lambert_w_m1(-1.0), None);
/// assert_eq!(lambert_w_m1(1.0), None);
/// ```
pub fn lambert_w_m1(z: f64) -> Option<f64> {
    swm1(z)
}
