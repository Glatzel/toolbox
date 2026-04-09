use std::path::PathBuf;
use std::sync::Arc;

use clap::Args;
use kioyu::start_dispatcher;

use crate::config::Config;
use crate::server::{AppContext, JobSpec, start_server};
#[derive(Debug, Args)]
pub(super) struct RunArgs {
    #[arg(default_value_os_t)]
    pub config: PathBuf,
}

pub(super) async fn execute(args: RunArgs) -> mischief::Result<()> {
    let config = Config::load_config(&args.config)?;

    // init kioyu
    let mut pool = kioyu::ResourcePool::new();
    pool.register("memory", config.kioyu.memory as usize)?;
    let kioyu_handle = start_dispatcher::<JobSpec>(pool);

    // init server
    let shared_state = Arc::new(AppContext {
        config,
        kioyu_handle,
    });
    start_server(shared_state).await
}
impl Default for RunArgs {
    fn default() -> Self {
        Self {
            config: PathBuf::from("shook.toml"),
        }
    }
}
