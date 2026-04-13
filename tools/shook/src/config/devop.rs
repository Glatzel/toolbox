use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::{OneOrMany, serde_as};
use validator::Validate;

use crate::config::IResolve;
pub(super) type RawConfigDevOp = ConfigDevOp;
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
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

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
pub struct ConfigDevOp {
    pub vendor: Vendor,

    pub token: String,

    pub webhook_secret: String,

    #[serde_as(as = "OneOrMany<_>")]
    #[validate(length(min = 1))]
    pub allowed_repositories: Vec<String>,

    #[serde_as(as = "OneOrMany<_>")]
    #[validate(length(min = 1))]
    pub allowed_users: Vec<String>,
}
impl IResolve<ConfigDevOp> for RawConfigDevOp {
    fn resolve(self) -> ConfigDevOp { self }
}
