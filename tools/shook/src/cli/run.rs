use std::path::PathBuf;
use std::sync::Arc;

use async_trait::async_trait;
use hashbrown::HashMap;
use kioyu::{IPayload, Job, ResourceRequest, start_dispatcher_unlimited};
use microsandbox::Sandbox;
use tokio::sync::Mutex;

use crate::cli::CommonArgs;
use crate::config::Config;
use crate::vm::{build_sandbox, drain_sandbox_handle, start_runner, stop_and_remove_sandbox};

// Payload that encapsulates one runner's full lifecycle.
struct RunnerPayload {
    runner_name: String,
    image: String,
    cpus: u8,
    memory: u32,
    volumes: HashMap<PathBuf, PathBuf>,
    envs: HashMap<String, String>,
    secrets: HashMap<String, (String, String)>,
    owner: String,
    repo: String,
    token: String,
    sandboxes: Arc<Mutex<Vec<Sandbox>>>,
}
#[async_trait]
impl IPayload for RunnerPayload {
    type Error = mischief::Report;
    async fn execute(&self) -> mischief::Result<()> {
        let sandbox = build_sandbox(
            &self.runner_name,
            &self.image,
            self.cpus,
            self.memory,
            &self.volumes,
            &HashMap::new(),
            &self.envs,
            &self.secrets,
        )
        .await?;
        let handle = start_runner(&sandbox, &self.owner, &self.repo, &self.token).await?;
        self.sandboxes.lock().await.push(sandbox);
        drain_sandbox_handle(handle).await;
        Ok(())
    }
}

pub(super) async fn execute(args: CommonArgs) -> mischief::Result<()> {
    let config = Config::load_config(&args.config)?;
    let sandboxes: Arc<Mutex<Vec<Sandbox>>> = Arc::new(Mutex::new(Vec::new()));

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
            .map(|sandbox| stop_and_remove_sandbox(sandbox)),
    )
    .await;

    Ok(())
}
