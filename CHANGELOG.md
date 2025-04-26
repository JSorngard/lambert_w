# Changelog

This file contains the changes to the crate since version 0.1.1.
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 1.2.15

- Added a note to the readme that the implementation of Fukushima's method is
 simple enough that if the input is known at compile time the optimizer can
 often evaluate the entire function at compile time as well. Every input I have tried on opt-level 3 has succeeded. For an example, see
 [this compiler explorer link](https://godbolt.org/#z:OYLghAFBqd5QCxAYwPYBMCmBRdBLAF1QCcAaPECAMzwBtMA7AQwFtMQByARg9KtQYEAysib0QXACx8BBAKoBnTAAUAHpwAMvAFYTStJg1DEArgoKkl9ZATwDKjdAGFUtEywYgA7AFZSjgBk8BkwAOXcAI0xiEAA2LlIAB1QFQjsGFzcPbz9k1NsBIJDwliiY%2BMtMawKGIQImYgJM909fSur0uoaCIrDI6LiE827m7Lbhxt6SssGASktUE2Jkdg4qBgBqYiYasQB9KhMGG3SAUgBmJzQGcw3QkA2zPAAvTFIN69uAEQen14vsBBThoAIIbcEbdQbKixaTAsEQhjuaI7Eh7NCYKg0ZB4RgEBQPU4%2BABCMMkF2JdyJX1I8IhGywDFQLGCqOI6NQmOxuMEBI2RNJsIpGy%2B1NpoNmGwAtADobD%2BV5iXSIfQCBskWxtkRiPzzl91citWiMVi8Di8QplfTwQA6YJEPaEaIQWZW6024iYABuLrd9Jt/Fo6AgGhtGnepy8TgU7neJqokacGxjLA2ACpIfyAEyU%2BOu85K0HWiFu1UMxjM1na3X6xmV5jajlcs08/F%2BiF2wSoR0EZ2uovFjYe72%2BgfFgOuYOh8MK6Oxj6crGJ5PudOZ045hdc/OFkFujUo6sAenLTJZDZI8MjotB8KP942z2QNcf2cpXCPmHh6wZAHcNMgEBPg8ZKSjK5zYHKkgKru9J4FQj7PhcSbTjB7bgmSIAgKEIKhG614bFUSgbPBiG6k4Fz6lwNqSFwWZ7F4GheHsPhSCxWbMZIsQ%2BHsWaYFwGjQZGsGDg%2BACSYAcF6mAbEwXqoHgWDoBsiQmAQthGLJtC0BsBAIDJqB6dEGwRNsxz6QoJFdrp%2BmEaorCJPQGy0MEADWVkbCQWA6kQ6qGRsLBMK5MmEMmqDOQ0wCYDa6EbA%2BAAqtmeoFwTBMAJlmcgCAkZZAgGQhRkBSkapoCwjlmjsmDKaZhhZZguUFbZPgaCZoVMIkiTEKgqh4IFNSeQhADqewtXpizAAg4oIsWD5MJZoVqXQLz1TZMkNksYjOagwANIQCCpqgjUhQwqkEDFY7WkcvWOZgbCCFVYCQDu%2BFePqREhQhT7kVBWFiaEABif1ifFACaaEXfSWrpPshzHDUQIQ4OT42rQDAulNg7WgKsWY1KobxDxUiSHsdEABwkxxvEAJzMbEGOY4O%2BNeFTDFU%2BcJMaFweznOcsTcxoLPnPTDPWtOpMs7EpM8ecPjs4xxM%2BFLeyk1mwsixC06cyxsJ7JIVPE1I5PnFT5Ns6TavqxsXg2l45zE1TVMaCTpPMaTXC07zmBSnTOODtRWbiyTOu%2BMTsSxCz%2Bvsz4Xvm77xbWxogcaOc9E%2BCbys09zbt7LEXt0Rb6tUzapO0XsNMs8n7OSJTNN8VKXA%2B4jxZinH/Ikq39LUQXIvTpn5waE7WaxHLAm61rkhwk3DOa1z8RO7zzEccTJfE1mGiN9NltW8XWbE14XgGwP/PMV4Xsb1vEL%2B/rZdDyT/cZ8xXB2znMfdwzCfp%2BHXN0VzbOr6T5w86qw7hCIuJd6KxADixQSvEsz0Xlnnc%2BF9pTUTTnzTme8sw8SHk7WIGgeK5ylLvN%2B9IW5T37JvAi71wab3pJhEAf1AahGBiDF6N49yvSvKCH8BhSjRAIHsf8QEQKwjArKMkND6ToH/IBZ4b4NjURaieNAnosL0M%2BPiLC2AKHXi4SCBKCA8CWVKokOgK1fJMGTGlJyc0lClFoAATysuYUwJwBCeU2KgRIBApT0GkjpLMskGDKSYBEVA0lHrwlUhEaEmwNF7G9PsTA9kyr0BdNKcR8phKllYGUARQj6EAFkQQAA04obCzGGHRnDQQcHmLQTgPheCeA4FoUgqBOAACUzBqgUIsZYMkNznB4KQAgmg6nzFciAZqYYB6%2BH7vrSQXguCkzpg0jgkhmnjPaZwXgBJwxjNaXU0gcBYBIBMWYsgFAIAXPoDEYgKy8F8DoL2YgBIIARG2REVkxAHGcBGd85gvyADyERtCYBsP83gpU7oEGBQwRx2ysARBMMAJwYhaAEm4LwLAgUjDiCOaQfAnoTjSSxW05JEK1KrBGfaKo2yXI1V%2BS4LA2yCDEF6lC0g0liBhKUF8W6hhgAuSMOM%2BYVADDAAUAANVxL%2BYFiRGBcv4IIEQYh2BSBkIIRQKh1CEt0KrAworTDmH0HgCIBInrtO8ekLFUokxeJ8X4qolEsy8HCdEDlWBLVQGYGwEAVU0gCG5WIEwqw15Zh4PMCAfr2ANCyiGtw7BVCrJzpIKURxXJMl/AwXxwQTCqClMAJEsx5iJEMGEVgqwU181hBmrNqAc15qRIW4tJgNgQBNQQZ81FVlVL2VUCFNQHDBNGJ4BIgRgh9D4TEBIeQg0ZFcC0PQ86ahTH6LO9oQ7Oj1EaGOvQVht0CC6JMKd0wBhDF3U0Jd2RL3dHXTOiQ8w%2BlLBWE%2B/QjStmEo6RwGtab63ZtzSKgtRakQdq7T24usR%2B2jLFfMfSTBvKUEmSAPWNoHZU13uHfBPgA5cD8OszZpAWltJ/XskABy4Mfo4G64j2yyOwaOaW7l0RUj2EkEAA%3D%3D%3D).
- Documentation improvements.
- Updated transitive dev-dependencies.

## 1.2.14

- Internal documentation improvements.

## 1.2.13

- Documentation improvements.

## 1.2.12

- Small improvements to the implementation of the complex Lambert W functions.

## 1.2.11

- Documentation improvements.

## 1.2.10

- Fixed a bug that could result in incorrect outputs from the complex `lambert_w`
 and `lambert_wf` functions near 0 on branch 1.
- Added an assertion to the first example in the readme that the computed
 omega constant has the claimed properties.
- Internal code improvements.

## 1.2.9

- Show the import from the `approx` crate in the doc-examples of the functions.
- Test the crate on multiple operating systems in CI.

## 1.2.8

- Internal code structure improvements.
- Internal doc improvements.
- Update transitive dev-dependencies.

## 1.2.7

- Internal code improvements.
- Internal doc improvements.
- Improvements to CI.

## 1.2.6

- Internal code improvements.
- Internal doc improvements.
- Update transitive dev-dependencies.

## 1.2.5

- Remove `libm` as a direct dependency.
 It is only used through the `num-traits` and `num-complex` crates now.
- Updated transitive dev-dependencies.

## 1.2.4

- Improvements to docs.
- Return early in the complex-valued functions if we know we will not be able to
 compute an answer

## 1.2.3

- Improvements to docs.

## 1.2.2

- Improvements to docs.

## 1.2.1

- Internal improvements to the implementation of the complex Lambert W functions.

## 1.2.0

- Added a `f32` version of the complex valued Lambert W function.

## 1.1.0

- Added an implementation that can compute any branch in the whole complex plane.
- Added the `must_use` attribute to all pure functions.
- Added the [`num-traits`](https://crates.io/crates/num-traits) and
 [`num-complex`](https://crates.io/crates/num-complex) crates to dependencies.
- Deprecated the `LambertW` trait, as this crate is not really the place for
 such API decisions to be made on behalf of others.
 It is also unclear how the trait should be defined given the newly introduced
 general implementation.
- Updated transitive dev-dependencies.

## 1.0.20

- Updated transitive dev-dependencies.

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
