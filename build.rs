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

    if let Ok(ENV_VAL) = env_val.as_ref().map(String::as_str) {
        // Enable the `assert_no_panic` cfg option.
        println!("cargo:rustc-cfg=assert_no_panic");

        let suggestion =
            format!("The `{NEEDED_PROFILE}` profile must be enabled to ensure no false positives.");

        // In order for `no-panic` to not cause false positives fat LTO needs to be enabled.
        // This crate defines a profile for that.
        // We emit a compilation warning if we can not determine that this profile is enabled.
        match parse_build_profile_name_from_environment()
            .as_ref()
            .map(|s| s.as_ref().map(String::as_str))
        {
            Ok(Some(NEEDED_PROFILE)) => (),
            Ok(Some(profile_name)) => {
                println!("cargo:warning=the `{ENV_KEY}` environment variable is set to {ENV_VAL}, but the profile seems to be set to `{profile_name}`. This could result in false positives. {suggestion}");
            }
            Ok(None) => {
                println!("cargo:warning=the `{ENV_KEY}` environment variable is set to {ENV_VAL}, but the build profile name could not be determined. {suggestion}");
            }
            Err(e) => {
                println!("cargo:warning=the `{ENV_KEY}` environment variable is set to {ENV_VAL}, but the `OUT_DIR` environment variable could not be read due to: {e}. The profile could therefore not be determined. {suggestion}");
            }
        }
    } else if let Ok(unexpected_env_val) = env_val {
        // The environment variable is set, but it is not set to the expected value.
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
            .map(|s| s.to_string())
    })
}
