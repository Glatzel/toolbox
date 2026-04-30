use crate::server::{AppContext, message::ReceiveMsg};
use axum::{
    body::Bytes,
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
};
use futures::{SinkExt, StreamExt};
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn xterm_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppContext>>,
) -> impl axum::response::IntoResponse {
    clerk::debug!("WebSocket upgrade request received");
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppContext>) {
    let shell = state.args.cmd.clone();
    let cwd = state.args.working_directory.display();
    clerk::info!(shell = %shell, cwd = %cwd, "WebSocket connected, spawning PTY");

    // ===== PTY SETUP =====
    let pty_system = NativePtySystem::default();
    let pair = pty_system.openpty(PtySize::default()).unwrap();

    let mut cmd = CommandBuilder::new(&state.args.cmd);
    cmd.cwd(&state.args.working_directory);

    let mut child = match pair.slave.spawn_command(cmd) {
        Ok(child) => {
            clerk::debug!(shell = %shell, cwd = %cwd, "Shell process spawned");
            child
        }
        Err(e) => {
            clerk::error!(shell = %shell, error = %e, "Failed to spawn shell process");
            return;
        }
    };

    let mut reader = pair.master.try_clone_reader().unwrap();
    let writer = Arc::new(Mutex::new(pair.master.take_writer().unwrap()));
    let (mut sender, mut receiver) = socket.split();

    // ===== PTY → WS =====
    tokio::task::spawn_blocking(move || {
        use std::io::Read;
        let mut buf = [0u8; 1024];
        clerk::debug!("PTY reader thread started");
        loop {
            match reader.read(&mut buf) {
                Ok(n) if n > 0 => {
                    let data = buf[..n].to_vec();
                    clerk::trace!(bytes = n, "PTY -> WS: {}", String::from_utf8_lossy(&data));
                    futures::executor::block_on(async {
                        if let Err(e) = sender.send(Message::Binary(Bytes::from(data))).await {
                            clerk::warn!(error = %e, "Failed to forward PTY output to WebSocket");
                        }
                    });
                }
                Ok(_) => {
                    clerk::debug!("PTY reader reached EOF");
                    break;
                }
                Err(e) => {
                    clerk::debug!(error = %e, "PTY reader error, closing");
                    break;
                }
            }
        }
        clerk::debug!("PTY reader thread exited");
    });

    // ===== WS → PTY =====
    clerk::debug!("Entering WS receive loop");
    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            clerk::trace!("WS -> PTY: {text}");
            match ReceiveMsg::parse(text.as_str()) {
                Ok(ReceiveMsg::Resize(msg)) => {
                    clerk::debug!(cols = msg.cols, rows = msg.rows, "Terminal resize");
                    if let Err(e) = pair.master.resize(PtySize {
                        rows: msg.rows,
                        cols: msg.cols,
                        pixel_width: 0,
                        pixel_height: 0,
                    }) {
                        clerk::warn!(error = %e, "Failed to resize PTY");
                    }
                }
                Ok(ReceiveMsg::Input(msg)) => {
                    clerk::trace!(bytes = msg.data.len(), "WS -> PTY input");
                    let mut w = writer.lock().await;
                    if let Err(e) = w.write_all(msg.data.as_bytes()) {
                        clerk::warn!(error = %e, "Failed to write input to PTY");
                    }
                }
                Err(e) => {
                    clerk::warn!(error = %e, "Failed to parse WebSocket message");
                }
            }
        }
    }

    clerk::info!("WebSocket disconnected, killing shell process");
    if let Err(e) = child.kill() {
        clerk::warn!(error = %e, "Failed to kill shell process");
    } else {
        clerk::debug!("Shell process killed");
    }
}
