use num_complex::{Complex64, ComplexFloat};

const NEG_INV_E: Complex64 = Complex64::new(crate::NEG_INV_E, 0.0);
const ZERO: Complex64 = Complex64::new(0.0, 0.0);
const ONE: Complex64 = Complex64::new(1.0, 0.0);
const I: Complex64 = Complex64::new(0.0, 1.0);
const INFINITY: Complex64 = Complex64::new(core::f64::INFINITY, 0.0);
const E: Complex64 = Complex64::new(core::f64::consts::E, 0.0);

/// Computes the given branch of the Lambert W function
/// with the Halley iteration method.
///
/// The branch is specified by the integer `k`.
///
/// The function is computed with a maximum of 30 iterations
/// and a precision threshold of 1e-30.
///
/// # Example
///
/// ```
/// use lambert_w::lambert_wk;
/// # use num_complex::Complex64;
/// # use approx::assert_abs_diff_eq;
///
/// let w = lambert_wk(2, Complex64::new(1.0, 2.0));
///
/// assert_abs_diff_eq!(w.re, -1.6869138779375397);
/// assert_abs_diff_eq!(w.im, 11.962631435322813);
/// ```
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn lambert_wk(k: i32, z: Complex64) -> Complex64 {
    const MAXITER: usize = 30;
    const PREC: f64 = 1e-30;

    if z == ZERO {
        if k == 0 {
            return ZERO;
        } else {
            return -INFINITY;
        }
    }
    if z == NEG_INV_E && (k == 0 || k == 1) {
        return -ONE;
    }
    if z == E && k == 0 {
        return ONE;
    }

    let mut w = initial_search_point(k, z);
    let mut w_prev;
    let mut iter = 0;
    loop {
        w_prev = w;
        let zez = zexpz(w);
        let zezd = zexpz_d(w);
        w -= 2.0 * ((zez - z) * zezd) / (2.0 * zezd * zezd - (zez - z) * zexpz_dd(w));

        iter += 1;

        if !(((w - w_prev).abs() > PREC) && iter < MAXITER) {
            break w;
        }
    }
}

/// Computes the initial search point for the Halley iteration method.
#[inline(always)]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
fn initial_search_point(k: i32, z: Complex64) -> Complex64 {
    let two_pi_k_i = Complex64::new(0.0, 2.0 * core::f64::consts::PI * (k as f64));
    let mut ip = z.ln() + two_pi_k_i - (z.ln() + two_pi_k_i).ln();
    let p = (2.0 * (E * z + 1.0)).sqrt();

    // We are close to the branch cut, the initial point must be chosen carefully
    if (z - NEG_INV_E).abs() <= 1.0 {
        if k == 0 {
            ip = -1.0 + p - 1.0 / 3.0 * p.powi(2) + 11.0 / 72.0 * p.powi(3);
        } else if k == 1 && z.im < 0.0 {
            ip = -1. - p - 1. / 3. * p * p - 11. / 72. * p * p * p;
        } else if k == -1 && z.im > 0.0 {
            ip = -1. - p - 1. / 3. * p * p - 11. / 72. * p * p * p;
        }
    }

    if k == 0 && (z - 0.5).abs() <= 0.5 {
        // (1,1) Pade approximant for W(0,a)
        ip = (0.35173371 * (0.1237166 + 7.061302897 * z)) / (2. + 0.827184 * (1. + 2. * z));
    }

    if k == -1 && (z - 0.5).abs() <= 0.5 {
        // (1,1) Pade approximant for W(-1,a)
        ip = -(((2.2591588985 + 4.22096 * I)
            * ((-14.073271 - 33.767687754 * I) * z - (12.7127 - 19.071643 * I) * (1. + 2. * z)))
            / (2. - (17.23103 - 10.629721 * I) * (1. + 2. * z)));
    }

    ip
}

/// Computes z*e^z.
#[inline(always)]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn zexpz(z: Complex64) -> Complex64 {
    z * z.exp()
}

/// Computes the first derivative of z*e^z = e^z + z*e^z.
#[inline(always)]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn zexpz_d(z: Complex64) -> Complex64 {
    let e = z.exp();
    e + z * e
}

/// Computes the second derivative of z*e^z = 2*e^z + z*e^z.
#[inline(always)]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn zexpz_dd(z: Complex64) -> Complex64 {
    let e = z.exp();
    e + e + z * e
}
