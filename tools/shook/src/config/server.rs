use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::config::IResolve;
pub(super) type RawConfigServer = ConfigServer;
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
#[serde(default = "default_config_server")]
pub struct ConfigServer {
    #[validate(range(min = 1, max = 65535))]
    pub port: u16,
}
pub(super) fn default_config_server() -> ConfigServer { ConfigServer { port: 8787 } }
impl IResolve<ConfigServer> for RawConfigServer {
    fn resolve(self) -> ConfigServer { self }
}
