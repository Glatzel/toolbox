use std::sync::Arc;

use axum::Router;
use axum::routing::get;
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

    clerk::debug!("Registered route: GET /ws");

    #[cfg(not(debug_assertions))]
    let router = {
        clerk::debug!("Registered fallback: embedded frontend static handler");
        router.fallback(frontend::static_handler)
    };

    #[cfg(debug_assertions)]
    clerk::debug!("Skipping embedded frontend (debug build)");

    router
}
pub async fn start_server(shared_state: Arc<AppContext>) -> mischief::Result<()> {
    let app = app(shared_state.clone());
    let port = shared_state.args.port;
    let addr = format!("0.0.0.0:{}", port);
    let url = format!("http://localhost:{}", port);

    clerk::info!(address = %addr, "Binding listener");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .inspect_err(|e| {
            clerk::error!(address = %addr, error = %e, "Failed to bind TCP listener");
        })?;

    #[cfg(debug_assertions)]
    clerk::info!("Frontend: disabled in debug mode (serving from disk via vite)");
    #[cfg(not(debug_assertions))]
    clerk::info!("Frontend: embedded assets enabled");
    clerk::info!("Server started → {url}");
    axum::serve(listener, app).await.inspect_err(|e| {
        clerk::error!(error = %e, "Server exited with error");
    })?;
    Ok(())
}
