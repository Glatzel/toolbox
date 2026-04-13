use std::sync::Arc;

use kioyu::start_dispatcher;

use crate::cli::CommonArgs;
use crate::config::Config;
use crate::server::{AppContext, JobSpec, start_server};

pub(super) async fn execute(args: CommonArgs) -> mischief::Result<()> {
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
