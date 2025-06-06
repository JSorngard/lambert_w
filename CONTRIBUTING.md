# Contributing

I welcome contributions from anyone in the form of suggestions, bug reports,
pull requests, and feedback.

The only requirements on code style is to run `cargo fmt` on the code.
Do not worry if you forget, as this will be caught in CI.
You may also wish to run the test suite with all valid feature combinations
before submission as well, as it's faster to run those locally than to wait for CI.

Any added code paths are expected to have tests that cover them. Exceptions can
be made through discussion on the PR, e.g. testing the exact output of a `Display`
implementation for an error type may not be necessary.

If you add a feature then all possible feature combinations are should be tested
in CI.

Unsafe code will not be accepted.

Also note that any code contribution will be dual licensed under both MIT and
Apache-2.0 as stated in the readme.
