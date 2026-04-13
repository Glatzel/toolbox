use std::sync::Arc;

use microsandbox::Sandbox;
use tokio::sync::Mutex;

use crate::cli::CommonArgs;
use crate::config::Config;
use crate::vm::{build_sandbox, drain_sandbox_handle, start_runner, stop_and_remove_sandbox};

pub(super) async fn execute(args: CommonArgs) -> mischief::Result<()> {
    let config = Config::load_config(&args.config)?;
    let sandboxes: Arc<Mutex<Vec<Sandbox>>> = Arc::new(Mutex::new(Vec::new()));

    let mut tasks = Vec::new();

    for (name, runner) in config.runners.into_iter() {
        for i in 0..runner.count {
            let runner_name = format!("{name}-{i}");
            let image = runner.image.clone();
            let volumes = runner.volumes.clone();
            let envs = runner.envs.clone();
            let secrets = runner.secrets.clone();
            let owner = config.devop.allowed_users[0].clone();
            let repo = config.devop.allowed_repositories[0].clone();
            let token = config.devop.token.clone();

            let sandboxes = Arc::clone(&sandboxes);

            let task = tokio::task::spawn(async move {
                let sandbox = build_sandbox(
                    &runner_name,
                    &image,
                    runner.cpus,
                    runner.memory,
                    &volumes,
                    &envs,
                    &secrets,
                )
                .await
                .unwrap();

                let handle = start_runner(&sandbox, &owner, &repo, &token).await.unwrap();
                sandboxes.lock().await.push(sandbox);
                drain_sandbox_handle(handle).await;
            });

            tasks.push(task);
        }
    }

    // Wait for Ctrl-C
    tokio::signal::ctrl_c().await?;
    clerk::info!("Ctrl-C received, stopping all sandboxes...");

    // Abort all spawn tasks still running
    for task in tasks {
        task.abort();
    }

    // Stop and remove all created sandboxes
    let sandboxes = sandboxes.lock().await;
    let mut shutdown_tasks = Vec::new();
    for sandbox in sandboxes.iter() {
        shutdown_tasks.push(async move {
            stop_and_remove_sandbox(sandbox).await;
        });
    }

    futures::future::join_all(shutdown_tasks).await;

    Ok(())
}
