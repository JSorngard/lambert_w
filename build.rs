use std::env::{var, VarError};

fn main() {
    // If you change this, remember to also change the environment variable
    // that is set in the CI configuration.
    const ENV_KEY: &str = "LAMBERT_W_ENSURE_NO_PANICS";

    // Tell cargo to re-run the build script if any file changes.
    // Usefull if you're debuggig the build script itself.
    //println!("cargo:rerun-if-changed=NULL");

    // Tell cargo to only re-run the build script if the build script itself changes.
    // Useful for releasing the crate or when not debugging the build script.
    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rustc-check-cfg=cfg(assert_no_panic)");

    // Get the value of the environment variable at ENV_VAR and map it to lowercase.
    let env_val = var(ENV_KEY).map(|val| val.to_ascii_lowercase());

    // If the environment variable at ENV_KEY is set to "true" we use the `no-panic` crate to attempt to verify that the crate can not panic.
    // This requires the `release-lto` profile to be enabled, otherwise it will result in false positives.
    if env_val == Ok(String::from("true")) {
        match profile_name() {
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
    }
}

/// Reads the build profile name from the OUT_DIR environment variable.
///
/// If the environment variable could not be read it returns a `VarError`,
/// and if the profile name could not be determined it returns an `Ok(None)`.
fn profile_name() -> Result<Option<String>, VarError> {
    // The profile name is always the 3rd last part of the path (with 1 based indexing).
    // e.g. /code/core/target/cli/build/my-build-info-9f91ba6f99d7a061/out
    var("OUT_DIR").map(|env_var_val| {
        env_var_val
            .split(std::path::MAIN_SEPARATOR)
            .nth_back(3)
            .map(|s| s.to_string())
    })
}
