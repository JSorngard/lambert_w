//! Fast and accurate evaluation of the real valued parts of the principal and secondary branches of the [Lambert W function](https://en.wikipedia.org/wiki/Lambert_W_function)
//! with the [method of Toshio Fukushima][1].
//!
//! This method works by splitting the domain of the function into subdomains, and on each subdomain it uses a rational function
//! evaluated on a simple transformation of the input to describe the function.  
//! It is implemented in code as conditional switches on the input value followed by either a square root (and possibly a division) or a logarithm
//! and then a series of multiplications and additions by fixed constants and finished with a division.
//!
//! The crate provides two approximations of each branch, one with 50 bits of accuracy and one with 24 bits.
//! The one with 50 bits of accuracy uses higher degree polynomials in the rational functions compared to the one with only 24 bits,
//! and thus more of the multiplications and additions by constants.
//!
//! While the approximation with 24 bits of accuracy is defined on 64 bit floats in the paper,
//! this crate can also evaluate it on 32 bit floats for a slight reduction in accuracy.
//! This reduction in accuracy has not been quantified by the author.
//!
//! The crate is `no_std`, but can optionally depend on the standard library through features for a potential performance gain.
//!
//! ## Examples
//!
//! Compute the value of the [Omega constant](https://en.wikipedia.org/wiki/Omega_constant) with the principal branch of the Lambert W function:
#![cfg_attr(
    feature = "50bits",
    doc = r##"
```
# use approx::assert_abs_diff_eq;
use lambert_w::lambert_w0;

let Ω = lambert_w0(1.0);

assert_abs_diff_eq!(Ω, 0.5671432904097838);
```
"##
)]
//! Evaluate the secondary branch of the Lambert W function at -ln(2)/2:
#![cfg_attr(
    feature = "50bits",
    doc = r##"
```
# use approx::assert_abs_diff_eq;
use lambert_w::lambert_wm1;

let z = -f64::ln(2.0) / 2.0;

let mln4 = lambert_wm1(z);

assert_abs_diff_eq!(mln4, -f64::ln(4.0));
```
"##
)]
//! The macro is from the [`approx`](https://docs.rs/approx/latest/approx/) crate, and is used in the documentation examples of this crate.
//! The assertion passes if the two supplied values are the same to within floating point error, or within an optional epsilon.
//!
//! ## Features
//!
//! `50bits` *(enabled by default)*: enables the function versions with 50 bits of accuracy on 64 bit floats.
//!
//! `24bits` *(enabled by default)*: enables the function versions with 24 bits of accuracy on 64 bit floats,
//! as well as the implementation on 32 bit floats.
//!
//! You can disable one of the above features to potentially save a little bit of binary size.
//!
//! `estrin`: uses [Estrin's scheme](https://en.wikipedia.org/wiki/Estrin's_scheme) to evaluate the polynomials in the rational functions.
//! While this results in more assembly instructions, they are mostly independent of each other,
//! and this increases instruction level parallelism on modern hardware for a total performance gain.
//! May result in slight numerical instability, which can be reduced if the target CPU has fused multiply-add instructions.
//!
//! One of the below features must be enabled:
//!
//! `std`: use the standard library to compute square roots and logarithms
//! for a potential performance gain. When this feature is disabled the crate is `no_std`.
//!
//! `libm` *(enabled by default)*: if the `std` feature is disabled, this feature uses the [`libm`]
//! crate to compute square roots and logarithms instead of the standard library.
//!
//! [1]: https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(all(not(feature = "std"), not(feature = "libm")))]
compile_error!("at least one of the `std` or `libm` feature flags must be enabled");

#[cfg(feature = "50bits")]
mod dw0c;
#[cfg(feature = "50bits")]
mod dwm1c;
mod elementary;
mod rational;
#[cfg(feature = "24bits")]
mod sw0;
#[cfg(feature = "24bits")]
mod sw0f;
#[cfg(feature = "24bits")]
mod swm1;
#[cfg(feature = "24bits")]
mod swm1f;

/// The negative inverse of e (-1/e).
///
/// This is the smallest input value for which the Lambert W functions in this crate return a value.
pub const NEG_INV_E: f64 = -0.367_879_441_171_442_33;

/// 1/sqrt(e)
const INV_SQRT_E: f64 = 0.606_530_659_712_633_4;

/// The Omega constant (Ω).
///
/// Fulfills the equation Ωe^Ω = 1.
pub const OMEGA: f64 = 0.567_143_290_409_783_8;

#[cfg(feature = "24bits")]
/// The principal branch of the Lambert W function computed to 24 bits of accuracy on `f64`s.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use approx::assert_abs_diff_eq;
/// use lambert_w::sp_lambert_w0;
///
/// let Ω = sp_lambert_w0(1.0);
///
/// assert_abs_diff_eq!(Ω, 0.5671432904097838, epsilon = 1e-7);
/// ```
/// Arguments smaller than -1/e (≈ -0.36787944117144233) result in [`NAN`](f64::NAN):
/// ```
/// # use lambert_w::sp_lambert_w0;
/// assert!(sp_lambert_w0(-1.0).is_nan());
/// ```
///
/// # Reference
///
/// [Toshio Fukushima, Precise and fast computation of Lambert W function by piecewise minimax rational function approximation with variable transformation](https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation).
pub fn sp_lambert_w0(z: f64) -> f64 {
    sw0::sw0(z)
}

#[cfg(feature = "24bits")]
/// The secondary branch of the Lambert W function computed to 24 bits of accuracy on `f64`s.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use approx::assert_abs_diff_eq;
/// use lambert_w::sp_lambert_wm1;
///
/// let mln4 = sp_lambert_wm1(-f64::ln(2.0) / 2.0);
///
/// assert_abs_diff_eq!(mln4, -f64::ln(4.0), epsilon = 1e-9);
/// ```
/// Arguments smaller than -1/e (≈ -0.36787944117144233) or larger than 0 result in [`NAN`](f64::NAN):
/// ```
/// # use lambert_w::sp_lambert_wm1;
/// assert!(sp_lambert_wm1(-1.0).is_nan());
/// assert!(sp_lambert_wm1(1.0).is_nan());
/// ```
///
/// # Reference
///
/// [Toshio Fukushima, Precise and fast computation of Lambert W function by piecewise minimax rational function approximation with variable transformation](https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation).
pub fn sp_lambert_wm1(z: f64) -> f64 {
    swm1::swm1(z)
}

#[cfg(feature = "50bits")]
/// The principal branch of the Lambert W function computed to 50 bits of accuracy.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use approx::assert_abs_diff_eq;
/// use lambert_w::lambert_w0;
///
/// let Ω = lambert_w0(1.0);
///
/// assert_abs_diff_eq!(Ω, 0.5671432904097838);
/// ```
/// Arguments smaller than -1/e (≈ -0.36787944117144233) result in [`NAN`](f64::NAN):
/// ```
/// # use lambert_w::lambert_w0;
/// assert!(lambert_w0(-1.0).is_nan());
/// ```
///
/// # Reference
///
/// [Toshio Fukushima, Precise and fast computation of Lambert W function by piecewise minimax rational function approximation with variable transformation](https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation).
pub fn lambert_w0(z: f64) -> f64 {
    dw0c::dw0c(z - NEG_INV_E)
}

#[cfg(feature = "24bits")]
/// The principal branch of the Lambert W function, computed with `f32`s.
///
/// Uses the same approximation as [`sp_lambert_w0`] but computing it with 32 bit floats
/// results in slightly reduced accuracy.  
/// This accuracy reduction has not been quantified by the author.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use approx::assert_abs_diff_eq;
/// use lambert_w::lambert_w0f;
///
/// let Ω = lambert_w0f(1.0);
///
/// assert_abs_diff_eq!(Ω, 0.56714329);
/// ```
/// Arguments smaller than -1/e (≈ -0.36787944) result in [`NAN`](f32::NAN):
/// ```
/// # use lambert_w::lambert_w0f;
/// assert!(lambert_w0f(-1.0).is_nan());
/// ```
///
/// # Reference
///
/// [Toshio Fukushima, Precise and fast computation of Lambert W function by piecewise minimax rational function approximation with variable transformation](https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation).
pub fn lambert_w0f(z: f32) -> f32 {
    sw0f::sw0f(z)
}

#[cfg(feature = "50bits")]
/// The secondary branch of the Lambert W function computed to 50 bits of accuracy.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use approx::assert_abs_diff_eq;
/// use lambert_w::lambert_wm1;
///
/// let mln4 = lambert_wm1(-f64::ln(2.0) / 2.0);
///
/// assert_abs_diff_eq!(mln4, -f64::ln(4.0), epsilon = 1e-14);
/// ```
/// Arguments smaller than -1/e (≈ -0.36787944117144233) or larger than 0 result in [`NAN`](f64::NAN):
/// ```
/// # use lambert_w::lambert_wm1;
/// assert!(lambert_wm1(-1.0).is_nan());
/// assert!(lambert_wm1(1.0).is_nan());
/// ```
///
/// # Reference
///
/// [Toshio Fukushima, Precise and fast computation of Lambert W function by piecewise minimax rational function approximation with variable transformation](https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation).
pub fn lambert_wm1(z: f64) -> f64 {
    dwm1c::dwm1c(z, z - NEG_INV_E)
}

#[cfg(feature = "24bits")]
/// The secondary branch of the Lambert W function, computed with `f32`s.
///
/// Uses the same approximation as [`sp_lambert_wm1`] but computing it with 32 bit floats
/// results in slightly reduced accuracy.  
/// This accuracy reduction has not been quantified by the author.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use approx::assert_abs_diff_eq;
/// use lambert_w::lambert_wm1f;
///
/// let mln4 = lambert_wm1f(-f32::ln(2.0) / 2.0);
///
/// assert_abs_diff_eq!(mln4, -f32::ln(4.0));
/// ```
/// Arguments smaller than -1/e (≈ -0.36787944) or larger than 0 result in [`NAN`](f32::NAN):
/// ```
/// # use lambert_w::lambert_wm1f;
/// assert!(lambert_wm1f(-1.0).is_nan());
/// assert!(lambert_wm1f(1.0).is_nan());
/// ```
///
/// # Reference
///
/// [Toshio Fukushima, Precise and fast computation of Lambert W function by piecewise minimax rational function approximation with variable transformation](https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation).
pub fn lambert_wm1f(z: f32) -> f32 {
    swm1f::swm1f(z)
}

#[cfg(all(test, any(feature = "24bits", feature = "50bits")))]
mod test {
    // A lot of these tests are less stringent when the `estrin` feature flag is activated.
    // This is because Estrin's scheme is less numerically stable,
    // and CI may also not have fused multiply-add instructions to reduce the instabillity.

    #[cfg(feature = "50bits")]
    use super::{lambert_w0, lambert_wm1};
    #[cfg(feature = "24bits")]
    use super::{lambert_w0f, lambert_wm1f, sp_lambert_w0, sp_lambert_wm1};
    use approx::assert_abs_diff_eq;
    use core::f64::consts::E;

    #[cfg(feature = "50bits")]
    #[test]
    fn test_lambert_w0() {
        assert!(lambert_w0(-1.0 / E - f64::EPSILON).is_nan());
        assert_abs_diff_eq!(
            lambert_w0(-2.678_794_411_714_424e-1),
            -3.993_824_525_397_807e-1
        );
        assert_abs_diff_eq!(
            lambert_w0(6.321_205_588_285_577e-1),
            4.167_039_988_177_658e-1
        );
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(lambert_w0(9.632120558828557), 1.721757710976171);
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(
            lambert_w0(9.632120558828557),
            1.721757710976171,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(lambert_w0(9.963_212_055_882_856e1), 3.382785211058958);
        assert_abs_diff_eq!(lambert_w0(9.996_321_205_588_285e2), 5.249293782013269);
        assert_abs_diff_eq!(
            lambert_w0(9.999_632_120_558_828e3),
            7.231813718542178,
            epsilon = 1e-14
        );
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(lambert_w0(9.999_963_212_055_883e4), 9.284_568_107_521_96);
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(
            lambert_w0(9.999963212055883e+04),
            9.284568107521959,
            epsilon = 1e-14
        );
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(lambert_w0(9.999_996_321_205_589e5), 1.138_335_774_796_812e1);
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(
            lambert_w0(9.999996321205589e+05),
            1.138335774796812e+01,
            epsilon = 1e-14
        );
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(lambert_w0(9.999_999_632_120_559e6), 1.351_434_397_605_273e1);
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(
            lambert_w0(9.999999632120559e+06),
            1.351434397605273e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w0(9.999_999_963_212_056e7),
            1.566_899_671_199_287e1,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w0(9.999_999_996_321_206e8),
            1.784_172_596_707_312e1,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w0(9.999_999_999_632_12e9),
            2.002_868_541_326_992e1,
            epsilon = 1e-14
        );
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(
            lambert_w0(9.999_999_999_963_213e10),
            2.222_712_273_495_755e1
        );
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(
            lambert_w0(9.999999999963213e+10),
            2.222712273495755e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w0(9.999_999_999_996_321e11),
            2.443_500_440_493_456e1,
            epsilon = 1e-14
        );
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(
            lambert_w0(9.999_999_999_999_633e12),
            2.665_078_750_870_219e1,
            epsilon = 1e-14
        );
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(
            lambert_w0(9.999999999999633e+12),
            2.665078750870219e+01,
            epsilon = 1e-13
        );
        assert_abs_diff_eq!(
            lambert_w0(9.999_999_999_999_963e13),
            2.887_327_487_929_93e1,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w0(9.999_999_999_999_996e14),
            3.110_151_971_159_478e1,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(lambert_w0(1e16), 3.333_476_076_844_818e1, epsilon = 1e-14);
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(lambert_w0(1e17), 3.557_237_716_651_325e1);
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(
            lambert_w0(1.000000000000000e+17),
            3.557237716651325e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(lambert_w0(1e18), 3.781_385_607_558_877e1, epsilon = 1e-14);
        assert_abs_diff_eq!(lambert_w0(1e19), 4.005_876_916_198_432e1, epsilon = 1e-14);
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(lambert_w0(1e20), 4.230_675_509_173_839e1);
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(
            lambert_w0(1.000000000000000e+20),
            4.230675509173839e+01,
            epsilon = 1e-14
        );
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(lambert_w0(1e40), 8.763_027_715_194_72e1);
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(
            lambert_w0(1.000000000000000e+40),
            8.763027715194720e+01,
            epsilon = 1e-13
        );
        assert_abs_diff_eq!(lambert_w0(1e80), 1.790_193_137_415_062e2, epsilon = 1e-13);
        assert_abs_diff_eq!(lambert_w0(1e120), 2.707_091_661_024_979e2, epsilon = 1e-13);
        assert_abs_diff_eq!(lambert_w0(1e160), 3.625_205_337_614_976e2);
        assert_abs_diff_eq!(lambert_w0(f64::MAX), 703.2270331047702, epsilon = 1e-12);
    }

    #[cfg(feature = "24bits")]
    #[test]
    fn test_sp_lambert_w0() {
        assert!(sp_lambert_w0(-1.0 / E - f64::EPSILON).is_nan());
        assert_abs_diff_eq!(
            sp_lambert_w0(-2.678_794_411_714_424e-1),
            -3.993_824_525_397_807e-1,
            epsilon = 1e-7
        );
        assert_abs_diff_eq!(
            sp_lambert_w0(6.321_205_588_285_577e-1),
            4.167_039_988_177_658e-1,
            epsilon = 1e-7
        );
        assert_abs_diff_eq!(
            sp_lambert_w0(9.632120558828557),
            1.721757710976171,
            epsilon = 1e-7
        );
        assert_abs_diff_eq!(
            sp_lambert_w0(9.963_212_055_882_856e1),
            3.382785211058958,
            epsilon = 1e-7
        );
        assert_abs_diff_eq!(
            sp_lambert_w0(9.996_321_205_588_285e2),
            5.249293782013269,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w0(9.999_632_120_558_828e3),
            7.231813718542178,
            epsilon = 1e-7
        );
        assert_abs_diff_eq!(
            sp_lambert_w0(9.999_963_212_055_883e4),
            9.284_568_107_521_96,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w0(9.999_996_321_205_589e5),
            1.138_335_774_796_812e1,
            epsilon = 1e-8
        );
        assert_abs_diff_eq!(
            sp_lambert_w0(9.999_999_632_120_559e6),
            1.351_434_397_605_273e1,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w0(9.999_999_963_212_056e7),
            1.566_899_671_199_287e1,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w0(9.999_999_996_321_206e8),
            1.784_172_596_707_312e1,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w0(9.999_999_999_632_12e9),
            2.002_868_541_326_992e1,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w0(9.999_999_999_963_213e10),
            2.222_712_273_495_755e1,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w0(9.999_999_999_996_321e11),
            2.443_500_440_493_456e1,
            epsilon = 1e-5
        );
        assert_abs_diff_eq!(
            sp_lambert_w0(9.999_999_999_999_633e12),
            2.665_078_750_870_219e1,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w0(9.999_999_999_999_963e13),
            2.887_327_487_929_93e1,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w0(9.999_999_999_999_996e14),
            3.110_151_971_159_478e1,
            epsilon = 1e-5
        );
        assert_abs_diff_eq!(sp_lambert_w0(1e16), 3.333_476_076_844_818e1, epsilon = 1e-6);
        assert_abs_diff_eq!(sp_lambert_w0(1e17), 3.557_237_716_651_325e1, epsilon = 1e-5);
        assert_abs_diff_eq!(sp_lambert_w0(1e18), 3.781_385_607_558_877e1, epsilon = 1e-5);
        assert_abs_diff_eq!(sp_lambert_w0(1e19), 4.005_876_916_198_432e1, epsilon = 1e-6);
        assert_abs_diff_eq!(sp_lambert_w0(1e20), 4.230_675_509_173_839e1, epsilon = 1e-5);
        assert_abs_diff_eq!(sp_lambert_w0(1e40), 8.763_027_715_194_72e1, epsilon = 1e-5);
        assert_abs_diff_eq!(sp_lambert_w0(1e80), 1.790_193_137_415_062e2, epsilon = 1e-5);
        assert_abs_diff_eq!(
            sp_lambert_w0(1e120),
            2.707_091_661_024_979e2,
            epsilon = 1e-4
        );
        assert_abs_diff_eq!(
            sp_lambert_w0(1e160),
            3.625_205_337_614_976e2,
            epsilon = 1e-4
        );
        assert_abs_diff_eq!(sp_lambert_w0(f64::MAX), 703.2270331047702, epsilon = 1e-4);
    }

    #[cfg(feature = "24bits")]
    #[test]
    fn test_lambert_w0f() {
        assert!(lambert_w0f(-1.0 / core::f32::consts::E - f32::EPSILON).is_nan());
        assert_abs_diff_eq!(
            lambert_w0f(-2.678_794_3e-1),
            -3.993_824_4e-1,
            epsilon = 1e-7
        );
        assert_abs_diff_eq!(lambert_w0f(6.321_205_5e-1), 4.167_04e-1, epsilon = 1e-7);
        assert_abs_diff_eq!(lambert_w0f(9.632_12), 1.721_757_8, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_w0f(9.963_212e1), 3.382_785_3, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_w0f(9.996_321_4e2), 5.249_294, epsilon = 1e-6);
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(lambert_w0f(9.999_632e3), 7.231_814, epsilon = 1e-7);
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(lambert_w0f(9.999_632e3), 7.231_814, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_w0f(9.999_963e4), 9.284_568, epsilon = 1e-6);
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(lambert_w0f(9.999_996e5), 1.138_335_8e1, epsilon = 1e-8);
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(lambert_w0f(9.999_996e5), 1.138_335_8e1, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_w0f(1e7), 1.351_434_4e1, epsilon = 1e-6);
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(lambert_w0f(1e8), 1.566_899_7e1, epsilon = 1e-6);
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(lambert_w0f(1e8), 1.566_899_7e1, epsilon = 1e-5);
        assert_abs_diff_eq!(lambert_w0f(1e9), 1.784_172_6e1, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_w0f(1e10), 2.002_868_5e1, epsilon = 1e-5);
        assert_abs_diff_eq!(lambert_w0f(1e11), 2.222_712_3e1, epsilon = 1e-5);
        assert_abs_diff_eq!(lambert_w0f(1e12), 2.443_500_5e1, epsilon = 1e-5);
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(lambert_w0f(1e13), 2.665_078_7e1, epsilon = 1e-6);
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(lambert_w0f(1e13), 2.665_078_7e1, epsilon = 1e-5);
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(lambert_w0f(1e14), 2.887_327_6e1, epsilon = 1e-6);
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(lambert_w0f(1e14), 2.887_327_6e1, epsilon = 1e-5);
        assert_abs_diff_eq!(lambert_w0f(1e15), 3.110_152e1, epsilon = 1e-5);
        assert_abs_diff_eq!(lambert_w0f(1e16), 3.333_476_3e1, epsilon = 1e-5);
        assert_abs_diff_eq!(lambert_w0f(1e17), 3.557_237_6e1, epsilon = 1e-5);
        assert_abs_diff_eq!(lambert_w0f(1e18), 3.781_385_4e1, epsilon = 1e-5);
        assert_abs_diff_eq!(lambert_w0f(1e19), 4.005_877e1, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_w0f(1e20), 4.230_675_5e1, epsilon = 1e-5);
        assert_abs_diff_eq!(lambert_w0f(f32::MAX), 84.288_59, epsilon = 1e-5);
    }

    #[cfg(feature = "50bits")]
    #[test]
    fn test_lambert_wm1() {
        assert!(lambert_wm1(-1.0 / E - f64::EPSILON).is_nan());
        assert_abs_diff_eq!(
            lambert_wm1(-3.578_794_411_714_423e-1),
            -1.253493791367214,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_wm1(-2.678_794_411_714_424e-1),
            -2.020625228775403,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(lambert_wm1(-1e-1), -3.577152063957297);
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(lambert_wm1(-3e-2), -5.144482721515681);
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(
            lambert_wm1(-3.000000000000000e-02),
            -5.144482721515681,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(lambert_wm1(-1e-2), -6.472775124394005, epsilon = 1e-14);
        assert_abs_diff_eq!(lambert_wm1(-3e-3), -7.872521380098709, epsilon = 1e-14);
        assert_abs_diff_eq!(lambert_wm1(-1e-3), -9.118006470402742, epsilon = 1e-14);
        assert_abs_diff_eq!(
            lambert_wm1(-3.000_000_000_000_001e-4),
            -1.045_921_112_040_1e1,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_wm1(-1e-4),
            -1.166_711_453_256_636e1,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_wm1(-3e-5),
            -1.297_753_279_184_081e1,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_wm1(-1e-5),
            -1.416_360_081_581_018e1,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_wm1(-1.000000000000004e-75),
            -1.778_749_628_219_512e2,
            epsilon = 1e-13
        );
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(
            lambert_wm1(-1.000000000000008e-145),
            -3.397_029_099_254_29e2
        );
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(
            lambert_wm1(-1.000000000000008e-145),
            -3.397029099254290e+02,
            epsilon = 1e-12
        );
        assert!(lambert_wm1(f64::EPSILON).is_nan());
    }

    #[cfg(feature = "24bits")]
    #[test]
    fn test_sp_lambert_wm1() {
        assert!(sp_lambert_wm1(-1.0 / E - f64::EPSILON).is_nan());
        assert_abs_diff_eq!(
            sp_lambert_wm1(-3.578_794_411_714_423e-1),
            -1.253493791367214,
            epsilon = 1e-7
        );
        assert_abs_diff_eq!(
            sp_lambert_wm1(-2.678_794_411_714_424e-1),
            -2.020625228775403,
            epsilon = 1e-7
        );
        assert_abs_diff_eq!(sp_lambert_wm1(-1e-1), -3.577152063957297, epsilon = 1e-9);
        assert_abs_diff_eq!(sp_lambert_wm1(-3e-2), -5.144482721515681, epsilon = 1e-9);
        assert_abs_diff_eq!(sp_lambert_wm1(-1e-2), -6.472775124394005, epsilon = 1e-6);
        assert_abs_diff_eq!(sp_lambert_wm1(-3e-3), -7.872521380098709, epsilon = 1e-6);
        assert_abs_diff_eq!(sp_lambert_wm1(-1e-3), -9.118006470402742, epsilon = 1e-6);
        assert_abs_diff_eq!(
            sp_lambert_wm1(-3.000_000_000_000_001e-4),
            -1.045_921_112_040_1e1,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_wm1(-1e-4),
            -1.166_711_453_256_636e1,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_wm1(-3e-5),
            -1.297_753_279_184_081e1,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_wm1(-1e-5),
            -1.416_360_081_581_018e1,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_wm1(-1.000000000000004e-75),
            -1.778_749_628_219_512e2,
            epsilon = 1e-5
        );
        assert_abs_diff_eq!(
            sp_lambert_wm1(-1.000000000000008e-145),
            -3.397_029_099_254_29e2,
            epsilon = 1e-4
        );
        assert!(sp_lambert_wm1(f64::EPSILON).is_nan());
    }

    #[cfg(feature = "24bits")]
    #[test]
    fn test_lambert_wm1f() {
        assert!(lambert_wm1f(-1.0 / core::f32::consts::E - f32::EPSILON).is_nan());
        assert_abs_diff_eq!(lambert_wm1f(-3.578_794_3e-1), -1.253_493_8, epsilon = 1e-6);
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(lambert_wm1f(-2.678_794_3e-1), -2.020_625, epsilon = 1e-7);
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(
            lambert_wm1f(-2.678_794_411_714_424e-1),
            -2.020625228775403,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(lambert_wm1f(-1e-1), -3.577_152, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_wm1f(-3e-2), -5.144_482_6, epsilon = 1e-9);
        assert_abs_diff_eq!(lambert_wm1f(-1e-2), -6.472_775, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_wm1f(-3e-3), -7.872_521_4, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_wm1f(-1e-3), -9.118_007, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_wm1f(-3e-4), -1.045_921_1e1, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_wm1f(-1e-4), -1.166_711_4e1, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_wm1f(-3e-5), -1.297_753_2e1, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_wm1f(-1e-5), -1.416_360_1e1, epsilon = 1e-6);
        assert!(lambert_wm1f(f32::EPSILON).is_nan());
    }
}
