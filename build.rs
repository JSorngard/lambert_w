use std::env::{var, VarError};

fn main() {
    // If you change this, remember to also change the environment variable
    // that is set in the CI configuration.
    const ENV_KEY: &str = "LAMBERT_W_ENSURE_NO_PANICS";

    // Re-run the build script if the lock file changes.
    println!("cargo:rerun-if-changed=Cargo.lock");
    // Or if the environment variable changes.
    println!("cargo:rerun-if-env-changed={ENV_KEY}");

    // Make cargo aware of the `assert_no_panic` cfg option
    println!("cargo:rustc-check-cfg=cfg(assert_no_panic)");

    // Get the value of the environment variable at `ENV_KEY` and map it to lowercase.
    let env_val = var(ENV_KEY).map(|mut val| {
        val.make_ascii_lowercase();
        val
    });

    // If the environment variable at `ENV_KEY` is set to "true" we use the `no-panic` crate to attempt to verify that the crate can not panic.
    // This requires the `release-lto` profile to be enabled, otherwise it will result in false positives.
    if env_val == Ok(String::from("true")) {
        // Enable the `assert_no_panic` cfg option.
        println!("cargo:rustc-cfg=assert_no_panic");

        match parse_profile_name_from_environment() {
            Ok(Some(profile_name)) => {
                if profile_name != "release-lto" {
                    panic!("cargo:warning=the `{ENV_KEY}` environment variable is set to \"true\", but a profile that could result in false positives seems to be enabled. False positives can be removed by enabling the \"release-lto\" profile.");
                }
            }
            Ok(None) => {
                panic!("cargo:warning=the `{ENV_KEY}` environment variable is set to \"true\", but the build profile name could not be determined. The \"release-lto\" profile must be enabled to ensure no false positives.");
            }
            Err(e) => {
                panic!("cargo:warning=the `{ENV_KEY}` environment variable is set to \"true\", but the `OUT_DIR` environment variable could not be read due to: {e}\n The profile could therefore not be determined. The \"release-lto\" profile must be enabled to ensure no false positives.");
            }
        }
    }
}

/// Reads the build profile name from the OUT_DIR environment variable.
///
/// If the environment variable could not be read it returns a `VarError`,
/// and if the profile name could not be determined it returns an `Ok(None)`.
fn parse_profile_name_from_environment() -> Result<Option<String>, VarError> {
    // The profile name is always the 3rd last part of the path (with 1 based indexing).
    // e.g. /code/core/target/cli/build/my-build-info-9f91ba6f99d7a061/out
    var("OUT_DIR").map(|env_var_val| {
        env_var_val
            .split(std::path::MAIN_SEPARATOR)
            .nth_back(3)
            .map(|s| s.to_string())
    })
}
