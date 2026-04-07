use std::path::PathBuf;
use std::sync::Arc;

use clap::Args;
use kioyu::dispatcher::start_dispatcher;

use crate::config::Config;
use crate::server::{AppContext, JobSpec, start_server};
#[derive(Debug, Args)]
pub struct RunArgs {
    pub config: Option<PathBuf>,
}

pub(super) async fn execute(args: RunArgs) -> mischief::Result<()> {
    // init config
    let config_path = args.config.unwrap_or_else(|| PathBuf::from("shook.toml"));
    let config = Config::load_config(&config_path)?;

    // init kioyu
    let mut pool = kioyu::resource::ResourcePool::new();
    pool.register("memory", config.kioyu.memory as usize)?;
    let kioyu_handle = start_dispatcher::<JobSpec>(pool);

    // init server
    let shared_state = Arc::new(AppContext {
        config,
        kioyu_handle,
    });
    start_server(shared_state).await
}
