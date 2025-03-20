//! This module contains the general implementation of the Lambert W function.
//! This implementation is capable of computing the function at any point in the complex plane on any branch.

use num_complex::{Complex64, ComplexFloat};

use core::f64::consts::{E, PI};

use crate::NEG_INV_E;

/// Computes branch `k` of the complex valued Lambert W function
/// using Halley's method.
///
/// The return value is a tuple where the first element is the
/// real part and the second element is the imaginary part.
///
/// Close to the branch cut at -1/e this function may be slightly less accurate.
///
/// If you know you want the principal or secondary branches where they are real valued,
/// take a look at the [`lambert_w0`](crate::lambert_w0) or [`lambert_wm1`](crate::lambert_wm1) functions and their 32-bit equivalents instead.
/// They can be up to two orders of magnitude faster.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use lambert_w::lambert_w;
///
/// // W_2(1 + 2i)
/// let w = lambert_w(2, 1.0, 2.0);
///
/// assert_eq!(w, (-1.6869138779375397, 11.962631435322813));
/// ```
///
/// Returns [`NAN`](f64::NAN)s if any of the inputs are infinite:
///
/// ```
/// # use lambert_w::lambert_w;
/// let w = lambert_w(-13, f64::INFINITY, 0.0);
///
/// assert!(w.0.is_nan() && w.1.is_nan());
/// ```
///
/// or `NAN`:
///
/// ```
/// # use lambert_w::lambert_w;
/// let w = lambert_w(1_000, 0.0, f64::NAN);
///
/// assert!(w.0.is_nan() && w.1.is_nan());
/// ```
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
#[must_use = "this is a pure function that only returns a value and has no side effects"]
pub fn lambert_w(k: i32, z_re: f64, z_im: f64) -> (f64, f64) {
    const MAX_ITER: usize = 30;
    /// If the absolute difference between two consecutive iterations is less than this value,
    /// the iteration stops.
    const PREC: f64 = 1e-30;
    const I: Complex64 = Complex64::I;

    let z = Complex64::new(z_re, z_im);

    // region: special cases

    if z == 0.0.into() {
        if k == 0 {
            return (0.0, 0.0);
        } else {
            return (f64::NEG_INFINITY, 0.0);
        }
    }
    if z == NEG_INV_E.into() && (k == 0 || k == -1) {
        return (-1.0, 0.0);
    }
    if z == E.into() && k == 0 {
        return (1.0, 0.0);
    }

    // endregion: special cases

    // region: determine initial search point

    let two_pi_k_i = 2.0 * PI * f64::from(k) * I;
    let mut w = z.ln() + two_pi_k_i - (z.ln() + two_pi_k_i).ln();

    // Choose the initial point more carefully when we are close to the branch cut.
    if (z - NEG_INV_E).abs() <= 1.0 {
        let p = (2.0 * (E * z + 1.0)).sqrt();
        let p2 = -1.0 + p - 1.0 / 3.0 * p * p;
        let p3 = 11.0 / 72.0 * p * p * p;
        if k == 0 {
            w = p2 + p3;
        } else if (k == 1 && z.im < 0.0) || (k == -1 && z.im > 0.0) {
            w = p2 - p3;
        }
    }

    if k == 0 && (z - 0.5).abs() <= 0.5 {
        // Order (1,1) Padé approximant for the principal branch
        w = (0.35173371 * (0.1237166 + 7.061302897 * z)) / (2.0 + 0.827184 * (1.0 + 2.0 * z));
    }

    if k == -1 && (z - 0.5).abs() <= 0.5 {
        // Order (1,1) Padé approximant for the secondary branch
        w = -(((2.2591588985 + 4.22096 * I)
            * ((-14.073271 - 33.767687754 * I) * z - (12.7127 - 19.071643 * I) * (1.0 + 2.0 * z)))
            / (2.0 - (17.23103 - 10.629721 * I) * (1.0 + 2.0 * z)));
    }

    // endregion: determine initial search point

    // region: Halley iteration

    let mut iter = 0;
    loop {
        let w_prev = w;
        let ew = w.exp();
        let wew = w * ew;
        let wew_d = ew + wew;
        let wew_dd = ew + wew_d;
        w -= 2.0 * ((wew - z) * wew_d) / (2.0 * wew_d * wew_d - (wew - z) * wew_dd);

        iter += 1;

        if (w - w_prev).abs() <= PREC || iter >= MAX_ITER {
            return (w.re, w.im);
        }
    }

    // endregion: Halley iteration
}
