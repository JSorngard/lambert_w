use super::{INV_SQRT_E as X0, NEG_INV_E as Z0};
use crate::{
    elementary::{lnf, sqrtf},
    rational::rational_3f,
};

const INV_SQRT_E: f32 = X0 as f32;
const NEG_INV_E: f32 = Z0 as f32;

pub fn swm1f(z: f32) -> f32 {
    if z < NEG_INV_E {
        f32::NAN
    } else if z <= -0.207_293_777_640_384_15 {
        // W >= -2.483, Y_-1

        rational_3f(
            -z / (INV_SQRT_E + sqrtf(z - NEG_INV_E)),
            [
                -6.383_722_822_801_905,
                -74.968_653_259_594_05,
                -19.714_821_552_432_483,
                70.677_326_667_809_24,
            ],
            [
                1.0,
                24.295_836_951_878_69,
                64.112_460_611_386_04,
                17.994_497_369_039_312,
            ],
        )
    } else if z <= -0.071_507_705_083_841_95 {
        // W >= -4.032, Y_-2

        rational_3f(
            -z / (INV_SQRT_E + sqrtf(z - NEG_INV_E)),
            [
                -7.723_328_481_229_978,
                -352.484_690_970_429_14,
                -1_242.008_890_368_567_8,
                1_171.647_596_062_049_8,
            ],
            [
                1.0,
                77.681_242_588_997_4,
                648.564_312_140_752_5,
                566.701_549_764_361_6,
            ],
        )
    } else if z <= -0.020_704_412_621_717_48 {
        // W >= -5.600, Y_-3

        rational_3f(
            -z / (INV_SQRT_E + sqrtf(z - NEG_INV_E)),
            [
                -9.137_773_141_758_155,
                -1_644.724_479_150_889,
                -28_105.096_098_779_683,
                3_896.079_810_390_921_5,
            ],
            [
                1.0,
                272.375_261_351_239_7,
                7_929.224_261_291_35,
                23_980.122_860_821_313,
            ],
        )
    } else if z <= -0.005_480_012_945_209_444 {
        // W >= -7.178, Y_-4

        rational_3f(
            -z / (INV_SQRT_E + sqrtf(z - NEG_INV_E)),
            [
                -10.603_388_239_566_373,
                -7_733.348_521_498_648,
                -575_482.407_079_644_3,
                -2.154_552_604_188_978e6,
            ],
            [
                1.0,
                1_021.793_856_606_681_7,
                111_300.229_154_865_21,
                1.261_425_640_008_844_2e6,
            ],
        )
    } else if z <= -0.001_367_466_989_250_804_2 {
        // W >= -8.766, Y_-5

        rational_3f(
            -z / (INV_SQRT_E + sqrtf(z - NEG_INV_E)),
            [
                -12.108_699_273_343_438,
                -36_896.535_108_166_376,
                -1.183_112_672_010_605_4e7,
                -2.756_583_081_394_092_4e8,
            ],
            [
                1.0,
                4_044.975_306_488_07,
                1.741_827_761_903_000_5e6,
                7.843_690_738_080_69e7,
            ],
        )
    } else if z <= -0.000_326_142_267_310_725_66 {
        // W >= -10.367, Y_-6

        rational_3f(
            -z / (INV_SQRT_E + sqrtf(z - NEG_INV_E)),
            [
                -13.646_761_936_746_191,
                -179_086.115_857_151_48,
                -2.508_463_493_521_464_2e8,
                -2.934_370_049_483_371_4e10,
            ],
            [
                1.0,
                16_743.826_607_737_14,
                2.980_965_094_601_174_4e7,
                5.573_951_481_695_8e9,
            ],
        )
    } else if z <= -0.000_074_906_612_036_101_44 {
        // W >= -11.983, Y_-7

        rational_3f(
            -z / (INV_SQRT_E + sqrtf(z - NEG_INV_E)),
            [
                -15.212_958_142_001_646,
                -884_954.687_981_689_6,
                -5.529_815_437_863_348e9,
                -3.093_418_743_531_467e12,
            ],
            [
                1.0,
                72_009.255_525_210_12,
                5.505_900_767_187_58e8,
                4.432_489_486_700_034e11,
            ],
        )
    } else if z <= -1.096_244_452_641_099_5e-19 {
        // W >= -47.518, V_-8

        rational_3f(
            lnf(-z),
            [
                -0.032_401_163_177_791_084,
                2.028_194_214_474_250_4,
                -0.527_524_312_425_927_1,
                0.017_340_294_772_717_584,
            ],
            [
                1.0,
                -0.450_042_744_438_917_4,
                0.017_154_705_753_566_295,
                -5.243_819_620_271_836e-7,
            ],
        )
    } else if z <= -2.509_609_929_994_59e-136 {
        // W >= -317.993, V_-9

        rational_3f(
            lnf(-z),
            [
                -1.441_124_659_581_209_7,
                1.281_926_963_998_047_7,
                -0.074_979_356_113_812_33,
                0.000_476_363_091_620_691_5,
            ],
            [
                1.0,
                -0.072_000_873_723_868_65,
                0.000_475_489_329_895_970_3,
                -4.171_497_924_754_684e-10,
            ],
        )
    } else if z < 0.0 {
        // V_-10

        rational_3f(
            lnf(-z),
            [
                -3.310_876_091_171_045,
                1.050_067_880_993_517_6,
                -0.008_236_749_582_134_32,
                5.528_956_159_491_019e-6,
            ],
            [
                1.0,
                -0.008_189_272_743_331_552,
                5.528_007_600_971_195e-6,
                -3.922_277_308_457_406_3e-14,
            ],
        )
    } else {
        f32::NAN
    }
}
