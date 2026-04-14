use std::sync::Arc;

use hashbrown::HashMap;
use kioyu::{Job, ResourceRequest, start_dispatcher_unlimited};
use microsandbox::Sandbox;
use tokio::sync::Mutex;

use crate::cli::CommonArgs;
use crate::config::Config;
use crate::vm::{RunnerPayload, stop_and_remove_sandbox};

pub(super) async fn execute(args: CommonArgs) -> mischief::Result<()> {
    let config = Config::load_config(&args.config)?;
    let sandboxes: Arc<Mutex<HashMap<String, Sandbox>>> = Arc::new(Mutex::new(HashMap::new()));

    let dispatcher = start_dispatcher_unlimited::<RunnerPayload>();

    for (name, runner) in config.runners.into_iter() {
        for i in 0..runner.count {
            let job = Job::new(
                format!("{name}-{i}"),
                RunnerPayload {
                    runner_name: format!("{name}-{i}"),
                    image: runner.image.clone(),
                    cpus: runner.cpus,
                    memory: runner.memory,
                    volumes: runner.volumes.clone(),
                    ports: Default::default(),
                    envs: runner.envs.clone(),
                    secrets: runner.secrets.clone(),
                    owner: config.devop.allowed_users[0].clone(),
                    repo: config.devop.allowed_repositories[0].clone(),
                    token: config.devop.token.clone(),
                    sandboxes: Arc::clone(&sandboxes),
                },
                ResourceRequest::none(), // no resource constraints in unlimited mode
            );
            dispatcher.submit(job).await?;
        }
    }

    // Wait for Ctrl-C
    tokio::signal::ctrl_c().await?;
    clerk::info!("Ctrl-C received, stopping all sandboxes...");

    dispatcher.shutdown().await;

    // Stop and remove all created sandboxes
    let sandboxes = sandboxes.lock().await;
    futures::future::join_all(
        sandboxes
            .iter()
            .map(|(_, sandbox)| stop_and_remove_sandbox(sandbox)),
    )
    .await;

    Ok(())
}
