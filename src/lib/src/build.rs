use Config;

use std::error::Error;
use std::fmt::{Display, Error as FormatError, Formatter};

/// Config of the `build` subcommand.
#[derive(Debug)]
pub struct BuildConfig<'a> {
    pub bin: Option<&'a str>,
    pub features: Vec<&'a str>,
}

#[derive(Debug)]
pub enum BuildError {
    InvalidCargoToml { path: String },
    InvalidAndroidToml { path: String },

}

impl Error for BuildError {
    fn description(&self) -> &str {
        match self {
            BuildError::InvalidCargoToml { .. } => { "Could not find Cargo.toml" },
            BuildError::InvalidAndroidToml { .. } => { "Could not find Android.toml" },
        }
    }
}

/// Builds the project and the APK.
pub fn execute_build(config: &Config, build_config: &BuildConfig) -> Result<(), BuildError> {
    Ok(())
}
