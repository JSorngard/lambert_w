// Copyright 2025 Johanna Sörngård
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This module contains an implementation of the approximation of the secondary
//! branch of the Lambert W function
//! with 24 bits of accuracy from Fukushima's paper.
//! It returns [`f32::NAN`] if the input is smaller than -1/e, is `NAN`, or is larger than 0.

use crate::generic_math::{ln, rational_function, sqrt};

const INV_SQRT_E: f32 = super::INV_SQRT_E as f32;
const NEG_INV_E: f32 = super::NEG_INV_E as f32;

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn swm1f(z: f32) -> f32 {
    // The critical arguments and coefficients are the same as in the `swm1` module,
    // but their precision has been truncated to fit in 32-bit floats.

    if z < NEG_INV_E {
        f32::NAN
    } else if z == NEG_INV_E as f32 {
        -1.0
    } else if z <= -0.207_293_78 {
        // W >= -2.483, Y_-1

        rational_function(
            -z / (INV_SQRT_E + sqrt(z - NEG_INV_E)),
            [-6.383_723, -74.968_65, -19.714_82, 70.677_33],
            [1.0, 24.295_837, 64.112_46, 17.994_497],
        )
    } else if z <= -0.071_507_71 {
        // W >= -4.032, Y_-2

        rational_function(
            -z / (INV_SQRT_E + sqrt(z - NEG_INV_E)),
            [-7.723_328_6, -352.484_68, -1_242.008_9, 1_171.647_6],
            [1.0, 77.681_244, 648.564_33, 566.701_54],
        )
    } else if z <= -0.020_704_413 {
        // W >= -5.600, Y_-3

        rational_function(
            -z / (INV_SQRT_E + sqrt(z - NEG_INV_E)),
            [-9.137_773_5, -1_644.724_5, -28_105.096, 3_896.079_8],
            [1.0, 272.375_27, 7_929.224, 23_980.123],
        )
    } else if z <= -0.005_480_013 {
        // W >= -7.178, Y_-4

        rational_function(
            -z / (INV_SQRT_E + sqrt(z - NEG_INV_E)),
            [-10.603_388, -7_733.348_6, -575_482.44, -2.154_552_5e6],
            [1.0, 1_021.793_9, 111_300.23, 1.261_425_6e6],
        )
    } else if z <= -0.001_367_467 {
        // W >= -8.766, Y_-5

        rational_function(
            -z / (INV_SQRT_E + sqrt(z - NEG_INV_E)),
            [-12.108_699, -36_896.535, -1.183_112_7e7, -2.756_583e8],
            [1.0, 4_044.975_3, 1.741_827_8e6, 7.843_690_4e7],
        )
    } else if z <= -0.000_326_142_27 {
        // W >= -10.367, Y_-6

        rational_function(
            -z / (INV_SQRT_E + sqrt(z - NEG_INV_E)),
            [-13.646_762, -179_086.11, -2.508_463_5e8, -2.934_37e10],
            [1.0, 16_743.826, 2.980_965e7, 5.573_951_5e9],
        )
    } else if z <= -0.000_074_906_61 {
        // W >= -11.983, Y_-7

        rational_function(
            -z / (INV_SQRT_E + sqrt(z - NEG_INV_E)),
            [-15.212_958, -884_954.7, -5.529_815_6e9, -3.093_418_7e12],
            [1.0, 72_009.26, 5.505_901e8, 4.432_489_3e11],
        )
    } else if z <= -1.096_244_5e-19 {
        // W >= -47.518, V_-8

        rational_function(
            ln(-z),
            [-0.032_401_163, 2.028_194_2, -0.527_524_3, 0.017_340_295],
            [1.0, -0.450_042_75, 0.017_154_707, -5.243_819_6e-7],
        )
    } else if z < 0.0 {
        // W >= -317.993, V_-9

        rational_function(
            ln(-z),
            [-1.441_124_7, 1.281_927, -0.074_979_36, 0.000_476_363_1],
            [1.0, -0.072_000_876, 0.000_475_489_33, -4.171_498e-10],
        )
    } else if z == 0.0 {
        f32::NEG_INFINITY
    } else {
        f32::NAN
    }
}
