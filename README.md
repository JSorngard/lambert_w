# `lambert_w`

[![Crates.io Version](https://img.shields.io/crates/v/lambert_w?logo=rust)](https://crates.io/crates/lambert_w)
[![Docs.rs Documentation](https://img.shields.io/docsrs/lambert_w?logo=docs.rs)](https://docs.rs/lambert_w/latest/lambert_w/)
[![Github Repository Link](https://img.shields.io/badge/github-JSorngard%2Flambert__w-8da0cb?logo=github)](https://github.com/JSorngard/lambert_w)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/JSorngard/lambert_w/rust.yml?logo=github&label=CI)](https://github.com/JSorngard/lambert_w/actions/workflows/rust.yml)
[![Code Coverage](https://codecov.io/gh/JSorngard/lambert_w/graph/badge.svg?token=F61FO63ZKW)](https://codecov.io/gh/JSorngard/lambert_w)

Fast and accurate evaluation of the real valued parts of the principal and
secondary branches of the [Lambert W function](https://en.wikipedia.org/wiki/Lambert_W_function)
with the method of Toshio Fukushima \[[1](#references)\].

This method is not iterative, it doesn't allocate memory, and it doesn't use
recursion or loops except for what might already be included in the
implementations of the logarithm and square root functions.

Instead it works by dividing the function’s domain into subdomains. For each one,
it uses a simple transformation of the input inserted into a rational function
to approximate the function's value.

The implementation uses conditional switches on the input value to select the
appropriate subdomain, followed by either a square root (and possibly a division)
or a logarithm. Then it performs a series of multiplications and additions
using constants from a look-up table, and finishes the calculation with a division.

The crate provides two approximations of each branch, one with 50 bits of
accuracy (implemented on 64-bit floats) and one with 24 bits
(implemented on 32- and 64-bit floats). The one with 50 bits of accuracy uses higher
degree polynomials in the rational functions compared to the one with only 24 bits,
and thus more of the multiplications and additions by constants.

This crate can evaluate the approximation with 24 bits of accuracy on
32-bit floats, even though it is defined on 64-bit floats in Fukushima's paper.
This may result in a reduction in the accuracy to less than 24 bits,
but this reduction has not been quantified by the author of this crate.

The crate is `no_std` compatible, but can optionally depend on the standard
library through features for a potential performance gain.

## Examples

Compute the value of the
[omega constant](https://en.wikipedia.org/wiki/Omega_constant) with the
principal branch of the Lambert W function:

```rust
use lambert_w::lambert_w0;

let Ω = lambert_w0(1.0);

assert_abs_diff_eq!(Ω, 0.5671432904097839);
```

Evaluate the secondary branch of the Lambert W function at -ln(2)/2:

```rust
use lambert_w::lambert_wm1;

let mln4 = lambert_wm1(-f64::ln(2.0) / 2.0);

assert_abs_diff_eq!(mln4, -f64::ln(4.0));
```

Do it on 32-bit floats:

```rust
use lambert_w::{lambert_w0f, lambert_wm1f};

let Ω = lambert_w0f(1.0);
let mln4 = lambert_wm1f(-f32::ln(2.0) / 2.0);

assert_abs_diff_eq!(Ω, 0.56714329);
assert_abs_diff_eq!(mln4, -f32::ln(4.0));
```

The implementation can handle extreme inputs just as well:

```rust
use lambert_w::{lambert_w0, lambert_wm1};

let big = lambert_w0(f64::MAX);
let tiny = lambert_wm1(-1e-308);

assert_relative_eq!(big, 703.2270331047702, max_relative = 4e-16);
assert_relative_eq!(tiny, -715.7695669234213, max_relative = 4e-16);
```

Importing the `LambertW` trait lets you call the functions with postfix notation:

```rust
use lambert_w::LambertW;

let z = 2.0 * f64::ln(2.0);

assert_abs_diff_eq!(z.lambert_w0(), f64::ln(2.0));
```

## Features

One of the below features must be enabled:

`libm` *(enabled by default)*: if the `std` feature is disabled,
this feature uses the [`libm`](https://crates.io/crates/libm) crate to compute
square roots and logarithms during function evaluation instead of the standard library.

`std`: use the standard library to compute square roots and logarithms for a
potential performance gain. When this feature is disabled the crate is `no_std` compatible.

## References

\[1\]: Toshio Fukushima.
**Precise and fast computation of Lambert W function by piecewise minimax
rational function approximation with variable transformation**.
DOI: [10.13140/RG.2.2.30264.37128](https://doi.org/10.13140/RG.2.2.30264.37128).
November 2020.

[⬆️ Back to top](#lambert_w).

<br>

### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
