//! Fast evaluation of the real valued parts of the principal and secondary branches of the [Lambert W function](https://en.wikipedia.org/wiki/Lambert_W_function)
//! using the [method of Toshio Fukushima](https://www.researchgate.net/publication/346309410_Precise_anfast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation)
//! to either 24 or 50 bits of accuracy.
//!
//! This method works by splitting the domain of the function into subdomains, and on each subdomain it uses a rational function
//! evaluated on a simple transformation of the input to describe the function.  
//! It is implemented in code as conditional switches on the input value followed by either a square root (and possibly a division) or a logarithm
//! and then a series of multiplications and additions by fixed constants and finished with a division.
//!
//! The functions with 50 bits of accuracy use higher degree Padé approximants, and thus more of the multiplications and additions.
//!
//! ## Examples
//!
//! Compute the value of the [Omega constant](https://en.wikipedia.org/wiki/Omega_constant) with the principal branch of the Lambert W function to 50 bits of accuracy:
#![cfg_attr(
    feature = "50bits",
    doc = r##"
```
# use approx::assert_abs_diff_eq;
use lambert_w::lambert_w_0;

let Ω = lambert_w_0(1.0);

assert_abs_diff_eq!(Ω, 0.5671432904097838);
```
"##
)]
//!
//! or to only 24 bits of accuracy, but with faster execution time:
#![cfg_attr(
    feature = "24bits",
    doc = r##"
```
# use approx::assert_abs_diff_eq;
use lambert_w::sp_lambert_w_0;

let Ω = sp_lambert_w_0(1.0);

assert_abs_diff_eq!(Ω, 0.5671432904097838, epsilon = 1e-7);
```
"##
)]
//! Evaluate the secondary branch of the Lambert W function at -ln(2)/2 to 50 and 24 bits of accuracy:
#![cfg_attr(
    all(feature = "50bits", feature = "24bits"),
    doc = r##"
```
# use approx::assert_abs_diff_eq;
use lambert_w::{lambert_w_m1, sp_lambert_w_m1};

let z = -f64::ln(2.0) / 2.0;

let mln4_50b = lambert_w_m1(z);
let mln4_24b = sp_lambert_w_m1(z);

assert_abs_diff_eq!(mln4_50b, -f64::ln(4.0));
assert_abs_diff_eq!(mln4_24b, -f64::ln(4.0), epsilon = 1e-9);
```
"##
)]
//! The macro is from the [`approx`](https://docs.rs/approx/latest/approx/) crate, and is used in the documentation examples of this crate.
//! The assertion passes if the two supplied values are the same to within floating point error, or within an optional epsilon.
//!
//! ## Feature flags
//!
//! `50bits` *(enabled by default)*: enables the more accurate function versions with 50 bits of accuracy.
//!
//! `24bits` *(enabled by default)*: enables the faster function versions with 24 bits of accuracy.
//!
//! You can disable one of the above feature flags to potentially save a little bit of binary size.
//!
//! `estrin`: uses [Estrin's scheme](https://en.wikipedia.org/wiki/Estrin's_scheme) instead of [Horner's Method](https://en.wikipedia.org/wiki/Horner%27s_method)
//! with the help of the [`fast_polynomial`](https://docs.rs/fast_polynomial/0.2.0/fast_polynomial/) crate to evaluate the polynomials in the rational functions.
//! While this results in more assembly instructions, they are mostly independent of each other,
//! and this increases instruction level parallelism on modern hardware for a total performance gain.
//! May result in slight numerical instability, which can be mitigated if the target CPU has fused multiply-add instructions.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "50bits")]
mod dw0c;
#[cfg(feature = "50bits")]
mod dwm1c;
mod pade;
#[cfg(feature = "24bits")]
mod sw0;
#[cfg(feature = "24bits")]
mod swm1;

/// The negative inverse of e (-1/e).
///
/// This is the smallest input value for which the Lambert W functions in this crate return a value.
pub const NEG_INV_E: f64 = -0.367_879_441_171_442_33;

/// 1/sqrt(e)
const INV_SQRT_E: f64 = 0.606_530_659_712_633_4;

/// The Omega constant (Ω).
///
/// Fulfills the equation Ω*e^Ω = 1.
pub const OMEGA: f64 = 0.567_143_290_409_783_8;

#[cfg(feature = "24bits")]
/// Computes the principal branch of the Lambert W function, W_0(`z`), to 24 bits of accuracy, if `z` >= -1/e.
///
/// Uses the piecewise minimax rational function approximation with variable transformation method of Toshio Fukushima.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use approx::assert_abs_diff_eq;
/// use lambert_w::sp_lambert_w_0;
///
/// let Ω = sp_lambert_w_0(1.0);
///
/// assert_abs_diff_eq!(Ω, 0.5671432904097838, epsilon = 1e-7);
/// ```
/// Arguments smaller than -1/e (≈ -0.36787944117144233) result in [`NAN`](f64::NAN):
/// ```
/// # use lambert_w::sp_lambert_w_0;
/// assert!(sp_lambert_w_0(-1.0).is_nan());
/// ```
pub fn sp_lambert_w_0(z: f64) -> f64 {
    sw0::sw0(z)
}

#[cfg(feature = "24bits")]
/// Computes the secondary branch of the Lambert W function, W_-1(`z`), to 24 bits of accuracy, if -1/e <= `z` <= 0.
///
/// Uses the piecewise minimax rational function approximation with variable transformation method of Toshio Fukushima.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use approx::assert_abs_diff_eq;
/// use lambert_w::sp_lambert_w_m1;
///
/// let mln4 = sp_lambert_w_m1(-f64::ln(2.0) / 2.0);
///
/// assert_abs_diff_eq!(mln4, -f64::ln(4.0), epsilon = 1e-9);
/// ```
/// Arguments smaller than -1/e (≈ -0.36787944117144233) or larger than 0 result in [`NAN`](f64::NAN):
/// ```
/// # use lambert_w::sp_lambert_w_m1;
/// assert!(sp_lambert_w_m1(-1.0).is_nan());
/// assert!(sp_lambert_w_m1(1.0).is_nan());
/// ```
pub fn sp_lambert_w_m1(z: f64) -> f64 {
    swm1::swm1(z)
}

#[cfg(feature = "50bits")]
/// Computes the principal branch of the Lambert W function, W_0(`z`), to 50 bits of accuracy, if `z` >= -1/e.
///
/// Uses the piecewise minimax rational function approximation with variable transformation method of Toshio Fukushima.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use approx::assert_abs_diff_eq;
/// use lambert_w::lambert_w_0;
///
/// let Ω = lambert_w_0(1.0);
///
/// assert_abs_diff_eq!(Ω, 0.5671432904097838);
/// ```
/// Arguments smaller than -1/e (≈ -0.36787944117144233) result in [`NAN`](f64::NAN):
/// ```
/// # use lambert_w::lambert_w_0;
/// assert!(lambert_w_0(-1.0).is_nan());
/// ```
pub fn lambert_w_0(z: f64) -> f64 {
    dw0c::dw0c(z - NEG_INV_E)
}

#[cfg(feature = "50bits")]
/// Computes the secondary branch of the Lambert W function, W_-1(`z`), to 50 bits of accuracy, if -1/e <= `z` <= 0.
///
/// Uses the piecewise minimax rational function approximation with variable transformation method of Toshio Fukushima.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use approx::assert_abs_diff_eq;
/// use lambert_w::lambert_w_m1;
///
/// let mln4 = lambert_w_m1(-f64::ln(2.0) / 2.0);
///
/// assert_abs_diff_eq!(mln4, -f64::ln(4.0), epsilon = 1e-14);
/// ```
/// Arguments smaller than -1/e (≈ -0.36787944117144233) or larger than 0 result in [`NAN`](f64::NAN):
/// ```
/// # use lambert_w::lambert_w_m1;
/// assert!(lambert_w_m1(-1.0).is_nan());
/// assert!(lambert_w_m1(1.0).is_nan());
/// ```
pub fn lambert_w_m1(z: f64) -> f64 {
    dwm1c::dwm1c(z, z - NEG_INV_E)
}

#[cfg(all(test, any(feature = "24bits", feature = "50bits")))]
mod test {
    // A lot of these tests are less stringent when the `estrin` feature flag is activated.
    // This is because CI may not have fused multiply-add instructions, which creates numerical instabillity.

    #[cfg(feature = "50bits")]
    use super::{lambert_w_0, lambert_w_m1};
    #[cfg(feature = "24bits")]
    use super::{sp_lambert_w_0, sp_lambert_w_m1};
    use approx::assert_abs_diff_eq;
    use core::f64::consts::E;

    #[cfg(feature = "50bits")]
    #[test]
    fn test_lambert_w_0() {
        assert!(lambert_w_0(-1.0 / E - f64::EPSILON).is_nan());
        assert_abs_diff_eq!(lambert_w_0(-2.678794411714424e-01), -3.993824525397807e-01);
        assert_abs_diff_eq!(lambert_w_0(6.321205588285577e-01), 4.167039988177658e-01);
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(lambert_w_0(9.632120558828557), 1.721757710976171);
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(
            lambert_w_0(9.632120558828557),
            1.721757710976171,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(lambert_w_0(9.963212055882856e+01), 3.382785211058958);
        assert_abs_diff_eq!(lambert_w_0(9.996321205588285e+02), 5.249293782013269);
        assert_abs_diff_eq!(
            lambert_w_0(9.999632120558828e+03),
            7.231813718542178,
            epsilon = 1e-14
        );
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(lambert_w_0(9.999963212055883e+04), 9.284568107521959);
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(
            lambert_w_0(9.999963212055883e+04),
            9.284568107521959,
            epsilon = 1e-14
        );
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(lambert_w_0(9.999996321205589e+05), 1.138335774796812e+01);
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(
            lambert_w_0(9.999996321205589e+05),
            1.138335774796812e+01,
            epsilon = 1e-14
        );
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(lambert_w_0(9.999999632120559e+06), 1.351434397605273e+01);
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(
            lambert_w_0(9.999999632120559e+06),
            1.351434397605273e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_0(9.999999963212056e+07),
            1.566899671199287e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_0(9.999999996321206e+08),
            1.784172596707312e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_0(9.999999999632120e+09),
            2.002868541326992e+01,
            epsilon = 1e-14
        );
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(lambert_w_0(9.999999999963213e+10), 2.222712273495755e+01);
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(
            lambert_w_0(9.999999999963213e+10),
            2.222712273495755e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_0(9.999999999996321e+11),
            2.443500440493456e+01,
            epsilon = 1e-14
        );
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(
            lambert_w_0(9.999999999999633e+12),
            2.665078750870219e+01,
            epsilon = 1e-14
        );
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(
            lambert_w_0(9.999999999999633e+12),
            2.665078750870219e+01,
            epsilon = 1e-13
        );
        assert_abs_diff_eq!(
            lambert_w_0(9.999999999999962e+13),
            2.887327487929930e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_0(9.999999999999996e+14),
            3.110151971159478e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_0(1.000000000000000e+16),
            3.333476076844818e+01,
            epsilon = 1e-14
        );
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(lambert_w_0(1.000000000000000e+17), 3.557237716651325e+01);
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(
            lambert_w_0(1.000000000000000e+17),
            3.557237716651325e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_0(1.000000000000000e+18),
            3.781385607558877e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_0(1.000000000000000e+19),
            4.005876916198432e+01,
            epsilon = 1e-14
        );
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(lambert_w_0(1.000000000000000e+20), 4.230675509173839e+01);
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(
            lambert_w_0(1.000000000000000e+20),
            4.230675509173839e+01,
            epsilon = 1e-14
        );
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(lambert_w_0(1.000000000000000e+40), 8.763027715194720e+01);
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(
            lambert_w_0(1.000000000000000e+40),
            8.763027715194720e+01,
            epsilon = 1e-13
        );
        assert_abs_diff_eq!(
            lambert_w_0(1.000000000000000e+80),
            1.790193137415062e+02,
            epsilon = 1e-13
        );
        assert_abs_diff_eq!(
            lambert_w_0(1.000000000000000e+120),
            2.707091661024979e+02,
            epsilon = 1e-13
        );
        assert_abs_diff_eq!(lambert_w_0(1.000000000000000e+160), 3.625205337614976e+02);
    }

    #[cfg(feature = "24bits")]
    #[test]
    fn test_sp_lambert_w_0() {
        assert!(sp_lambert_w_0(-1.0 / E - f64::EPSILON).is_nan());
        assert_abs_diff_eq!(
            sp_lambert_w_0(-2.678794411714424e-01),
            -3.993824525397807e-01,
            epsilon = 1e-7
        );
        assert_abs_diff_eq!(
            sp_lambert_w_0(6.321205588285577e-01),
            4.167039988177658e-01,
            epsilon = 1e-7
        );
        assert_abs_diff_eq!(
            sp_lambert_w_0(9.632120558828557),
            1.721757710976171,
            epsilon = 1e-7
        );
        assert_abs_diff_eq!(
            sp_lambert_w_0(9.963212055882856e+01),
            3.382785211058958,
            epsilon = 1e-7
        );
        assert_abs_diff_eq!(
            sp_lambert_w_0(9.996321205588285e+02),
            5.249293782013269,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w_0(9.999632120558828e+03),
            7.231813718542178,
            epsilon = 1e-7
        );
        assert_abs_diff_eq!(
            sp_lambert_w_0(9.999963212055883e+04),
            9.284568107521959,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w_0(9.999996321205589e+05),
            1.138335774796812e+01,
            epsilon = 1e-8
        );
        assert_abs_diff_eq!(
            sp_lambert_w_0(9.999999632120559e+06),
            1.351434397605273e+01,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w_0(9.999999963212056e+07),
            1.566899671199287e+01,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w_0(9.999999996321206e+08),
            1.784172596707312e+01,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w_0(9.999999999632120e+09),
            2.002868541326992e+01,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w_0(9.999999999963213e+10),
            2.222712273495755e+01,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w_0(9.999999999996321e+11),
            2.443500440493456e+01,
            epsilon = 1e-5
        );
        assert_abs_diff_eq!(
            sp_lambert_w_0(9.999999999999633e+12),
            2.665078750870219e+01,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w_0(9.999999999999962e+13),
            2.887327487929930e+01,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w_0(9.999999999999996e+14),
            3.110151971159478e+01,
            epsilon = 1e-5
        );
        assert_abs_diff_eq!(
            sp_lambert_w_0(1.000000000000000e+16),
            3.333476076844818e+01,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w_0(1.000000000000000e+17),
            3.557237716651325e+01,
            epsilon = 1e-5
        );
        assert_abs_diff_eq!(
            sp_lambert_w_0(1.000000000000000e+18),
            3.781385607558877e+01,
            epsilon = 1e-5
        );
        assert_abs_diff_eq!(
            sp_lambert_w_0(1.000000000000000e+19),
            4.005876916198432e+01,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w_0(1.000000000000000e+20),
            4.230675509173839e+01,
            epsilon = 1e-5
        );
        assert_abs_diff_eq!(
            sp_lambert_w_0(1.000000000000000e+40),
            8.763027715194720e+01,
            epsilon = 1e-5
        );
        assert_abs_diff_eq!(
            sp_lambert_w_0(1.000000000000000e+80),
            1.790193137415062e+02,
            epsilon = 1e-5
        );
        assert_abs_diff_eq!(
            sp_lambert_w_0(1.000000000000000e+120),
            2.707091661024979e+02,
            epsilon = 1e-4
        );
        assert_abs_diff_eq!(
            sp_lambert_w_0(1.000000000000000e+160),
            3.625205337614976e+02,
            epsilon = 1e-4
        );
    }

    #[cfg(feature = "50bits")]
    #[test]
    fn test_lambert_w_m1() {
        assert!(lambert_w_m1(-1.0 / E - f64::EPSILON).is_nan());
        assert_abs_diff_eq!(
            lambert_w_m1(-3.578794411714423e-01),
            -1.253493791367214,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_m1(-2.678794411714424e-01),
            -2.020625228775403,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(lambert_w_m1(-1.000000000000000e-01), -3.577152063957297);
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(lambert_w_m1(-3.000000000000000e-02), -5.144482721515681);
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(
            lambert_w_m1(-3.000000000000000e-02),
            -5.144482721515681,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_m1(-1.000000000000000e-02),
            -6.472775124394005,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_m1(-3.000000000000000e-03),
            -7.872521380098709,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_m1(-1.000000000000000e-03),
            -9.118006470402742,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_m1(-3.000000000000001e-04),
            -1.045921112040100e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_m1(-1.000000000000000e-04),
            -1.166711453256636e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_m1(-3.000000000000000e-05),
            -1.297753279184081e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_m1(-1.000000000000000e-05),
            -1.416360081581018e+01,
            epsilon = 1e-14
        );
        assert_abs_diff_eq!(
            lambert_w_m1(-1.000000000000004e-75),
            -1.778749628219512e+02,
            epsilon = 1e-13
        );
        #[cfg(not(feature = "estrin"))]
        assert_abs_diff_eq!(
            lambert_w_m1(-1.000000000000008e-145),
            -3.397029099254290e+02
        );
        #[cfg(feature = "estrin")]
        assert_abs_diff_eq!(
            lambert_w_m1(-1.000000000000008e-145),
            -3.397029099254290e+02,
            epsilon = 1e-12
        );
        assert!(lambert_w_m1(f64::EPSILON).is_nan());
    }

    #[cfg(feature = "24bits")]
    #[test]
    fn test_sp_lambert_w_m1() {
        assert!(sp_lambert_w_m1(-1.0 / E - f64::EPSILON).is_nan());
        assert_abs_diff_eq!(
            sp_lambert_w_m1(-3.578794411714423e-01),
            -1.253493791367214,
            epsilon = 1e-7
        );
        assert_abs_diff_eq!(
            sp_lambert_w_m1(-2.678794411714424e-01),
            -2.020625228775403,
            epsilon = 1e-7
        );
        assert_abs_diff_eq!(
            sp_lambert_w_m1(-1.000000000000000e-01),
            -3.577152063957297,
            epsilon = 1e-9
        );
        assert_abs_diff_eq!(
            sp_lambert_w_m1(-3.000000000000000e-02),
            -5.144482721515681,
            epsilon = 1e-9
        );
        assert_abs_diff_eq!(
            sp_lambert_w_m1(-1.000000000000000e-02),
            -6.472775124394005,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w_m1(-3.000000000000000e-03),
            -7.872521380098709,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w_m1(-1.000000000000000e-03),
            -9.118006470402742,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w_m1(-3.000000000000001e-04),
            -1.045921112040100e+01,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w_m1(-1.000000000000000e-04),
            -1.166711453256636e+01,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w_m1(-3.000000000000000e-05),
            -1.297753279184081e+01,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w_m1(-1.000000000000000e-05),
            -1.416360081581018e+01,
            epsilon = 1e-6
        );
        assert_abs_diff_eq!(
            sp_lambert_w_m1(-1.000000000000004e-75),
            -1.778749628219512e+02,
            epsilon = 1e-5
        );
        assert_abs_diff_eq!(
            sp_lambert_w_m1(-1.000000000000008e-145),
            -3.397029099254290e+02,
            epsilon = 1e-4
        );
        assert!(sp_lambert_w_m1(f64::EPSILON).is_nan());
    }
}
