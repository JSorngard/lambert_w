use num_complex::{Complex64, ComplexFloat};

use crate::{lambert_w0, lambert_wm1, NEG_INV_E};

/// Computes branch `k` of the Lambert W function
/// with the Halley iteration method.
///
/// This implementation delegates to [`lambert_w0`] and [`lambert_wm1`] when z is real and
/// in the appropriate range.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use lambert_w::lambert_wk;
/// use num_complex::Complex64;
///
/// let w = lambert_wk(2, Complex64::new(1.0, 2.0));
///
/// assert_eq!(w.re, -1.6869138779375397);
/// assert_eq!(w.im, 11.962631435322813);
/// ```
// Based on <https://github.com/IstvanMezo/LambertW-function>.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
#[must_use = "this is a pure function that only returns a value and has no side effects"]
pub fn lambert_wk(k: i32, z: Complex64) -> Complex64 {
    const Z_NEG_INV_E: Complex64 = Complex64::new(NEG_INV_E, 0.0);
    const Z_ZERO: Complex64 = Complex64::new(0.0, 0.0);
    const I: Complex64 = Complex64::new(0.0, 1.0);
    const E: Complex64 = Complex64::new(core::f64::consts::E, 0.0);
    const MAX_ITER: usize = 30;
    /// The maximum (absolute) error of the result.
    const PREC: f64 = 1e-30;

    // region: special cases

    if z.im == 0.0 && z.re >= NEG_INV_E {
        if k == 0 {
            return lambert_w0(z.re).into();
        } else if k == -1 && z.re <= 0.0 {
            return lambert_wm1(z.re).into();
        }
    }
    if z == Z_ZERO {
        if k == 0 {
            return Z_ZERO;
        } else {
            return f64::NEG_INFINITY.into();
        }
    }
    if z == Z_NEG_INV_E && k == 1 {
        return (-1.0).into();
    }

    // endregion: special cases

    // region: determine initial search point

    let two_pi_k_i = Complex64::new(0.0, 2.0 * core::f64::consts::PI * f64::from(k));
    let mut w = z.ln() + two_pi_k_i - (z.ln() + two_pi_k_i).ln();
    let p = (2.0 * (E * z + 1.0)).sqrt();

    // We are close to the branch cut, the initial point must be chosen carefully
    if (z - Z_NEG_INV_E).abs() <= 1.0 {
        if k == 0 {
            w = -1.0 + p - 1.0 / 3.0 * p * p + 11.0 / 72.0 * p * p * p;
        } else if (k == 1 && z.im < 0.0) || (k == -1 && z.im > 0.0) {
            w = -1.0 - p - 1.0 / 3.0 * p * p - 11.0 / 72.0 * p * p * p;
        }
    }

    if k == 0 && (z - 0.5).abs() <= 0.5 {
        // (1,1) Pade approximant for W(0,a)
        w = (0.35173371 * (0.1237166 + 7.061302897 * z)) / (2.0 + 0.827184 * (1.0 + 2.0 * z));
    }

    if k == -1 && (z - 0.5).abs() <= 0.5 {
        // (1,1) Pade approximant for W(-1,a)
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

        if !(((w - w_prev).abs() > PREC) && iter < MAX_ITER) {
            return w;
        }
    }

    // endregion: Halley iteration
}
