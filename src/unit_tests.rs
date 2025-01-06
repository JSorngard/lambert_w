use super::{
    elementary::{ln, lnf, sqrt, sqrtf},
    rational::{
        rational_3_over_3, rational_3_over_3f, rational_4_over_3, rational_4_over_3f,
        rational_7_over_7, rational_8_over_7,
    },
};
use approx::{assert_abs_diff_eq, assert_relative_eq};

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

#[test]
fn sanity_check_log() {
    assert!(ln(-1.0).is_nan());
    assert_abs_diff_eq!(ln(1.0), 0.0);
    assert_abs_diff_eq!(ln(2.0), core::f64::consts::LN_2);
    assert_abs_diff_eq!(ln(f64::MAX), 709.782_712_893_384);
}

#[test]
fn sanity_check_logf() {
    assert!(lnf(-1.0).is_nan());
    assert_abs_diff_eq!(lnf(1.0), 0.0);
    assert_abs_diff_eq!(lnf(2.0), core::f32::consts::LN_2);
    assert_abs_diff_eq!(lnf(f32::MAX), 88.722_839);
}

#[test]
fn sanity_check_sqrt() {
    assert!(sqrt(-1.0).is_nan());
    assert_abs_diff_eq!(sqrt(0.0), 0.0);
    assert_abs_diff_eq!(sqrt(1.0), 1.0);
    assert_abs_diff_eq!(sqrt(2.0), 1.414_213_562_373_095_1);
    assert_abs_diff_eq!(sqrt(4.0), 2.0);
    assert_abs_diff_eq!(sqrt(f64::MAX), 1.340_780_792_994_259_6e154);
}

#[test]
fn sanity_check_sqrtf() {
    assert!(sqrtf(-1.0).is_nan());
    assert_abs_diff_eq!(sqrtf(0.0), 0.0);
    assert_abs_diff_eq!(sqrtf(1.0), 1.0);
    assert_abs_diff_eq!(sqrtf(2.0), 1.414_213_5);
    assert_abs_diff_eq!(sqrtf(4.0), 2.0);
    assert_relative_eq!(sqrtf(f32::MAX), 1.844_674_4e19);
}
