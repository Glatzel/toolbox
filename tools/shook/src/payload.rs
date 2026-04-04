use axum::{http::HeaderMap, response::Response};
use serde::{Deserialize, Serialize};
use strum::{EnumString, VariantNames};
use validator::{Validate, ValidationError};

use crate::config::Config;
pub mod github;
pub trait IRunnerSpec {
    fn runner_spec(
     
        headers: &HeaderMap,
        body: &str,
        config: &Config,
    ) -> Result<RunnerSpec, Response>;
}
#[derive(Debug, Validate)]
pub struct RunnerSpec {
    pub _image: String,
    #[validate(custom(function = "validate_os"))]
    pub os: Os,
    #[validate(custom(function = "validate_arch"))]
    pub arch: Arch,
    pub _cpu_mhz: usize,
    pub _memory_mb: usize,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumString, VariantNames)]
pub enum Os {
    #[serde(rename = "windows", alias = "win")]
    Windows,
    #[serde(rename = "macos", alias = "mac", alias = "osx")]
    MacOs,
    #[serde(rename = "linux")]
    Linux,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumString, VariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum Arch {
    #[serde(rename = "x64", alias = "x86_64", alias = "amd64")]
    X64,
    #[serde(rename = "arm64", alias = "aarch64")]
    Arm64,
}
fn validate_os(os: &Os) -> Result<(), ValidationError> {
    match (os, std::env::consts::OS) {
        (Os::Linux, "linux") => Ok(()),
        (_, "linux") => Err(ValidationError::new("Host OS is Linux, required OS is: ")),
        (Os::Windows, "windows") => Ok(()),
        (_, "windows") => Err(ValidationError::new("Host OS is Windows, required OS is: ")),
        (Os::MacOs, "macos") => Ok(()),
        (_, "macos") => Err(ValidationError::new("Host OS is MacOS, required OS is: ")),
        _ => Err(ValidationError::new("unsupported os")),
    }
}
fn validate_arch(arch: &Arch) -> Result<(), ValidationError> {
    match (arch, std::env::consts::ARCH) {
        (Arch::X64, "x86_64") => Ok(()),
        (_, "x86_64") => Err(ValidationError::new(
            "Host arch is x86_64, required arch is: ",
        )),
        (Arch::Arm64, "aarch64") => Ok(()),
        (_, "aarch64") => Err(ValidationError::new(
            "Host arch is aarch64, required arch is: ",
        )),
        _ => Err(ValidationError::new("unsupported arch")),
    }
}
