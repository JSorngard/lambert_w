//! This file contains tests of the public API of the crate.

use lambert_w::{
    lambert_w0, lambert_w0f, lambert_wm1, lambert_wm1f, sp_lambert_w0, sp_lambert_wm1, LambertW,
    NEG_INV_E, OMEGA,
};

use approx::{assert_abs_diff_eq, assert_relative_eq};

#[test]
fn test_lambert_w0() {
    assert!(lambert_w0(NEG_INV_E - f64::EPSILON).is_nan());
    assert_abs_diff_eq!(lambert_w0(NEG_INV_E), -1.0);
    assert_abs_diff_eq!(
        lambert_w0(NEG_INV_E + f64::EPSILON),
        -0.999_999_965_255_7976
    );
    assert_abs_diff_eq!(
        lambert_w0(-2.678_794_411_714_424e-1),
        -3.993_824_525_397_807e-1
    );
    assert_abs_diff_eq!(lambert_w0(1.0), OMEGA);
    assert_abs_diff_eq!(
        lambert_w0(6.321_205_588_285_577e-1),
        4.167_039_988_177_658e-1
    );
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
    assert_relative_eq!(lambert_w0(1e16), 3.333_476_076_844_818e1);
    assert_relative_eq!(lambert_w0(1e17), 3.557_237_716_651_325e1);
    assert_relative_eq!(lambert_w0(1e18), 3.781_385_607_558_877e1);
    assert_relative_eq!(lambert_w0(1e19), 4.005_876_916_198_432e1);
    assert_relative_eq!(lambert_w0(1e20), 4.230_675_509_173_839e1);
    assert_relative_eq!(lambert_w0(1e40), 8.763_027_715_194_72e1);
    assert_relative_eq!(lambert_w0(1e80), 1.790_193_137_415_062e2);
    assert_relative_eq!(lambert_w0(1e120), 2.707_091_661_024_979e2);
    assert_relative_eq!(lambert_w0(1e160), 3.625_205_337_614_976e2);
    assert_relative_eq!(
        lambert_w0(f64::MAX),
        703.227_033_104_770_2,
        max_relative = 4e-16
    );
}

#[test]
fn test_sp_lambert_w0() {
    assert!(sp_lambert_w0(NEG_INV_E - f64::EPSILON).is_nan());
    assert_abs_diff_eq!(sp_lambert_w0(NEG_INV_E), -1.0, epsilon = 1e-7);
    assert_abs_diff_eq!(
        sp_lambert_w0(-2.678_794_411_714_424e-1),
        -3.993_824_525_397_807e-1,
        epsilon = 1e-7
    );
    assert_abs_diff_eq!(sp_lambert_w0(1.0), OMEGA, epsilon = 1e-7);
    assert_abs_diff_eq!(
        sp_lambert_w0(6.321_205_588_285_577e-1),
        4.167_039_988_177_658e-1,
        epsilon = 1e-7,
    );
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
}

#[test]
fn test_lambert_w0f() {
    assert!(lambert_w0f(NEG_INV_E as f32 - f32::EPSILON).is_nan());
    assert_abs_diff_eq!(lambert_w0f(NEG_INV_E as f32), -1.0);
    assert_abs_diff_eq!(lambert_w0f(-2.678_794_3e-1), -3.993_824_4e-1,);
    assert_abs_diff_eq!(lambert_w0f(6.321_205_5e-1), 4.167_04e-1);
    assert_abs_diff_eq!(lambert_w0f(9.632_12), 1.721_757_8);
    assert_relative_eq!(lambert_w0f(9.963_212e1), 3.382_785_3, max_relative = 1e-6);
    assert_relative_eq!(lambert_w0f(9.996_321_4e2), 5.249_294);
    assert_relative_eq!(lambert_w0f(9.999_632e3), 7.231_814);
    assert_relative_eq!(lambert_w0f(9.999_963e4), 9.284_568);
    assert_relative_eq!(lambert_w0f(9.999_996e5), 1.138_335_8e1);
    assert_relative_eq!(lambert_w0f(1e7), 1.351_434_4e1);
    assert_relative_eq!(lambert_w0f(1e8), 1.566_899_7e1);
    assert_relative_eq!(lambert_w0f(1e9), 1.784_172_6e1);
    assert_relative_eq!(lambert_w0f(1e10), 2.002_868_5e1);
    assert_relative_eq!(lambert_w0f(1e11), 2.222_712_3e1);
    assert_relative_eq!(lambert_w0f(1e12), 2.443_500_5e1, max_relative = 1e-6);
    assert_relative_eq!(lambert_w0f(1e13), 2.665_078_7e1);
    assert_relative_eq!(lambert_w0f(1e14), 2.887_327_6e1);
    assert_relative_eq!(lambert_w0f(1e15), 3.110_152e1);
    assert_relative_eq!(lambert_w0f(1e16), 3.333_476_3e1);
    assert_relative_eq!(lambert_w0f(1e17), 3.557_237_6e1);
    assert_relative_eq!(lambert_w0f(1e18), 3.781_385_4e1);
    assert_relative_eq!(lambert_w0f(1e19), 4.005_877e1);
    assert_relative_eq!(lambert_w0f(1e20), 4.230_675_5e1);
    assert_relative_eq!(lambert_w0f(f32::MAX), 84.288_59);
}

#[test]
fn test_lambert_wm1() {
    assert!(lambert_wm1(NEG_INV_E - f64::EPSILON).is_nan());
    assert_abs_diff_eq!(lambert_wm1(NEG_INV_E), -1.0);
    assert_relative_eq!(
        lambert_wm1(-3.578_794_411_714_423e-1),
        -1.253_493_791_367_214,
        max_relative = 4e-16
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
        max_relative = 5e-16
    );
    assert_relative_eq!(
        lambert_wm1(-3e-3),
        -7.872_521_380_098_709,
        max_relative = 5e-16
    );
    assert_relative_eq!(lambert_wm1(-1e-3), -9.118_006_470_402_742);
    assert_relative_eq!(
        lambert_wm1(-3.000_000_000_000_001e-4),
        -1.045_921_112_040_1e1,
        max_relative = 5e-16
    );
    assert_relative_eq!(
        lambert_wm1(-1e-4),
        -1.166_711_453_256_636e1,
        max_relative = 5e-16
    );
    assert_relative_eq!(
        lambert_wm1(-3e-5),
        -1.297_753_279_184_081e1,
        max_relative = 5e-16
    );
    assert_relative_eq!(
        lambert_wm1(-1e-5),
        -1.416_360_081_581_018e1,
        max_relative = 5e-16
    );
    assert_relative_eq!(
        lambert_wm1(-1.000_000_000_000_004e-75),
        -1.778_749_628_219_512e2,
    );
    assert_relative_eq!(
        lambert_wm1(-1.000_000_000_000_008e-145),
        -3.397_029_099_254_29e2
    );
    assert!(lambert_wm1(f64::EPSILON).is_nan());
}

#[test]
fn test_sp_lambert_wm1() {
    assert!(sp_lambert_wm1(NEG_INV_E - f64::EPSILON).is_nan());
    assert_relative_eq!(sp_lambert_wm1(NEG_INV_E), -1.0, max_relative = 1e-7);
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
    assert!(sp_lambert_wm1(f64::EPSILON).is_nan());
}

#[test]
fn test_lambert_wm1f() {
    assert!(lambert_wm1f(NEG_INV_E as f32 - f32::EPSILON).is_nan());
    assert_abs_diff_eq!(lambert_wm1f(NEG_INV_E as f32), -1.0);
    assert_relative_eq!(
        lambert_wm1f(-3.578_794_3e-1),
        -1.253_493_8,
        max_relative = 1e-6
    );
    assert_relative_eq!(lambert_wm1f(-2.678_794_3e-1), -2.020_625);
    assert_relative_eq!(lambert_wm1f(-1e-1), -3.577_152, max_relative = 1e-6);
    assert_relative_eq!(lambert_wm1f(-3e-2), -5.144_482_6);
    assert_relative_eq!(lambert_wm1f(-1e-2), -6.472_775, max_relative = 1e-6);
    assert_relative_eq!(lambert_wm1f(-3e-3), -7.872_521_4);
    assert_relative_eq!(lambert_wm1f(-1e-3), -9.118_007);
    assert_relative_eq!(lambert_wm1f(-3e-4), -1.045_921_1e1);
    assert_relative_eq!(lambert_wm1f(-1e-4), -1.166_711_4e1);
    assert_relative_eq!(lambert_wm1f(-3e-5), -1.297_753_2e1);
    assert_relative_eq!(lambert_wm1f(-1e-5), -1.416_360_1e1);
    assert_relative_eq!(lambert_wm1f(-1e-20), -49.962_986);
    assert!(lambert_wm1f(f32::EPSILON).is_nan());
}

#[test]
fn test_trait_impl_on_f64() {
    assert_abs_diff_eq!(
        (-2.678_794_411_714_424e-1_f64).lambert_w0(),
        -3.993_824_525_397_807e-1
    );
    assert_relative_eq!(
        (-3.578_794_411_714_423e-1_f64).lambert_wm1(),
        -1.253493791367214,
        max_relative = 4e-16
    );
}

#[test]
fn test_trait_impl_on_f32() {
    assert_abs_diff_eq!(6.321_205_5e-1_f32.lambert_w0(), 4.167_04e-1);
    assert_relative_eq!(
        (-3.578_794_3e-1_f32).lambert_wm1(),
        -1.253_493_8,
        max_relative = 1e-6
    );
}
