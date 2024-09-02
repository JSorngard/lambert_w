# lambert_w

[![Static Badge](https://img.shields.io/badge/github-JSorngard%2Flambert__w-8da0cb?logo=github)](https://github.com/JSorngard/lambert_w)
[![Crates.io Version](https://img.shields.io/crates/v/lambert_w?logo=rust)](https://crates.io/crates/lambert_w)
[![docs.rs](https://img.shields.io/docsrs/lambert_w?logo=docs.rs)](https://docs.rs/lambert_w/latest/lambert_w/)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/JSorngard/lambert_w/rust.yml?logo=github&label=CI)](https://github.com/JSorngard/lambert_w/actions/workflows/rust.yml)

Fast and accurate evaluation of the real valued parts of the principal and
secondary branches of the [Lambert W function](https://en.wikipedia.org/wiki/Lambert_W_function)
with the [method of Toshio Fukushima][1].

This method works by splitting the domain of the function into subdomains,
and on each subdomain it uses a rational function
evaluated on a simple transformation of the input to describe the function.  
It is implemented in code as conditional switches on the input value followed by
either a square root (and possibly a division) or a logarithm and then a series
of multiplications and additions by fixed constants and finished with a division.

The crate provides two approximations of each branch, one with 50 bits of
accuracy and one with 24 bits. The one with 50 bits of accuracy uses higher
degree polynomials in the rational functions compared to the one with only 24 bits,
and thus more of the multiplications and additions by constants.

While the approximation with 24 bits of accuracy is defined on 64 bit floats in
the paper, this crate can also evaluate it on 32 bit floats for a slight
reduction in accuracy. This reduction in accuracy has not been quantified by the
author.

The crate is `no_std`, but can optionally depend on the standard
library through features for a potential performance gain.

## Examples

Compute the value of the
[Omega constant](https://en.wikipedia.org/wiki/Omega_constant) with the
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

## Features

`50bits` *(enabled by default)*: enables the function versions
with 50 bits of accuracy on 64 bit floats.

`24bits` *(enabled by default)*: enables the function versions
with 24 bits of accuracy on 64 bit floats,
as well as the implementation on 32 bit floats.

You can disable one of the above features to potentially save a little bit
of binary size.

`estrin`: uses [Estrin's scheme](https://en.wikipedia.org/wiki/Estrin's_scheme)
to evaluate the polynomials in the rational functions.
While this results in more assembly instructions, they are mostly independent
of each other, and this increases instruction level parallelism on modern
hardware for a total performance gain. May result in slight numerical instability,
which can be reduced if the target CPU has fused multiply-add instructions.

One of the below features must be enabled:

`std`: use the standard library to compute square roots and logarithms
for a potential performance gain. When this feature is disabled the crate is `no_std`.

`libm` *(enabled by default)*: if the `std` feature is disabled, this feature
uses the [`libm`](https://docs.rs/libm/latest/libm/)
crate to compute square roots and logarithms instead of the standard library.

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[1]: https://www.researchgate.net/publication/346309410_Precise_and_fast_computation_of_Lambert_W_function_by_piecewise_minimax_rational_function_approximation_with_variable_transformation
