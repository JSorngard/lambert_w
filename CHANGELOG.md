# Changelog

This file contains the changes to the crate since version 0.1.1.

## 0.4.1

- Add the optional `estrin` feature that computes the Lambert W function up to
 ~25% faster by using
 [Estrin's scheme](https://en.wikipedia.org/wiki/Estrin's_scheme) to evaluate
 the polynomials in the Padé approximants.
 May result in slight numerical instability, which can be mitigated if the target
 cpu has fused multiply-add instructions.
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
