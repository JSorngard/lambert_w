//! Calculate the Lambert W function to 50 bits of accuracy.

mod dw0c;
mod dwm1c;

use dw0c::dw0c;
use dwm1c::dwm1c;

use super::Z0;

/// Computes the principal branch of the Lambert W function, W_0(`z`), to 50 bits of accuracy, if `z` >= -1/e.
///
/// Uses the piecewise minimax rational function approximation with variable transformations method of Toshio Fukushima.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use approx::assert_abs_diff_eq;
/// use lambert_w::accurate::lambert_w_0;
///
/// let Ω = lambert_w_0(1.0).unwrap();
///
/// assert_abs_diff_eq!(Ω, 0.5671432904097838);
/// ```
/// Arguments smaller than -1/e (≈ -0.36787944117144233) result in `None`:
/// ```
/// # use lambert_w::accurate::lambert_w_0;
/// assert_eq!(lambert_w_0(-1.0), None);
/// ```
pub fn lambert_w_0(z: f64) -> Option<f64> {
    dw0c(z - Z0)
}

/// Computes the secondary branch of the Lambert W function, W_-1(`z`), to 50 bits of accuracy, if -1/e <= `z` <= 0.
///
/// Uses the piecewise minimax rational function approximation with variable transformations method of Toshio Fukushima.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use approx::assert_abs_diff_eq;
/// use lambert_w::accurate::lambert_w_m1;
///
/// let mln4 = lambert_w_m1(-f64::ln(2.0) / 2.0).unwrap();
///
/// assert_abs_diff_eq!(mln4, -f64::ln(4.0));
/// ```
/// Arguments smaller than -1/e (≈ -0.36787944117144233) or larger than 0 result in `None`:
/// ```
/// # use lambert_w::accurate::lambert_w_m1;
/// assert_eq!(lambert_w_m1(-1.0), None);
/// assert_eq!(lambert_w_m1(1.0), None);
/// ```
pub fn lambert_w_m1(z: f64) -> Option<f64> {
    dwm1c(z, z - Z0)
}
