use super::{INV_SQRT_E as X0, NEG_INV_E as Z0};

/// zc = z + 1/e
// Formatting this function takes a lot of time, so I have ran `cargo fmt` on it once, and now no one else has to / Johanna.
pub fn dwm1c(z: f64, zc: f64) -> f64 {
    use crate::pade::pade_7;
    if zc < 0.0 {
        f64::NAN
    } else if z <= -0.354_291_330_944_216_4 {
        // W >= -1.3, X_-1

        pade_7(
            zc.sqrt(),
            [
                -1.000_000_000_000_000_111_0,
                4.296_301_617_877_713,
                -4.099_140_792_400_746,
                -6.844_284_220_083_331,
                17.084_773_793_345_27,
                -13.015_133_123_886_661,
                3.930_360_862_953_985,
                -0.346_367_465_122_474_57,
            ],
            [
                1.0,
                -6.627_945_599_474_763,
                17.740_962_374_121_4,
                -24.446_872_319_343_477,
                18.249_006_287_190_618,
                -7.058_075_875_662_479,
                1.197_878_676_279_400_3,
                -0.053_875_778_140_352_6,
            ],
        )
    } else if z <= -0.188_726_882_822_894_35 {
        // W >= -2.637, Y_-1

        pade_7(
            -z / (X0 + (z - Z0).sqrt()),
            [
                -8.225_315_526_444_685,
                -813.207_067_320_014_9,
                -15_270.113_237_678_51,
                -79_971.585_089_674_15,
                -103_667.542_158_083_77,
                42_284.755_505_061_26,
                74_953.525_397_605_48,
                10_554.369_146_366_736,
            ],
            [
                1.0,
                146.363_151_616_695_7,
                3_912.476_137_253_924,
                31_912.693_749_754_846,
                92_441.293_717_108_62,
                94_918.733_120_470_35,
                29_531.165_406_571_745,
                1_641.680_896_033_037,
            ],
        )
    } else if z <= -0.060_497_597_226_958_34 {
        // W >= -4.253, Y_-2

        pade_7(
            -z / (X0 + (z - Z0).sqrt()),
            [
                -9.618_412_744_335_403,
                -3_557.856_904_301_800_6,
                -254_015.593_112_843_8,
                -5.392_389_363_067_063_5e6,
                -3.663_825_741_753_69e7,
                -6.148_431_948_622_697e7,
                3.042_169_037_744_613_4e7,
                3.972_813_905_487_932e7,
            ],
            [
                1.0,
                507.405_256_285_233,
                46_852.747_159_777_88,
                1.316_830_464_009_143_6e6,
                1.311_169_069_371_241_5e7,
                4.614_211_644_525_801_4e7,
                4.898_226_895_620_883e7,
                9.195_910_098_798_385e6,
            ],
        )
    } else if z <= -0.017_105_334_740_676_01 {
        // W >= -5.832, Y_-3

        pade_7(
            -z / (X0 + (z - Z0).sqrt()),
            [
                -11.038_489_462_297_466,
                -15_575.812_882_656_619,
                -4.249_294_730_489_777e6,
                -3.517_024_593_880_342e8,
                -9.865_916_303_661_137e9,
                -8.619_537_230_330_501e10,
                -1.328_633_557_402_761_5e11,
                1.598_954_643_442_066e11,
            ],
            [
                1.0,
                1_837.077_069_301_716_6,
                612_840.975_855_951,
                6.214_918_139_846_548_4e7,
                2.230_401_131_444_308_3e9,
                2.825_423_248_527_369_7e10,
                1.077_086_663_954_315_6e11,
                7.196_469_887_604_913e10,
            ],
        )
    } else if z <= -0.004_595_496_212_794_371 {
        // W >= -7.382, Y_-4

        pade_7(
            -z / (X0 + (z - Z0).sqrt()),
            [
                -12.474_405_916_395_746,
                -68_180.335_575_543_78,
                -7.184_659_984_562_01e7,
                -2.314_268_822_175_918_2e10,
                -2.580_137_833_794_529_3e12,
                -9.518_274_816_138_631e13,
                -8.607_325_098_621_033e14,
                1.404_194_185_333_996_1e14,
            ],
            [
                1.0,
                6_852.581_373_443_11,
                8.515_300_102_546_655e6,
                3.214_602_823_968_569_3e9,
                4.292_980_741_745_32e11,
                2.023_438_116_163_808_6e13,
                2.869_993_326_823_392_5e14,
                7.121_013_665_152_548e14,
            ],
        )
    } else if z <= -0.001_200_161_067_219_772_4 {
        // W >= -8.913, Y_-5

        pade_7(
            -z / (X0 + (z - Z0).sqrt()),
            [
                -13.921_651_376_890_072,
                -298_789.564_823_880_7,
                -1.231_301_993_732_209_2e9,
                -1.555_614_908_189_951e12,
                -6.868_534_110_677_271e14,
                -1.029_061_627_593_326_7e17,
                -4.140_468_370_161_965e18,
                -1.442_330_999_800_636_8e19,
            ],
            [
                1.0,
                26_154.955_236_499_143,
                1.239_308_727_744_204_1e8,
                1.783_292_270_247_076e11,
                9.077_260_816_381_084e13,
                1.631_473_474_005_425_2e16,
                8.837_132_386_123_351e17,
                8.416_662_064_338_502e18,
            ],
        )
    } else if z <= -0.000_307_288_059_321_915 {
        // W >= -10.433, Y_-6

        pade_7(
            -z / (X0 + (z - Z0).sqrt()),
            [
                -15.377_894_224_591_557,
                -1.312_231_200_509_698e6,
                -2.140_815_702_211_173_6e10,
                -1.071_828_743_155_781_3e14,
                -1.884_935_352_402_773_4e17,
                -1.139_485_860_730_931_1e20,
                -1.926_155_508_872_914_4e22,
                -3.997_845_208_667_69e23,
            ],
            [
                1.0,
                101_712.867_717_606_2,
                1.872_854_594_505_038e9,
                1.046_961_741_666_440_2e13,
                2.070_434_906_012_044_4e16,
                1.446_490_790_238_607_4e19,
                3.051_043_220_560_890_3e21,
                1.139_758_913_979_073_9e23,
            ],
        )
    } else if z <= -0.000_077_447_159_838_062_18 {
        // W >= -11.946, Y_-7

        pade_7(
            -z / (X0 + (z - Z0).sqrt()),
            [
                -16.841_701_411_264_98,
                -5.779_082_325_757_714e6,
                -3.775_723_079_125_64e11,
                -7.571_213_374_258_986e15,
                -5.347_933_891_601_147e19,
                -1.308_271_173_229_786_5e23,
                -9.146_277_700_452_142e25,
                -8.960_276_811_926_363e27,
            ],
            [
                1.0,
                401_820.466_662_307_27,
                2.921_151_813_690_049_4e10,
                6.445_613_537_341_029e14,
                5.031_180_957_649_953e18,
                1.387_904_123_971_628_9e22,
                1.157_514_616_751_351_5e25,
                1.719_922_018_594_775_7e27,
            ],
        )
    } else if z <= -4.580_811_969_815_817_5e-17 {
        // W >= -41.344, V_-8

        pade_7(
            (-z).ln(),
            [
                -2.083_626_038_401_644,
                1.612_243_624_227_149_6,
                5.446_426_495_963_720_5,
                -3.088_633_112_831_716,
                0.461_078_291_553_701_4,
                -0.023_553_839_118_456_38,
                0.000_405_389_041_702_534_04,
                -1.794_815_692_251_682_6e-6,
            ],
            [
                1.,
                2.369_964_891_270_301_5,
                -2.124_944_970_740_481_5,
                0.384_809_800_985_884_85,
                -0.021_720_009_380_176_607,
                0.000_394_058_628_906_086_36,
                -1.790_931_206_686_595_8e-6,
                3.115_367_330_813_367e-12,
            ],
        )
    } else if z <= -6.107_367_223_659_479e-79 {
        // W >= -185.316, V_-9

        pade_7(
            (-z).ln(),
            [
                0.160_453_837_665_705_42,
                2.221_418_252_446_151_4,
                -0.941_196_624_920_508_9,
                0.091_921_523_818_747_87,
                -0.002_906_976_053_317_166,
                0.000_032_707_247_990_255_96,
                -1.248_667_233_688_989_2e-7,
                1.224_743_827_986_178_6e-10,
            ],
            [
                1.,
                -0.702_549_960_878_703_4,
                0.080_974_347_786_703_19,
                -0.002_746_985_002_956_315_3,
                0.000_031_943_362_385_183_66,
                -1.239_062_068_732_166_7e-7,
                1.224_163_611_516_82e-10,
                -1.027_571_802_054_676_6e-17,
            ],
        )
    } else if z < 0.0 {
        // V_-10

        pade_7(
            (-z).ln(),
            [
                -1.274_217_970_307_544,
                1.369_665_880_542_138_4,
                -0.125_193_453_875_587_83,
                0.002_515_572_246_076_384_3,
                -0.000_015_748_033_750_499_976,
                3.431_608_538_691_379e-8,
                -2.502_524_288_534_043_7e-11,
                4.642_388_501_409_958e-15,
            ],
            [
                1.0,
                -0.114_200_064_741_524_65,
                0.002_428_523_383_212_26,
                -0.000_015_520_907_512_751_72,
                3.412_053_476_039_600_4e-8,
                -2.498_105_618_645_027_4e-11,
                4.641_976_809_305_971e-15,
                -1.360_871_393_694_260_3e-23,
            ],
        )
    } else {
        f64::NAN
    }
}
