//! cargo-apk
//!

mod build;

pub use build::{BuildConfig, execute_build};

/// Config of the main cargo-apk command.
#[derive(Debug)]
pub struct Config {
    /// Use verbose output
    pub verbose: bool,
}
