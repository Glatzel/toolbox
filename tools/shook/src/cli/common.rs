use std::path::PathBuf;

use clap::Args;
#[derive(Debug, Args)]
pub(super) struct CommonArgs {
    #[arg(default_value_os_t=PathBuf::from("shook.toml"))]
    pub config: PathBuf,
}
impl Default for CommonArgs {
    fn default() -> Self {
        Self {
            config: PathBuf::from("shook.toml"),
        }
    }
}
