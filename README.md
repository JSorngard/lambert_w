[![Static Badge](https://img.shields.io/badge/github-JSorngard%2Flambert__w-8da0cb?logo=github)](https://github.com/JSorngard/lambert_w)
[![Crates.io Version](https://img.shields.io/crates/v/lambert_w?logo=crates.io)](https://crates.io/crates/lambert_w)
[![docs.rs](https://img.shields.io/docsrs/lambert_w?logo=docs.rs)](https://docs.rs/lambert_w/0.1.0/lambert_w/)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/JSorngard/lambert_w/rust.yml?logo=github&label=CI)](https://github.com/JSorngard/lambert_w/actions/workflows/rust.yml)

# lambert_w

Fast evaluation of the real valued parts of the principal and secondary branches of the [Lambert W function](https://en.wikipedia.org/wiki/Lambert_W_function) using the [method of Toshio Fukushima](https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation) to either 24 or 50 bits of accuracy.

This method uses a piecewise minimax rational function approximation with variable transformations.
It is implemented in code as conditional switches on the input value followed by either a square root (and a possibly a division) or a logarithm and finished by a series of cumulative multiplies and additions by fixed constants.

## Examples

Compute the value of the [Omega constant](https://en.wikipedia.org/wiki/Omega_constant) with the principal branch of the Lambert W function to 50 bits of accuracy:
```rust
use lambert_w::accurate::lambert_w_0;

use approx::assert_abs_diff_eq;

let w = lambert_w_0(1.0).unwrap();

assert_abs_diff_eq!(w, 0.5671432904097838);
```

or to only 24 bits of accuracy, but with faster execution time:
```rust
use lambert_w::fast::lambert_w_0;

use approx::assert_abs_diff_eq;

let w = lambert_w_0(1.0).unwrap();

assert_abs_diff_eq!(w, 0.5671432904097838, epsilon = 1e-7);
```

## Speed-accuracy trade-off

The 50-bit accurate versions in the `accurate` module are more accurate, but slightly slower, than the 24-bit accurate versions in the `fast` module.
`fast::lambert_w_0` is around 15% faster than `accurate::lambert_w_0` and `fast::lambert_w_m1` is around 41% faster than `accurate::lambert_w_m1`.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
