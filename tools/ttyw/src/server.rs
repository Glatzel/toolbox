use std::sync::Arc;

use axum::{Router, routing::get};
mod xterm;

#[cfg(not(debug_assertions))]
mod frontend;
mod message;

pub struct AppContext {
    pub args: crate::cli::Args,
}

fn app(shared_state: Arc<AppContext>) -> Router {
    let router = Router::new()
        .route("/ws", get(xterm::xterm_handler))
        .with_state(shared_state);
    #[cfg(not(debug_assertions))]
    let router = router.fallback(frontend::static_handler);
    router
}
pub async fn start_server(shared_state: Arc<AppContext>) -> mischief::Result<()> {
    let app = app(shared_state.clone());
    let addr = format!("0.0.0.0:{}", shared_state.args.port);
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
