use std::path::PathBuf;

use hashbrown::HashMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};
use validator::{Validate, ValidationError};

use crate::config::IResolve;
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
#[validate(schema(function = "Self::validate_raw_config_runner"))]
pub(super) struct RawConfigRunner {
    #[validate(range(min = 1))]
    cpus: Option<u32>,

    #[validate(range(min = 1))]
    memory: Option<u32>,

    #[schemars(with = "Option<std::collections::HashMap<PathBuf, PathBuf>>")]
    volumes: Option<HashMap<PathBuf, PathBuf>>,

    #[schemars(with = "Option<std::collections::HashMap<u16, u16>>")]
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[validate(custom(function = "validate_ports"))]
    ports: Option<HashMap<u16, u16>>,

    #[schemars(with = "Option<std::collections::HashMap<String, String>>")]
    envs: Option<HashMap<String, String>>,

    #[schemars(with = "Option<std::collections::HashMap<String, (String, String)>>")]
    secrets: Option<HashMap<String, (String, String)>>,

    #[validate(custom(function = "validate_runners"))]
    #[serde(flatten)]
    #[schemars(with = "std::collections::HashMap<String, RawRunnerConfigInner>")]
    pub(super) runners: HashMap<String, RawRunnerConfigInner>,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "lowercase")]
enum RunnerResolveMode {
    Replace,
    #[default]
    Merge,
    Ignore,
}
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
pub(super) struct RawRunnerConfigInner {
    image: String,

    #[serde(default = "default_count")]
    #[validate(range(min = 1))]
    pub(super) count: u8,

    #[serde(default = "default_cpus")]
    #[validate(range(min = 1))]
    cpus: u8,

    #[serde(default = "default_memory")]
    #[validate(range(min = 1))]
    memory: u32,

    #[schemars(with = "Option<std::collections::HashMap<PathBuf, PathBuf>>")]
    volumes: Option<HashMap<PathBuf, PathBuf>>,
    #[serde(default)]
    volumes_mode: RunnerResolveMode,

    #[schemars(with = "Option<std::collections::HashMap<u16, u16>>")]
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[validate(custom(function = "validate_ports"))]
    ports: Option<HashMap<u16, u16>>,
    #[serde(default)]
    ports_mode: RunnerResolveMode,

    #[schemars(with = "Option<std::collections::HashMap<String, String>>")]
    envs: Option<HashMap<String, String>>,
    #[serde(default)]
    envs_mode: RunnerResolveMode,

    #[schemars(with = "Option<std::collections::HashMap<String, (String, String)>>")]
    secrets: Option<HashMap<String, (String, String)>>,
    #[serde(default)]
    secrets_mode: RunnerResolveMode,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigRunner {
    pub name: String,
    pub count: u8,
    pub image: String,
    pub cpus: u8,
    pub memory: u32,
    pub volumes: HashMap<PathBuf, PathBuf>,
    pub ports: HashMap<u16, u16>,
    pub envs: HashMap<String, String>,
    pub secrets: HashMap<String, (String, String)>,
}
const fn default_count() -> u8 { 1 }
const fn default_cpus() -> u8 { 1 }
const fn default_memory() -> u32 { 512 }

impl IResolve<HashMap<String, ConfigRunner>> for RawConfigRunner {
    fn resolve(self) -> HashMap<String, ConfigRunner> {
        let mut result = HashMap::new();

        for (name, runner) in self.runners {
            let volumes = resolve_map(self.volumes.clone(), runner.volumes, runner.volumes_mode);
            let ports = resolve_map(self.ports.clone(), runner.ports, runner.ports_mode);
            let envs = resolve_map(self.envs.clone(), runner.envs, runner.envs_mode);
            let secret = resolve_map(self.secrets.clone(), runner.secrets, runner.secrets_mode);
            let resolved = ConfigRunner {
                name,
                count: runner.count,
                image: runner.image,
                cpus: runner.cpus,
                memory: runner.memory,
                volumes,
                ports,
                envs,
                secrets: secret,
            };

            result.insert(resolved.name.clone(), resolved);
        }

        result
    }
}
impl RawConfigRunner {
    fn validate_raw_config_runner(config: &Self) -> Result<(), ValidationError> {
        for (name, runner) in config.runners.iter() {
            match (
                runner.count > 1,
                &config.ports,
                &runner.ports,
                runner.ports_mode,
            ) {
                (true, _, Some(runner_ports), _) => {
                    if !runner_ports.is_empty() {
                        clerk::error!(
                            "runner '{}' can not share ports with multiple instances",
                            name
                        );
                        return Err(ValidationError::new(
                            "runner can not share ports with multiple instances",
                        ));
                    }
                }
                (
                    true,
                    Some(runner_ports),
                    _,
                    RunnerResolveMode::Merge | RunnerResolveMode::Replace,
                ) => {
                    if !runner_ports.is_empty() {
                        clerk::error!(
                            "runner '{}' can not share global ports with multiple instances",
                            name
                        );
                        return Err(ValidationError::new(
                            "runner can not share global ports with multiple instances",
                        ));
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
impl RawRunnerConfigInner {}
fn resolve_map<K, V>(
    global: Option<HashMap<K, V>>,
    local: Option<HashMap<K, V>>,
    strategy: RunnerResolveMode,
) -> HashMap<K, V>
where
    K: Eq + std::hash::Hash,
{
    match strategy {
        RunnerResolveMode::Replace => local.unwrap_or_default(),
        RunnerResolveMode::Merge => {
            let mut base = global.unwrap_or_default();
            if let Some(local_map) = local {
                base.extend(local_map);
            }
            base
        }
        RunnerResolveMode::Ignore => global.unwrap_or_default(),
    }
}

fn validate_ports(ports: &HashMap<u16, u16>) -> Result<(), ValidationError> {
    for (local, remote) in ports {
        if local == &0 || remote == &0 {
            let mut err = ValidationError::new("invalid_port");
            err.message = Some(format!("invalid port mapping: {}:{}", local, remote).into());
            return Err(err);
        }
    }

    Ok(())
}

fn validate_runners(
    runners: &HashMap<String, RawRunnerConfigInner>,
) -> Result<(), ValidationError> {
    if runners.is_empty() {
        return Err(ValidationError::new("At least one runner is required"));
    }
    for (name, runner) in runners {
        runner.validate().map_err(|e| {
            let mut err = ValidationError::new("invalid_runner");
            err.message = Some(format!("runner '{}': {}", name, e).into());
            err
        })?;
    }
    Ok(())
}
