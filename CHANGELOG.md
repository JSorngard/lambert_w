# Changelog

This file contains the changes to the crate since version 0.1.1.
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 1.1.0

- Added an implementation that can compute any branch in the whole complex plane.
- Added the `must_use` attribute to all pure functions.
- Added the `num-traits` and `num-complex` crates to dependencies.

## 1.0.20 (unreleased)

- Updated dev-dependencies.

## 1.0.19

- Documentation improvements.

## 1.0.18

- Made the error bounds in tests stricter and clearer.
- Made small improvements to the examples in the readme.
- Added a CI job that checks and builds the crate for targets without a standard
 library.
- Added a CI job that locks all dependencies to the oldest possible version according
 to `Cargo.toml` and then checks the crate.
- Added daily CI jobs that test the crate on nightly.
- Added the "no-std" category.
- Updated transitive dev-dependencies.

## 1.0.17

- Clarified information in README.

## 1.0.16

- Mentioned some interesting properties of the method in the readme.

## 1.0.15

- Don't panic!
 Enabled static verification that this crate can not panic using the [`no-panic`](https://crates.io/crates/no_panic)
 crate.
 This does not affect any dependers, as `no-panic` is added as a dev-dependency.
- Added a CI job that uses the above to ensure no panics make it into the crate.
 This verification can also be run manually on a local copy of the crate by
 setting the environment variable `LAMBERT_W_ENSURE_NO_PANICS` to 1 and
 then running `cargo test --profile release-lto`.
- Implemented all the rational functions using a single generic function
  instead of several different hand made ones.
- Sped up the `semver-checks` CI job.
- Removed the "no_std" category from the crate, as it's already in the
 "no_std::no_alloc" category, which is a subset of "no_std".
- The text in the README is now also the crate documentation on docs.rs.
- Added tests of the functions on a large set of valid randomly generated inputs.
- Made the docs.rs badge use a different color and display the crate name
 instead of "passing".

## 1.0.14

- Updated the dev-dependency on `rand` to v0.9.0.
- Added a CI job that compiles the benchmarks.
- Added a CI job that tests the crate on the Rust beta branch.
- Updated transitive dev-dependencies.

## 1.0.13

- Removed the note about the accuracy on the trait functions,
 as that is different depending on the type that the trait is invoked on.
- Updated transitive dev-dependencies.

## 1.0.12

- Noted the accuracy of the functions on the trait in the example.
- Improvements to CI jobs.
- Updated dev-dependencies.

## 1.0.11

- Removed unnecessary import in `integration_tests.rs`.
- Improvements to CI jobs.

## 1.0.10

- Moved unit tests to their own module.
- Corrected some information in code comments.
- Simplified `semver-checks` CI job.
- Made minor changes to the code of the plot example to show the
 function clearer in the final image.
- Note adherence to semver in this log.

## 1.0.9

- Switched the way the crate depends on the standard library such that the
 implicit prelude is always the same.
- Sped up CI runs by using `taiki-e/install-action`.
- Added an example program that plots both branches of the function.

## 1.0.8

- Fixed a bug where the principal branch functions would return NaN when given
 infinite input.

## 1.0.7

- Moved tests to their own file.
- Made the accuracy of the tests clearer.

## 1.0.6

- Added more unit tests that verify and showcase the accuracy of the
 functions also at the branch point.
- Verify the MSRV in CI using `cargo-msrv`.
- Check semver compatibility in CI using `cargo-semver-checks`.

## 1.0.5

- Added a "References" section to the readme and docs.
- Added a "⬆️ Back to top" link to the end of the readme and docs, just after
 the references section.

## 1.0.4

- Added the "No standard library" category to the crate.

## 1.0.3

- Clarified that we do not depend on a specific `libm` patch version.
- Changed the `rust-version` field in `Cargo.toml` to 1.63
 since that is now the MSRV of `libm`.

## 1.0.2

- Updated `libm` dependency.

## 1.0.1

- Documentation improvements.

## 1.0.0

- Removed the `estrin` feature.
 If it was activated anywhere in the dependency tree the crate became less
 accurate for all users without them being able to do anything about it
 (as the compiler assumes that features are additive).
- Removed the `24bits` and `50bits` features. Their only use was to reduce binary
 size and speed up compile time by letting the user skip compilation of parts
 of the crate if they didn't use them. However, the crate is small and very quick
 to compile, and the unused code should be removed during dead code elimination anyway.

## 0.5.9

- Added the `LambertW` trait that lets the user call the Lambert W functions
 with postfix notation.

## 0.5.5 - 0.5.8

- Documentation improvements.

## 0.5.4

- Added `lambert_w0f` and `lambert_wm1f` functions that evaluate the 24-bit accurate
 approximation on 32-bit floats (though the approximation is expected to be
 slightly less accurate then).

## 0.5.2 and 0.5.3

- Documentation improvements.

## 0.5.1

- Added `std` and `libm` features to match the features on the optional
 dependency `fast_polynomial`.
- Made the crate `no_std` with a dependency on [`libm`](https://crates.io/crates/libm)
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

- Removed the last underscore in function names. E.g. `lambert_w_0` is renamed
 to `lambert_w0`. This makes them easier to type and the new names are similar
 to the names given to these functions in libraries in other languages.

## 0.4.4

- Documentation improvements.
- Updated the optional `fast_polynomial` dependency.

## 0.4.2 and 0.4.3

- Corrected a mistake in doc information.

## 0.4.1

- Added the optional `estrin` feature that computes the Lambert W function faster
 on modern hardware by using [Estrin's scheme](https://en.wikipedia.org/wiki/Estrin's_scheme)
 to evaluate the polynomials in the rational functions.
 May result in slight numerical instability, which can be reduced if the target
 CPU has fused multiply-add instructions.
- Lowered the MSRV to 1.60.0.
- It is no longer a forced `compile_error!` to disable both the `24bits` and
 `50bits` features.
- Documentation improvements.

## 0.4.0

### Breaking changes

- Made the Lambert W functions return `f64::NAN` when given inputs outside their
 domain. This is in line with how it is usually handled in the standard library.

### Other changes

- Now exports the constants `NEG_INV_E` and `OMEGA`.

## 0.3.0

### Breaking changes

- Removed the `fast` and `accurate` modules and instead export the functions directly.
- Add sp_* prefix to the 24 bit versions.

## 0.2.6

- Minor documentation improvements.

## 0.2.5

- Corrected the domain bounds in the function documentation strings.
- Other minor documentation improvements.

## 0.2.2, 0.2.3, and 0.2.4

- Documentation improvements.

## 0.2.1

- Added github repository badge to `README.md`.

## 0.2.0

### Breaking changes

- The Lambert W functions now return an `Option` instead of a `Result`
 with custom error types.

### Other changes

- The `fast` module is behind the (enabled by default) feature flag `24bits`.
- The `accurate` module is behind the (enabled by default) feature flag `50bits`.
