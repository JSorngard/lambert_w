// Copyright 2026 Johanna Sörngård
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This file contains unit tests for non-public functions.

use crate::{
    all_complex_branches::are_nearly_equal,
    generic_math::{ln, rational_function, sqrt}
};

use approx::{assert_abs_diff_eq, assert_relative_eq};
use num_complex::{c32, c64};

#[test]
fn test_are_nearly_equal() {
    assert!(are_nearly_equal(
        c32(0.0, 0.0),
        c32(0.0, 0.0),
        f32::EPSILON
    ));
    assert!(!are_nearly_equal(
        c32(0.0, f32::NAN),
        c32(0.0, 0.0),
        f32::EPSILON
    ));
    assert!(are_nearly_equal(c32(10.0, 0.0), c32(10.000001, 0.0), 0.01));
    assert!(!are_nearly_equal(
        c32(f32::MIN_POSITIVE / 3.0, 0.0),
        c32(f32::MIN_POSITIVE / 6.0, 0.0), 0.1));
}

#[test]
fn sanity_check_rational_3_over_3() {
    let n = [1.0, 2.0, 3.0, 4.0];
    let d = [5.0, 6.0, 7.0, 8.0];
    let x = 1.0;
    let expected = (1.0 + 2.0 + 3.0 + 4.0) / (5.0 + 6.0 + 7.0 + 8.0);
    assert_abs_diff_eq!(rational_function(x, n, d), expected);
}

#[test]
fn sanity_check_rational_3_over_3f() {
    let n = [1.0, 2.0, 3.0, 4.0];
    let d = [5.0, 6.0, 7.0, 8.0];
    let x: f32 = 1.0;
    let expected = (1.0 + 2.0 + 3.0 + 4.0) / (5.0 + 6.0 + 7.0 + 8.0);
    assert_abs_diff_eq!(rational_function(x, n, d), expected);
}

#[test]
fn sanity_check_rational_4_over_3() {
    let n = [1.0, 2.0, 3.0, 4.0, 5.0];
    let d = [6.0, 7.0, 8.0, 9.0];
    let x = 1.0;
    let expected = (1.0 + 2.0 + 3.0 + 4.0 + 5.0) / (6.0 + 7.0 + 8.0 + 9.0);
    assert_abs_diff_eq!(rational_function(x, n, d), expected);
}

#[test]
fn sanity_check_rational_4_over_3f() {
    let n = [1.0, 2.0, 3.0, 4.0, 5.0];
    let d = [6.0, 7.0, 8.0, 9.0];
    let x: f32 = 1.0;
    let expected = (1.0 + 2.0 + 3.0 + 4.0 + 5.0) / (6.0 + 7.0 + 8.0 + 9.0);
    assert_abs_diff_eq!(rational_function(x, n, d), expected);
}

#[test]
fn sanity_check_rational_7_over_7() {
    let n = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let d = [9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0];
    let x = 1.0;
    let expected = (1.0 + 2.0 + 3.0 + 4.0 + 5.0 + 6.0 + 7.0 + 8.0)
        / (9.0 + 10.0 + 11.0 + 12.0 + 13.0 + 14.0 + 15.0 + 16.0);
    assert_abs_diff_eq!(rational_function(x, n, d), expected);
}

#[test]
fn sanity_check_rational_8_over_7() {
    let n = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let d = [10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0];
    let x = 1.0;
    let expected = (1.0 + 2.0 + 3.0 + 4.0 + 5.0 + 6.0 + 7.0 + 8.0 + 9.0)
        / (10.0 + 11.0 + 12.0 + 13.0 + 14.0 + 15.0 + 16.0 + 17.0);
    assert_abs_diff_eq!(rational_function(x, n, d), expected);
}

#[test]
fn sanity_check_log() {
    assert!(ln(-1.0_f64).is_nan());
    assert_abs_diff_eq!(ln(1.0_f64), 0.0);
    assert_abs_diff_eq!(ln(2.0_f64), core::f64::consts::LN_2);
    assert_abs_diff_eq!(ln(f64::MAX), 709.782_712_893_384);
    assert_eq!(ln(f64::INFINITY), f64::INFINITY);
}

#[test]
fn sanity_check_logf() {
    assert!(ln(-1.0_f32).is_nan());
    assert_abs_diff_eq!(ln(1.0_f32), 0.0);
    assert_abs_diff_eq!(ln(2.0_f32), core::f32::consts::LN_2);
    assert_abs_diff_eq!(ln(f32::MAX), 88.722_84);
    assert_eq!(ln(f32::INFINITY), f32::INFINITY);
}

#[test]
fn sanity_check_sqrt() {
    assert!(sqrt(-1.0_f64).is_nan());
    assert_abs_diff_eq!(sqrt(0.0_f64), 0.0);
    assert_abs_diff_eq!(sqrt(1.0_f64), 1.0);
    assert_abs_diff_eq!(sqrt(2.0_f64), core::f64::consts::SQRT_2);
    assert_abs_diff_eq!(sqrt(4.0_f64), 2.0);
    assert_abs_diff_eq!(sqrt(f64::MAX), 1.340_780_792_994_259_6e154);
    assert_eq!(sqrt(f64::INFINITY), f64::INFINITY);
}

#[test]
fn sanity_check_sqrtf() {
    assert!(sqrt(-1.0_f32).is_nan());
    assert_abs_diff_eq!(sqrt(0.0_f32), 0.0);
    assert_abs_diff_eq!(sqrt(1.0_f32), 1.0);
    assert_abs_diff_eq!(sqrt(2.0_f32), core::f32::consts::SQRT_2);
    assert_abs_diff_eq!(sqrt(4.0_f32), 2.0);
    assert_relative_eq!(sqrt(f32::MAX), 1.844_674_4e19);
    assert_eq!(sqrt(f32::INFINITY), f32::INFINITY);
}

