use std::path::PathBuf;
use std::sync::Arc;

use axum::Router;
use axum::routing::post;
use clap::Args;

use crate::config::Config;
use crate::webhook::{AppContext, webhook};

#[derive(Debug, Args)]
pub struct RunArgs {
    pub config: Option<PathBuf>,
}
fn app(shared_state: Arc<AppContext>) -> Router {
    Router::new()
        .route("/webhook", post(webhook))
        .with_state(shared_state)
}
pub(super) async fn execute(args: RunArgs) -> mischief::Result<()> {
    let config_path = args.config.unwrap_or_else(|| PathBuf::from("shook.toml"));
    let config = Config::load_config(&config_path)?;
    let port = config.server.port;
    let shared_state = Arc::new(AppContext { config });

    clerk::debug!("Building router");
    let app = app(shared_state.clone());

    let addr = format!("0.0.0.0:{port}");
    clerk::info!(address = %addr, "Binding listener");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .inspect_err(|e| {
            clerk::error!(address = %addr, error = %e, "Failed to bind TCP listener");
        })?;

    clerk::info!(address = %addr, "Server started, waiting for connections");
    axum::serve(listener, app).await.inspect_err(|e| {
        clerk::error!(error = %e, "Server exited with error");
    })?;
    Ok(())
}
