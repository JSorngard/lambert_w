// Copyright 2025 Johanna Sörngård
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This module contains an implementation of the approximation of the principal
//! branch of the Lambert W function
//! with 24 bits of accuracy from Fukushima's paper.
//! It returns [`f32::NAN`] if the input is negative or `NAN`,
//! and [`f32::INFINITY`] if the input is positive infinity.

use crate::generic_math::{ln, rational_function, sqrt};

const NEG_INV_E: f32 = super::NEG_INV_E as f32;

#[inline(always)]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn sw0f(z: f32) -> f32 {
    if z < NEG_INV_E || z.is_nan() {
        f32::NAN
    } else if z <= 2.008_217_8 {
        // W <= 0.854, X_1

        rational_function(
            sqrt(z - NEG_INV_E),
            [
                -0.999_999_94,
                0.055_730_052,
                2.126_973_2,
                0.813_511_25,
                0.016_324_88,
            ],
            [1.0, 2.275_906_6, 1.367_597, 0.186_158_24],
        )
    } else if z <= 30.539_143 {
        // W <= 2.502, X_2

        rational_function(
            sqrt(z - NEG_INV_E),
            [-0.985_519_7, 1.077_497_6, 0.871_751, 0.054_352_727],
            [1.0, 1.186_101_4, 0.249_962_99, 0.006_881_369],
        )
    } else if z <= 371.669_83 {
        // W <= 4.430, X_3

        rational_function(
            sqrt(z - NEG_INV_E),
            [-0.762_397_1, 1.231_773_1, 0.243_424_48, 0.004_320_601_5],
            [1.0, 0.579_386_23, 0.046_601_43, 0.000_435_128_17],
        )
    } else if z <= 4_705.919 {
        // W <= 6.574, X_4

        rational_function(
            sqrt(z - NEG_INV_E),
            [0.085_801_244, 0.825_397_97, 0.039_781_96, 0.000_187_855_8],
            [1.0, 0.213_380_77, 0.005_462_672, 0.000_015_449_534],
        )
    } else if z <= 64_640.797 {
        // W <= 8.892, X_5

        rational_function(
            sqrt(z - NEG_INV_E),
            [1.621_924_5, 0.388_691_46, 0.004_575_064_4, 5.538_467e-6],
            [1.0, 0.065_219_46, 0.000_478_827_6, 3.809_482_8e-7],
        )
    } else if z <= 965_649.030_871_163_2 {
        // W <= 11.351, X_6

        rational_function(
            sqrt(z - NEG_INV_E),
            [3.621_899_6, 0.148_846_46, 0.000_424_696_22, 1.279_018e-7],
            [1.0, 0.017_985_659, 0.000_035_446_45, 7.506_249e-9],
        )
    } else if z <= 1.559_333_4e7 {
        // W <= 13.928, X_7

        rational_function(
            sqrt(z - NEG_INV_E),
            [5.907_337, 0.050_053_652, 0.000_034_072_15, 2.481_206_6e-9],
            [1.0, 0.004_655_899, 2.344_944_5e-6, 1.263_143e-10],
        )
    } else if z <= 2.702_564_2e8 {
        // W <= 16.605, X_8

        rational_function(
            sqrt(z - NEG_INV_E),
            [8.382_601, 0.015_360_346, 2.443_338_4e-6, 4.185_680_3e-11],
            [1.0, 0.001_150_742_3, 1.422_142_9e-7, 1.873_917_3e-12],
        )
    } else if z <= 4.995_019e9 {
        // W <= 19.368, X_9

        rational_function(
            sqrt(z - NEG_INV_E),
            [10.996_675, 0.004_394_213_7, 1.596_666_6e-7, 6.266_538_4e-13],
            [1.0, 0.000_273_837_56, 8.015_706e-9, 2.495_698_2e-14],
        )
    } else if z <= 9.791_115e10 {
        // W <= 22.207, X_10

        rational_function(
            sqrt(z - NEG_INV_E),
            [13.719_833, 0.001_187_444_4, 9.630_338e-9, 8.443_452e-15],
            [1.0, 0.000_063_056_37, 4.235_876_6e-10, 3.020_540_4e-16],
        )
    } else if z <= 2.025_975_4e12 {
        // W <= 25.114, X_11

        rational_function(
            sqrt(z - NEG_INV_E),
            [16.533_12, 0.000_305_831_26, 5.411_295e-10, 1.034_713e-16],
            [1.0, 0.000_014_099_161_5, 2.112_109_6e-11, 3.352_692_7e-18],
        )
    } else if z <= 4.407_744_6e13 {
        // W <= 28.082, X_12

        rational_function(
            sqrt(z - NEG_INV_E),
            [
                19.423_52,
                0.000_075_559_27,
                2.853_002_4e-11,
                1.162_962_7e-18,
            ],
            [1.0, 3.069_209_2e-6, 9.986_661e-13, 3.437_671_8e-20],
        )
    } else if z <= 1.004_838_2e15 {
        // W <= 31.106, X_13

        rational_function(
            sqrt(z - NEG_INV_E),
            [
                22.381_577,
                0.000_017_994_724,
                1.419_487_7e-12,
                1.207_110_5e-20,
            ],
            [1.0, 6.518_396e-7, 4.495_866_6e-14, 3.275_542_8e-22],
        )
    } else if z <= 2.393_255_2e16 {
        // W <= 34.182, X_14

        rational_function(
            sqrt(z - NEG_INV_E),
            [25.400_105, 4.146_738e-6, 6.696_27e-14, 1.163_790_5e-22],
            [1.0, 1.352_980_1e-7, 1.933_608e-15, 2.914_939_7e-24],
        )
    } else if z <= 5.939_799_6e17 {
        // W <= 37.306, X_15

        rational_function(
            sqrt(z - NEG_INV_E),
            [28.473_455, 9.274_682_5e-7, 3.006_899e-15, 1.047_355_7e-24],
            [1.0, 2.748_649e-8, 7.967_898_6e-17, 2.433_166_6e-26],
        )
    } else if z <= 1.532_693_8e19 {
        // W <= 40.475, X_16

        rational_function(
            sqrt(z - NEG_INV_E),
            [31.597_055, 2.018_422_5e-7, 1.289_578_8e-16, 8.836_117e-27],
            [1.0, 5.472_394_5e-9, 3.153_773e-18, 1.912_203_5e-28],
        )
    } else if z <= 4.103_566e20 {
        // W <= 43.687, X_17

        rational_function(
            sqrt(z - NEG_INV_E),
            [34.767_124, 4.283_08e-8, 5.297_588_5e-18, 7.014_551_6e-29],
            [1.0, 1.068_930_2e-9, 1.201_67e-19, 1.419_524_4e-30],
        )
    } else if z < f32::INFINITY {
        // W <= 319.673, U_18

        rational_function(
            ln(z),
            [-0.607_023_7, 0.698_287_2, 0.075_795_14, 0.000_516_692_6],
            [1.0, 0.079_048_43, 0.000_517_609_94, -4.243_840_3e-10],
        )
    } else {
        f32::INFINITY
    }
}
