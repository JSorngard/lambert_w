// Copyright 2026 Johanna Sörngård
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This module contains an implementation of the approximation of the principal
//! branch of the Lambert W function
//! with 24 bits of accuracy from Fukushima's paper.
//! It is based on the Fortran implementation of the name "sw0" by Fukushima.

// The coefficients in these rational minimax functions all have excessive precision.
// By keeping the full precision in the source code we can ensure that there is no confusion
// when comparing with the paper.
#![allow(clippy::excessive_precision)]

use crate::{
    generic_math::{ln, rational_function, sqrt},
    NEG_INV_E,
};

/// The principal branch of the Lambert W function computed to 24 bits of accuracy on 64-bit floats with Fukushima's method[^1].
///
/// # Examples
///
/// #### Basic usage
///
/// ```
/// use lambert_w::sp_lambert_w0;
/// use approx::assert_abs_diff_eq;
///
/// let Ω = sp_lambert_w0(1.0);
///
/// assert_abs_diff_eq!(Ω, 0.5671432904097839, epsilon = f64::from(f32::EPSILON));
/// ```
///
/// #### Special cases
///
/// For inputs of -1/e and 0 the function returns exactly -1 and 0 respectively,
/// while an infinite input gives [`INFINITY`](f64::INFINITY):
///
/// ```
/// use lambert_w::{sp_lambert_w0, NEG_INV_E};
///
/// assert_eq!(sp_lambert_w0(NEG_INV_E), -1.0);
/// assert_eq!(sp_lambert_w0(0.0), 0.0);
/// assert_eq!(sp_lambert_w0(f64::INFINITY), f64::INFINITY);
/// ```
///
/// Inputs smaller than -1/e, as well as inputs of [`NAN`](f64::NAN), result in [`NAN`](f64::NAN):
///
/// ```
/// use lambert_w::{sp_lambert_w0, NEG_INV_E};
///
/// assert!(sp_lambert_w0(NEG_INV_E.next_down()).is_nan());
/// assert!(sp_lambert_w0(f64::NAN).is_nan());
/// ```
///
/// # Reference
///
/// [^1]: [Toshio Fukushima, Precise and fast computation of Lambert W function by piecewise minimax rational function approximation with variable transformation](https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation).
#[must_use = "this is a pure function that only returns a value and has no side effects"]
pub fn sp_lambert_w0(z: f64) -> f64 {
    // The critical arguments used in the if statements are related to the numbers in table 3 of the paper, column one.
    // The coefficients in the rational functions are related from the tables 5 through 7 in the paper.
    // The actual numbers are taken from Fukushima's Fortran implementation, where they have higher precision.

    if z < NEG_INV_E {
        f64::NAN
    } else if z == NEG_INV_E {
        -1.0
    } else if z == 0.0 {
        0.0
    } else if z <= 2.008_217_811_584_472_656_3 {
        // W <= 0.854, X_1

        rational_function(
            sqrt(z - NEG_INV_E),
            [
                -0.999_999_940_395_401_883_3,
                0.055_730_052_161_777_802_73,
                2.126_973_249_105_317_442_1,
                0.813_511_236_783_528_788_0,
                0.016_324_880_146_070_158_165,
            ],
            [
                1.0,
                2.275_906_559_863_465_118_2,
                1.367_597_013_868_904_082_9,
                0.186_158_234_528_316_218_70,
            ],
        )
    } else if z <= 30.539_142_109_510_895_244 {
        // W <= 2.502, X_2

        rational_function(
            sqrt(z - NEG_INV_E),
            [
                -0.985_519_709_059_990_933_51,
                1.077_497_573_381_351_625_9,
                0.871_751_030_681_774_965_65,
                0.054_352_728_608_275_766_374,
            ],
            [
                1.0,
                1.186_101_403_701_543_363_7,
                0.249_962_984_308_281_622_21,
                0.006_881_368_648_675_912_400_2,
            ],
        )
    } else if z <= 371.669_843_713_757_760_70 {
        // W <= 4.430, X_3

        rational_function(
            sqrt(z - NEG_INV_E),
            [
                -0.762_397_113_463_688_842_39,
                1.231_773_161_336_359_583_3,
                0.243_424_471_130_566_945_10,
                0.004_320_601_393_878_235_597_4,
            ],
            [
                1.0,
                0.579_386_215_035_869_076_09,
                0.046_601_427_736_078_777_155,
                0.000_435_128_175_674_741_091_42,
            ],
        )
    } else if z <= 4_705.918_954_265_969_037_7 {
        // W <= 6.574, X_4

        rational_function(
            sqrt(z - NEG_INV_E),
            [
                0.085_801_247_434_391_388_025,
                0.825_397_980_997_483_437_22,
                0.039_781_960_760_329_074_310,
                0.000_187_855_789_275_837_990_84,
            ],
            [
                1.0,
                0.213_380_768_170_801_410_22,
                0.005_462_672_039_792_693_555_3,
                0.000_015_449_534_481_294_754_830,
            ],
        )
    } else if z <= 64_640.797_355_310_089_675 {
        // W <= 8.892, X_5

        rational_function(
            sqrt(z - NEG_INV_E),
            [
                1.621_924_538_347_016_874_2,
                0.388_691_451_325_166_635_64,
                0.004_575_064_267_850_351_111_1,
                5.538_467_214_864_449_887_3e-6,
            ],
            [
                1.0,
                0.065_219_460_735_182_414_183,
                0.000_478_827_607_890_225_089_40,
                3.809_482_814_629_240_109_9e-7,
            ],
        )
    } else if z <= 965_649.030_871_163_226_41 {
        // W <= 11.351, X_6

        rational_function(
            sqrt(z - NEG_INV_E),
            [
                3.621_899_608_569_591_996_9,
                0.148_846_467_548_801_598_69,
                0.000_424_696_224_099_984_031_46,
                1.279_017_971_037_421_684_6e-7,
            ],
            [
                1.0,
                0.017_985_659_319_608_747_571,
                0.000_035_446_449_757_357_845_802,
                7.506_249_296_303_704_700_5e-9,
            ],
        )
    } else if z <= 1.559_333_422_803_816_549_0e7 {
        // W <= 13.928, X_7

        rational_function(
            sqrt(z - NEG_INV_E),
            [
                5.907_336_973_960_809_058_9,
                0.050_053_653_594_737_112_977,
                0.000_034_072_148_625_204_698_714,
                2.481_206_469_365_548_349_2e-9,
            ],
            [
                1.0,
                0.004_655_899_001_684_321_048_8,
                2.344_944_586_080_881_205_0e-6,
                1.263_142_996_480_846_114_0e-10,
            ],
        )
    } else if z <= 2.702_564_027_724_190_315_7e8 {
        // W <= 16.605, X_8

        rational_function(
            sqrt(z - NEG_INV_E),
            [
                8.382_600_584_819_551_267_1,
                0.015_360_346_475_232_500_605,
                2.443_338_439_786_936_744_5e-6,
                4.185_680_326_411_854_963_9e-11,
            ],
            [
                1.0,
                0.001_150_742_322_378_586_783_0,
                1.422_142_847_481_351_640_8e-7,
                1.873_917_202_662_012_263_7e-12,
            ],
        )
    } else if z <= 4.995_018_739_704_194_635_5e9 {
        // W <= 19.368, X_9

        rational_function(
            sqrt(z - NEG_INV_E),
            [
                10.996_674_803_992_550_975,
                0.004_394_213_889_867_383_245_6,
                1.596_666_535_484_677_802_6e-7,
                6.266_538_284_496_873_224_2e-13,
            ],
            [
                1.0,
                0.000_273_837_576_757_036_474_31,
                8.015_706_231_969_030_420_9e-9,
                2.495_698_215_887_173_000_2e-14,
            ],
        )
    } else if z <= 9.791_115_441_672_696_027_7e10 {
        // W <= 22.207, X_10

        rational_function(
            sqrt(z - NEG_INV_E),
            [
                13.719_833_802_350_861_158,
                0.001_187_444_380_520_229_142_9,
                9.630_338_120_016_467_597_1e-9,
                8.443_452_423_226_162_880_8e-15,
            ],
            [
                1.0,
                0.000_063_056_372_424_395_349_289,
                4.235_876_603_109_884_029_1e-10,
                3.020_540_500_543_447_430_4e-16,
            ],
        )
    } else if z <= 2.025_975_385_630_209_968_3e12 {
        // W <= 25.114, X_11

        rational_function(
            sqrt(z - NEG_INV_E),
            [
                16.533_119_481_561_616_886,
                0.000_305_831_257_519_080_406_46,
                5.411_294_663_372_009_873_1e-10,
                1.034_713_033_370_471_127_5e-16,
            ],
            [
                1.0,
                0.000_014_099_161_212_376_339_883,
                2.112_109_541_235_469_506_3e-11,
                3.352_692_715_745_246_951_5e-18,
            ],
        )
    } else if z <= 4.407_744_425_147_793_893_9e13 {
        // W <= 28.082, X_12

        rational_function(
            sqrt(z - NEG_INV_E),
            [
                19.423_519_260_478_579_324,
                0.000_075_559_269_761_977_808_773,
                2.853_002_312_078_307_598_4e-11,
                1.162_962_709_646_357_956_9e-18,
            ],
            [
                1.0,
                3.069_209_278_972_785_565_7e-6,
                9.986_661_305_031_146_547_5e-13,
                3.437_671_711_698_391_503_8e-20,
            ],
        )
    } else if z <= 1.004_838_215_057_150_448_5e15 {
        // W <= 31.106, X_13

        rational_function(
            sqrt(z - NEG_INV_E),
            [
                22.381_576_050_041_913_103,
                0.000_017_994_724_029_162_552_053,
                1.419_487_642_040_223_025_9e-12,
                1.207_110_515_438_582_986_7e-20,
            ],
            [
                1.0,
                6.518_396_280_665_677_034_0e-7,
                4.495_866_571_281_253_615_0e-14,
                3.275_542_924_502_358_153_2e-22,
            ],
        )
    } else if z <= 2.393_255_260_235_983_768_7e16 {
        // W <= 34.182, X_14

        rational_function(
            sqrt(z - NEG_INV_E),
            [
                25.400_105_417_092_068_195,
                4.146_737_838_657_924_601_5e-6,
                6.696_269_721_968_179_153_9e-14,
                1.163_790_515_950_647_608_2e-22,
            ],
            [
                1.0,
                1.352_980_135_703_041_681_9e-7,
                1.933_608_178_532_575_035_3e-15,
                2.914_939_619_981_625_763_6e-24,
            ],
        )
    } else if z <= 5.939_799_659_746_575_434_6e17 {
        // W <= 37.306, X_15

        rational_function(
            sqrt(z - NEG_INV_E),
            [
                28.473_455_626_379_917_708,
                9.274_682_469_309_405_901_9e-7,
                3.006_899_015_933_680_528_7e-15,
                1.047_355_759_182_202_785_2e-24,
            ],
            [
                1.0,
                2.748_648_970_452_172_911_0e-8,
                7.967_898_707_103_613_339_8e-17,
                2.433_166_636_706_152_936_6e-26,
            ],
        )
    } else if z <= 1.532_693_858_990_176_659_2e19 {
        // W <= 40.475, X_16

        rational_function(
            sqrt(z - NEG_INV_E),
            [
                31.597_055_437_846_359_378,
                2.018_422_527_678_632_549_6e-7,
                1.289_578_819_651_290_889_8e-16,
                8.836_117_471_410_109_815_9e-27,
            ],
            [
                1.0,
                5.472_394_512_609_849_720_2e-9,
                3.153_772_917_992_919_175_8e-18,
                1.912_203_513_257_167_002_2e-28,
            ],
        )
    } else if z <= 4.103_565_939_888_539_413_5e20 {
        // W <= 43.687, X_17

        rational_function(
            sqrt(z - NEG_INV_E),
            [
                34.767_124_490_414_517_175,
                4.283_079_924_069_894_293_2e-8,
                5.297_588_412_103_653_440_7e-18,
                7.014_551_539_215_877_537_8e-29,
            ],
            [
                1.0,
                1.068_930_112_769_633_243_6e-9,
                1.201_669_906_178_942_295_1e-19,
                1.419_524_481_080_098_523_5e-30,
            ],
        )
    } else if z <= 2.172_370_661_049_060_431_7e141 {
        // W <= 319.673, U_18

        rational_function(
            ln(z),
            [
                -0.607_023_733_718_461_924_76,
                0.698_287_163_225_269_836_51,
                0.075_795_135_081_824_754_660,
                0.000_516_692_560_817_372_468_14,
            ],
            [
                1.0,
                0.079_048_429_972_306_018_289,
                0.000_517_609_908_992_059_784_68,
                -4.243_840_393_198_106_978_6e-10,
            ],
        )
    } else if z < f64::INFINITY {
        //    U_19

        rational_function(
            ln(z),
            [
                -3.132_005_602_886_366_119_2,
                0.948_894_657_265_326_009_02,
                0.008_317_815_296_164_439_321_7,
                5.558_784_815_783_349_289_0e-6,
            ],
            [
                1.0,
                0.008_365_681_867_773_005_876_9,
                5.559_715_493_597_327_536_4e-6,
                -3.748_153_583_315_120_222_2e-14,
            ],
        )
    } else if z.is_nan() {
        f64::NAN
    } else {
        f64::INFINITY
    }
}
