# lambert_w

Fast evaluation of the principal and secondary branches of the [Lambert W function](https://en.wikipedia.org/wiki/Lambert_W_function) using the method of [Toshio Fukushima](https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation) to either 24 or 50 bits of accuracy.

This method uses a piecewise minimax rational approximation of the function.

## Examples

Evaluate the principal branch of the Lambert W function to 50 bits of accuracy:
```rust
use lambert_w::accurate::lambert_w_0;
use core::f64::consts::PI;
use approx::assert_abs_diff_eq;

let w = lambert_w0(PI)?;

assert_abs_diff_eq!(w, 1.0736581947961492);
```

or to only 24 bits of accuracy, but with faster execution time:
```rust
use lambert_w::fast::lambert_w_0;
use core::f64::consts::PI;
use approx::assert_abs_diff_eq;

let w = lambert_w0(PI)?;

assert_abs_diff_eq!(w, 1.0736581947961492, epsilon = 1e-7);
```