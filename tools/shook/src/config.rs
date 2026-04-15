mod devop;
mod kioyu;
mod runner;
mod server;

use std::collections::HashMap;
use std::path::Path;

pub use devop::*;
use devop::{ConfigDevOp, RawConfigDevOp};
use kioyu::{ConfigKioyu, RawConfigKioyu, default_config_kioyu};
pub use runner::ConfigRunner;
use runner::RawConfigRunner;
use schemars::{JsonSchema, Schema, schema_for};
use serde::{Deserialize, Serialize};
use server::{ConfigServer, RawConfigServer, default_config_server};
use validator::Validate;
pub trait IResolve<T> {
    fn resolve(self) -> T;
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
#[serde(deny_unknown_fields)]
#[validate(schema(function = "Self::validate_config"))]
struct RawConfig {
    #[serde(default = "default_config_server")]
    #[validate(nested)]
    pub server: RawConfigServer,

    #[validate(nested)]
    pub devop: RawConfigDevOp,

    #[serde(default = "default_config_kioyu")]
    #[validate(nested)]
    pub kioyu: RawConfigKioyu,

    #[validate(nested)]
    pub runners: RawConfigRunner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ConfigServer,
    pub devop: ConfigDevOp,
    pub kioyu: ConfigKioyu,
    pub runners: HashMap<String, ConfigRunner>,
}
impl IResolve<Config> for RawConfig {
    fn resolve(self) -> Config {
        let mut config = Config {
            server: self.server.resolve(),
            devop: self.devop.resolve(),
            kioyu: self.kioyu.resolve(),
            runners: self.runners.resolve(),
        };
        for (name, runner) in config.runners.iter() {
            if config.kioyu.memory <= runner.memory {
                clerk::info!(
                    "Runner {} has memory {} which is less than or equal to Kioyu memory {}, updating Kioyu memory to {}.",
                    name,
                    runner.memory,
                    config.kioyu.memory,
                    runner.memory,
                );
                config.kioyu.memory = runner.memory;
            }
        }
        config
    }
}
impl Config {
    pub fn load_config(path: &Path) -> mischief::Result<Self> {
        clerk::debug!(path = %path.display(), "Loading config file");

        let content = std::fs::read_to_string(path).inspect_err(|e| {
            clerk::error!(path = %path.display(), error = %e, "Failed to read config file");
        })?;

        let raw_config: RawConfig = toml::from_str(&content).inspect_err(|e| {
            clerk::error!(path = %path.display(), error = %e, "Failed to parse config TOML");
        })?;
        clerk::debug!(?raw_config);
        raw_config.validate()?;

        let config = raw_config.resolve();

        clerk::info!(
            path = %path.display(),
            ?config,
            "Config loaded successfully"
        );

        Ok(config)
    }
}

impl RawConfig {
    fn validate_config(config: &Self) -> Result<(), validator::ValidationError> {
        if config
            .runners
            .runners
            .iter()
            .any(|(_, runner)| runner.count > 1)
        {
            if config.devop.allowed_users.len() > 1 {
                const MSG: &str = "Only one user is allowed when runner count > 1.";
                clerk::error!(MSG);
                return Err(validator::ValidationError::new(MSG));
            }
            if config.devop.allowed_repositories.len() > 1 {
                const MSG: &str = "Only one repository is allowed when runner count > 1.";
                clerk::error!(MSG);
                return Err(validator::ValidationError::new(MSG));
            }
        }
        Ok(())
    }
}

pub fn schema() -> Schema { schema_for!(RawConfig) }

#[cfg(test)]
mod tests {

    use mischief::IDiagnostic;
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_schema() {
        clerk::init_log_with_level(clerk::LevelFilter::TRACE);
        let schema = schema();
        insta::assert_json_snapshot!(schema);
    }
    #[rstest]
    #[case("custom_kioyu")]
    #[case("custom_server")]
    #[case("full_runners_config")]
    #[case("minimal")]
    #[case("runners_with_default")]
    #[case("runners_with_default_and_mode")]
    fn test_valid_config(#[case] config_name: &str) -> mischief::Result<()> {
        clerk::init_log_with_level(clerk::LevelFilter::TRACE);
        use std::path::PathBuf;
        let config = Config::load_config(&PathBuf::from(format!(
            "{}/test_data/config/valid/{}.toml",
            env!("CARGO_MANIFEST_DIR"),
            config_name
        )))?;
        let mut config = serde_json::to_value(config)?;
        config.sort_all_objects();
        insta::assert_snapshot!(
            format!("test_valid_config-{}", config_name),
            serde_json::to_string_pretty(&config).unwrap()
        );
        Ok(())
    }
    #[rstest]
    #[case("not_exist")]
    #[case("null")]
    #[case("repos_count")]
    #[case("runner_port")]
    #[case("server_port")]
    #[case("share_port")]
    #[case("unknown_field")]
    #[case("share_global_port")]
    #[case("users_counts")]
    #[case("zero_runner")]
    fn test_invalid_config(#[case] config_name: &str) -> mischief::Result<()> {
        clerk::init_log_with_level(clerk::LevelFilter::TRACE);
        use std::path::PathBuf;
        let err = Config::load_config(&PathBuf::from(format!(
            "{}/test_data/config/invalid/{}.toml",
            env!("CARGO_MANIFEST_DIR"),
            config_name
        )))
        .unwrap_err()
        .inner;
        println!("{}", err.description());
        insta::with_settings!({filters => vec![
            (r"\n│\n", "\n")
        ]}, {
            insta::assert_snapshot!(format!("test_invalid_config-{}", config_name), err.description());
        });
        Ok(())
    }
}
