// Copyright 2024-2026 Johanna Sörngård
// SPDX-License-Identifier: MIT OR Apache-2.0

// Fukushima's own Fortran implementation of the method in the paper can be found at
// https://www.researchgate.net/publication/346096162_xlambwtxt_Fortran_90_test_program_package_of_sw0_swm1_dw0c_and_dwm1c_low-_and_high-precision_procedures_computing_primary_and_secondary_branch_of_Lambert_W_function_W_0z_or_W_-1z_by_piecewise_minimax_

// These Markdown ideas are taken from https://linebender.org/blog/doc-include.
//
// This style is used in the readme itself to hide specific parts of it when rendered on docs.rs.
//! <style>
//! .rustdoc-hidden { display: none; }
//! </style>
// These links take precedence over the ones in the readme since they occur first.
// This allows us to switch where the listed hyperlinks take the user depending on whether the docs are read on crates.io or docs.rs.
//! [`approx`]: https://docs.rs/approx/latest/approx/
//! [`libm`]: https://docs.rs/libm/latest/libm/
#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]
#![forbid(clippy::unwrap_used)]
#![forbid(clippy::expect_used)]
#![forbid(clippy::panic)]
// Not forbid because we use this in some constant blocks.
#![deny(clippy::indexing_slicing)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(any(feature = "std", feature = "libm")))]
compile_error!("at least one of the `std` or `libm` features must be enabled");

mod all_complex_branches;
mod dw0c;
mod dwm1c;
mod generic_math;
mod sw0;
mod sw0f;
mod swm1;
mod swm1f;
#[cfg(test)]
mod unit_tests;

pub use all_complex_branches::{lambert_w, lambert_wf};
pub use dw0c::lambert_w0;
pub use dwm1c::lambert_wm1;
pub use sw0::sp_lambert_w0;
pub use sw0f::lambert_w0f;
pub use swm1::sp_lambert_wm1;
pub use swm1f::lambert_wm1f;

/// The floating point value closest to the negative reciprocal of e (-1/e), the branch point of the W function.
///
/// ```text
/// 64-bit floats:   f--          f-           f+           f++
/// real line:     --|------------|---x--------|------------|-------> z
///                                 -1/e
/// ```
/// f- is this constant and x marks the location of the true value of -1/e.
/// f+ is the constant [`SMALLEST_F64_IN_DOMAIN`].
///
/// # Example
///
/// ```
/// use lambert_w::{lambert_w0, NEG_INV_E};
///
/// assert!(lambert_w0(NEG_INV_E).is_nan());
/// assert!(!lambert_w0(NEG_INV_E.next_up()).is_nan())
/// ```
#[allow(clippy::excessive_precision)]
pub const NEG_INV_E: f64 = -0.367_879_441_171_442_321_595_523_770;

/// The difference between the true value of -1/e and `NEG_INV_E`.
const NEG_INV_E_CORRECTION: f64 = -1.2428753672788363e-17;

/// Same as [`NEG_INV_E_CORRECTION`] but for 32-bit floats.
///
/// The value that this constant corrects is then `NEG_INV_E as f32`, equal to -0.367_879_45.
const NEG_INV_E_CORRECTION_32_BIT: f32 = -9.149_756e-9;

/// This floating point value is the smallest one that is larger than or equal to -1/e.
/// It is *not* the one that is the closest to -1/e, rather this value is one step further in the direction towards positive infinity.
///
/// ```text
/// 64-bit floats:   f--          f-           f+           f++
/// real line:     --|------------|---x--------|------------|-------> z
///                                 -1/e
/// ```
/// f- is the constant [`NEG_INV_E`], f+ is this constant, and x marks the location of the true value of -1/e.
///
/// # Example
///
/// ```
/// use lambert_w::{lambert_w0, SMALLEST_F64_IN_DOMAIN};
///
/// assert!(!lambert_w0(SMALLEST_F64_IN_DOMAIN).is_nan());
/// assert!(lambert_w0(SMALLEST_F64_IN_DOMAIN.next_down()).is_nan());
/// ```
pub const SMALLEST_F64_IN_DOMAIN: f64 = -0.367_879_441_171_442_3;

/// Same as [`SMALLEST_F64_IN_DOMAIN`] but for 32-bit floats.
///
/// ```text
/// 32-bit floats:   f--          f-           f+           f++
/// real line:     --|------------|---x--------|------------|-------> z
///                                 -1/e
/// ```
/// f- is the constant [`NEG_INV_E`] cast to `f32`, f+ is this constant, and x marks the location of the true value of -1/e.
///
/// # Example
///
/// ```
/// use lambert_w::{lambert_w0f, SMALLEST_F32_IN_DOMAIN};
///
/// assert!(!lambert_w0f(SMALLEST_F32_IN_DOMAIN).is_nan());
/// assert!(lambert_w0f(SMALLEST_F32_IN_DOMAIN.next_down()).is_nan());
/// ```
pub const SMALLEST_F32_IN_DOMAIN: f32 = -0.367_879_42;

/// Closer to zero than this we use the Taylor expansion on the principal branch.
///
/// This value was determined empirically.
const PRINCIPAL_BRANCH_NEAR_ZERO_THRESHOLD: f64 = 0.0675;

/// Closer to zero than this we approximate the principal branch as z - z^2.
const PRINCIPAL_BRANCH_APPROXIMATELY_ZERO_THRESHOLD: f64 = 1e-9;

const PRINCIPAL_BRANCH_MAX_TRICKY_REGION: f64 = 0.15;

/// The floating point value closest to 1/sqrt(e).
#[allow(clippy::excessive_precision)]
const INV_SQRT_E: f64 = 0.606_530_659_712_633_423_603_799_535;

/// The floating point value closest to the omega constant (Ω).
///
/// Fulfills the equation Ωe^Ω = 1:
///
/// ```
/// use lambert_w::OMEGA;
/// use approx::assert_abs_diff_eq;
///
/// assert_abs_diff_eq!(OMEGA * f64::exp(OMEGA), 1.0);
/// ```
#[allow(clippy::excessive_precision)]
pub const OMEGA: f64 = 0.567_143_290_409_783_873;

/// Helper function that returns an array of Taylor coefficients to the given degree.
///
/// Needed because there is no nice short idiom to const convert an array into a smaller one at this MSRV.
#[allow(clippy::indexing_slicing)]
const fn w0_taylor_coefficients<const DEGREE: usize>() -> [f64; DEGREE] {
    /// The coefficients of the Taylor expansion of the principal branch for |z| < 1/e.
    ///
    /// Source: equation (5) in section 4.13 of the NIST Digital Library of Mathematical Functions.
    /// Evaluated with `mpmath` in `python`.
    const TAYLOR_COEFFICIENTS: [f64; 99] = [
        1.0,
        -1.0,
        1.5,
        -2.6666666666666665,
        5.208333333333333,
        -10.8,
        23.343055555555555,
        -52.01269841269841,
        118.62522321428571,
        -275.5731922398589,
        649.7871723434745,
        -1551.1605194805195,
        3741.4497029592385,
        -9104.500241158019,
        22324.3085127066,
        -55103.621972903835,
        136808.86090394293,
        -341422.05066583835,
        855992.9659966076,
        -2154990.2060910882,
        5445552.922314462,
        -13807330.002166629,
        35117044.985139236,
        -89568002.56102797,
        229041684.61879498,
        -587103504.1171799,
        1508256053.8577929,
        -3882630161.293189,
        10013943136.654829,
        -25873567362.657608,
        66962097093.58074,
        -173571165959.9198,
        450568046564.2354,
        -1171223178256.4873,
        3048462517882.447,
        -7944240398206.812,
        20726462893727.055,
        -54134309927745.64,
        141537001323842.75,
        -370420141783066.7,
        970343148988291.6,
        -2544149870142830.0,
        6676186549945615.0,
        -1.7533383884497608e+16,
        4.60827878998424e+16,
        -1.2120840403841997e+17,
        3.1903175600194195e+17,
        -8.402903354222312e+17,
        2.21465954871603e+18,
        -5.840571433838374e+18,
        1.5412168778694736e+19,
        -4.069322719765488e+19,
        1.0750309245601006e+20,
        -2.841523479625779e+20,
        7.514576699787185e+20,
        -1.9882580395853338e+21,
        5.263181221319305e+21,
        -1.393875696142799e+22,
        3.6931170823009246e+22,
        -9.789238808348462e+22,
        2.5958848046636118e+23,
        -6.886470406171994e+23,
        1.82758313942499e+24,
        -4.852007501582215e+24,
        1.2886191206597694e+25,
        -3.423588987248066e+25,
        9.098881018831215e+25,
        -2.419018815528663e+26,
        6.433261047507392e+26,
        -1.7114323043280685e+27,
        4.554293613099411e+27,
        -1.21230351895065e+28,
        3.227952889170164e+28,
        -8.597359350864599e+28,
        2.2904550168353216e+29,
        -6.103713000670467e+29,
        1.6269682265597198e+30,
        -4.337842335304133e+30,
        1.1568455628188089e+31,
        -3.085895746764332e+31,
        8.233581124561597e+31,
        -2.1973308306283072e+32,
        5.865416934086536e+32,
        -1.5660180769945114e+33,
        4.182027286033164e+33,
        -1.1170355799760446e+34,
        2.9842494563739106e+34,
        -7.974238176573383e+34,
        2.1312151963492572e+35,
        -5.697017455033175e+35,
        1.523169120402824e+36,
        -4.0731206898104024e+36,
        1.089389810261902e+37,
        -2.914167919306647e+37,
        7.796855402425589e+37,
        -2.0863947853703042e+38,
        5.583983227480016e+38,
        -1.494723567161428e+39,
        4.001708187541245e+39,
    ];

    let mut arr = [0.0; DEGREE];
    let mut i = 0;
    while i < DEGREE {
        arr[i] = TAYLOR_COEFFICIENTS[i];
        i += 1;
    }
    arr
}

/// Same as w0_taylor_coefficients but returns f32 values instead of f64 values.
const fn w0f_taylor_coefficients<const DEGREE: usize>() -> [f32; DEGREE] {
    let mut arr = [0.0; DEGREE];
    let mut i = 0;
    while i < DEGREE {
        arr[i] = w0_taylor_coefficients::<DEGREE>()[i] as f32;
        i += 1;
    }
    arr
}
