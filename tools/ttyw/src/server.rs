use std::sync::{Arc, Mutex};
mod message;
use axum::{
    Router,
    body::Bytes,
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    routing::get,
};
use futures::{SinkExt, StreamExt};
use message::*;
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};

pub struct AppContext {
    pub args: crate::cli::Args,
}

fn app(shared_state: Arc<AppContext>) -> Router {
    Router::new()
        .route("/ws", get(ws_handler))
        .with_state(shared_state)
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
async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppContext>>,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}
async fn handle_socket(socket: WebSocket, state: Arc<AppContext>) {
    // ===== PTY SETUP =====
    let pty_system = NativePtySystem::default();
    let pair = pty_system.openpty(PtySize::default()).unwrap();

    let mut cmd = CommandBuilder::new(&state.args.shell);
    cmd.cwd(&state.args.working_directory);
    let mut child = pair.slave.spawn_command(cmd).unwrap();

    let mut reader = pair.master.try_clone_reader().unwrap();
    let writer = Arc::new(Mutex::new(pair.master.take_writer().unwrap()));
    let (mut sender, mut receiver) = socket.split();

    // ===== PTY → WS =====
    tokio::task::spawn_blocking(move || {
        use std::io::Read;
        let mut buf = [0u8; 1024];
        loop {
            match reader.read(&mut buf) {
                Ok(n) if n > 0 => {
                    // cannot .await here → use blocking_send pattern
                    let data = buf[..n].to_vec();
                    clerk::trace!("PTY -> WS: {}", String::from_utf8_lossy(&data));
                    // use a channel OR block_on
                    futures::executor::block_on(async {
                        let _ = sender.send(Message::Binary(Bytes::from(data))).await;
                    });
                }
                _ => break,
            }
        }
    });

    // ===== WS → PTY =====
    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            clerk::trace!("WS → PTY: {text}");
            match ReceiveMsg::parse(text.as_str()) {
                Ok(ReceiveMsg::Resize(msg)) => {
                    clerk::trace!("Resize: cols={} rows={}", msg.cols, msg.rows);
                    let _ = pair.master.resize(PtySize {
                        rows: msg.rows,
                        cols: msg.cols,
                        pixel_width: 0,
                        pixel_height: 0,
                    });
                }
                Ok(ReceiveMsg::Input(msg)) => {
                    let mut w = writer.lock().unwrap();
                    let _ = w.write_all(msg.data.as_bytes());
                }
                Err(e) => {
                    clerk::warn!("{e}")
                }
            }
        }
    }

    let _ = child.kill();
    clerk::debug!("Children killed.")
}
