use super::{INV_SQRT_E as X0, NEG_INV_E as Z0};

#[cfg(not(feature = "fma"))]
/// The original implementation of the secondary branch of the Lambert W function by Toshio Fukushima, accurate to 50 bits, ported to Rust.
///
/// zc = z + 1/e
// Formatting this function takes a lot of time, so I have ran `cargo fmt` on it once, and now no one else has to / Johanna.
#[rustfmt::skip]
pub fn dwm1c(z: f64, zc: f64) -> f64 {
    if zc < 0.0 {
        f64::NAN
    } else if z <= -0.3542913309442164 {
        // W >= -1.3, X_-1

        let x = zc.sqrt();

        (-1.0000000000000001110
            + x * (4.296_301_617_877_713
                + x * (-4.099_140_792_400_746
                    + x * (-6.844_284_220_083_331
                        + x * (17.084_773_793_345_27
                            + x * (-13.015_133_123_886_661
                                + x * (3.930_360_862_953_985
                                    + x * (-0.346_367_465_122_474_57))))))))
            / (1.
                + x * (-6.627_945_599_474_763
                    + x * (17.740_962_374_121_4
                        + x * (-24.446_872_319_343_477
                            + x * (18.249_006_287_190_618
                                + x * (-7.058_075_875_662_479
                                    + x * (1.197_878_676_279_400_3
                                        + x * (-0.053_875_778_140_352_6))))))))
    } else if z <= -0.188_726_882_822_894_35 {
        // W >= -2.637, Y_-1
        
        let x = -z / (X0 + (z - Z0).sqrt());

        (-8.225_315_526_444_685
            + x * (-813.207_067_320_014_9
                + x * (-15_270.113_237_678_51
                    + x * (-79_971.585_089_674_15
                        + x * (-103_667.542_158_083_77
                            + x * (42_284.755_505_061_26
                                + x * (74_953.525_397_605_48 + x * 10_554.369_146_366_736)))))))
            / (1.
                + x * (146.363_151_616_695_7
                    + x * (3_912.476_137_253_924
                        + x * (31_912.693_749_754_846
                            + x * (92_441.293_717_108_62
                                + x * (94_918.733_120_470_35
                                    + x * (29_531.165_406_571_745 + x * 1_641.680_896_033_037)))))))
    } else if z <= -0.060_497_597_226_958_34 {
        // W >= -4.253, Y_-2

        let x = -z / (X0 + (z - Z0).sqrt());

        (-9.618_412_744_335_403
            + x * (-3_557.856_904_301_800_6
                + x * (-254_015.593_112_843_8
                    + x * (-5.392_389_363_067_063_5e6
                        + x * (-3.663_825_741_753_69e7
                            + x * (-6.148_431_948_622_697e7
                                + x * (3.042_169_037_744_613_4e7
                                    + x * 3.972_813_905_487_932e7)))))))
            / (1.
                + x * (507.405_256_285_233
                    + x * (46_852.747_159_777_88
                        + x * (1.316_830_464_009_143_6e6
                            + x * (1.311_169_069_371_241_5e7
                                + x * (4.614_211_644_525_801_4e7
                                    + x * (4.898_226_895_620_883e7
                                        + x * 9.195_910_098_798_385e6)))))))
    } else if z <= -0.017_105_334_740_676_01 {
        // W >= -5.832, Y_-3

        let x = -z / (X0 + (z - Z0).sqrt());

        (-11.038_489_462_297_466
            + x * (-15_575.812_882_656_619
                + x * (-4.249_294_730_489_777e6
                    + x * (-3.517_024_593_880_342e8
                        + x * (-9.865_916_303_661_137e9
                            + x * (-8.619_537_230_330_501e10
                                + x * (-1.328_633_557_402_761_5e11
                                    + x * 1.598_954_643_442_066e11)))))))
            / (1.
                + x * (1_837.077_069_301_716_6
                    + x * (612_840.975_855_951
                        + x * (6.214_918_139_846_548_4e7
                            + x * (2.230_401_131_444_308_3e9
                                + x * (2.825_423_248_527_369_7e10
                                    + x * (1.077_086_663_954_315_6e11
                                        + x * 7.196_469_887_604_913e10)))))))
    } else if z <= -0.004_595_496_212_794_371 {
        // W >= -7.382, Y_-4

        let x = -z / (X0 + (z - Z0).sqrt());

        (-12.474_405_916_395_746
            + x * (-68_180.335_575_543_78
                + x * (-7.184_659_984_562_01e7
                    + x * (-2.314_268_822_175_918_2e10
                        + x * (-2.580_137_833_794_529_3e12
                            + x * (-9.518_274_816_138_631e13
                                + x * (-8.607_325_098_621_033e14
                                    + x * 1.404_194_185_333_996_1e14)))))))
            / (1.
                + x * (6_852.581_373_443_11
                    + x * (8.515_300_102_546_655e6
                        + x * (3.214_602_823_968_569_3e9
                            + x * (4.292_980_741_745_32e11
                                + x * (2.023_438_116_163_808_6e13
                                    + x * (2.869_993_326_823_392_5e14
                                        + x * 7.121_013_665_152_548e14)))))))
    } else if z <= -0.001_200_161_067_219_772_4 {
        // W >= -8.913, Y_-5

        let x = -z / (X0 + (z - Z0).sqrt());

        (-13.921_651_376_890_072
            + x * (-298_789.564_823_880_7
                + x * (-1.231_301_993_732_209_2e9
                    + x * (-1.555_614_908_189_951e12
                        + x * (-6.868_534_110_677_271e14
                            + x * (-1.029_061_627_593_326_7e17
                                + x * (-4.140_468_370_161_965e18
                                    + x * (-1.442_330_999_800_636_8e19))))))))
            / (1.
                + x * (26_154.955_236_499_143
                    + x * (1.239_308_727_744_204_1e8
                        + x * (1.783_292_270_247_076e11
                            + x * (9.077_260_816_381_084e13
                                + x * (1.631_473_474_005_425_2e16
                                    + x * (8.837_132_386_123_351e17
                                        + x * 8.416_662_064_338_502e18)))))))
    } else if z <= -0.000_307_288_059_321_915 {
        // W >= -10.433, Y_-6

        let x = -z / (X0 + (z - Z0).sqrt());

        (-15.377_894_224_591_557
            + x * (-1.312_231_200_509_698e6
                + x * (-2.140_815_702_211_173_6e10
                    + x * (-1.071_828_743_155_781_3e14
                        + x * (-1.884_935_352_402_773_4e17
                            + x * (-1.139_485_860_730_931_1e20
                                + x * (-1.926_155_508_872_914_4e22
                                    + x * (-3.997_845_208_667_69e23))))))))
            / (1.
                + x * (101_712.867_717_606_2
                    + x * (1.872_854_594_505_038e9
                        + x * (1.046_961_741_666_440_2e13
                            + x * (2.070_434_906_012_044_4e16
                                + x * (1.446_490_790_238_607_4e19
                                    + x * (3.051_043_220_560_890_3e21
                                        + x * 1.139_758_913_979_073_9e23)))))))
    } else if z <= -0.000_077_447_159_838_062_18 {
        // W >= -11.946, Y_-7

        let x = -z / (X0 + (z - Z0).sqrt());

        (-16.841_701_411_264_98
            + x * (-5.779_082_325_757_714e6
                + x * (-3.775_723_079_125_64e11
                    + x * (-7.571_213_374_258_986e15
                        + x * (-5.347_933_891_601_147e19
                            + x * (-1.308_271_173_229_786_5e23
                                + x * (-9.146_277_700_452_142e25
                                    + x * (-8.960_276_811_926_363e27))))))))
            / (1.
                + x * (401_820.466_662_307_27
                    + x * (2.921_151_813_690_049_4e10
                        + x * (6.445_613_537_341_029e14
                            + x * (5.031_180_957_649_953e18
                                + x * (1.387_904_123_971_628_9e22
                                    + x * (1.157_514_616_751_351_5e25
                                        + x * 1.719_922_018_594_775_7e27)))))))
    } else if z <= -4.580_811_969_815_817_5e-17 {
        // W >= -41.344, V_-8

        let u = (-z).ln();

        (-2.083_626_038_401_644
            + u * (1.612_243_624_227_149_6
                + u * (5.446_426_495_963_720_5
                    + u * (-3.088_633_112_831_716
                        + u * (0.461_078_291_553_701_4
                            + u * (-0.023_553_839_118_456_38
                                + u * (0.000_405_389_041_702_534_04
                                    + u * (-1.794_815_692_251_682_6e-6))))))))
            / (1.
                + u * (2.369_964_891_270_301_5
                    + u * (-2.124_944_970_740_481_5
                        + u * (0.384_809_800_985_884_85
                            + u * (-0.021_720_009_380_176_607
                                + u * (0.000_394_058_628_906_086_36
                                    + u * (-1.790_931_206_686_595_8e-6
                                        + u * 3.115_367_330_813_367e-12)))))))
    } else if z <= -6.107_367_223_659_479e-79 {
        // W >= -185.316, V_-9

        let u = (-z).ln();

        (0.160_453_837_665_705_42
            + u * (2.221_418_252_446_151_4
                + u * (-0.941_196_624_920_508_9
                    + u * (0.091_921_523_818_747_87
                        + u * (-0.002_906_976_053_317_166
                            + u * (0.000_032_707_247_990_255_96
                                + u * (-1.248_667_233_688_989_2e-7
                                    + u * 1.224_743_827_986_178_6e-10)))))))
            / (1.
                + u * (-0.702_549_960_878_703_4
                    + u * (0.080_974_347_786_703_19
                        + u * (-0.002_746_985_002_956_315_3
                            + u * (0.000_031_943_362_385_183_66
                                + u * (-1.239_062_068_732_166_7e-7
                                    + u * (1.224_163_611_516_82e-10
                                        + u * (-1.027_571_802_054_676_6e-17))))))))
    } else if z < 0.0 {
        // V_-10

        let u = (-z).ln();
        
        (-1.274_217_970_307_544
            + u * (1.369_665_880_542_138_4
                + u * (-0.125_193_453_875_587_83
                    + u * (0.002_515_572_246_076_384_3
                        + u * (-0.000_015_748_033_750_499_976
                            + u * (3.431_608_538_691_379e-8
                                + u * (-2.502_524_288_534_043_7e-11
                                    + u * 4.642_388_501_409_958e-15)))))))
            / (1.
                + u * (-0.114_200_064_741_524_65
                    + u * (0.002_428_523_383_212_26
                        + u * (-0.000_015_520_907_512_751_72
                            + u * (3.412_053_476_039_600_4e-8
                                + u * (-2.498_105_618_645_027_4e-11
                                    + u * (4.641_976_809_305_971e-15
                                        + u * (-1.360_871_393_694_260_3e-23))))))))
    } else {
        f64::NAN
    }
}

#[cfg(feature = "fma")]
/// This function is the same as the above except the polynomials have been simplified.
pub fn dwm1c(z: f64, zc: f64) -> f64 {
    use crate::pade::pade_7;
    if zc < 0.0 {
        f64::NAN
    } else if z <= -0.3542913309442164 {
        // W >= -1.3, X_-1

        let x = zc.sqrt();
        let x2 = x * x;
        let x4 = x2 * x2;

        pade_7(
            x,
            x2,
            x4,
            [
                -1.0000000000000001110,
                4.29630161787771,
                -4.09914079240075,
                -6.84428422008333,
                17.0847737933453,
                -13.0151331238867,
                3.93036086295399,
                -0.346367465122475,
            ],
            [
                1.0,
                -6.62794559947476,
                17.7409623741214,
                -24.4468723193435,
                18.2490062871906,
                -7.05807587566248,
                1.1978786762794,
                -0.0538757781403526,
            ],
        )
    } else if z <= -0.188_726_882_822_894_35 {
        // W >= -2.637, Y_-1

        let x = -z / (X0 + (z - Z0).sqrt());
        let x2 = x * x;
        let x4 = x2 * x2;

        pade_7(
            x,
            x2,
            x4,
            [
                -8.22531552644469,
                -813.207067320015,
                -15270.1132376785,
                -79971.5850896741,
                -103667.542158084,
                42284.7555050613,
                74953.5253976055,
                10554.3691463667,
            ],
            [
                1.0,
                146.363151616696,
                3912.47613725392,
                31912.6937497548,
                92441.2937171086,
                94918.7331204704,
                29531.1654065717,
                1641.68089603304,
            ],
        )
    } else if z <= -0.060_497_597_226_958_34 {
        // W >= -4.253, Y_-2

        let x = -z / (X0 + (z - Z0).sqrt());
        let x2 = x * x;
        let x4 = x2 * x2;

        pade_7(
            x,
            x2,
            x4,
            [
                -9.6184127443354,
                -3557.8569043018,
                -254015.593112844,
                -5392389.36306706,
                -36638257.4175369,
                -61484319.486227,
                30421690.3774461,
                39728139.0548793,
            ],
            [
                1.0,
                507.405256285233,
                46852.7471597779,
                1316830.46400914,
                13111690.6937124,
                46142116.445258,
                48982268.9562088,
                9195910.09879838,
            ],
        )
    } else if z <= -0.017_105_334_740_676_01 {
        // W >= -5.832, Y_-3

        let x = -z / (X0 + (z - Z0).sqrt());
        let x2 = x * x;
        let x4 = x2 * x2;

        pade_7(
            x,
            x2,
            x4,
            [
                -11.0384894622975,
                -15575.8128826566,
                -4249294.73048978,
                -351702459.388034,
                -9865916303.66114,
                -86195372303.305,
                -132863355740.276,
                159895464344.207,
            ],
            [
                1.0,
                1837.07706930172,
                612840.975855951,
                62149181.3984655,
                2230401131.44431,
                28254232485.2737,
                107708666395.432,
                71964698876.0491,
            ],
        )
    } else if z <= -0.004_595_496_212_794_371 {
        // W >= -7.382, Y_-4

        let x = -z / (X0 + (z - Z0).sqrt());
        let x2 = x * x;
        let x4 = x2 * x2;

        pade_7(
            x,
            x2,
            x4,
            [
                -12.4744059163957,
                -68180.3355755438,
                -71846599.8456201,
                -23142688221.7592,
                -2580137833794.53,
                -95182748161386.3,
                -860732509862103.0,
                140419418533400.0,
            ],
            [
                1.0,
                6852.58137344311,
                8515300.10254665,
                3214602823.96857,
                429298074174.532,
                20234381161638.1,
                286999332682339.0,
                712101366515255.0,
            ],
        )
    } else if z <= -0.001_200_161_067_219_772_4 {
        // W >= -8.913, Y_-5

        let x = -z / (X0 + (z - Z0).sqrt());
        let x2 = x * x;
        let x4 = x2 * x2;

        pade_7(
            x,
            x2,
            x4,
            [
                -13.9216513768901,
                -298789.564823881,
                -1231301993.73221,
                -1555614908189.95,
                -686853411067727.0,
                -1.02906162759333e+17,
                -4.14046837016197e+18,
                -1.44233099980064e+19,
            ],
            [
                1.0,
                26154.9552364991,
                123930872.77442,
                178329227024.708,
                90772608163810.8,
                1.63147347400543e+16,
                8.83713238612335e+17,
                8.4166620643385e+18,
            ],
        )
    } else if z <= -0.000_307_288_059_321_915 {
        // W >= -10.433, Y_-6

        let x = -z / (X0 + (z - Z0).sqrt());
        let x2 = x * x;
        let x4 = x2 * x2;

        pade_7(
            x,
            x2,
            x4,
            [
                -15.3778942245916,
                -1312231.2005097,
                -21408157022.1117,
                -107182874315578.0,
                -1.88493535240277e+17,
                -1.13948586073093e+20,
                -1.92615550887291e+22,
                -3.99784520866769e+23,
            ],
            [
                1.0,
                101712.867717606,
                1872854594.50504,
                10469617416664.4,
                2.07043490601204e+16,
                1.44649079023861e+19,
                3.05104322056089e+21,
                1.13975891397907e+23,
            ],
        )
    } else if z <= -0.000_077_447_159_838_062_18 {
        // W >= -11.946, Y_-7

        let x = -z / (X0 + (z - Z0).sqrt());
        let x2 = x * x;
        let x4 = x2 * x2;

        pade_7(
            x,
            x2,
            x4,
            [
                -16.841701411265,
                -5779082.32575771,
                -377572307912.564,
                -7.57121337425899e+15,
                -5.34793389160115e+19,
                -1.30827117322979e+23,
                -9.14627770045214e+25,
                -8.96027681192636e+27,
            ],
            [
                1.0,
                401820.466662307,
                29211518136.9005,
                644561353734103.0,
                5.03118095764995e+18,
                1.38790412397163e+22,
                1.15751461675135e+25,
                1.71992201859478e+27,
            ],
        )
    } else if z <= -4.580_811_969_815_817_5e-17 {
        // W >= -41.344, V_-8

        let u = (-z).ln();

        (-2.083_626_038_401_644
            + u * (1.612_243_624_227_149_6
                + u * (5.446_426_495_963_720_5
                    + u * (-3.088_633_112_831_716
                        + u * (0.461_078_291_553_701_4
                            + u * (-0.023_553_839_118_456_38
                                + u * (0.000_405_389_041_702_534_04
                                    + u * (-1.794_815_692_251_682_6e-6))))))))
            / (1.
                + u * (2.369_964_891_270_301_5
                    + u * (-2.124_944_970_740_481_5
                        + u * (0.384_809_800_985_884_85
                            + u * (-0.021_720_009_380_176_607
                                + u * (0.000_394_058_628_906_086_36
                                    + u * (-1.790_931_206_686_595_8e-6
                                        + u * 3.115_367_330_813_367e-12)))))))
    } else if z <= -6.107_367_223_659_479e-79 {
        // W >= -185.316, V_-9

        let u = (-z).ln();

        (0.160_453_837_665_705_42
            + u * (2.221_418_252_446_151_4
                + u * (-0.941_196_624_920_508_9
                    + u * (0.091_921_523_818_747_87
                        + u * (-0.002_906_976_053_317_166
                            + u * (0.000_032_707_247_990_255_96
                                + u * (-1.248_667_233_688_989_2e-7
                                    + u * 1.224_743_827_986_178_6e-10)))))))
            / (1.
                + u * (-0.702_549_960_878_703_4
                    + u * (0.080_974_347_786_703_19
                        + u * (-0.002_746_985_002_956_315_3
                            + u * (0.000_031_943_362_385_183_66
                                + u * (-1.239_062_068_732_166_7e-7
                                    + u * (1.224_163_611_516_82e-10
                                        + u * (-1.027_571_802_054_676_6e-17))))))))
    } else if z < 0.0 {
        // V_-10

        let u = (-z).ln();

        (-1.274_217_970_307_544
            + u * (1.369_665_880_542_138_4
                + u * (-0.125_193_453_875_587_83
                    + u * (0.002_515_572_246_076_384_3
                        + u * (-0.000_015_748_033_750_499_976
                            + u * (3.431_608_538_691_379e-8
                                + u * (-2.502_524_288_534_043_7e-11
                                    + u * 4.642_388_501_409_958e-15)))))))
            / (1.
                + u * (-0.114_200_064_741_524_65
                    + u * (0.002_428_523_383_212_26
                        + u * (-0.000_015_520_907_512_751_72
                            + u * (3.412_053_476_039_600_4e-8
                                + u * (-2.498_105_618_645_027_4e-11
                                    + u * (4.641_976_809_305_971e-15
                                        + u * (-1.360_871_393_694_260_3e-23))))))))
    } else {
        f64::NAN
    }
}
