//! # lambert_w
//!
//! Fast evaluation of the real valued parts of the principal and secondary branches of the [Lambert W function](https://en.wikipedia.org/wiki/Lambert_W_function) using the [method of Toshio Fukushima](https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation) to either 24 or 50 bits of accuracy.
//!
//! This method uses a piecewise minimax rational function approximation.
//!
//! ## Examples
//!
//! Compute the value of the [Omega constant](https://en.wikipedia.org/wiki/Omega_constant) with the principal branch of the Lambert W function to 50 bits of accuracy:
#![cfg_attr(
    feature = "50bits",
    doc = r##"
```
use lambert_w::accurate::lambert_w_0;

use approx::assert_abs_diff_eq;

let w = lambert_w_0(1.0).unwrap();

assert_abs_diff_eq!(w, 0.5671432904097838);
```
"##
)]
//!
//! or to only 24 bits of accuracy, but with faster execution time:
#![cfg_attr(
    feature = "24bits",
    doc = r##"
```
use lambert_w::fast::lambert_w_0;

use approx::assert_abs_diff_eq;

let w = lambert_w_0(1.0).unwrap();

assert_abs_diff_eq!(w, 0.5671432904097838, epsilon = 1e-7);
```
"##
)]
//!
//! ## Speed-accuracy trade-off
//!
//! The 50-bit accurate versions in the [`accurate`] module are more accurate, but slightly slower, than the 24-bit accurate versions in the [`fast`] module.
//! [`fast::lambert_w_0`] is around 15% faster than [`accurate::lambert_w_0`] and [`fast::lambert_w_m1`] is around 41% faster than [`accurate::lambert_w_m1`].
//!
//! ## Feature flags
//!
//! You can disable one of these feature flags to potentially save a little bit of binary size.
//!
//! `50bits` *(enabled by default)*: enables the more accurate function versions with 50 bits of accuracy.
//!
//! `24bits` *(enabled by default)*: enables the faster function versions with 24 bits of accuracy.
//!
//! It is a compile error to disable both features.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(not(any(feature = "50bits", feature = "24bits")))]
compile_error!("one or both of the '24bits' and '50bits' features must be enabled");

#[cfg(feature = "50bits")]
pub mod accurate;
#[cfg(feature = "24bits")]
pub mod fast;

// -1/e
const Z0: f64 = -0.367_879_441_171_442_33;

// 1/sqrt(e)
const X0: f64 = 0.606_530_659_712_633_4;

#[cfg(all(test, any(feature = "24bits", feature = "50bits")))]
mod tets {
    #[cfg(feature = "50bits")]
    use super::accurate::{lambert_w_0 as lambert_w_0_50, lambert_w_m1 as lambert_w_m1_50};
    #[cfg(feature = "24bits")]
    use super::fast::{lambert_w_0 as lambert_w_0_24, lambert_w_m1 as lambert_w_m1_24};
    use approx::assert_abs_diff_eq;
    use core::f64::consts::E;

    #[cfg(feature = "50bits")]
    #[test]
    fn test_lambert_w_0_50() {
        assert_eq!(lambert_w_0_50(-1.0 / E - f64::EPSILON), None);
        assert_abs_diff_eq!(
            lambert_w_0_50(-2.678794411714424e-01).unwrap(),
            -3.993824525397807e-01
        );
        assert_abs_diff_eq!(
            lambert_w_0_50(6.321205588285577e-01).unwrap(),
            4.167039988177658e-01
        );
        assert_abs_diff_eq!(
            lambert_w_0_50(9.632120558828557).unwrap(),
            1.721757710976171
        );
        assert_abs_diff_eq!(
            lambert_w_0_50(9.963212055882856e+01).unwrap(),
            3.382785211058958
        );
        assert_abs_diff_eq!(
            lambert_w_0_50(9.996321205588285e+02).unwrap(),
            5.249293782013269
        );
        assert_abs_diff_eq!(
            lambert_w_0_50(9.999632120558828e+03).unwrap(),
            7.231813718542178,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_0_50(9.999963212055883e+04).unwrap(),
            9.284568107521959
        );
        assert_abs_diff_eq!(
            lambert_w_0_50(9.999996321205589e+05).unwrap(),
            1.138335774796812e+01
        );
        assert_abs_diff_eq!(
            lambert_w_0_50(9.999999632120559e+06).unwrap(),
            1.351434397605273e+01
        );
        assert_abs_diff_eq!(
            lambert_w_0_50(9.999999963212056e+07).unwrap(),
            1.566899671199287e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_0_50(9.999999996321206e+08).unwrap(),
            1.784172596707312e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_0_50(9.999999999632120e+09).unwrap(),
            2.002868541326992e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_0_50(9.999999999963213e+10).unwrap(),
            2.222712273495755e+01
        );
        assert_abs_diff_eq!(
            lambert_w_0_50(9.999999999996321e+11).unwrap(),
            2.443500440493456e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_0_50(9.999999999999633e+12).unwrap(),
            2.665078750870219e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_0_50(9.999999999999962e+13).unwrap(),
            2.887327487929930e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_0_50(9.999999999999996e+14).unwrap(),
            3.110151971159478e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_0_50(1.000000000000000e+16).unwrap(),
            3.333476076844818e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_0_50(1.000000000000000e+17).unwrap(),
            3.557237716651325e+01
        );
        assert_abs_diff_eq!(
            lambert_w_0_50(1.000000000000000e+18).unwrap(),
            3.781385607558877e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_0_50(1.000000000000000e+19).unwrap(),
            4.005876916198432e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_0_50(1.000000000000000e+20).unwrap(),
            4.230675509173839e+01
        );
        assert_abs_diff_eq!(
            lambert_w_0_50(1.000000000000000e+40).unwrap(),
            8.763027715194720e+01
        );
        assert_abs_diff_eq!(
            lambert_w_0_50(1.000000000000000e+80).unwrap(),
            1.790193137415062e+02,
            epsilon = 1e-13
        );
        assert_abs_diff_eq!(
            lambert_w_0_50(1.000000000000000e+120).unwrap(),
            2.707091661024979e+02,
            epsilon = 1e-13
        );
        assert_abs_diff_eq!(
            lambert_w_0_50(1.000000000000000e+160).unwrap(),
            3.625205337614976e+02
        );
    }

    #[cfg(feature = "24bits")]
    #[test]
    fn test_lambert_w_0_24() {
        assert_eq!(lambert_w_0_24(-1.0 / E - f64::EPSILON), None);
        assert_abs_diff_eq!(
            lambert_w_0_24(-2.678794411714424e-01).unwrap(),
            -3.993824525397807e-01,
            epsilon = 1e-7
        );
        assert_abs_diff_eq!(
            lambert_w_0_24(6.321205588285577e-01).unwrap(),
            4.167039988177658e-01,
            epsilon = 1e-7
        );
        assert_abs_diff_eq!(
            lambert_w_0_24(9.632120558828557).unwrap(),
            1.721757710976171,
            epsilon = 1e-7
        );
        assert_abs_diff_eq!(
            lambert_w_0_24(9.963212055882856e+01).unwrap(),
            3.382785211058958,
            epsilon = 1e-7
        );
        assert_abs_diff_eq!(
            lambert_w_0_24(9.996321205588285e+02).unwrap(),
            5.249293782013269,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            lambert_w_0_24(9.999632120558828e+03).unwrap(),
            7.231813718542178,
            epsilon = 1e-7
        );
        assert_abs_diff_eq!(
            lambert_w_0_24(9.999963212055883e+04).unwrap(),
            9.284568107521959,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            lambert_w_0_24(9.999996321205589e+05).unwrap(),
            1.138335774796812e+01,
            epsilon = 1e-8
        );
        assert_abs_diff_eq!(
            lambert_w_0_24(9.999999632120559e+06).unwrap(),
            1.351434397605273e+01,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            lambert_w_0_24(9.999999963212056e+07).unwrap(),
            1.566899671199287e+01,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            lambert_w_0_24(9.999999996321206e+08).unwrap(),
            1.784172596707312e+01,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            lambert_w_0_24(9.999999999632120e+09).unwrap(),
            2.002868541326992e+01,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            lambert_w_0_24(9.999999999963213e+10).unwrap(),
            2.222712273495755e+01,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            lambert_w_0_24(9.999999999996321e+11).unwrap(),
            2.443500440493456e+01,
            epsilon = 1e-5
        );
        assert_abs_diff_eq!(
            lambert_w_0_24(9.999999999999633e+12).unwrap(),
            2.665078750870219e+01,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            lambert_w_0_24(9.999999999999962e+13).unwrap(),
            2.887327487929930e+01,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            lambert_w_0_24(9.999999999999996e+14).unwrap(),
            3.110151971159478e+01,
            epsilon = 1e-5
        );
        assert_abs_diff_eq!(
            lambert_w_0_24(1.000000000000000e+16).unwrap(),
            3.333476076844818e+01,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            lambert_w_0_24(1.000000000000000e+17).unwrap(),
            3.557237716651325e+01,
            epsilon = 1e-5
        );
        assert_abs_diff_eq!(
            lambert_w_0_24(1.000000000000000e+18).unwrap(),
            3.781385607558877e+01,
            epsilon = 1e-5
        );
        assert_abs_diff_eq!(
            lambert_w_0_24(1.000000000000000e+19).unwrap(),
            4.005876916198432e+01,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            lambert_w_0_24(1.000000000000000e+20).unwrap(),
            4.230675509173839e+01,
            epsilon = 1e-5
        );
        assert_abs_diff_eq!(
            lambert_w_0_24(1.000000000000000e+40).unwrap(),
            8.763027715194720e+01,
            epsilon = 1e-5
        );
        assert_abs_diff_eq!(
            lambert_w_0_24(1.000000000000000e+80).unwrap(),
            1.790193137415062e+02,
            epsilon = 1e-5
        );
        assert_abs_diff_eq!(
            lambert_w_0_24(1.000000000000000e+120).unwrap(),
            2.707091661024979e+02,
            epsilon = 1e-4
        );
        assert_abs_diff_eq!(
            lambert_w_0_24(1.000000000000000e+160).unwrap(),
            3.625205337614976e+02,
            epsilon = 1e-4
        );
    }

    #[cfg(feature = "50bits")]
    #[test]
    fn test_lambert_w_m1_50() {
        assert_eq!(lambert_w_m1_50(-1.0 / E - f64::EPSILON), None);
        assert_abs_diff_eq!(
            lambert_w_m1_50(-3.578794411714423e-01).unwrap(),
            -1.253493791367214,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_m1_50(-2.678794411714424e-01).unwrap(),
            -2.020625228775403,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_m1_50(-1.000000000000000e-01).unwrap(),
            -3.577152063957297
        );
        assert_abs_diff_eq!(
            lambert_w_m1_50(-3.000000000000000e-02).unwrap(),
            -5.144482721515681
        );
        assert_abs_diff_eq!(
            lambert_w_m1_50(-1.000000000000000e-02).unwrap(),
            -6.472775124394005,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_m1_50(-3.000000000000000e-03).unwrap(),
            -7.872521380098709,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_m1_50(-1.000000000000000e-03).unwrap(),
            -9.118006470402742,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_m1_50(-3.000000000000001e-04).unwrap(),
            -1.045921112040100e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_m1_50(-1.000000000000000e-04).unwrap(),
            -1.166711453256636e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_m1_50(-3.000000000000000e-05).unwrap(),
            -1.297753279184081e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_m1_50(-1.000000000000000e-05).unwrap(),
            -1.416360081581018e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_m1_50(-1.000000000000004e-75).unwrap(),
            -1.778749628219512e+02,
            epsilon = 1e-13
        );
        assert_abs_diff_eq!(
            lambert_w_m1_50(-1.000000000000008e-145).unwrap(),
            -3.397029099254290e+02
        );
        assert_eq!(lambert_w_m1_50(f64::EPSILON), None);
    }

    #[cfg(feature = "24bits")]
    #[test]
    fn test_lambert_w_m1_24() {
        assert_eq!(lambert_w_m1_24(-1.0 / E - f64::EPSILON), None);
        assert_abs_diff_eq!(
            lambert_w_m1_24(-3.578794411714423e-01).unwrap(),
            -1.253493791367214,
            epsilon = 1e-7
        );
        assert_abs_diff_eq!(
            lambert_w_m1_24(-2.678794411714424e-01).unwrap(),
            -2.020625228775403,
            epsilon = 1e-7
        );
        assert_abs_diff_eq!(
            lambert_w_m1_24(-1.000000000000000e-01).unwrap(),
            -3.577152063957297,
            epsilon = 1e-9
        );
        assert_abs_diff_eq!(
            lambert_w_m1_24(-3.000000000000000e-02).unwrap(),
            -5.144482721515681,
            epsilon = 1e-9
        );
        assert_abs_diff_eq!(
            lambert_w_m1_24(-1.000000000000000e-02).unwrap(),
            -6.472775124394005,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            lambert_w_m1_24(-3.000000000000000e-03).unwrap(),
            -7.872521380098709,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            lambert_w_m1_24(-1.000000000000000e-03).unwrap(),
            -9.118006470402742,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            lambert_w_m1_24(-3.000000000000001e-04).unwrap(),
            -1.045921112040100e+01,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            lambert_w_m1_24(-1.000000000000000e-04).unwrap(),
            -1.166711453256636e+01,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            lambert_w_m1_24(-3.000000000000000e-05).unwrap(),
            -1.297753279184081e+01,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            lambert_w_m1_24(-1.000000000000000e-05).unwrap(),
            -1.416360081581018e+01,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            lambert_w_m1_24(-1.000000000000004e-75).unwrap(),
            -1.778749628219512e+02,
            epsilon = 1e-5
        );
        assert_abs_diff_eq!(
            lambert_w_m1_24(-1.000000000000008e-145).unwrap(),
            -3.397029099254290e+02,
            epsilon = 1e-4
        );
        assert_eq!(lambert_w_m1_24(f64::EPSILON), None);
    }
}
