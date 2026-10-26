// Copyright 2024-2026 Johanna Sörngård
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This file contains tests of the public API of the crate.
//!
//! Every test function utilizes [`assert_abs_diff_eq!`] for as long as possible,
//! and then switches to [`assert_relative_eq!`] when the first assertion would fail.

use lambert_w::{
    lambert_w, lambert_w0, lambert_w0f, lambert_wf, lambert_wm1, lambert_wm1f, sp_lambert_w0,
    sp_lambert_wm1, NEG_INV_E, OMEGA,
};

use approx::{assert_abs_diff_eq, assert_relative_eq};

use rand::{distr::Uniform, rngs::SmallRng, RngExt, SeedableRng};

const RANDOM_TEST_SIZE: usize = 1_000_000;

#[test]
fn test_lambert_w0() {
    assert!(lambert_w0(NEG_INV_E - f64::EPSILON).is_nan());
    assert!(lambert_w0(f64::NAN).is_nan());
    assert_eq!(lambert_w0(NEG_INV_E), -1.0);
    assert_abs_diff_eq!(
        lambert_w0(NEG_INV_E + f64::EPSILON),
        -0.999_999_965_255_797_6
    );
    assert_abs_diff_eq!(
        lambert_w0(-2.678_794_411_714_424e-1),
        -3.993_824_525_397_807e-1
    );
    assert_eq!(lambert_w0(0.0), 0.0);
    assert_abs_diff_eq!(
        lambert_w0(6.321_205_588_285_577e-1),
        4.167_039_988_177_658e-1
    );
    assert_abs_diff_eq!(lambert_w0(1.0), OMEGA);
    assert_abs_diff_eq!(lambert_w0(9.632_120_558_828_557), 1.721_757_710_976_171);
    assert_abs_diff_eq!(lambert_w0(9.963_212_055_882_856e1), 3.382_785_211_058_958);
    assert_abs_diff_eq!(lambert_w0(9.996_321_205_588_285e2), 5.249_293_782_013_269);
    assert_relative_eq!(lambert_w0(9.999_632_120_558_828e3), 7.231_813_718_542_178);
    assert_relative_eq!(lambert_w0(9.999_963_212_055_883e4), 9.284_568_107_521_96);
    assert_relative_eq!(lambert_w0(9.999_996_321_205_589e5), 1.138_335_774_796_812e1);
    assert_relative_eq!(lambert_w0(9.999_999_632_120_559e6), 1.351_434_397_605_273e1);
    assert_relative_eq!(lambert_w0(9.999_999_963_212_056e7), 1.566_899_671_199_287e1);
    assert_relative_eq!(lambert_w0(9.999_999_996_321_206e8), 1.784_172_596_707_312e1);
    assert_relative_eq!(lambert_w0(9.999_999_999_632_12e9), 2.002_868_541_326_992e1);
    assert_relative_eq!(
        lambert_w0(9.999_999_999_963_213e10),
        2.222_712_273_495_755e1
    );
    assert_relative_eq!(
        lambert_w0(9.999_999_999_996_321e11),
        2.443_500_440_493_456e1
    );
    assert_relative_eq!(
        lambert_w0(9.999_999_999_999_633e12),
        2.665_078_750_870_219e1
    );
    assert_relative_eq!(lambert_w0(9.999_999_999_999_963e13), 2.887_327_487_929_93e1);
    assert_relative_eq!(
        lambert_w0(9.999_999_999_999_996e14),
        3.110_151_971_159_478e1
    );
    assert_relative_eq!(
        lambert_w0(1e15),
        3.110_151_971_159_477_699_1e1,
        max_relative = 1.1 * f64::EPSILON
    );
    assert_relative_eq!(lambert_w0(1e16), 3.333_476_076_844_818e1);
    assert_relative_eq!(lambert_w0(1e17), 3.557_237_716_651_325e1);
    assert_relative_eq!(lambert_w0(1e18), 3.781_385_607_558_877e1);
    assert_relative_eq!(lambert_w0(1e19), 4.005_876_916_198_432e1);
    assert_relative_eq!(lambert_w0(1e20), 4.230_675_509_173_839e1);
    assert_relative_eq!(lambert_w0(1e30), 6.490_463_377_004_612e1);
    assert_relative_eq!(lambert_w0(1e40), 8.763_027_715_194_72e1);
    assert_relative_eq!(lambert_w0(1e60), 1.332_627_825_918_033e2);
    assert_relative_eq!(lambert_w0(1e80), 1.790_193_137_415_062e2);
    assert_relative_eq!(lambert_w0(1e120), 2.707_091_661_024_979e2);
    assert_relative_eq!(lambert_w0(1e160), 3.625_205_337_614_976e2);
    assert_relative_eq!(
        lambert_w0(f64::MAX),
        703.227_033_104_770_2,
        max_relative = 1.5 * f64::EPSILON,
    );
    assert_eq!(lambert_w0(f64::INFINITY), f64::INFINITY);

    let mut rng = SmallRng::seed_from_u64(1);
    let range = Uniform::new(NEG_INV_E, f64::MAX).unwrap();
    for _ in 0..RANDOM_TEST_SIZE {
        assert!(lambert_w0(rng.sample(range)).is_finite());
    }
}

#[test]
fn test_sp_lambert_w0() {
    assert!(sp_lambert_w0(NEG_INV_E - f64::EPSILON).is_nan());
    assert!(sp_lambert_w0(f64::NAN).is_nan());
    assert_eq!(sp_lambert_w0(NEG_INV_E), -1.0);
    assert_abs_diff_eq!(
        sp_lambert_w0(-2.678_794_411_714_424e-1),
        -3.993_824_525_397_807e-1,
        epsilon = 1e-7
    );
    assert_eq!(sp_lambert_w0(0.0), 0.0);
    assert_abs_diff_eq!(
        sp_lambert_w0(6.321_205_588_285_577e-1),
        4.167_039_988_177_658e-1,
        epsilon = 1e-7,
    );
    assert_abs_diff_eq!(sp_lambert_w0(1.0), OMEGA, epsilon = 1e-7);
    assert_abs_diff_eq!(
        sp_lambert_w0(9.632120558828557),
        1.721_757_710_976_171,
        epsilon = 1e-7,
    );
    assert_abs_diff_eq!(
        sp_lambert_w0(9.963_212_055_882_856e1),
        3.382785211058958,
        epsilon = 1e-7,
    );
    assert_relative_eq!(
        sp_lambert_w0(9.996_321_205_588_285e2),
        5.249_293_782_013_269,
        max_relative = 1e-7,
    );
    assert_relative_eq!(
        sp_lambert_w0(9.999_632_120_558_828e3),
        7.231_813_718_542_178,
        max_relative = 1e-7,
    );
    assert_relative_eq!(
        sp_lambert_w0(9.999_963_212_055_883e4),
        9.284_568_107_521_96,
        max_relative = 1e-7,
    );
    assert_relative_eq!(
        sp_lambert_w0(9.999_996_321_205_589e5),
        1.138_335_774_796_812e1,
        max_relative = 1e-7,
    );
    assert_relative_eq!(
        sp_lambert_w0(9.999_999_632_120_559e6),
        1.351_434_397_605_273e1,
        max_relative = 1e-7,
    );
    assert_relative_eq!(
        sp_lambert_w0(9.999_999_963_212_056e7),
        1.566_899_671_199_287e1,
        max_relative = 1e-7,
    );
    assert_relative_eq!(
        sp_lambert_w0(9.999_999_996_321_206e8),
        1.784_172_596_707_312e1,
        max_relative = 1e-7,
    );
    assert_relative_eq!(
        sp_lambert_w0(9.999_999_999_632_12e9),
        2.002_868_541_326_992e1,
        max_relative = 1e-7,
    );
    assert_relative_eq!(
        sp_lambert_w0(9.999_999_999_963_213e10),
        2.222_712_273_495_755e1,
        max_relative = 1e-7,
    );
    assert_relative_eq!(
        sp_lambert_w0(9.999_999_999_996_321e11),
        2.443_500_440_493_456e1,
        max_relative = 1e-7,
    );
    assert_relative_eq!(
        sp_lambert_w0(9.999_999_999_999_633e12),
        2.665_078_750_870_219e1,
        max_relative = 1e-7,
    );
    assert_relative_eq!(
        sp_lambert_w0(9.999_999_999_999_963e13),
        2.887_327_487_929_93e1,
        max_relative = 1e-7,
    );
    assert_relative_eq!(
        sp_lambert_w0(9.999_999_999_999_996e14),
        3.110_151_971_159_478e1,
        max_relative = 1e-7,
    );
    assert_relative_eq!(
        sp_lambert_w0(1e16),
        3.333_476_076_844_818e1,
        max_relative = 1e-7,
    );
    assert_relative_eq!(
        sp_lambert_w0(1e17),
        3.557_237_716_651_325e1,
        max_relative = 1e-7,
    );
    assert_relative_eq!(
        sp_lambert_w0(1e18),
        3.781_385_607_558_877e1,
        max_relative = 1e-7,
    );
    assert_relative_eq!(
        sp_lambert_w0(1e19),
        4.005_876_916_198_432e1,
        max_relative = 1e-7,
    );
    assert_relative_eq!(
        sp_lambert_w0(1e20),
        4.230_675_509_173_839e1,
        max_relative = 1e-7,
    );
    assert_relative_eq!(
        sp_lambert_w0(1e40),
        8.763_027_715_194_72e1,
        max_relative = 1e-7,
    );
    assert_relative_eq!(
        sp_lambert_w0(1e80),
        1.790_193_137_415_062e2,
        max_relative = 1e-7,
    );
    assert_relative_eq!(
        sp_lambert_w0(1e120),
        2.707_091_661_024_979e2,
        max_relative = 1e-7,
    );
    assert_relative_eq!(
        sp_lambert_w0(1e160),
        3.625_205_337_614_976e2,
        max_relative = 1e-7,
    );
    assert_relative_eq!(
        sp_lambert_w0(f64::MAX),
        703.227_033_104_770_2,
        max_relative = 1e-7,
    );
    assert_eq!(sp_lambert_w0(f64::INFINITY), f64::INFINITY);

    let mut rng = SmallRng::seed_from_u64(1);
    let range = Uniform::new(NEG_INV_E, f64::MAX).unwrap();
    for _ in 0..RANDOM_TEST_SIZE {
        assert!(sp_lambert_w0(rng.sample(range)).is_finite());
    }
}

#[test]
fn test_lambert_w0f() {
    assert!(lambert_w0f(NEG_INV_E as f32 - f32::EPSILON).is_nan());
    assert!(lambert_w0f(f32::NAN).is_nan());
    assert_eq!(lambert_w0f(NEG_INV_E as f32), -1.0);
    assert_abs_diff_eq!(lambert_w0f(-2.678_794_3e-1), -3.993_824_4e-1,);
    assert_eq!(lambert_w0f(0.0), 0.0);
    assert_abs_diff_eq!(lambert_w0f(6.321_205_5e-1), 4.167_04e-1);
    assert_abs_diff_eq!(lambert_w0f(9.632_12), 1.721_757_8);
    assert_relative_eq!(
        lambert_w0f(9.963_212e1),
        3.382_785_3,
        max_relative = 1.2 * f32::EPSILON
    );
    assert_relative_eq!(lambert_w0f(9.996_321_4e2), 5.249_294);
    assert_relative_eq!(lambert_w0f(9.999_632e3), 7.231_814);
    assert_relative_eq!(lambert_w0f(9.999_963e4), 9.284_568);
    assert_relative_eq!(lambert_w0f(9.999_996e5), 1.138_335_8e1);
    assert_relative_eq!(lambert_w0f(1e7), 1.351_434_4e1);
    assert_relative_eq!(lambert_w0f(1e8), 1.566_899_7e1);
    assert_relative_eq!(lambert_w0f(1e9), 1.784_172_6e1);
    assert_relative_eq!(lambert_w0f(1e10), 2.002_868_5e1);
    assert_relative_eq!(lambert_w0f(1e11), 2.222_712_3e1);
    assert_relative_eq!(
        lambert_w0f(1e12),
        2.443_500_5e1,
        max_relative = 1.35 * f32::EPSILON
    );
    assert_relative_eq!(lambert_w0f(1e13), 2.665_078_7e1);
    assert_relative_eq!(lambert_w0f(1e14), 2.887_327_6e1);
    assert_relative_eq!(lambert_w0f(1e15), 3.110_152e1);
    assert_relative_eq!(lambert_w0f(1e16), 3.333_476_3e1);
    assert_relative_eq!(lambert_w0f(1e17), 3.557_237_6e1);
    assert_relative_eq!(lambert_w0f(5.939_799_6e17), 3.730_645_559e1);
    assert_relative_eq!(lambert_w0f(5.939_799_6e17_f32.next_down()), 3.730_645_549e1);
    assert_relative_eq!(lambert_w0f(1e18), 3.781_385_4e1);
    assert_relative_eq!(lambert_w0f(1e19), 4.005_877e1);
    assert_relative_eq!(lambert_w0f(1e20), 4.230_675_5e1);
    assert_relative_eq!(lambert_w0f(f32::MAX), 84.288_59);
    assert_eq!(lambert_w0f(f32::INFINITY), f32::INFINITY);

    let mut rng = SmallRng::seed_from_u64(1);
    let range = Uniform::new(NEG_INV_E as f32, f32::MAX).unwrap();
    for _ in 0..RANDOM_TEST_SIZE {
        assert!(lambert_w0f(rng.sample(range)).is_finite());
    }
}

#[test]
fn test_lambert_wm1() {
    assert!(lambert_wm1(NEG_INV_E - f64::EPSILON).is_nan());
    assert!(lambert_wm1(f64::NAN).is_nan());
    assert_eq!(lambert_wm1(NEG_INV_E), -1.0);
    assert_relative_eq!(
        lambert_wm1(-3.578_794_411_714_423e-1),
        -1.253_493_791_367_214,
        max_relative = 1.6 * f64::EPSILON,
    );
    assert_relative_eq!(
        lambert_wm1(-2.678_794_411_714_424e-1),
        -2.020_625_228_775_403,
    );
    assert_relative_eq!(lambert_wm1(-1e-1), -3.577_152_063_957_297);
    assert_relative_eq!(lambert_wm1(-3e-2), -5.144_482_721_515_681);
    assert_relative_eq!(
        lambert_wm1(-1e-2),
        -6.472_775_124_394_005,
        max_relative = 1.9 * f64::EPSILON
    );
    assert_relative_eq!(
        lambert_wm1(-3e-3),
        -7.872_521_380_098_709,
        max_relative = 1.02 * f64::EPSILON
    );
    assert_relative_eq!(lambert_wm1(-1e-3), -9.118_006_470_402_742);
    assert_relative_eq!(
        lambert_wm1(-3.000_000_000_000_001e-4),
        -1.045_921_112_040_1e1,
        max_relative = 1.53 * f64::EPSILON
    );
    assert_relative_eq!(
        lambert_wm1(-1e-4),
        -1.166_711_453_256_636e1,
        max_relative = 2.1 * f64::EPSILON
    );
    assert_relative_eq!(
        lambert_wm1(-3e-5),
        -1.297_753_279_184_081e1,
        max_relative = 1.9 * f64::EPSILON
    );
    assert_relative_eq!(
        lambert_wm1(-1e-5),
        -1.416_360_081_581_018e1,
        max_relative = 1.7 * f64::EPSILON
    );
    assert_relative_eq!(
        lambert_wm1(-1.000_000_000_000_004e-75),
        -1.778_749_628_219_512e2,
    );
    assert_relative_eq!(
        lambert_wm1(-1.000_000_000_000_008e-145),
        -3.397_029_099_254_29e2
    );
    assert_eq!(lambert_wm1(0.0), f64::NEG_INFINITY);
    assert!(lambert_wm1(f64::EPSILON).is_nan());
    assert!(lambert_wm1(f64::INFINITY).is_nan());

    let mut rng = SmallRng::seed_from_u64(1);
    let range = Uniform::new_inclusive(NEG_INV_E, 0.0).unwrap();
    for _ in 0..RANDOM_TEST_SIZE {
        assert!(lambert_wm1(rng.sample(range)).is_finite());
    }
}

#[test]
fn test_sp_lambert_wm1() {
    assert!(sp_lambert_wm1(NEG_INV_E - f64::EPSILON).is_nan());
    assert!(sp_lambert_wm1(f64::NAN).is_nan());
    assert_eq!(sp_lambert_wm1(NEG_INV_E), -1.0);
    assert_relative_eq!(
        sp_lambert_wm1(-3.578_794_411_714_423e-1),
        -1.253_493_791_367_214,
        max_relative = 1e-7
    );
    assert_relative_eq!(
        sp_lambert_wm1(-2.678_794_411_714_424e-1),
        -2.020_625_228_775_403,
        max_relative = 1e-7,
    );
    assert_relative_eq!(
        sp_lambert_wm1(-1e-1),
        -3.577_152_063_957_297,
        max_relative = 1e-9
    );
    assert_relative_eq!(
        sp_lambert_wm1(-3e-2),
        -5.144_482_721_515_681,
        max_relative = 1e-9
    );
    assert_relative_eq!(
        sp_lambert_wm1(-1e-2),
        -6.472_775_124_394_005,
        max_relative = 1e-7
    );
    assert_relative_eq!(
        sp_lambert_wm1(-3e-3),
        -7.872_521_380_098_709,
        max_relative = 1e-7
    );
    assert_relative_eq!(
        sp_lambert_wm1(-1e-3),
        -9.118_006_470_402_742,
        max_relative = 1e-7
    );
    assert_relative_eq!(
        sp_lambert_wm1(-3.000_000_000_000_001e-4),
        -1.045_921_112_040_1e1,
        max_relative = 1e-7
    );
    assert_relative_eq!(
        sp_lambert_wm1(-1e-4),
        -1.166_711_453_256_636e1,
        max_relative = 1e-7
    );
    assert_relative_eq!(
        sp_lambert_wm1(-3e-5),
        -1.297_753_279_184_081e1,
        max_relative = 1e-7
    );
    assert_relative_eq!(
        sp_lambert_wm1(-1e-5),
        -1.416_360_081_581_018e1,
        max_relative = 1e-7
    );
    assert_relative_eq!(
        sp_lambert_wm1(-1.000_000_000_000_004e-75),
        -1.778_749_628_219_512e2,
        max_relative = 1e-7
    );
    assert_relative_eq!(
        sp_lambert_wm1(-1.000_000_000_000_008e-145),
        -3.397_029_099_254_29e2,
        max_relative = 1e-7
    );
    assert_eq!(sp_lambert_wm1(0.0), f64::NEG_INFINITY);
    assert!(sp_lambert_wm1(f64::EPSILON).is_nan());
    assert!(sp_lambert_wm1(f64::INFINITY).is_nan());

    let mut rng = SmallRng::seed_from_u64(1);
    let range = Uniform::new_inclusive(NEG_INV_E, 0.0).unwrap();
    for _ in 0..RANDOM_TEST_SIZE {
        assert!(sp_lambert_wm1(rng.sample(range)).is_finite());
    }
}

#[test]
fn test_lambert_wm1f() {
    assert!(lambert_wm1f(NEG_INV_E as f32 - f32::EPSILON).is_nan());
    assert!(lambert_wm1f(f32::NAN).is_nan());
    assert_eq!(lambert_wm1f(NEG_INV_E as f32), -1.0);
    assert_relative_eq!(
        lambert_wm1f(-3.578_794_3e-1),
        -1.253_493_8,
        max_relative = 1.6 * f32::EPSILON
    );
    assert_relative_eq!(lambert_wm1f(-2.678_794_3e-1), -2.020_625);
    assert_relative_eq!(
        lambert_wm1f(-1e-1),
        -3.577_152,
        max_relative = 1.2 * f32::EPSILON
    );
    assert_relative_eq!(lambert_wm1f(-3e-2), -5.144_482_6);
    assert_relative_eq!(
        lambert_wm1f(-1e-2),
        -6.472_775,
        max_relative = 1.3 * f32::EPSILON
    );
    assert_relative_eq!(lambert_wm1f(-3e-3), -7.872_521_4);
    assert_relative_eq!(lambert_wm1f(-1e-3), -9.118_007);
    assert_relative_eq!(lambert_wm1f(-3e-4), -1.045_921_1e1);
    assert_relative_eq!(lambert_wm1f(-1e-4), -1.166_711_4e1);
    assert_relative_eq!(lambert_wm1f(-3e-5), -1.297_753_2e1);
    assert_relative_eq!(lambert_wm1f(-1e-5), -1.416_360_1e1);
    assert_relative_eq!(lambert_wm1f(-1e-20), -49.962_986);
    assert_eq!(lambert_wm1f(0.0), f32::NEG_INFINITY);
    assert!(lambert_wm1f(f32::EPSILON).is_nan());
    assert!(lambert_wm1f(f32::INFINITY).is_nan());

    let mut rng = SmallRng::seed_from_u64(1);
    let range = Uniform::new_inclusive(NEG_INV_E as f32, 0.0).unwrap();
    for _ in 0..RANDOM_TEST_SIZE {
        assert!(lambert_wm1f(rng.sample(range)).is_finite());
    }
}

macro_rules! assert_complex_approx_eq {
    ($left:expr, $right:expr) => {
        let left = $left;
        let right = $right;
        assert_abs_diff_eq!(left.0, right.0);
        assert_abs_diff_eq!(left.1, right.1);
    };
    ($left:expr, $right:expr, epsilon = $prec:expr) => {
        let left = $left;
        let right = $right;
        let prec = $prec;
        assert_abs_diff_eq!(left.0, right.0, epsilon = prec);
        assert_abs_diff_eq!(left.1, right.1, epsilon = prec);
    };

    ($left:expr, $right:expr, max_relative = $prec:expr) => {
        let left = $left;
        let right = $right;
        let prec = $prec;
        assert_relative_eq!(left.0, right.0, max_relative = prec);
        assert_relative_eq!(left.1, right.1, max_relative = prec);
    };
}

#[test]
fn test_iterative_close_to_zero() {
    // Very close to zero on branches != 0 this function is much less accurate.
    let w = lambert_w(-1, -1e-80, 0.0, f64::EPSILON);
    assert_relative_eq!(w.0, -189.450_937_525_646_627_592, max_relative = 0.00001);
    assert_relative_eq!(w.1, 0.0, max_relative = 1.0);
}

#[test]
fn test_iterative_version() {
    let err_tol = f64::EPSILON;
    assert_eq!(lambert_w(0, NEG_INV_E, 0.0, err_tol), (-1.0, 0.0));
    assert_eq!(lambert_w(0, 1.0, 0.0, err_tol), (OMEGA, 0.0));
    assert_eq!(lambert_w(0, core::f64::consts::E, 0.0, err_tol), (1.0, 0.0));
    assert_eq!(lambert_w(0, 2.0, 0.0, err_tol), (0.8526055020137255, 0.0));
    assert_eq!(lambert_w(0, 0.0, 0.0, err_tol), (0.0, 0.0));
    assert_eq!(lambert_w(1, 0.0, 0.0, err_tol), (f64::NEG_INFINITY, 0.0));

    for err_tol_pow in 1..10 {
        let err_tol = f64::powi(10.0, -err_tol_pow);
        assert!(
            (lambert_w(0, 10.0, 0.0, err_tol).0 - lambert_w(0, 10.0, 0.0, err_tol).0).abs()
                <= err_tol
        );
    }

    assert_complex_approx_eq!(
        lambert_w(0, NEG_INV_E + 0.1, 0.0, err_tol),
        (-0.399_382_452_539_780_7, 0.0)
    );
    assert_complex_approx_eq!(
        lambert_w(1, NEG_INV_E + 0.1, -1.0, err_tol),
        (-0.955_746_684_806_075_3, 2.516_952_771_719_245_7),
        epsilon = 2.0 * f64::EPSILON
    );
    assert_complex_approx_eq!(
        lambert_w(-1, NEG_INV_E + 0.1, 1.0, err_tol),
        (-0.955_746_684_806_075_3, -2.5169527717192458),
        epsilon = 2.0 * f64::EPSILON
    );
    assert_complex_approx_eq!(
        lambert_w(-1, 0.5, 0.0, err_tol),
        (
            -2.259158898533606, //187
            -4.220_960_969_266_197
        )
    );
    assert_complex_approx_eq!(lambert_w(0, 10.0, 0.0, err_tol), (1.745528002740699, 0.0));
    assert_complex_approx_eq!(
        lambert_w(0, 100.0, 0.0, err_tol),
        (3.385_630_140_290_05, 0.0)
    );
    assert_complex_approx_eq!(
        lambert_w(0, 1000.0, 0.0, err_tol),
        (5.249_602_852_401_596, 0.0)
    );
    assert_complex_approx_eq!(
        lambert_w(0, 10000.0, 0.0, err_tol),
        (7.231_846_038_093_373, 0.0)
    );
    assert_complex_approx_eq!(
        lambert_w(-1, -f64::ln(2.0) / 2.0, 0.0, err_tol),
        (-f64::ln(4.0), 0.0)
    );
    // Close to the branch cut
    assert_complex_approx_eq!(lambert_w(-1, NEG_INV_E, 0.0, err_tol), (-1.0, 0.0));
    assert_complex_approx_eq!(
        lambert_w(10, NEG_INV_E + 0.1, 0.0, err_tol),
        (-5.484_673_997_441_509, 64.317_580_321_338_81)
    );
    assert_complex_approx_eq!(
        lambert_w(1, -1e-05, -1e-05, err_tol),
        (-1.37923465336253e+01, 8.46711143530535e-01),
        max_relative = 8.0 * f64::EPSILON
    );
    // Very big branch index
    assert_complex_approx_eq!(
        lambert_w(1_000_000, 100.0, 100.0, err_tol),
        (-10.701_643_723_106_727, 6.283_184_521_779_72e6)
    );
    // NaNs
    assert!(lambert_w(0, f64::NAN, 0.0, err_tol).0.is_nan());
    assert!(lambert_w(0, f64::NAN, 0.0, err_tol).1.is_nan());
    assert!(lambert_w(0, 0.0, f64::NAN, err_tol).0.is_nan());
    assert!(lambert_w(0, 0.0, f64::NAN, err_tol).1.is_nan());
    assert!(lambert_w(0, f64::NAN, f64::NAN, err_tol).0.is_nan());
    assert!(lambert_w(0, f64::NAN, f64::NAN, err_tol).1.is_nan());
    assert!(lambert_w(0, 0.0, 0.0, f64::NAN).0.is_nan());
    assert!(lambert_w(0, 0.0, 0.0, f64::NAN).1.is_nan());
    // Infinity
    assert!(lambert_w(0, f64::INFINITY, 0.0, err_tol).0.is_nan());
    assert!(lambert_w(0, f64::INFINITY, 0.0, err_tol).1.is_nan());
    assert!(lambert_w(0, 0.0, f64::INFINITY, err_tol).0.is_nan());
    assert!(lambert_w(0, 0.0, f64::INFINITY, err_tol).1.is_nan());
    assert!(lambert_w(0, f64::INFINITY, f64::INFINITY, err_tol)
        .0
        .is_nan());
    assert!(lambert_w(0, f64::INFINITY, f64::INFINITY, err_tol)
        .1
        .is_nan());
}

#[test]
fn test_32_bit_iterative_version() {
    let err_tol = f32::EPSILON;
    assert_eq!(lambert_wf(0, NEG_INV_E as f32, 0.0, err_tol), (-1.0, 0.0));
    assert_eq!(lambert_wf(0, 1.0, 0.0, err_tol), (OMEGA as f32, 0.0));
    assert_eq!(
        lambert_wf(0, core::f32::consts::E, 0.0, err_tol),
        (1.0, 0.0)
    );
    assert_eq!(lambert_wf(0, 2.0, 0.0, err_tol), (0.852_605_5, 0.0));
    assert_eq!(lambert_wf(0, 0.0, 0.0, err_tol), (0.0, 0.0));
    assert_eq!(lambert_wf(1, 0.0, 0.0, err_tol), (f32::NEG_INFINITY, 0.0));

    for err_tol_pow in 1..10 {
        let err_tol = f32::powi(10.0, -err_tol_pow);
        assert!(
            (lambert_wf(0, 10.0, 0.0, err_tol).0 - lambert_wf(0, 10.0, 0.0, err_tol).0).abs()
                <= err_tol
        );
    }

    assert_complex_approx_eq!(
        lambert_wf(0, NEG_INV_E as f32 + 0.1, 0.0, err_tol),
        (-0.399_382_44, 0.0)
    );
    assert_complex_approx_eq!(
        lambert_wf(1, NEG_INV_E as f32 + 0.1, -1.0, err_tol),
        (-0.955_746_7, 2.516_952_8)
    );
    assert_complex_approx_eq!(
        lambert_wf(-1, NEG_INV_E as f32 + 0.1, 1.0, err_tol),
        (-0.955_746_7, -2.516_952_8)
    );
    assert_complex_approx_eq!(
        lambert_wf(-1, 0.5, 0.0, err_tol),
        (-2.259_158_8, -4.220_961)
    );
    assert_complex_approx_eq!(lambert_wf(0, 10.0, 0.0, err_tol), (1.745_528, 0.0));
    assert_complex_approx_eq!(lambert_wf(0, 100.0, 0.0, err_tol), (3.385_630_1, 0.0));
    assert_complex_approx_eq!(lambert_wf(0, 1000.0, 0.0, err_tol), (5.249_603, 0.0));
    assert_complex_approx_eq!(lambert_wf(0, 10000.0, 0.0, err_tol), (7.231_846, 0.0));
    assert_complex_approx_eq!(
        lambert_wf(-1, -f32::ln(2.0) / 2.0, 0.0, err_tol),
        (-f32::ln(4.0), 0.0)
    );
    // Close to the branch cut
    assert_complex_approx_eq!(lambert_wf(-1, NEG_INV_E as f32, 0.0, err_tol), (-1.0, 0.0));
    assert_complex_approx_eq!(
        lambert_wf(10, NEG_INV_E as f32 + 0.1, 0.0, err_tol),
        (-5.484_674, 64.317_58)
    );
    // Very big branch index
    assert_complex_approx_eq!(
        lambert_wf(32767, 100.0, 100.0, err_tol),
        (-7.2833066, 205880.34)
    );
    // NaNs
    assert!(lambert_wf(0, f32::NAN, 0.0, err_tol).0.is_nan());
    assert!(lambert_wf(0, f32::NAN, 0.0, err_tol).1.is_nan());
    assert!(lambert_wf(0, 0.0, f32::NAN, err_tol).0.is_nan());
    assert!(lambert_wf(0, 0.0, f32::NAN, err_tol).1.is_nan());
    assert!(lambert_wf(0, f32::NAN, f32::NAN, err_tol).0.is_nan());
    assert!(lambert_wf(0, f32::NAN, f32::NAN, err_tol).1.is_nan());
    assert!(lambert_wf(0, 0.0, 0.0, f32::NAN).0.is_nan());
    assert!(lambert_wf(0, 0.0, 0.0, f32::NAN).1.is_nan());
    // Infinity
    assert!(lambert_wf(0, f32::INFINITY, 0.0, err_tol).0.is_nan());
    assert!(lambert_wf(0, f32::INFINITY, 0.0, err_tol).1.is_nan());
    assert!(lambert_wf(0, 0.0, f32::INFINITY, err_tol).0.is_nan());
    assert!(lambert_wf(0, 0.0, f32::INFINITY, err_tol).1.is_nan());
    assert!(lambert_wf(0, f32::INFINITY, f32::INFINITY, err_tol)
        .0
        .is_nan());
    assert!(lambert_wf(0, f32::INFINITY, f32::INFINITY, err_tol)
        .1
        .is_nan());
}

#[test]
fn principal_branches_match_high_precision_near_zero() {
    for (x, expected) in W0_NEAR_ZERO_ANSWERS {
        let actual = lambert_w0(x);
        assert_eq!(actual.is_sign_negative(), x.is_sign_negative());
        assert_relative_eq!(actual, expected);

        let actual = sp_lambert_w0(x);
        assert_eq!(actual.is_sign_negative(), x.is_sign_negative());
        assert_relative_eq!(
            actual,
            expected,
            max_relative = f64::from(f32::EPSILON) / 2.0
        );
    }

    for (x, expected) in W0F_NEAR_ZERO_ANSWERS {
        let actual = lambert_w0f(x);
        assert_eq!(actual.is_sign_negative(), x.is_sign_negative());
        assert_relative_eq!(actual, expected);
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

static W0_NEAR_ZERO_ANSWERS: [(f64, f64); 64] = [
    (-0.05000000000000001, -0.05270598355154636),
    (-0.05, -0.05270598355154635),
    (-0.049999999999999996, -0.05270598355154634),
    (-0.049, -0.051594484792194056),
    (-0.046, -0.0482751302674623),
    (-0.043, -0.04497821826162204),
    (-0.04, -0.04170340843648448),
    (-0.037, -0.03845036855451688),
    (-0.034, -0.03521877421614823),
    (-0.031, -0.03200830860783763),
    (-0.028, -0.028818662260374234),
    (-0.025, -0.025649532816907288),
    (-0.022, -0.02250062481023407),
    (-0.019, -0.019371649448900245),
    (-0.016, -0.016262324411691897),
    (-0.013, -0.013172373650122379),
    (-0.01, -0.010101527198538754),
    (-0.007, -0.0070495209914932635),
    (-0.004, -0.004016096688044622),
    (-0.001, -0.001001001502671886),
    (-0.0001, -0.00010001000150026672),
    (-1e-06, -1.0000010000014999e-06),
    (-1e-08, -1.0000000100000002e-08),
    (-1e-12, -1.000000000001e-12),
    (-1e-20, -1e-20),
    (-1e-30, -1e-30),
    (-1e-100, -1e-100),
    (-1e-300, -1e-300),
    (-2.2250738585072014e-308, -2.2250738585072014e-308),
    (-2.225073858507201e-308, -2.225073858507201e-308),
    (-1e-323, -1e-323),
    (-5e-324, -5e-324),
    (5e-324, 5e-324),
    (1e-323, 1e-323),
    (2.225073858507201e-308, 2.225073858507201e-308),
    (2.2250738585072014e-308, 2.2250738585072014e-308),
    (1e-300, 1e-300),
    (1e-100, 1e-100),
    (1e-30, 1e-30),
    (1e-20, 1e-20),
    (1e-12, 9.99999999999e-13),
    (1e-08, 9.999999900000002e-09),
    (1e-06, 9.999990000015e-07),
    (0.0001, 9.999000149973339e-05),
    (0.001, 0.0009990014973385308),
    (0.004, 0.003984095322622809),
    (0.007, 0.0069515081836181115),
    (0.01, 0.009901473843595012),
    (0.013, 0.012834221220445039),
    (0.016, 0.015749974523523304),
    (0.019, 0.018648953385605776),
    (0.022, 0.02153137299017165),
    (0.025, 0.02439744419421069),
    (0.028, 0.02724737364674536),
    (0.031, 0.030081363903248264),
    (0.034, 0.03289961353612636),
    (0.037, 0.03570231724143523),
    (0.04, 0.03848966594197857),
    (0.043, 0.04126184688694076),
    (0.046, 0.04401904374819318),
    (0.049, 0.04676143671340832),
    (0.049999999999999996, 0.04767230860012937),
    (0.05, 0.04767230860012938),
    (0.05000000000000001, 0.04767230860012938),
];
static W0F_NEAR_ZERO_ANSWERS: [(f32, f32); 60] = [
    (-0.05000000447034836, -0.05270598828792572),
    (-0.05000000074505806, -0.05270598456263542),
    (-0.04999999701976776, -0.05270598083734512),
    (-0.04899999871850014, -0.051594484597444534),
    (-0.04600000008940697, -0.04827513173222542),
    (-0.0430000014603138, -0.044978220015764236),
    (-0.03999999910593033, -0.04170340672135353),
    (-0.03700000047683716, -0.038450367748737335),
    (-0.03400000184774399, -0.035218775272369385),
    (-0.03099999949336052, -0.03200830891728401),
    (-0.02800000086426735, -0.028818663209676743),
    (-0.02500000037252903, -0.025649532675743103),
    (-0.02199999988079071, -0.022500624880194664),
    (-0.01899999938905239, -0.019371649250388145),
    (-0.01600000075995922, -0.016262324526906013),
    (-0.013000000268220901, -0.013172374106943607),
    (-0.009999999776482582, -0.010101526975631714),
    (-0.007000000216066837, -0.007049521431326866),
    (-0.004000000189989805, -0.004016096703708172),
    (-0.0010000000474974513, -0.0010010015685111284),
    (-9.999999747378752e-05, -0.00010001000191550702),
    (-9.999999974752427e-07, -1.0000010206567822e-06),
    (-9.99999993922529e-09, -9.99999993922529e-09),
    (-9.999999960041972e-13, -9.999999960041972e-13),
    (-9.999999682655225e-21, -9.999999682655225e-21),
    (-1.0000000031710769e-30, -1.0000000031710769e-30),
    (-1.1754943508222875e-38, -1.1754943508222875e-38),
    (-1.1754942106924411e-38, -1.1754942106924411e-38),
    (-2.802596928649634e-45, -2.802596928649634e-45),
    (-1.401298464324817e-45, -1.401298464324817e-45),
    (1.401298464324817e-45, 1.401298464324817e-45),
    (2.802596928649634e-45, 2.802596928649634e-45),
    (1.1754942106924411e-38, 1.1754942106924411e-38),
    (1.1754943508222875e-38, 1.1754943508222875e-38),
    (1.0000000031710769e-30, 1.0000000031710769e-30),
    (9.999999682655225e-21, 9.999999682655225e-21),
    (9.999999960041972e-13, 9.999999960041972e-13),
    (9.99999993922529e-09, 9.99999993922529e-09),
    (9.999999974752427e-07, 9.999989742937032e-07),
    (9.999999747378752e-05, 9.999000030802563e-05),
    (0.0010000000474974513, 0.0009990015532821417),
    (0.004000000189989805, 0.003984095528721809),
    (0.007000000216066837, 0.006951508577913046),
    (0.009999999776482582, 0.009901473298668861),
    (0.013000000268220901, 0.012834221124649048),
    (0.01600000075995922, 0.0157499760389328),
    (0.01899999938905239, 0.01864895224571228),
    (0.02199999988079071, 0.021531373262405396),
    (0.02500000037252903, 0.02439744397997856),
    (0.02800000086426735, 0.02724737487733364),
    (0.03099999949336052, 0.030081363394856453),
    (0.03400000184774399, 0.03289961442351341),
    (0.03700000047683716, 0.03570231795310974),
    (0.03999999910593033, 0.03848966583609581),
    (0.0430000014603138, 0.04126184806227684),
    (0.04600000008940697, 0.04401904344558716),
    (0.04899999871850014, 0.04676143452525139),
    (0.04999999701976776, 0.04767230525612831),
    (0.05000000074505806, 0.04767230898141861),
    (0.05000000447034836, 0.04767231270670891),
];
