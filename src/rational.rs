//! Rational functions evaluated with Horner's method.

// The #[inline(always)] annotations are motivated by benchmarks, especially of the 50 bit functions.

/// Rational function consisting of two third degree polynomials.
///
/// The first set of coefficients are for the polynomial in the numerator
/// and the second set are the coefficients of the polynomial in the denominator.
#[inline(always)]
pub fn rational_3_over_3(x: f64, [n0, n1, n2, n3]: [f64; 4], [d0, d1, d2, d3]: [f64; 4]) -> f64 {
    (n0 + x * (n1 + x * (n2 + x * n3))) / (d0 + x * (d1 + x * (d2 + x * d3)))
}

/// Rational function consisting of two third degree polynomials.
///
/// The first set of coefficients are for the polynomial in the numerator
/// and the second set are the coefficients of the polynomial in the denominator.
#[inline(always)]
pub fn rational_3_over_3f(x: f32, [n0, n1, n2, n3]: [f32; 4], [d0, d1, d2, d3]: [f32; 4]) -> f32 {
    (n0 + x * (n1 + x * (n2 + x * n3))) / (d0 + x * (d1 + x * (d2 + x * d3)))
}

/// Rational function consisting of a fourth degree polynomial divided by a third degree polynomial.
///
/// The first set of coefficients are for the polynomial in the numerator
/// and the second set are the coefficients of the polynomial in the denominator.
#[inline(always)]
pub fn rational_4_over_3(
    x: f64,
    [n0, n1, n2, n3, n4]: [f64; 5],
    [d0, d1, d2, d3]: [f64; 4],
) -> f64 {
    (n0 + x * (n1 + x * (n2 + x * (n3 + x * n4)))) / (d0 + x * (d1 + x * (d2 + x * d3)))
}

/// Rational function consisting of a fourth degree polynomial divided by a third degree polynomial.
///
/// The first set of coefficients are for the polynomial in the numerator
/// and the second set are the coefficients of the polynomial in the denominator.
#[inline(always)]
pub fn rational_4_over_3f(
    x: f32,
    [n0, n1, n2, n3, n4]: [f32; 5],
    [d0, d1, d2, d3]: [f32; 4],
) -> f32 {
    (n0 + x * (n1 + x * (n2 + x * (n3 + x * n4)))) / (d0 + x * (d1 + x * (d2 + x * d3)))
}

/// Rational function consisting of two seventh degree polynomials.
///
/// The first set of coefficients are for the polynomial in the numerator
/// and the second set are the coefficients of the polynomial in the denominator.
#[inline(always)]
pub fn rational_7_over_7(
    x: f64,
    [n0, n1, n2, n3, n4, n5, n6, n7]: [f64; 8],
    [d0, d1, d2, d3, d4, d5, d6, d7]: [f64; 8],
) -> f64 {
    (n0 + x * (n1 + x * (n2 + x * (n3 + x * (n4 + x * (n5 + x * (n6 + x * n7)))))))
        / (d0 + x * (d1 + x * (d2 + x * (d3 + x * (d4 + x * (d5 + x * (d6 + x * d7)))))))
}

/// Rational function consisting of an eigth degree polynomial divided by a seventh degree polynomial.
///
/// The first set of coefficients are for the polynomial in the numerator
/// and the second set are the coefficients of the polynomial in the denominator.
#[inline(always)]
pub fn rational_8_over_7(
    x: f64,
    [n0, n1, n2, n3, n4, n5, n6, n7, n8]: [f64; 9],
    [d0, d1, d2, d3, d4, d5, d6, d7]: [f64; 8],
) -> f64 {
    (n0 + x * (n1 + x * (n2 + x * (n3 + x * (n4 + x * (n5 + x * (n6 + x * (n7 + x * n8))))))))
        / (d0 + x * (d1 + x * (d2 + x * (d3 + x * (d4 + x * (d5 + x * (d6 + x * d7)))))))
}

#[cfg(test)]
mod test {
    use super::*;

    use approx::assert_abs_diff_eq;

    #[test]
    fn sanity_check_rational_3_over_3() {
        let n = [1.0, 2.0, 3.0, 4.0];
        let d = [5.0, 6.0, 7.0, 8.0];
        let x = 1.0;
        let expected = (1.0 + 2.0 + 3.0 + 4.0) / (5.0 + 6.0 + 7.0 + 8.0);
        assert_abs_diff_eq!(rational_3_over_3(x, n, d), expected);
    }

    #[test]
    fn sanity_check_rational_3_over_3f() {
        let n = [1.0, 2.0, 3.0, 4.0];
        let d = [5.0, 6.0, 7.0, 8.0];
        let x = 1.0;
        let expected = (1.0 + 2.0 + 3.0 + 4.0) / (5.0 + 6.0 + 7.0 + 8.0);
        assert_abs_diff_eq!(rational_3_over_3f(x, n, d), expected);
    }

    #[test]
    fn sanity_check_rational_4_over_3() {
        let n = [1.0, 2.0, 3.0, 4.0, 5.0];
        let d = [6.0, 7.0, 8.0, 9.0];
        let x = 1.0;
        let expected = (1.0 + 2.0 + 3.0 + 4.0 + 5.0) / (6.0 + 7.0 + 8.0 + 9.0);
        assert_abs_diff_eq!(rational_4_over_3(x, n, d), expected);
    }

    #[test]
    fn sanity_check_rational_4_over_3f() {
        let n = [1.0, 2.0, 3.0, 4.0, 5.0];
        let d = [6.0, 7.0, 8.0, 9.0];
        let x = 1.0;
        let expected = (1.0 + 2.0 + 3.0 + 4.0 + 5.0) / (6.0 + 7.0 + 8.0 + 9.0);
        assert_abs_diff_eq!(rational_4_over_3f(x, n, d), expected);
    }

    #[test]
    fn sanity_check_rational_7_over_7() {
        let n = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let d = [9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0];
        let x = 1.0;
        let expected = (1.0 + 2.0 + 3.0 + 4.0 + 5.0 + 6.0 + 7.0 + 8.0)
            / (9.0 + 10.0 + 11.0 + 12.0 + 13.0 + 14.0 + 15.0 + 16.0);
        assert_abs_diff_eq!(rational_7_over_7(x, n, d), expected);
    }

    #[test]
    fn sanity_check_rational_8_over_7() {
        let n = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let d = [10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0];
        let x = 1.0;
        let expected = (1.0 + 2.0 + 3.0 + 4.0 + 5.0 + 6.0 + 7.0 + 8.0 + 9.0)
            / (10.0 + 11.0 + 12.0 + 13.0 + 14.0 + 15.0 + 16.0 + 17.0);
        assert_abs_diff_eq!(rational_8_over_7(x, n, d), expected);
    }
}
