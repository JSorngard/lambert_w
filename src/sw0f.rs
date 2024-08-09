use super::NEG_INV_E as Z0;
use crate::{
    elementary::{lnf, sqrtf},
    rational::{rational_3f, rational_4_3f},
};

const NEG_INV_E: f32 = Z0 as f32;

pub fn sw0f(z: f32) -> f32 {
    if z < NEG_INV_E {
        f32::NAN
    } else if z <= 2.008_217_811_584_472_7 {
        // W <= 0.854, X_1

        rational_4_3f(
            sqrtf(z - NEG_INV_E),
            [
                -0.999_999_940_395_401_9,
                0.055_730_052_161_777_8,
                2.126_973_249_105_317_3,
                0.813_511_236_783_528_8,
                0.016_324_880_146_070_16,
            ],
            [
                1.0,
                2.275_906_559_863_465,
                1.367_597_013_868_904,
                0.186_158_234_528_316_23,
            ],
        )
    } else if z <= 30.539_142_109_510_895 {
        // W <= 2.502, X_2

        rational_3f(
            sqrtf(z - NEG_INV_E),
            [
                -0.985_519_709_059_991,
                1.077_497_573_381_351_7,
                0.871_751_030_681_775,
                0.054_352_728_608_275_766,
            ],
            [
                1.0,
                1.186_101_403_701_543_4,
                0.249_962_984_308_281_62,
                0.006_881_368_648_675_912,
            ],
        )
    } else if z <= 371.669_843_713_757_76 {
        // W <= 4.430, X_3

        rational_3f(
            sqrtf(z - NEG_INV_E),
            [
                -0.762_397_113_463_688_9,
                1.231_773_161_336_359_6,
                0.243_424_471_130_566_95,
                0.004_320_601_393_878_236,
            ],
            [
                1.0,
                0.579_386_215_035_869_1,
                0.046_601_427_736_078_775,
                0.000_435_128_175_674_741_1,
            ],
        )
    } else if z <= 4_705.918_954_265_969 {
        // W <= 6.574, X_4

        rational_3f(
            sqrtf(z - NEG_INV_E),
            [
                0.085_801_247_434_391_38,
                0.825_397_980_997_483_4,
                0.039_781_960_760_329_076,
                0.000_187_855_789_275_838,
            ],
            [
                1.0,
                0.213_380_768_170_801_4,
                0.005_462_672_039_792_693_5,
                0.000_015_449_534_481_294_754,
            ],
        )
    } else if z <= 64_640.797_355_310_09 {
        // W <= 8.892, X_5

        rational_3f(
            sqrtf(z - NEG_INV_E),
            [
                1.621_924_538_347_016_9,
                0.388_691_451_325_166_64,
                0.004_575_064_267_850_351_5,
                5.538_467_214_864_45e-6,
            ],
            [
                1.0,
                0.065_219_460_735_182_41,
                0.000_478_827_607_890_225_1,
                3.809_482_814_629_24e-7,
            ],
        )
    } else if z <= 965_649.030_871_163_2 {
        // W <= 11.351, X_6

        rational_3f(
            sqrtf(z - NEG_INV_E),
            [
                3.621_899_608_569_592,
                0.148_846_467_548_801_6,
                0.000_424_696_224_099_984,
                1.279_017_971_037_421_7e-7,
            ],
            [
                1.0,
                0.017_985_659_319_608_747,
                0.000_035_446_449_757_357_85,
                7.506_249_296_303_705e-9,
            ],
        )
    } else if z <= 1.559_333_422_803_816_6e7 {
        // W <= 13.928, X_7

        rational_3f(
            sqrtf(z - NEG_INV_E),
            [
                5.907_336_973_960_809,
                0.050_053_653_594_737_11,
                0.000_034_072_148_625_204_7,
                2.481_206_469_365_548_5e-9,
            ],
            [
                1.0,
                0.004_655_899_001_684_321,
                2.344_944_586_080_881e-6,
                1.263_142_996_480_846_2e-10,
            ],
        )
    } else if z <= 2.702_564_027_724_190_4e8 {
        // W <= 16.605, X_8

        rational_3f(
            sqrtf(z - NEG_INV_E),
            [
                8.382_600_584_819_551,
                0.015_360_346_475_232_501,
                2.443_338_439_786_936_6e-6,
                4.185_680_326_411_855e-11,
            ],
            [
                1.0,
                0.001_150_742_322_378_586_9,
                1.422_142_847_481_351_6e-7,
                1.873_917_202_662_012_4e-12,
            ],
        )
    } else if z <= 4.995_018_739_704_195e9 {
        // W <= 19.368, X_9

        rational_3f(
            sqrtf(z - NEG_INV_E),
            [
                10.996_674_803_992_551,
                0.004_394_213_889_867_383,
                1.596_666_535_484_678e-7,
                6.266_538_284_496_873e-13,
            ],
            [
                1.0,
                0.000_273_837_576_757_036_47,
                8.015_706_231_969_03e-9,
                2.495_698_215_887_173e-14,
            ],
        )
    } else if z <= 9.791_115_441_672_696e10 {
        // W <= 22.207, X_10

        rational_3f(
            sqrtf(z - NEG_INV_E),
            [
                13.719_833_802_350_86,
                0.001_187_444_380_520_229_2,
                9.630_338_120_016_468e-9,
                8.443_452_423_226_163e-15,
            ],
            [
                1.0,
                0.000_063_056_372_424_395_35,
                4.235_876_603_109_884_3e-10,
                3.020_540_500_543_447_3e-16,
            ],
        )
    } else if z <= 2.025_975_385_630_21e12 {
        // W <= 25.114, X_11

        rational_3f(
            sqrtf(z - NEG_INV_E),
            [
                16.533_119_481_561_616,
                0.000_305_831_257_519_080_4,
                5.411_294_663_372_01e-10,
                1.034_713_033_370_471_1e-16,
            ],
            [
                1.0,
                0.000_014_099_161_212_376_34,
                2.112_109_541_235_469_5e-11,
                3.352_692_715_745_247e-18,
            ],
        )
    } else if z <= 4.407_744_425_147_794e13 {
        // W <= 28.082, X_12

        rational_3f(
            sqrtf(z - NEG_INV_E),
            [
                19.423_519_260_478_578,
                0.000_075_559_269_761_977_81,
                2.853_002_312_078_307_5e-11,
                1.162_962_709_646_357_9e-18,
            ],
            [
                1.0,
                3.069_209_278_972_785_8e-6,
                9.986_661_305_031_147e-13,
                3.437_671_711_698_392e-20,
            ],
        )
    } else if z <= 1.004_838_215_057_150_5e15 {
        // W <= 31.106, X_13

        rational_3f(
            sqrtf(z - NEG_INV_E),
            [
                22.381_576_050_041_915,
                0.000_017_994_724_029_162_553,
                1.419_487_642_040_223e-12,
                1.207_110_515_438_583e-20,
            ],
            [
                1.0,
                6.518_396_280_665_677e-7,
                4.495_866_571_281_253_6e-14,
                3.275_542_924_502_358e-22,
            ],
        )
    } else if z <= 2.393_255_260_235_983_6e16 {
        // W <= 34.182, X_14

        rational_3f(
            sqrtf(z - NEG_INV_E),
            [
                25.400_105_417_092_067,
                4.146_737_838_657_924e-6,
                6.696_269_721_968_18e-14,
                1.163_790_515_950_647_6e-22,
            ],
            [
                1.0,
                1.352_980_135_703_041_8e-7,
                1.933_608_178_532_575e-15,
                2.914_939_619_981_625_7e-24,
            ],
        )
    } else if z <= 5.939_799_659_746_575e17 {
        // W <= 37.306, X_15

        rational_3f(
            sqrtf(z - NEG_INV_E),
            [
                28.473_455_626_379_916,
                9.274_682_469_309_406e-7,
                3.006_899_015_933_680_6e-15,
                1.047_355_759_182_202_7e-24,
            ],
            [
                1.0,
                2.748_648_970_452_172_8e-8,
                7.967_898_707_103_613e-17,
                2.433_166_636_706_153e-26,
            ],
        )
    } else if z <= 1.532_693_858_990_176_7e19 {
        // W <= 40.475, X_16

        rational_3f(
            sqrtf(z - NEG_INV_E),
            [
                31.597_055_437_846_36,
                2.018_422_527_678_632_4e-7,
                1.289_578_819_651_291e-16,
                8.836_117_471_410_11e-27,
            ],
            [
                1.0,
                5.472_394_512_609_85e-9,
                3.153_772_917_992_919e-18,
                1.912_203_513_257_167e-28,
            ],
        )
    } else if z <= 4.103_565_939_888_539_6e20 {
        // W <= 43.687, X_17

        rational_3f(
            sqrtf(z - NEG_INV_E),
            [
                34.767_124_490_414_52,
                4.283_079_924_069_894_4e-8,
                5.297_588_412_103_653e-18,
                7.014_551_539_215_878e-29,
            ],
            [
                1.0,
                1.068_930_112_769_633_2e-9,
                1.201_669_906_178_942_4e-19,
                1.419_524_481_080_098_4e-30,
            ],
        )
    } else {
        //    U_19

        rational_3f(
            lnf(z),
            [
                -3.132_005_602_886_366,
                0.948_894_657_265_326,
                0.008_317_815_296_164_44,
                5.558_784_815_783_349e-6,
            ],
            [
                1.0,
                0.008_365_681_867_773_006,
                5.559_715_493_597_327e-6,
                -3.748_153_583_315_12e-14,
            ],
        )
    }
}
