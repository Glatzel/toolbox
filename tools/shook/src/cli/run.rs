use kioyu::{Job, ResourceRequest, start_dispatcher_unlimited};

use crate::cli::CommonArgs;
use crate::config::Config;
use crate::vm::RunnerPayload;

pub(super) async fn execute(args: CommonArgs) -> mischief::Result<()> {
    let config = Config::load_config(&args.config)?;

    let dispatcher = start_dispatcher_unlimited::<RunnerPayload>();

    for (name, runner) in config.runners.into_iter() {
        for i in 0..runner.count {
            let job = Job::new(
                format!("{name}-{i}"),
                RunnerPayload::new(
                    format!("{name}-{i}"),
                    runner.image.clone(),
                    runner.cpus,
                    runner.memory,
                    runner.volumes.clone(),
                    Default::default(),
                    runner.envs.clone(),
                    runner.secrets.clone(),
                    config.devop.allowed_users[0].clone(),
                    config.devop.allowed_repositories[0].clone(),
                    config.devop.token.clone(),
                ),
                ResourceRequest::none(), // no resource constraints in unlimited mode
                config.kioyu.max_retries,
            );
            dispatcher.submit(job).await?;
        }
    }
    tokio::signal::ctrl_c().await?;
    clerk::info!("Ctrl-C received, stopping all sandboxes...");
    dispatcher.shutdown().await;
    clerk::info!("All sandboxes stopped.");
    Ok(())
}
