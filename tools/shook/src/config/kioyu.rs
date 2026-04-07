use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::config::IResolve;

pub(super) type RawConfigKioyu = ConfigKioyu;
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Validate)]
#[serde(default = "default_config_kioyu")]
pub struct ConfigKioyu {
    pub memory_mb: usize,
}
fn default_config_kioyu() -> ConfigKioyu { ConfigKioyu { memory_mb: 1024 } }
impl IResolve<ConfigKioyu> for RawConfigKioyu {
    fn resolve(self) -> ConfigKioyu { self }
}
