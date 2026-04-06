use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::config::IResolve;
pub(super) type RawConfigDevOp = ConfigDevOp;
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Vendor {
    #[serde(rename = "bitbucket", alias = "atlassian")]
    Bitbucket,
    Forgejo,
    Gitea,
    Github,
    Gitlab,
    Woodpecker,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ConfigDevOp {
    pub vendor: Vendor,
    pub webhook_secret: String,
    #[validate(length(min = 1))]
    pub allowed_repositories: Vec<String>,
    #[validate(length(min = 1))]
    pub allowed_users: Vec<String>,
}
impl IResolve<ConfigDevOp> for RawConfigDevOp {
    fn resolve(self) -> ConfigDevOp { self }
}
