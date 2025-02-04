use std::env::{var, VarError};

fn main() {
    const ENV_KEY: &str = "LAMBERT_W_ENSURE_NO_PANIC";

    // Tell cargo to re-run the build script if any file changes.
    // This line could be removed if we decide that the check for panics
    // should only be done in CI.
    println!("cargo:rerun-if-changed=NULL");

    // Tell cargo to only re-run the build script if the build script itself changes.
    //println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rustc-check-cfg=cfg(assert_no_panic)");

    // Get the value of the environment variable at ENV_VAR and map it to lowercase.
    let env_val = var(ENV_KEY).map(|val| val.to_ascii_lowercase());

    // If the environment variable at ENV_KEY is set to "true" we use the `no-panic` crate to attempt to verify that the crate can not panic.
    // This requires the `release-lto` profile to be enabled, otherwise it will result in false positives.
    if env_val == Ok(String::from("true")) {
        match build_profile_name() {
            Ok(Some(profile_name)) => {
                if profile_name == "release-lto" {
                    println!("cargo:rustc-cfg=assert_no_panic");
                } else {
                    panic!("The `{ENV_KEY}` environment variable is set, but the release-lto profile is not enabled. It must be enabled to correctly check for panics.");
                }
            }
            Ok(None) => {
                panic!("The `{ENV_KEY}` environment variable is set, but the build profile name could not be determined. It must be \"release-lto\" to correctly check for panics.");
            }
            Err(e) => {
                panic!("The `{ENV_KEY}` environment variable is set, but the OUT_DIR environment variable could not be read: {e}");
            }
        }
    } else {
        panic!("The `{ENV_KEY}` environment variable is set to {env_val:?}, but it must be set to \"true\" to check for panics.");
    }
}

/// Reads the build profile name from the OUT_DIR environment variable.
/// Returns `None` if the OUT_DIR environment variable is not set or if the profile name could not be determined.
fn build_profile_name() -> Result<Option<String>, VarError> {
    // The profile name is always the 3rd last part of the path (with 1 based indexing).
    // e.g. /code/core/target/cli/build/my-build-info-9f91ba6f99d7a061/out
    var("OUT_DIR").map(|env_var_val| {
        env_var_val
            .split(std::path::MAIN_SEPARATOR)
            .nth_back(3)
            .map(|s| s.to_string())
    })
}
