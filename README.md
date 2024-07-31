[![Static Badge](https://img.shields.io/badge/github-JSorngard%2Flambert__w-8da0cb?logo=github)](https://github.com/JSorngard/lambert_w)
[![Crates.io Version](https://img.shields.io/crates/v/lambert_w?logo=crates.io)](https://crates.io/crates/lambert_w)
[![docs.rs](https://img.shields.io/docsrs/lambert_w?logo=docs.rs)](https://docs.rs/lambert_w/latest/lambert_w/)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/JSorngard/lambert_w/rust.yml?logo=github&label=CI)](https://github.com/JSorngard/lambert_w/actions/workflows/rust.yml)

# lambert_w

Fast evaluation of the real valued parts of the principal and secondary branches of the [Lambert W function](https://en.wikipedia.org/wiki/Lambert_W_function) using the [method of Toshio Fukushima](https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation) to either 24 or 50 bits of accuracy.

This method uses a piecewise minimax rational function approximation with variable transformations.  
It is implemented in code as conditional switches on the input value followed by either a square root (and possibly a division) or a logarithm and finished by a series of multiplications and additions by fixed constants and a division. 

The functions with 50 bits of accuracy use more switches for a finer split of the domain and more of the final multiplications and additions than the functions with 24 bits of accuracy.

## Examples

Compute the value of the [Omega constant](https://en.wikipedia.org/wiki/Omega_constant) with the principal branch of the Lambert W function to 50 bits of accuracy:
```rust
use lambert_w::lambert_w_0;

use approx::assert_abs_diff_eq;

let Ω = lambert_w_0(1.0);

assert_abs_diff_eq!(Ω, 0.5671432904097838);
```

or to only 24 bits of accuracy, but with faster execution time:
```rust
use lambert_w::sp_lambert_w_0;

let Ω = sp_lambert_w_0(1.0);

assert_abs_diff_eq!(Ω, 0.5671432904097838, epsilon = 1e-7);
```
Evaluate the secondary branch of the Lambert W function at -ln(2)/2 to 50 bits of accuracy:
```rust
use lambert_w::lambert_w_m1;

let mln4 = lambert_w_m1(-f64::ln(2.0) / 2.0);

assert_abs_diff_eq!(mln4, -f64::ln(4.0));
```
or 24 bits:
```rust
use lambert_w::sp_lambert_w_m1;

let mln4 = sp_lambert_w_m1(-f64::ln(2.0) / 2.0);

assert_abs_diff_eq!(mln4, -f64::ln(4.0), epsilon = 1e-9);
```

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
