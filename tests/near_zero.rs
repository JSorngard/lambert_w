//! Regression tests for cancellation in the real principal branches.
//!
//! The 50-bit path allows eight binary64 epsilons for evaluation and the
//! transition to the rational approximation. The low-precision binary64 path
//! uses 2^-24; binary32 allows eight epsilons for its additional rounding.

use approx::assert_relative_eq;
use lambert_w::{lambert_w0, lambert_w0f, sp_lambert_w0};

#[test]
fn principal_branches_match_high_precision_near_zero() {
    for row in include_str!("near_zero.csv").lines().skip(1) {
        let fields: Vec<_> = row.split(',').collect();
        match fields[0] {
            "f64" => {
                let x: f64 = fields[1].parse().unwrap();
                let expected: f64 = fields[2].parse().unwrap();
                for (function, tolerance) in [
                    (lambert_w0 as fn(f64) -> f64, 8.0 * f64::EPSILON),
                    (sp_lambert_w0, f64::from(f32::EPSILON) / 2.0),
                ] {
                    let actual = function(x);
                    assert_eq!(actual.is_sign_negative(), x.is_sign_negative(), "{row}");
                    assert!(
                        ((actual - expected) / expected).abs() <= tolerance,
                        "{row}: got {actual:e}"
                    );
                }
            }
            "f32" => {
                let x: f32 = fields[1].parse().unwrap();
                let expected: f32 = fields[2].parse().unwrap();
                let actual = lambert_w0f(x);
                assert_eq!(actual.is_sign_negative(), x.is_sign_negative(), "{row}");
                assert!(
                    ((actual - expected) / expected).abs() <= 8.0 * f32::EPSILON,
                    "{row}: got {actual:e}"
                );
            }
            kind => panic!("unexpected reference type {kind}"),
        }
    }
}

#[test]
fn principal_branches_preserve_signed_zero() {
    for x in [0.0_f64, -0.0] {
        assert_eq!(lambert_w0(x).to_bits(), x.to_bits());
        assert_eq!(sp_lambert_w0(x).to_bits(), x.to_bits());
    }
    for x in [0.0_f32, -0.0] {
        assert_eq!(lambert_w0f(x).to_bits(), x.to_bits());
    }
}

#[test]
fn principal_branches_satisfy_inverse_identity_near_zero() {
    for i in -1000..=1000 {
        if i == 0 {
            continue;
        }
        let x = f64::from(i) * 0.05 / 1000.0;
        let w = lambert_w0(x);
        assert_relative_eq!(w * w.exp(), x, max_relative = 8.0 * f64::EPSILON);
        let w = sp_lambert_w0(x);
        assert_relative_eq!(w * w.exp(), x, max_relative = f64::from(f32::EPSILON) / 2.0);
        let x = x as f32;
        let w = lambert_w0f(x);
        assert_relative_eq!(w * w.exp(), x, max_relative = 8.0 * f32::EPSILON);
    }
}
