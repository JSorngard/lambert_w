//! Rational functions evaluated with Horner's method.

pub fn rational_function<const N: usize, const D: usize>(
    x: f64,
    numerator: [f64; N],
    denominator: [f64; D],
) -> f64 {
    numerator.into_iter().rev().fold(0.0, |acc, n| acc * x + n)
        / denominator
            .into_iter()
            .rev()
            .fold(0.0, |acc, d| acc * x + d)
}

pub fn rational_functionf<const N: usize, const D: usize>(
    x: f32,
    numerator: [f32; N],
    denominator: [f32; D],
) -> f32 {
    numerator.into_iter().rev().fold(0.0, |acc, n| acc * x + n)
        / denominator
            .into_iter()
            .rev()
            .fold(0.0, |acc, d| acc * x + d)
}
