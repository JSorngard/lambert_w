// Copyright 2025 Johanna Sörngård
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::env;

fn main() {
    /// The name of the environment variable that can be set to enable the check for panics.
    // If you change this, remember to also change the environment variable
    // that is set in the CI job that checks for panics.
    const ENV_KEY: &str = "LAMBERT_W_ENSURE_NO_PANICS";

    /// The value we look for in the environment variable to enable the check for panics.
    // If you change this, remember to also change the value that the variable is set to
    // in the CI job that checks for panics.
    const ENV_VAL: &str = "1";

    /// The profile needed to ensure no false positives in the check for panics.
    // If you change this, remember to also change the profile that is set in the CI job
    // that checks for panics.
    const NEEDED_PROFILE: &str = "release-lto";

    // Changes to the above constants need to be mentioned in the CHANGELOG.md file.

    // Re-run the build script if the environment variable changes.
    // This makes the state of the check match the state of the environment variable.
    println!("cargo:rerun-if-env-changed={ENV_KEY}");

    // Make cargo aware of the `assert_no_panic` cfg option
    println!("cargo:rustc-check-cfg=cfg(assert_no_panic)");

    let env_val = env::var(ENV_KEY);

    if env_val.as_ref().map(String::as_str) == Ok(ENV_VAL) {
        // Enable the `assert_no_panic` cfg option.
        println!("cargo:rustc-cfg=assert_no_panic");

        let suggestion = format!(
            "The `{NEEDED_PROFILE}` build profile must be enabled to ensure no false positives."
        );

        // In order for `no-panic` to not cause false positives fat LTO needs to be enabled.
        // This crate defines a profile for that.
        // We emit a compilation warning if we can not determine that this profile is enabled.
        match parse_build_profile_name_from_environment() {
            Ok(Some(profile_name)) => {
                if profile_name != NEEDED_PROFILE {
                    println!("cargo:warning=the `{ENV_KEY}` environment variable is set to \"{ENV_VAL}\", but the `{profile_name}` build profile seems to be enabled. {suggestion}");
                }
            }
            Ok(None) => {
                println!("cargo:warning=the `{ENV_KEY}` environment variable is set to \"{ENV_VAL}\", but the build profile could not be determined. {suggestion}");
            }
            Err(e) => {
                println!("cargo:warning=the `{ENV_KEY}` environment variable is set to \"{ENV_VAL}\", but the `OUT_DIR` environment variable could not be read due to: {e}. The build profile could therefore not be determined. {suggestion}");
            }
        }
    } else if let Ok(unexpected_env_val) = env_val {
        println!(
            "cargo:warning=the `{ENV_KEY}` environment variable is set to \"{unexpected_env_val}\", but it must be set to \"{ENV_VAL}\" to enable the check for panics."
        );
    }
}

/// Reads the build profile name from the `OUT_DIR` environment variable.
///
/// If everything works as expected it returns an `Ok(Some(String))`.
///
/// If the environment variable could not be read by the standard library
/// it returns an `Err(VarError)`, and if the profile name could not
/// be determined it returns an `Ok(None)`.
fn parse_build_profile_name_from_environment() -> Result<Option<String>, env::VarError> {
    // Taken from <https://stackoverflow.com/a/73603419/3774277>.

    // The profile name is always the 3rd last part of the path (with 1 based indexing).
    // e.g. /code/core/target/cli/build/my-build-info-9f91ba6f99d7a061/out
    env::var("OUT_DIR").map(|env_var_val| {
        env_var_val
            .split(std::path::MAIN_SEPARATOR)
            .nth_back(3)
            .map(std::borrow::ToOwned::to_owned)
    })
}
