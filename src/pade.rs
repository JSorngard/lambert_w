//! Padé approximants that use Estrin's scheme with fused multiply-add instructions for better performance if the `fma` feature is enabled,
//! and otherwise it uses the more typical Horner's method.

#[cfg(feature = "24bits")]
/// Padé approximant consisting of two third degree polynomials.
///
/// The first set of coefficients are for the polynomial in the numerator
/// and the second set are the coefficients of the polynomial in the denominator.
///
/// If the `fma` feature is enabled this uses Estrin's scheme and fused multiply-add instructions, otherwise it uses the more typical Horner's method.
#[inline(always)]
pub(crate) fn pade_3(x: f64, [n0, n1, n2, n3]: [f64; 4], [d0, d1, d2, d3]: [f64; 4]) -> f64 {
    #[cfg(feature = "fma")]
    {
        use fast_polynomial::polynomials::poly_3;
        let x2 = x * x;
        poly_3(x, x2, n0, n1, n2, n3) / poly_3(x, x2, d0, d1, d2, d3)
    }

    #[cfg(not(feature = "fma"))]
    {
        (n0 + x * (n1 + x * (n2 + x * n3))) / (d0 + x * (d1 + x * (d2 + x * d3)))
    }
}

#[cfg(feature = "24bits")]
/// Padé approximant consisting of a fourth degree polynomial divided by a third degree polynomial.
///
/// The first set of coefficients are for the polynomial in the numerator
/// and the second set are the coefficients of the polynomial in the denominator.
///
/// If the `fma` feature is enabled this uses Estrin's scheme and fused multiply-add instructions, otherwise it uses the more typical Horner's method.
#[inline(always)]
pub(crate) fn pade_4_3(x: f64, [n0, n1, n2, n3, n4]: [f64; 5], [d0, d1, d2, d3]: [f64; 4]) -> f64 {
    #[cfg(feature = "fma")]
    {
        use fast_polynomial::polynomials::{poly_3, poly_4};
        let x2 = x * x;
        let x4 = x2 * x2;
        poly_4(x, x2, x4, n0, n1, n2, n3, n4) / poly_3(x, x2, d0, d1, d2, d3)
    }

    #[cfg(not(feature = "fma"))]
    {
        (n0 + x * (n1 + x * (n2 + x * (n3 + x * n4)))) / (d0 + x * (d1 + x * (d2 + x * d3)))
    }
}

#[cfg(feature = "50bits")]
/// Padé approximant consisting of two seventh degree polynomials.
///
/// The first set of coefficients are for the polynomial in the numerator
/// and the second set are the coefficients of the polynomial in the denominator.
///
/// If the `fma` feature is enabled this uses Estrin's scheme and fused multiply-add instructions, otherwise it uses the more typical Horner's method.
#[inline(always)]
pub(crate) fn pade_7(
    x: f64,
    [n0, n1, n2, n3, n4, n5, n6, n7]: [f64; 8],
    [d0, d1, d2, d3, d4, d5, d6, d7]: [f64; 8],
) -> f64 {
    #[cfg(feature = "fma")]
    {
        use fast_polynomial::polynomials::poly_7;

        let x2 = x * x;
        let x4 = x2 * x2;
        poly_7(x, x2, x4, n0, n1, n2, n3, n4, n5, n6, n7)
            / poly_7(x, x2, x4, d0, d1, d2, d3, d4, d5, d6, d7)
    }
    #[cfg(not(feature = "fma"))]
    {
        (n0 + x * (n1 + x * (n2 + x * (n3 + x * (n4 + x * (n5 + x * (n6 + x * n7)))))))
            / (d0 + x * (d1 + x * (d2 + x * (d3 + x * (d4 + x * (d5 + x * (d6 + x * d7)))))))
    }
}

#[cfg(feature = "50bits")]
/// Padé approximant consisting of an eigth degree polynomial divided by a seventh degree polynomial.
///
/// The first set of coefficients are for the polynomial in the numerator
/// and the second set are the coefficients of the polynomial in the denominator.
///
/// If the `fma` feature is enabled this uses Estrin's scheme and fused multiply-add instructions, otherwise it uses the more typical Horner's method.
#[inline(always)]
pub(crate) fn pade_8_7(
    x: f64,
    [n0, n1, n2, n3, n4, n5, n6, n7, n8]: [f64; 9],
    [d0, d1, d2, d3, d4, d5, d6, d7]: [f64; 8],
) -> f64 {
    #[cfg(feature = "fma")]
    {
        use fast_polynomial::polynomials::{poly_7, poly_8};

        let x2 = x * x;
        let x4 = x2 * x2;
        let x8 = x4 * x4;
        poly_8(x, x2, x4, x8, n0, n1, n2, n3, n4, n5, n6, n7, n8)
            / poly_7(x, x2, x4, d0, d1, d2, d3, d4, d5, d6, d7)
    }

    #[cfg(not(feature = "fma"))]
    {
        (n0 + x * (n1 + x * (n2 + x * (n3 + x * (n4 + x * (n5 + x * (n6 + x * (n7 + x * n8))))))))
            / (d0 + x * (d1 + x * (d2 + x * (d3 + x * (d4 + x * (d5 + x * (d6 + x * d7)))))))
    }
}
