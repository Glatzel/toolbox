use std::sync::Arc;

use microsandbox::Sandbox;
use tokio::sync::Mutex;

use crate::cli::CommonArgs;
use crate::config::Config;

pub(super) async fn execute(args: CommonArgs) -> mischief::Result<()> {
    let config = Config::load_config(&args.config)?;
    let sandboxes: Arc<Mutex<Vec<Sandbox>>> = Arc::new(Mutex::new(Vec::new()));

    let mut tasks = Vec::new();

    for (name, runner) in config.runners.into_iter() {
        for i in 0..runner.count {
            let name = name.clone();
            let image = runner.image.clone();
            let volumes = runner.volumes.clone();
            let envs = runner.envs.clone();
            let secrets = runner.secrets.clone();
            let user = config.devop.allowed_users[0].clone();
            let repo = config.devop.allowed_repositories[0].clone();
            let token = config.devop.token.clone();
            let sandboxes = Arc::clone(&sandboxes);

            let task = tokio::task::spawn(async move {
                let mut builder = Sandbox::builder(format!("{name}-{i}"))
                    .image(image.as_ref())
                    .cpus(runner.cpus)
                    .memory(runner.memory)
                    .replace()
                    .entrypoint(["./start-runner.sh", &user, &repo, &token]);
                for (host, guest) in volumes.iter() {
                    builder = builder.volume(guest.to_string_lossy().as_ref(), |m| m.bind(host));
                }
                for (key, value) in envs.iter() {
                    builder = builder.env(key, value);
                }
                for (key, (value, url)) in secrets.iter() {
                    builder = builder.secret(|s| s.env(key).value(value).allow_host(url));
                }
                clerk::debug!("Sandbox builder configured: {name}");
                let sandbox = builder.create().await.unwrap();
                clerk::debug!("Sandbox created: {name}-{i}");

                sandboxes.lock().await.push(sandbox);
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
        let name = sandbox.name().to_string();
        shutdown_tasks.push(async move {
            if let Err(e) = sandbox.stop_and_wait().await {
                clerk::error!("Failed to stop sandbox {name}: {e}");
            }
            if let Err(e) = Sandbox::remove(&name).await {
                clerk::error!("Failed to remove sandbox {name}: {e}");
            }
            clerk::debug!("Sandbox removed: {name}");
        });
    }
    futures::future::join_all(shutdown_tasks).await;

    Ok(())
}
