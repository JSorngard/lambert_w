# Changelog

This file contains the changes to the crate since version 0.1.1.

## Unreleased

- Remove `estrin` feature.
 If it was activated anywhere in the dependency tree the crate became less
 accurate for all users (as the compiler assumes that features are additive),
 without them being able to do anything about it.

## 0.5.9

- Add the `LambertW` trait that lets the user call the Lambert W functions
 with postfix notation.

## 0.5.5 - 0.5.8

- Documentation improvements.

## 0.5.4

- Add `lambert_w0f` and `lambert_wm1f` functions that evaluate the 24-bit accurate
 approximation on 32-bit floats (though the approximation is expected to be
 slightly less accurate then).

## 0.5.2 and 0.5.3

- Documentation improvements.

## 0.5.1

- Add `std` and `libm` features to match the features on the optional
 dependency `fast_polynomial`.
- Make the crate `no_std` with a dependency on [`libm`](https://crates.io/crates/libm)
 by default. The crate can be made to depend on the standard library instead of
 `libm` by disabling default features and enabling the `std` feature. This can
 result in a performance gain.

## 🗑️~~0.5.0~~

Yanked because 0.5.1 adds a default feature that hides previously included
behavior.
Therefore upgrading from 0.5.0 to 0.5.1 was a breaking change if the user
had disabled default features. By yanking this version the breaking change
happens when upgrading from 0.4.4 to 0.5.1, which requires an intentional
choice by the user, and wont happen automatically with `cargo update` as before.

### Breaking changes

- Remove last underscore in function names. E.g. `lambert_w_0` is renamed to `lambert_w0`.
 This makes them easier to type and the new names are similar to the names given
 to these functions in libraries in other languages.

## 0.4.4

- Documentation improvements.
- Update the optional `fast_polynomial` dependency.

## 0.4.2 and 0.4.3

- Correct mistake in doc information.

## 0.4.1

- Add the optional `estrin` feature that computes the Lambert W function faster
 on modern hardware by using [Estrin's scheme](https://en.wikipedia.org/wiki/Estrin's_scheme)
 to evaluate the polynomials in the rational functions.
 May result in slight numerical instability, which can be reduced if the target
 CPU has fused multiply-add instructions.
- Lower the MSRV to 1.60.0.
- No longer a forced `compile_error!` to disable both the `24bits` and `50bits` features.
- Documentation improvements.

## 0.4.0

### Breaking changes

- Make the Lambert W functions return `f64::NAN` when given inputs outside their
 domain. This is in line with how it is usually handled in the standard library.

### Other changes

- Export the constants `NEG_INV_E` and `OMEGA`.

## 0.3.0

### Breaking changes

- Removed the `fast` and `accurate` modules and instead export the functions directly.
- Add sp_* prefix to the 24 bit versions.

## 0.2.6

- Minor documentation improvements.

## 0.2.5

- Correct the domain bounds in the function documentation strings.
- Other minor documentation improvements.

## 0.2.2, 0.2.3, and 0.2.4

- Documentation improvements.

## 0.2.1

- Add github repository badge to `README.md`.

## 0.2.0

### Breaking changes

- Lambert W functions now return an `Option` instead of a `Result`
 with custom error types.

### Other changes

- The `fast` module is behind the (enabled by default) feature flag `24bits`.
- The `accurate` module is behind the (enabled by default) feature flag `50bits`.
