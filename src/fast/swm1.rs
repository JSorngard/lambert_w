//! The original implementation of the secondary branch of the Lambert W function by Toshio Fukushima, accurate to 24 bits, ported to Rust.

use super::super::{X0, Z0};
use crate::{LambertWm1Error, LambertWm1ErrorReason};

/// 24-bit accuracy computation of secondary branch of the Lambert W function, W_-1(z),
/// defined as the solution of nonlinear equation, W exp(W) = z, when W < -1
/// by piecewise minimax rational function approximation
///
/// Created by T. Fukushima,
/// ported to Rust by Johanna Sörngård
///
/// Reference: T. Fukushima (2020) to be submitted
///  "Precise and fast computation of Lambert W-functions by piecewise
///   rational function approximation with variable transformation"
pub fn swm1(z: f64) -> Result<f64, LambertWm1Error> {
    if z < Z0 {
        Err(LambertWm1Error::new(
            LambertWm1ErrorReason::TooSmallArgument,
        ))
    } else if z <= -0.207_293_777_640_384_15 {
        // W >= -2.483, Y_-1
        let x = -z / (X0 + (z - Z0).sqrt());
        Ok((-6.383_722_822_801_905
            + x * (-74.968_653_259_594_05
                + x * (-19.714_821_552_432_483 + x * 70.677_326_667_809_24)))
            / (1.
                + x * (24.295_836_951_878_69
                    + x * (64.112_460_611_386_04 + x * 17.994_497_369_039_312))))
    } else if z <= -0.071_507_705_083_841_95 {
        // W >= -4.032, Y_-2
        let x = -z / (X0 + (z - Z0).sqrt());
        Ok((-7.723_328_481_229_978
            + x * (-352.484_690_970_429_14
                + x * (-1_242.008_890_368_567_8 + x * 1_171.647_596_062_049_8)))
            / (1.
                + x * (77.681_242_588_997_4
                    + x * (648.564_312_140_752_5 + x * 566.701_549_764_361_6))))
    } else if z <= -0.020_704_412_621_717_48 {
        // W >= -5.600, Y_-3
        let x = -z / (X0 + (z - Z0).sqrt());
        Ok((-9.137_773_141_758_155
            + x * (-1_644.724_479_150_889
                + x * (-28_105.096_098_779_683 + x * 3_896.079_810_390_921_5)))
            / (1.
                + x * (272.375_261_351_239_7
                    + x * (7_929.224_261_291_35 + x * 23_980.122_860_821_313))))
    } else if z <= -0.005_480_012_945_209_444 {
        // W >= -7.178, Y_-4
        let x = -z / (X0 + (z - Z0).sqrt());
        Ok((-10.603_388_239_566_373
            + x * (-7_733.348_521_498_648
                + x * (-575_482.407_079_644_3 + x * (-2.154_552_604_188_978e6))))
            / (1.
                + x * (1_021.793_856_606_681_7
                    + x * (111_300.229_154_865_21 + x * 1.261_425_640_008_844_2e6))))
    } else if z <= -0.001_367_466_989_250_804_2 {
        // W >= -8.766, Y_-5
        let x = -z / (X0 + (z - Z0).sqrt());
        Ok((-12.108_699_273_343_438
            + x * (-36_896.535_108_166_376
                + x * (-1.183_112_672_010_605_4e7 + x * (-2.756_583_081_394_092_4e8))))
            / (1.
                + x * (4_044.975_306_488_07
                    + x * (1.741_827_761_903_000_5e6 + x * 7.843_690_738_080_69e7))))
    } else if z <= -0.000_326_142_267_310_725_66 {
        // W >= -10.367, Y_-6
        let x = -z / (X0 + (z - Z0).sqrt());
        Ok((-13.646_761_936_746_191
            + x * (-179_086.115_857_151_48
                + x * (-2.508_463_493_521_464_2e8 + x * (-2.934_370_049_483_371_4e10))))
            / (1.
                + x * (16_743.826_607_737_14
                    + x * (2.980_965_094_601_174_4e7 + x * 5.573_951_481_695_8e9))))
    } else if z <= -0.000_074_906_612_036_101_44 {
        // W >= -11.983, Y_-7
        let x = -z / (X0 + (z - Z0).sqrt());
        Ok((-15.212_958_142_001_646
            + x * (-884_954.687_981_689_6
                + x * (-5.529_815_437_863_348e9 + x * (-3.093_418_743_531_467e12))))
            / (1.
                + x * (72_009.255_525_210_12
                    + x * (5.505_900_767_187_58e8 + x * 4.432_489_486_700_034e11))))
    } else if z <= -1.096_244_452_641_099_5e-19 {
        // W >= -47.518, V_-8
        let u = (-z).ln();
        Ok((-0.032_401_163_177_791_084
            + u * (2.028_194_214_474_250_4
                + u * (-0.527_524_312_425_927_1 + u * 0.017_340_294_772_717_584)))
            / (1.
                + u * (-0.450_042_744_438_917_4
                    + u * (0.017_154_705_753_566_295 + u * (-5.243_819_620_271_836e-7)))))
    } else if z <= -2.509_609_929_994_59e-136 {
        // W >= -317.993, V_-9
        let u = (-z).ln();
        Ok((-1.441_124_659_581_209_7
            + u * (1.281_926_963_998_047_7
                + u * (-0.074_979_356_113_812_33 + u * 0.000_476_363_091_620_691_5)))
            / (1.
                + u * (-0.072_000_873_723_868_65
                    + u * (0.000_475_489_329_895_970_3 + u * (-4.171_497_924_754_684e-10)))))
    } else if z < 0.0 {
        // V_-10
        let u = (-z).ln();
        Ok((-3.310_876_091_171_045
            + u * (1.050_067_880_993_517_6
                + u * (-0.008_236_749_582_134_32 + u * 5.528_956_159_491_019e-6)))
            / (1.
                + u * (-0.008_189_272_743_331_552
                    + u * (5.528_007_600_971_195e-6 + u * (-3.922_277_308_457_406_3e-14)))))
    } else {
        Err(LambertWm1Error::new(
            LambertWm1ErrorReason::PositiveArgument,
        ))
    }
}
