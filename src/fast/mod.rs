//! Calculate the Lambert W function to 24 bits of accuracy.

mod sw0;
mod swm1;

use sw0::sw0;
use swm1::swm1;

/// Computes the principal branch of the Lambert W function, W_0(`z`), to 24 bits of accuracy, if `z` >= -1/e.
///
/// Uses the piecewise minimax rational function approximation with variable transformations method of Toshio Fukushima.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use approx::assert_abs_diff_eq;
/// use lambert_w::fast::lambert_w_0;
///
/// let w = lambert_w_0(1.0).unwrap();
///
/// assert_abs_diff_eq!(w, 0.5671432904097838, epsilon = 1e-7);
/// ```
/// Arguments smaller than -1/e (≈ -0.36787944117144233) result in `None`:
/// ```
/// # use lambert_w::fast::lambert_w_0;
/// assert_eq!(lambert_w_0(-1.0), None);
/// ```
pub fn lambert_w_0(z: f64) -> Option<f64> {
    sw0(z)
}

/// Computes the secondary branch of the Lambert W function, W_-1(`z`), to 24 bits of accuracy, if -1/e <= `z` <= 0.
///
/// Uses the piecewise minimax rational function approximation with variable transformations method of Toshio Fukushima.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use approx::assert_abs_diff_eq;
/// use lambert_w::fast::lambert_w_m1;
///
/// let w = lambert_w_m1(-f64::ln(2.0) / 2.0).unwrap();
///
/// assert_abs_diff_eq!(w, -f64::ln(4.0), epsilon = 1e-9);
/// ```
/// Arguments smaller than -1/e (≈ -0.36787944117144233) or larger than 0 result in `None`:
/// ```
/// # use lambert_w::fast::lambert_w_m1;
/// assert_eq!(lambert_w_m1(-1.0), None);
/// assert_eq!(lambert_w_m1(1.0), None);
/// ```
pub fn lambert_w_m1(z: f64) -> Option<f64> {
    swm1(z)
}
