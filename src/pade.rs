use fast_polynomial::polynomials::poly_3;

pub fn pade_3(x: f64, x2: f64, [n0, n1, n2, n3]: [f64; 4], [d0, d1, d2, d3]: [f64; 4]) -> f64 {
    poly_3(x, x2, n0, n1, n2, n3) / poly_3(x, x2, d0, d1, d2, d3)
}
