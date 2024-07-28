[![Crates.io Version](https://img.shields.io/crates/v/lambert_w?logo=crates.io)](https://crates.io/crates/lambert_w)
[![docs.rs](https://img.shields.io/docsrs/lambert_w?logo=docs.rs)](https://docs.rs/lambert_w/0.1.0/lambert_w/)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/JSorngard/lambert_w/rust.yml?logo=github&label=CI)](https://github.com/JSorngard/lambert_w/actions/workflows/rust.yml)

# lambert_w

Fast evaluation of the principal and secondary branches of the [Lambert W function](https://en.wikipedia.org/wiki/Lambert_W_function) using the [method of Toshio Fukushima](https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation) to either 24 or 50 bits of accuracy.

This method uses a piecewise minimax rational approximation of the function.

This crate is a Rust port of the original Fortran 90 code by T. Fukushima.

## Examples

Evaluate the principal branch of the Lambert W function to 50 bits of accuracy:
```rust
use lambert_w::accurate::lambert_w_0;
use core::f64::consts::PI;
use approx::assert_abs_diff_eq;

let w = lambert_w_0(PI)?;

assert_abs_diff_eq!(w, 1.0736581947961492);
```

or to only 24 bits of accuracy, but with faster execution time:
```rust
use lambert_w::fast::lambert_w_0;
use core::f64::consts::PI;
use approx::assert_abs_diff_eq;

let w = lambert_w_0(PI)?;

assert_abs_diff_eq!(w, 1.0736581947961492, epsilon = 1e-7);
```