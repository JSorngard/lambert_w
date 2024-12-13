//! # `lambert_w`
//!
//! Fast and accurate evaluation of the real valued parts of the principal and secondary branches of the [Lambert W function](https://en.wikipedia.org/wiki/Lambert_W_function)
//! with the method of Toshio Fukushima \[[1](#references)\].
//!
//! This method is not iterative, it doesn't allocate memory, and it doesn't use recursion or loops
//! except for what might already be included in the implementations of the logarithm and square root functions.
//!
//! Instead it works by dividing the function’s domain into subdomains. For each one,
//! it uses a simple transformation of the input inserted into a rational function to approximate the function's value.
//!
//! The implementation uses conditional switches on the input value to select the appropriate subdomain,
//! followed by either a square root (and possibly a division) or a logarithm.
//! Then it performs a series of multiplications and additions using constants from a look-up table, and finishes the calculation with a division.
//!
//! The crate provides two approximations of each branch, one with 50 bits of accuracy (implemented on 64-bit floats) and one with 24 bits
//! (implemented on 32- and 64-bit floats). The one with 50 bits of accuracy uses higher degree polynomials
//! in the rational functions compared to the one with only 24 bits,
//! and thus more of the multiplications and additions by constants.
//!
//! This crate can evaluate the approximation with 24 bits of accuracy on
//! 32-bit floats, even though it is defined on 64-bit floats in Fukushima's paper.
//! This may result in a reduction in the accuracy to less than 24 bits,
//! but this reduction has not been quantified by the author of this crate.
//!
//! The crate is `no_std` compatible, but can optionally depend on the standard library through features for a potential performance gain.
//!
//! ## Examples
//!
//! Compute the value of the [omega constant](https://en.wikipedia.org/wiki/Omega_constant) with the principal branch of the Lambert W function:
//!
//! ```
//! # use approx::assert_abs_diff_eq;
//! use lambert_w::lambert_w0;
//!
//! let Ω = lambert_w0(1.0);
//!
//! assert_abs_diff_eq!(Ω, 0.5671432904097839);
//! ```
//!
//! Evaluate the secondary branch of the Lambert W function at -ln(2)/2:
//!
//! ```
//! # use approx::assert_abs_diff_eq;
//! use lambert_w::lambert_wm1;
//!
//! let mln4 = lambert_wm1(-f64::ln(2.0) / 2.0);
//!
//! assert_abs_diff_eq!(mln4, -f64::ln(4.0));
//! ```
//!
//! Do it on 32-bit floats:
//!
//!```
//!# use approx::assert_abs_diff_eq;
//! use lambert_w::{lambert_w0f, lambert_wm1f};
//!
//! let Ω = lambert_w0f(1.0);
//! let mln4 = lambert_wm1f(-f32::ln(2.0) / 2.0);
//!
//! assert_abs_diff_eq!(Ω, 0.56714329);
//! assert_abs_diff_eq!(mln4, -f32::ln(4.0));
//! ```
//!
//! The implementation can handle extreme inputs just as well:
//!
//! ```
//! # use approx::assert_relative_eq;
//! use lambert_w::{lambert_w0, lambert_wm1};
//!
//! let big = lambert_w0(f64::MAX);
//! let tiny = lambert_wm1(-1e-308);
//!
//! assert_relative_eq!(big, 703.2270331047702, max_relative = 4e-16);
//! assert_relative_eq!(tiny, -715.7695669234213, max_relative = 4e-16);
//! ```
//!
//! Importing the [`LambertW`] trait lets you call the functions with postfix notation:
//!
//! ```
//! # use approx::assert_abs_diff_eq;
//! use lambert_w::LambertW;
//!
//! let z = 2.0 * f64::ln(2.0);
//!
//! assert_abs_diff_eq!(z.lambert_w0(), f64::ln(2.0));
//! ```
//!
//! The macros are from the [`approx`](https://docs.rs/approx/latest/approx/) crate, and are used in the documentation examples of this crate.
//! The assertion passes if the two supplied values are the same to within floating point error, or within an optional epsilon or relative difference.
//!
//! ## Features
//!
//! One of the below features must be enabled:
//!
//! `libm` *(enabled by default)*: if the `std` feature is disabled, this feature uses the [`libm`]
//! crate to compute square roots and logarithms during function evaluation instead of the standard library.
//!
//! `std`: use the standard library to compute square roots and logarithms for a potential performance gain.
//! When this feature is disabled the crate is `no_std` compatible.
//!
//! ## References
//!
//! \[1\]: Toshio Fukushima.
//! **Precise and fast computation of Lambert W function by piecewise minimax rational function approximation with variable transformation**.
//! DOI: [10.13140/RG.2.2.30264.37128](https://doi.org/10.13140/RG.2.2.30264.37128).
//! November 2020.
//!
//! [⬆️ Back to top](#lambert_w).

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

#[cfg(all(not(feature = "std"), not(feature = "libm")))]
compile_error!("at least one of the `std` or `libm` features must be enabled");

mod dw0c;
mod dwm1c;
mod elementary;
mod rational;
mod sw0;
mod sw0f;
mod swm1;
mod swm1f;

/// The negative inverse of e (-1/e).
///
/// This is the smallest input value for which the Lambert W functions in this crate return a value.
//            Rounded from -0.367_879_441_171_442_322
pub const NEG_INV_E: f64 = -0.367_879_441_171_442_32;

/// 1/sqrt(e)
//         Rounded from 0.606_530_659_712_633_423
const INV_SQRT_E: f64 = 0.606_530_659_712_633_4;

/// The omega constant (Ω).
///
/// Fulfills the equation Ωe^Ω = 1.
///
/// Has been rounded to the closest available `f64` value.
//        Rounded from 0.567_143_290_409_783_87
pub const OMEGA: f64 = 0.567_143_290_409_783_8;
// If we round the last two digits (87) to 9 rustc sets the constant to
//                     0.567_143_290_409_784
// which is further away from the true value than what we get if we round them to 8.

/// The principal branch of the Lambert W function computed to 24 bits of accuracy on `f64`s.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use approx::assert_abs_diff_eq;
/// use lambert_w::sp_lambert_w0;
///
/// let Ω = sp_lambert_w0(1.0);
///
/// assert_abs_diff_eq!(Ω, 0.5671432904097839, epsilon = 1e-7);
/// ```
///
/// Arguments smaller than -1/e (≈ -0.36787944117144233) result in [`NAN`](f64::NAN):
///
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

/// The secondary branch of the Lambert W function computed to 24 bits of accuracy on `f64`s.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use approx::assert_abs_diff_eq;
/// use lambert_w::sp_lambert_wm1;
///
/// let mln4 = sp_lambert_wm1(-f64::ln(2.0) / 2.0);
///
/// assert_abs_diff_eq!(mln4, -f64::ln(4.0), epsilon = 1e-9);
/// ```
///
/// Arguments smaller than -1/e (≈ -0.36787944117144233) or larger than 0 result in [`NAN`](f64::NAN):
///
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

/// The principal branch of the Lambert W function computed to 50 bits of accuracy.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use approx::assert_abs_diff_eq;
/// use lambert_w::lambert_w0;
///
/// let Ω = lambert_w0(1.0);
///
/// assert_abs_diff_eq!(Ω, 0.5671432904097839);
/// ```
///
/// Arguments smaller than -1/e (≈ -0.36787944117144233) result in [`NAN`](f64::NAN):
///
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

/// The principal branch of the Lambert W function, computed with `f32`s.
///
/// Uses the same approximation as [`sp_lambert_w0`] but computing it with 32-bit floats
/// results in slightly reduced accuracy.  
/// This accuracy reduction has not been quantified.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use approx::assert_abs_diff_eq;
/// use lambert_w::lambert_w0f;
///
/// let Ω = lambert_w0f(1.0);
///
/// assert_abs_diff_eq!(Ω, 0.56714329);
/// ```
///
/// Arguments smaller than -1/e (≈ -0.36787944) result in [`NAN`](f32::NAN):
///
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

/// The secondary branch of the Lambert W function computed to 50 bits of accuracy.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use approx::assert_abs_diff_eq;
/// use lambert_w::lambert_wm1;
///
/// let mln4 = lambert_wm1(-f64::ln(2.0) / 2.0);
///
/// assert_abs_diff_eq!(mln4, -f64::ln(4.0), epsilon = 1e-14);
/// ```
///
/// Arguments smaller than -1/e (≈ -0.36787944117144233) or larger than 0 result in [`NAN`](f64::NAN):
///
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

/// The secondary branch of the Lambert W function, computed with `f32`s.
///
/// Uses the same approximation as [`sp_lambert_wm1`] but computing it with 32-bit floats
/// results in slightly reduced accuracy.  
/// This accuracy reduction has not been quantified.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use approx::assert_abs_diff_eq;
/// use lambert_w::lambert_wm1f;
///
/// let mln4 = lambert_wm1f(-f32::ln(2.0) / 2.0);
///
/// assert_abs_diff_eq!(mln4, -f32::ln(4.0));
/// ```
///
/// Arguments smaller than -1/e (≈ -0.36787944) or larger than 0 result in [`NAN`](f32::NAN):
///
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

/// Enables evaluation of the principal and secondary branches of the Lambert W function
/// on the types that implement this trait.
pub trait LambertW {
    /// The type returned by the Lambert W functions when acting on a value of type `Self`.
    type Output;

    /// The principal branch of the Lambert W function.
    fn lambert_w0(self) -> Self::Output;

    /// The secondary branch of the Lambert W function.
    fn lambert_wm1(self) -> Self::Output;
}

impl LambertW for f32 {
    type Output = Self;
    /// The principal branch of the Lambert W function.
    ///
    /// Evaluated with the approximation with 24-bits of accuracy from the paper, but on 32-bit floats.
    ///
    /// Arguments smaller than -1/e (≈ -0.36787944) result in [`NAN`](f32::NAN).
    ///
    /// Delegates to the [`lambert_w0f`] function.
    #[inline]
    fn lambert_w0(self) -> Self::Output {
        lambert_w0f(self)
    }
    /// The secondary branch of the Lambert W function.
    ///
    /// Evaluated with the approximation with 24-bits of accuracy from the paper, but on 32-bit floats.
    ///
    /// Arguments smaller than -1/e (≈ -0.36787944) or larger than 0 result in [`NAN`](f32::NAN).
    ///
    /// Delegates to the [`lambert_wm1f`] function.
    #[inline]
    fn lambert_wm1(self) -> Self::Output {
        lambert_wm1f(self)
    }
}

impl LambertW for f64 {
    type Output = Self;
    /// The principal branch of the Lambert W function evaluated to 50 bits of accuracy.
    ///
    /// Arguments smaller than -1/e (≈ -0.36787944117144233) result in [`NAN`](f64::NAN).
    ///
    /// Delegates to the [`lambert_w0`] function.
    #[inline]
    fn lambert_w0(self) -> Self::Output {
        lambert_w0(self)
    }
    /// The secondary branch of the Lambert W function evaluated to 50 bits of accuracy.
    ///
    /// Arguments smaller than -1/e (≈ -0.36787944117144233) or larger than 0 result in [`NAN`](f64::NAN).
    ///
    /// Delegates to the [`lambert_wm1`] function.
    #[inline]
    fn lambert_wm1(self) -> Self::Output {
        lambert_wm1(self)
    }
}

#[cfg(test)]
mod test {
    use super::{
        lambert_w0, lambert_w0f, lambert_wm1, lambert_wm1f, sp_lambert_w0, sp_lambert_wm1, LambertW,
    };
    use crate::NEG_INV_E;

    use approx::assert_abs_diff_eq;

    #[test]
    fn test_lambert_w0() {
        assert!(lambert_w0(NEG_INV_E - f64::EPSILON).is_nan());
        assert_abs_diff_eq!(lambert_w0(NEG_INV_E), -1.0);
        assert_abs_diff_eq!(lambert_w0(NEG_INV_E + f64::EPSILON), -0.9999999652557976);
        assert_abs_diff_eq!(
            lambert_w0(-2.678_794_411_714_424e-1),
            -3.993_824_525_397_807e-1
        );
        assert_abs_diff_eq!(
            lambert_w0(6.321_205_588_285_577e-1),
            4.167_039_988_177_658e-1
        );
        assert_abs_diff_eq!(lambert_w0(9.632120558828557), 1.721757710976171);
        assert_abs_diff_eq!(lambert_w0(9.963_212_055_882_856e1), 3.382785211058958);
        assert_abs_diff_eq!(lambert_w0(9.996_321_205_588_285e2), 5.249293782013269);
        assert_abs_diff_eq!(
            lambert_w0(9.999_632_120_558_828e3),
            7.231813718542178,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(lambert_w0(9.999_963_212_055_883e4), 9.284_568_107_521_96);
        assert_abs_diff_eq!(lambert_w0(9.999_996_321_205_589e5), 1.138_335_774_796_812e1);
        assert_abs_diff_eq!(lambert_w0(9.999_999_632_120_559e6), 1.351_434_397_605_273e1);
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
        assert_abs_diff_eq!(
            lambert_w0(9.999_999_999_963_213e10),
            2.222_712_273_495_755e1
        );
        assert_abs_diff_eq!(
            lambert_w0(9.999_999_999_996_321e11),
            2.443_500_440_493_456e1,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w0(9.999_999_999_999_633e12),
            2.665_078_750_870_219e1,
            epsilon = 1e-14
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
        assert_abs_diff_eq!(lambert_w0(1e17), 3.557_237_716_651_325e1);
        assert_abs_diff_eq!(lambert_w0(1e18), 3.781_385_607_558_877e1, epsilon = 1e-14);
        assert_abs_diff_eq!(lambert_w0(1e19), 4.005_876_916_198_432e1, epsilon = 1e-14);
        assert_abs_diff_eq!(lambert_w0(1e20), 4.230_675_509_173_839e1);
        assert_abs_diff_eq!(lambert_w0(1e40), 8.763_027_715_194_72e1);
        assert_abs_diff_eq!(lambert_w0(1e80), 1.790_193_137_415_062e2, epsilon = 1e-13);
        assert_abs_diff_eq!(lambert_w0(1e120), 2.707_091_661_024_979e2, epsilon = 1e-13);
        assert_abs_diff_eq!(lambert_w0(1e160), 3.625_205_337_614_976e2);
        assert_abs_diff_eq!(lambert_w0(f64::MAX), 703.2270331047702, epsilon = 1e-12);
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

    #[test]
    fn test_lambert_w0f() {
        assert!(lambert_w0f(-1.0 / core::f32::consts::E - f32::EPSILON).is_nan());
        assert_abs_diff_eq!(lambert_w0f(NEG_INV_E as f32), -1.0);
        assert_abs_diff_eq!(
            lambert_w0f(-2.678_794_3e-1),
            -3.993_824_4e-1,
            epsilon = 1e-7
        );
        assert_abs_diff_eq!(lambert_w0f(6.321_205_5e-1), 4.167_04e-1, epsilon = 1e-7);
        assert_abs_diff_eq!(lambert_w0f(9.632_12), 1.721_757_8, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_w0f(9.963_212e1), 3.382_785_3, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_w0f(9.996_321_4e2), 5.249_294, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_w0f(9.999_632e3), 7.231_814, epsilon = 1e-7);
        assert_abs_diff_eq!(lambert_w0f(9.999_963e4), 9.284_568, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_w0f(9.999_996e5), 1.138_335_8e1, epsilon = 1e-8);
        assert_abs_diff_eq!(lambert_w0f(1e7), 1.351_434_4e1, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_w0f(1e8), 1.566_899_7e1, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_w0f(1e9), 1.784_172_6e1, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_w0f(1e10), 2.002_868_5e1, epsilon = 1e-5);
        assert_abs_diff_eq!(lambert_w0f(1e11), 2.222_712_3e1, epsilon = 1e-5);
        assert_abs_diff_eq!(lambert_w0f(1e12), 2.443_500_5e1, epsilon = 1e-5);
        assert_abs_diff_eq!(lambert_w0f(1e13), 2.665_078_7e1, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_w0f(1e14), 2.887_327_6e1, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_w0f(1e15), 3.110_152e1, epsilon = 1e-5);
        assert_abs_diff_eq!(lambert_w0f(1e16), 3.333_476_3e1, epsilon = 1e-5);
        assert_abs_diff_eq!(lambert_w0f(1e17), 3.557_237_6e1, epsilon = 1e-5);
        assert_abs_diff_eq!(lambert_w0f(1e18), 3.781_385_4e1, epsilon = 1e-5);
        assert_abs_diff_eq!(lambert_w0f(1e19), 4.005_877e1, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_w0f(1e20), 4.230_675_5e1, epsilon = 1e-5);
        assert_abs_diff_eq!(lambert_w0f(f32::MAX), 84.288_59, epsilon = 1e-5);
    }

    #[test]
    fn test_lambert_wm1() {
        assert!(lambert_wm1(NEG_INV_E - f64::EPSILON).is_nan());
        assert_abs_diff_eq!(lambert_wm1(NEG_INV_E), -1.0);
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
        assert_abs_diff_eq!(lambert_wm1(-3e-2), -5.144482721515681);
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
        assert_abs_diff_eq!(
            lambert_wm1(-1.000000000000008e-145),
            -3.397_029_099_254_29e2
        );
        assert!(lambert_wm1(f64::EPSILON).is_nan());
    }

    #[test]
    fn test_sp_lambert_wm1() {
        assert!(sp_lambert_wm1(NEG_INV_E - f64::EPSILON).is_nan());
        assert_abs_diff_eq!(sp_lambert_wm1(NEG_INV_E), -1.0, epsilon = 1e-7);
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

    #[test]
    fn test_lambert_wm1f() {
        assert!(lambert_wm1f(NEG_INV_E as f32 - f32::EPSILON).is_nan());
        assert_abs_diff_eq!(lambert_wm1f(NEG_INV_E as f32), -1.0);
        assert_abs_diff_eq!(lambert_wm1f(-3.578_794_3e-1), -1.253_493_8, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_wm1f(-2.678_794_3e-1), -2.020_625, epsilon = 1e-7);
        assert_abs_diff_eq!(lambert_wm1f(-1e-1), -3.577_152, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_wm1f(-3e-2), -5.144_482_6, epsilon = 1e-9);
        assert_abs_diff_eq!(lambert_wm1f(-1e-2), -6.472_775, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_wm1f(-3e-3), -7.872_521_4, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_wm1f(-1e-3), -9.118_007, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_wm1f(-3e-4), -1.045_921_1e1, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_wm1f(-1e-4), -1.166_711_4e1, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_wm1f(-3e-5), -1.297_753_2e1, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_wm1f(-1e-5), -1.416_360_1e1, epsilon = 1e-6);
        assert_abs_diff_eq!(lambert_wm1f(-1e-20), -49.962_986);
        assert!(lambert_wm1f(f32::EPSILON).is_nan());
    }

    #[test]
    fn test_trait_impl_on_f64() {
        assert_abs_diff_eq!(
            (-2.678_794_411_714_424e-1_f64).lambert_w0(),
            -3.993_824_525_397_807e-1
        );
        assert_abs_diff_eq!(
            (-3.578_794_411_714_423e-1_f64).lambert_wm1(),
            -1.253493791367214,
            epsilon = 1e-14
        );
    }

    #[test]
    fn test_trait_impl_on_f32() {
        assert_abs_diff_eq!(6.321_205_5e-1_f32.lambert_w0(), 4.167_04e-1, epsilon = 1e-7);
        assert_abs_diff_eq!(
            (-3.578_794_3e-1_f32).lambert_wm1(),
            -1.253_493_8,
            epsilon = 1e-6
        );
    }
}
