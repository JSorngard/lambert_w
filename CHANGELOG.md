# Changelog

This file contains the changes to the crate since version 0.1.1.
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 1.2.15 (unreleased)

- Added a note to the README that the implementation of Fukushima's method is
 simple enough that if the input is known at compile time the optimizer can
 sometimes evaluate the entire function at compile time as well. The only reason
 for using the word "somtimes" in that sentence is that I haven't tested all inputs.
 Every input I have tried on opt-level 3 has succeeded. For an example, see
 [this compiler explorer link](https://godbolt.org/#z:OYLghAFBqd5QCxAYwPYBMCmBRdBLAF1QCcAaPECAMzwBtMA7AQwFtMQByARg9KtQYEAysib0QXACx8BBAKoBnTAAUAHpwAMvAFYTStJg1DEArgoKkl9ZATwDKjdAGFUtEywYhJANlKOAMngMmABy7gBGmMQg3gBMpAAOqAqEdgwubh5evkkptgKBwWEskdFxlpjW%2BQxCBEzEBBnunj4VVWm19QSFoRFRMfHmXU1ZrUMNPcWlAwCUlqgmxMjsHAD06wDUAF7IGwCkAMwAItv7sQBCG1yrmHsaAIJUDBvoAO4ayBA7IBtU3pIzDYAWkO2F%2B/32AHZznd7ht4Rs8FRtrtDk4NhoAHQaKEwh4IgngyQgEAhe4hWEEvaQk6VJSI5E7fYHJyHE5cTGSLixAD6kI0kJ5AFYpMLYoKfEKebFMFwNJJcZTCfDNgBJMAcABumA2TE1qDwWHQGwSJgItiMutotA2BAQOtQdqiG3CxEMyHtCkRglQtvtG0wqlYCXoG1oQQA1t6NiQsMRbb6GI6NiwmBGdYQNgpfQZiMBMJilcrNgAVf3ETCpoJBYAut0MD2Ir1OlPJAgbNAsEN4UQETDG13uhAx5EtoU48KZpgJBLEVCqPCp6ojjYAdR5OLtC2ACFIRcJmyYXszZroeC2mGb/uYBEWYjDqGA9UICBYK5bQVNBEL%2BOVGxMDCLiGlaMH26BgJAMyHHicIItStK0PSSIosy6J/MSICqiEABiWGqiWACaiq/sqbrVGIPJUABNhpBA%2B5/iimK0AwEBzPRf57EKMEMQxQJYt4XBSlIkg8tyAAconitKACcgq%2BOxPEYpi3iQtJfLSQcokaFwPIHAc3i6RoakHHuJGKQiWIaGJaneGJUoHEKmn8iJQp2TyYnxApPGWdpwr/DykjSSJUgSQc0kSRpYmmbB5kIpCmKQgcInSdJGiiWJgpiVwcn6ZgQLyWZsUcrE1mif5kJCiJ3jeGpQWaUKeVRV5DHxVZakaAcvJCuF7mybpWU8t4eXctFsUEtJmJiVyPKye1ekBVJskykCXAFTFPGcUco3mZx3FjfCHLbftll9QcGhpbE3hOXKAW%2BZI0jNX%2BPk6QJaX6YK4oiVNImxBoa37fC8UeSJkKQsF52GYKkJ5f9APFUFM2XaJZ29YKXBJYNjVHWNrU9TVOncjpGk/WJBzDZ5hXmRNU28nEEnjj9sS8s5w2w/tK2Yt1BnaSDsRSpdaXeBoUpDUCsQPZTyqbdjBJQWZ8EBohOrUntyroSSWG4SE%2BEEfu8GwvrDywk8GxkWkFFUQ21RomgDDmBsIQ/GY56YKQHYCPbRxOykF6gnRZnqES2MMO4URMEQxA8mgmBUDQyB4KBCg/Lt6HQQ70v7lgSYsEE4ckFHqAx3HCeCEn%2BxcanByXEcGcPICIIHGC6HEet9DtiHbBkSQzInB3YcRwXRc9iXBAKF5mJBEQPKEFErHjxWmpz5LGyYvwtDoBAllu9STgKO4bvR7HO9Zu4GwAFQbKoZyXIfVBQVXXn7m3LyMKgOc3t3bIv9nucD7fw%2BJ3HpPVA08%2BzECXutAkmIF4QIYqvVwG8t5Ql3vvd2Rdj57zfBfK%2BewLhoNjvfGC%2B4%2B5d3jKsb%2Bb9f4kANjSA2DwTYGBKFEAgPJ3hfB%2BOheuoIiQtwJG8D4Xxr5XGxBschaAKwknVigD2o8STYDlvcQ2iiHibCEAgBY68XQ6n5LETmSUMp/TBhlFSXBpK6mQOI/AloiCwk2Agc0CQk7rFeC4zErxXBUDdKmWgCQEBMExJ2VYn4zSHGwngNk/hWClAIKuXBQouIaDibEVkFwklZRWkk7CsQ4lCmwGk/4K0NAZL5rEaSSTpJn2yXzXJaSqnxOwHKcpmS6lChrio8hhhjSZjwF6HRejJAGJUgMyEQyfz3E2Pcds9AjztjtOHJsGxXh%2BPbAATwWBsfM7ZkJrJMO7Bg2oGh%2Bh1PaK%2B6Bw5MGjC2W2QxBCXP9AsAgX5FlBHQKgV4CZdS/FoKgcONYTQGluR3UoYzTThF%2BM8a5LDMCagooGYM9BWLAm4c3FWT8onMNYRoag/wSQAFl7gAA1REbF0RoBRSiOBzFoJwIUvBPAcC0KQVAnAABKZh2zZkWMsM4BweCkAIJoSlcwIwgHHNic6FUzpBUkJCLgYlfDUo4JIOlgqmWcF4EnDQ/LBVzDgLAJAnYEh0CiOQSghrjXRGIHKoWfA6BgKThAcIqrJzMGICszgfKXX1BWQAeXCNoTANgPW8E7GwQQPqGC0HdQy3gWBwgmGAE4MQiFg2kCwKmIw4gY1prwBWGi2ok7ZsDIGs0Kw%2BWT0qKq8Mg43UuCwKqggxBFypoOeEZImAjiVkMMAcMRgdV8AMMABQAA1BOrwfUJEYKm/gggRBiHYFIGQghFAqHUNm3Q8QDB9tMOYfQeBwhJ0gkyhI1RC1AnRKgE9QJ6DaloGyA4vBUAHKbVgQ9UBmBsBAP2VIAhSAwrcCsX6sQeC6o/eweoHo/1iBMOwVQ8rBqSCBABCMSZXgMGvUEEwqggTABDjMOYCRDChFYCsODBkCkmBQ289DvasM4ZDhsCAO6CC7A5KTbEMwNWVEDdUBwDBnCuGaHoAIQRehMOiFwRIyQf3pEE1kSTuQZOTD6BJtoPGOh1AaCMTwkmrDqYEJ0CYompj9F05pxocmdOWHM8p8TEg5icqWAuqlNKVXZuZRwMjCGkNUbQxhkO2HcO7KY%2By1jk0Dgcd4AKmN%2BHSD2iYHGSgwqvATRStJcWNVhZChKoJfQnBlWkHpYyjzGqQBaui1oWLirYhueK%2Bq7VMW5gHJSPYSQQA).

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
