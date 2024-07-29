/// The original implementation of the principal branch of the Lambert W function by Toshio Fukushima, accurate to 50 bits, ported to Rust.
/// 
/// zc = z + 1/e
// Formatting this function takes a lot of time, so I have ran `cargo fmt` on it once, and now no one else has to / Johanna.
#[rustfmt::skip]
pub fn dw0c(zc: f64) -> Option<f64> {
    if zc < 0.0 {
        None
    } else if zc <= 2.549_893_906_503_473_6 {
        // W <= 0.893, X_1
        let x = zc.sqrt();
        Some((-0.999_999_999_999_999_9
            + x * (-2.739_966_866_820_366
                + x * (0.026_164_207_726_990_4
                    + x * (6.370_916_807_894_901
                        + x * (7.101_328_651_785_403
                            + x * (2.980_082_678_300_685_3
                                + x * (0.488_195_968_137_898_7
                                    + x * (0.023_753_035_787_333_61
                                        + x * 0.000_077_365_760_093_772_43))))))))
            / (1.
                + x * (5.071_610_848_417_428
                    + x * (9.986_838_818_354_528
                        + x * (9.660_755_192_207_887
                            + x * (4.794_372_899_133_612
                                + x * (1.162_970_347_770_452_3
                                    + x * (0.118_494_625_007_337_55
                                        + x * 0.003_432_652_513_240_222_5))))))))
    } else if zc <= 43.613_924_462_669_37 {
        // W <= 2.754, X_2
        let x = zc.sqrt();
        Some((-0.999_978_018_005_789_1
            + x * (-0.704_157_515_904_836
                + x * (2.123_226_083_280_252_8
                    + x * (2.389_676_070_293_572
                        + x * (0.777_653_118_050_291_8
                            + x * (0.089_686_698_993_644_75
                                + x * (0.003_306_248_575_374_64
                                    + x * 0.000_025_106_760_479_132_852)))))))
            / (1.
                + x * (3.035_602_682_808_541
                    + x * (3.143_453_015_128_678
                        + x * (1.372_315_656_659_244_7
                            + x * (0.258_446_974_157_442_1
                                + x * (0.019_551_162_251_819_045
                                    + x * (0.000_487_759_332_445_301_26
                                        + x * 2.316_511_684_107_315_5e-6))))))))
    } else if zc <= 598.453_533_718_782_8 {
        // W <= 4.821, X_3
        let x = zc.sqrt();
        Some((-0.989_674_203_372_735
            + x * (0.595_876_806_063_943_8
                + x * (1.422_508_301_815_194_3
                    + x * (0.448_828_891_683_238_1
                        + x * (0.044_504_943_332_390_03
                            + x * (0.001_521_879_483_541_957_8
                                + x * (0.000_016_072_263_556_502_22
                                    + x * 3.372_337_302_030_651e-8)))))))
            / (1.
                + x * (1.695_940_239_462_619_7
                    + x * (0.809_685_734_155_009
                        + x * (0.140_020_349_998_170_23
                            + x * (0.009_357_187_849_379_016
                                + x * (0.000_232_514_875_933_897_72
                                    + x * (1.806_017_075_150_299e-6
                                        + x * 2.575_066_733_701_592_3e-9))))))))
    } else if zc <= 8_049.491_985_075_761_5 {
        // W <= 7.041, X_4
        let x = zc.sqrt();
        Some((-0.773_164_919_972_062_3
            + x * (1.139_133_350_429_670_3
                + x * (0.431_161_172_552_170_74
                    + x * (0.035_773_078_319_037_505
                        + x * (0.000_964_416_405_805_590_9
                            + x * (8.972_385_459_867_587e-6
                                + x * (2.562_350_314_411_772_5e-8
                                    + x * 1.434_881_377_841_663_1e-11)))))))
            / (1.
                + x * (0.746_572_874_565_144_2
                    + x * (0.126_297_770_334_193_5
                        + x * (0.006_974_151_295_956_318
                            + x * (0.000_140_893_392_443_553_55
                                + x * (1.025_743_288_315_294_3e-6
                                    + x * (2.290_268_719_011_923e-9
                                        + x * 9.279_423_101_326_45e-13))))))))
    } else if zc <= 111_124.954_121_217_82 {
        // W <= 9.380, X_5
        let x = zc.sqrt();
        Some((0.120_071_016_715_536_88
            + x * (0.833_526_408_299_128_3
                + x * (0.070_142_775_916_948_34
                    + x * (0.001_484_635_798_547_512_4
                        + x * (0.000_010_478_757_366_110_155
                            + x * (2.571_589_298_707_103_7e-8
                                + x * (1.938_421_447_960_647_4e-11
                                    + x * 2.844_704_903_913_941e-15)))))))
            / (1.
                + x * (0.253_967_388_456_191_27
                    + x * (0.012_839_238_907_330_318
                        + x * (0.000_202_753_756_325_109_98
                            + x * (1.148_295_607_344_914e-6
                                + x * (2.318_837_060_567_426_4e-9
                                    + x * (1.427_199_416_574_256_4e-12
                                        + x * 1.588_483_694_239_479_6e-16))))))))
    } else if zc <= 1.587_042_981_208_229_7e6 {
        // W <= 11.809, X_6
        let x = zc.sqrt();
        Some((1.722_110_443_993_771_1
            + x * (0.399_195_942_864_842_8
                + x * (0.007_988_554_014_068_503
                    + x * (0.000_042_889_742_253_257_923
                        + x * (7.814_682_818_052_987e-8
                            + x * (4.981_963_876_435_468e-11
                                + x * (9.765_088_971_426_53e-15
                                    + x * 3.705_299_728_172_172_6e-19)))))))
            / (1.
                + x * (0.074_007_438_118_020_55
                    + x * (0.001_033_350_150_669_774_1
                        + x * (4.436_085_803_572_751e-6
                            + x * (6.782_291_231_637_104e-9
                                + x * (3.683_435_670_763_949e-12
                                    + x * (6.083_615_956_026_604e-16
                                        + x * 1.814_986_933_598_122_7e-20))))))))
    } else if zc <= 2.341_470_840_187_546e7 {
        // W <= 14.308, X_7
        let x = zc.sqrt();
        Some((3.752_931_402_343_454_3
            + x * (0.154_913_426_903_578_07
                + x * (0.000_756_631_406_759_007_9
                    + x * (1.027_160_923_596_997_8e-6
                        + x * (4.785_324_767_593_006e-10
                            + x * (7.832_804_077_027_547e-14
                                + x * (3.943_303_375_839_104e-18
                                    + x * 3.823_286_220_566_028_6e-23)))))))
            / (1.
                + x * (0.020_112_985_338_854_444
                    + x * (0.000_074_712_286_154_830_14
                        + x * (8.480_059_800_369_383e-8
                            + x * (3.418_242_413_037_691_4e-11
                                + x * (4.886_625_913_969_095_5e-15
                                    + x * (2.122_337_362_683_463_5e-19
                                        + x * 1.664_298_567_126_058_3e-24))))))))
    } else if zc <= 3.557_647_430_800_996_4e8 {
        // W <= 16.865, X_8
        let x = zc.sqrt();
        Some((6.019_654_205_560_656
            + x * (0.053_496_672_841_797_86
                + x * (0.000_064_340_849_275_316_5
                    + x * (2.196_909_010_009_596_7e-8
                        + x * (2.592_798_893_703_306_3e-12
                            + x * (1.077_919_816_180_152_7e-16
                                + x * (1.378_042_409_101_789_9e-21
                                    + x * 3.376_897_315_074_255e-27)))))))
            / (1.
                + x * (0.005_280_968_370_423_337_4
                    + x * (5.102_050_121_938_956e-6
                        + x * (1.501_831_229_227_083_1e-9
                            + x * (1.567_770_663_641_318_8e-13
                                + x * (5.799_204_123_891_188e-18
                                    + x * (6.513_317_077_032_078e-23
                                        + x * 1.320_508_013_921_340_6e-28))))))))
    } else if zc <= 5.550_171_629_616_363e9 {
        // W <= 19.468, X_9
        let x = zc.sqrt();
        Some((8.428_026_850_098_97
            + x * (0.017_155_758_546_279_713
                + x * (5.083_662_066_982_932e-6
                    + x * (4.335_490_369_183_258_4e-10
                        + x * (1.284_101_714_564_558_3e-14
                            + x * (1.341_910_676_974_588_6e-19
                                + x * (4.310_169_845_549_223e-25
                                    + x * 2.642_243_342_208_819e-31)))))))
            / (1.
                + x * (0.001_357_200_675_459_530_1
                    + x * (3.353_524_348_142_62e-7
                        + x * (2.520_696_924_642_126_4e-11
                            + x * (6.713_622_627_306_053e-16
                                + x * (6.332_422_668_085_468_6e-21
                                    + x * (1.812_816_740_001_377_6e-26
                                        + x * 9.366_203_005_813_68e-33))))))))
    } else if zc <= 8.867_470_483_965_778e10 {
        // W <= 22.112, X_10
        let x = zc.sqrt();
        Some((10.931_063_230_472_498
            + x * (0.005_222_423_454_024_553_5
                + x * (3.799_610_571_181_013e-7
                    + x * (8.030_579_353_341_036e-12
                        + x * (5.913_978_562_709_06e-17
                            + x * (1.538_202_035_953_303e-22
                                + x * (1.228_894_412_626_811e-28
                                    + x * 1.866_508_927_066_012_3e-35)))))))
            / (1.
                + x * (0.000_343_287_025_511_975_8
                    + x * (2.139_535_151_853_884_3e-8
                        + x * (4.052_417_018_663_159_3e-13
                            + x * (2.718_142_431_533_571e-18
                                + x * (6.453_898_663_835_549e-24
                                    + x * (4.649_461_378_588_898_6e-30
                                        + x * 6.044_202_436_729_939e-37))))))))
    } else if zc <= 1.447_779_186_527_290_3e12 {
        // W <= 24.791, X_11
        let x = zc.sqrt();
        Some((13.502_943_080_893_871
            + x * (0.001_528_463_650_634_626_6
                + x * (2.715_696_735_826_234_5e-8
                    + x * (1.411_039_405_124_216_2e-13
                        + x * (2.560_573_431_121_973e-19
                            + x * (1.642_129_372_442_533_8e-25
                                + x * (3.232_494_469_143_584e-32
                                    + x * 1.205_466_264_125_178_3e-39)))))))
            / (1.
                + x * (0.000_085_701_512_879_089_46
                    + x * (1.331_124_443_575_269_2e-9
                        + x * (6.278_892_444_038_535e-15
                            + x * (1.048_378_815_225_220_4e-20
                                + x * (6.194_349_996_624_916e-27
                                    + x * (1.110_156_786_034_091_8e-33
                                        + x * 3.589_738_112_830_896_4e-41))))))))
    } else if zc <= 2.411_145_863_251_185e13 {
        // W <= 27.500, X_12
        let x = zc.sqrt();
        Some((16.128_076_167_439_016
            + x * (0.000_433_603_851_764_670_7
                + x * (1.869_640_387_182_092e-9
                    + x * (2.369_179_576_690_148_7e-15
                        + x * (1.050_319_182_696_315_4e-21
                            + x * (1.646_192_757_360_676_3e-28
                                + x * (7.913_827_608_347_452e-36
                                    + x * 7.184_589_034_370_167e-44)))))))
            / (1.
                + x * (0.000_021_154_255_263_102_94
                    + x * (8.100_611_544_232_328e-11
                        + x * (9.415_598_602_216_99e-17
                            + x * (3.872_512_790_229_53e-23
                                + x * (5.634_465_111_557_057e-30
                                    + x * (2.486_095_108_421_003e-37
                                        + x * 1.978_830_473_742_778_7e-45))))))))
    } else if zc <= 4.089_703_644_260_084_4e14 {
        // W <= 30.236, X_13
        let x = zc.sqrt();
        Some((18.796_301_105_534_486
            + x * (0.000_119_894_433_396_464_69
                + x * (1.246_337_752_867_686_3e-10
                    + x * (3.821_945_685_801_037e-17
                        + x * (4.105_569_393_025_208e-24
                            + x * (1.559_523_145_604_846_4e-31
                                + x * (1.815_717_355_307_799e-39
                                    + x * 3.980_799_776_432_617e-48)))))))
            / (1.
                + x * (5.169_103_198_835_992e-6
                    + x * (4.832_557_182_331_371e-12
                        + x * (1.370_788_874_691_692_8e-18
                            + x * (1.375_456_085_002_448e-25
                                + x * (4.881_188_297_566_18e-33
                                    + x * (5.251_864_182_817_021e-41
                                        + x * 1.019_211_959_313_475_6e-49))))))))
    } else if zc <= 7.055_590_147_678_997e15 {
        // W <= 32.996, X_14
        let x = zc.sqrt();
        Some((21.500_582_830_667_334
            + x * (0.000_032_441_943_237_735_277
                + x * (8.076_496_341_683_755e-12
                    + x * (5.948_844_550_612_289e-19
                        + x * (1.536_410_618_721_586e-26
                            + x * (1.403_323_129_700_238_7e-34
                                + x * (3.925_987_271_230_577e-43
                                    + x * 2.062_908_638_225_773_6e-52)))))))
            / (1.
                + x * (1.251_531_764_243_385e-6
                    + x * (2.831_031_421_481_707e-13
                        + x * (1.942_366_641_612_364e-20
                            + x * (4.712_861_600_415_736e-28
                                + x * (4.043_334_739_183_994_4e-36
                                    + x * (1.051_514_144_383_118_8e-44
                                        + x * 4.931_649_093_543_692_6e-54))))))))
    } else if zc <= 1.236_660_755_797_672_8e17 {
        // W <= 35.779, X_15
        let x = zc.sqrt();
        Some((24.235_812_532_416_976
            + x * (8.616_150_599_577_68e-6
                + x * (5.103_343_156_186_827e-13
                    + x * (8.964_239_366_584_964e-21
                        + x * (5.525_436_418_109_742e-29
                            + x * (1.204_507_272_405_060_6e-37
                                + x * (8.037_299_717_652_684e-47
                                    + x * 1.004_914_081_214_649_3e-56)))))))
            / (1.
                + x * (3.004_676_184_474_948e-7
                    + x * (1.630_910_427_085_546_4e-14
                        + x * (2.684_227_103_029_893e-22
                            + x * (1.561_967_263_245_888_1e-30
                                + x * (3.213_168_903_039_798_6e-39
                                    + x * (2.003_239_624_530_768_4e-48
                                        + x * 2.252_027_455_467_633e-58))))))))
    } else if zc <= 2.199_937_348_793_1e18 {
        // W <= 38.582, X_16
        let x = zc.sqrt();
        Some((26.998_134_347_987_44
            + x * (2.251_225_776_757_228_4e-6
                + x * (3.152_123_075_986_696_7e-14
                    + x * (1.311_403_571_979_063e-22
                        + x * (1.915_678_403_396_236_5e-31
                            + x * (9.896_700_305_344_48e-41
                                + x * (1.564_042_389_844_843_4e-50
                                    + x * 4.621_619_304_066_487e-61)))))))
            / (1.
                + x * (7.157_267_637_090_758e-8
                    + x * (9.250_050_609_111_575e-16
                        + x * (3.623_981_958_278_757_5e-24
                            + x * (5.018_771_249_380_042e-33
                                + x * (2.456_586_198_821_807e-42
                                    + x * (3.643_565_843_399_166e-52
                                        + x * 9.743_249_064_015_534e-63))))))))
    } else if zc <= 3.968_539_219_834_401_6e19 {
        // W <= 41.404, X_17
        let x = zc.sqrt();
        Some((29.784_546_702_831_97
            + x * (5.797_176_439_217_133e-7
                + x * (1.906_987_279_260_195e-15
                    + x * (1.866_870_087_085_876_3e-24
                        + x * (6.420_051_095_337_094e-34
                            + x * (7.807_662_465_081_897e-44
                                + x * (2.902_963_869_695_631_7e-54
                                    + x * 2.014_187_045_856_618e-65)))))))
            / (1.
                + x * (1.692_446_318_046_970_5e-8
                    + x * (5.170_393_431_125_454e-17
                        + x * (4.787_153_272_156_007e-26
                            + x * (1.566_440_583_254_515e-35
                                + x * (1.811_313_798_238_133_2e-45
                                    + x * (6.345_415_028_949_542e-56
                                        + x * 4.007_296_402_524_44e-67))))))))
    } else if zc <= 1.412_707_514_527_465_2e104 {
        // W <= 234.358, U_18
        let y = zc.ln();
        Some((0.744_134_994_601_267_8
            + y * (0.414_032_436_180_059_14
                + y * (0.260_125_641_667_734_16
                    + y * (0.021_450_457_095_960_294
                        + y * (0.000_518_723_772_647_059_1
                            + y * (4.357_469_356_831_998e-6
                                + y * (1.236_306_605_892_170_7e-8
                                    + y * 9.019_414_776_630_996e-12)))))))
            / (1.
                + y * (0.334_878_110_674_670_1
                    + y * (0.023_756_834_394_570_627
                        + y * (0.000_542_256_330_089_077_3
                            + y * (4.437_898_005_257_962e-6
                                + y * (1.243_658_549_766_81e-8
                                    + y * (9.022_582_586_763_186e-12
                                        + y * (-4.205_783_627_010_972e-19)))))))))
    } else {
        //   U_19
        let y = zc.ln();
        Some((-0.615_144_128_127_297_6
            + y * (0.679_793_101_336_309_3
                + y * (0.089_685_353_704_585_82
                    + y * (0.001_564_494_148_398_938
                        + y * (7.734_990_187_817_636e-6
                            + y * (1.289_164_754_669_943_5e-8
                                + y * (7.089_032_598_897_381_6e-12
                                    + y * 9.841_979_033_427_972e-16)))))))
            / (1.
                + y * (0.097_300_263_710_401_44
                    + y * (0.001_610_367_274_844_206
                        + y * (7.824_774_100_307_7e-6
                            + y * (1.294_926_130_897_134_6e-8
                                + y * (7.098_691_121_934_283e-12
                                    + y * (9.842_628_504_222_704e-16
                                        + y * (-1.596_014_725_260_605_6e-24)))))))))
    }
}
