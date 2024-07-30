use fast_polynomial::polynomials::{poly_3, poly_4};

/// Padé approximant consisting of two third degree polynomials.
/// The first set of coefficients are for the polynomial in the numerator
/// and the second set are the coefficients of the polynomial in the denominator.
#[inline(always)]
pub fn pade_3(x: f64, x2: f64, [n0, n1, n2, n3]: [f64; 4], [d0, d1, d2, d3]: [f64; 4]) -> f64 {
    poly_3(x, x2, n0, n1, n2, n3) / poly_3(x, x2, d0, d1, d2, d3)
}

/// Padé approximant consisting of a fourth degree polynomial divided by a third degree polynomial.
/// The first set of coefficients are for the polynomial in the numerator
/// and the second set are the coefficients of the polynomial in the denominator.
#[inline(always)]
pub fn pade_4_3(
    x: f64,
    x2: f64,
    x4: f64,
    [n0, n1, n2, n3, n4]: [f64; 5],
    [d0, d1, d2, d3]: [f64; 4],
) -> f64 {
    poly_4(x, x2, x4, n0, n1, n2, n3, n4) / poly_3(x, x2, d0, d1, d2, d3)
}
