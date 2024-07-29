## 0.3.0

### Breaking changes

 - Removed the `fast` and `accurate` modules and export the functions directly.
 - Add s_* prefix to the 24 bit versions.

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

 - Lambert W functions now return an `Option` instead of a `Result` with custom error types.

### Other changes

 - The `fast` module is behind the (enabled by default) feature flag `24bits`.
 - The `accurate` module is behind the (enabled by default) feature flag `50bits`.
