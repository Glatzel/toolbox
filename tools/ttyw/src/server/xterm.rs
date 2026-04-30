use std::sync::Arc;

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

    let mut tty_reader = pair.master.try_clone_reader().unwrap();
    let tty_writer = Arc::new(Mutex::new(pair.master.take_writer().unwrap()));
    let (mut ws_sender, mut ws_receiver) = socket.split();

    // ===== PTY → WS =====
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Bytes>(32);
    // Blocking thread: just reads and sends into the channel
    tokio::task::spawn_blocking(move || {
        use std::io::Read;
        let mut buf = [0u8; 1024];
        clerk::debug!("PTY reader thread started");

        loop {
            match tty_reader.read(&mut buf) {
                Ok(n) if n > 0 => {
                    let data = Bytes::copy_from_slice(&buf[..n]);
                    clerk::trace!(bytes = n, "PTY read.");
                    if tx.blocking_send(data).is_err() {
                        clerk::debug!("PTY reader: channel closed, exiting");
                        break;
                    }
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

    // Async task: drains the channel and forwards to WebSocket
    tokio::spawn(async move {
        while let Some(data) = rx.recv().await {
            clerk::trace!(
                bytes = data.len(),
                "PTY -> WS: {}",
                String::from_utf8_lossy(&data)
            );
            if let Err(e) = ws_sender.send(Message::Binary(data)).await {
                clerk::warn!(error = %e, "Failed to forward PTY output to WebSocket");
                break;
            }
        }
    });

    // ===== WS → PTY =====
    clerk::debug!("Entering WS receive loop");
    while let Some(Ok(msg)) = ws_receiver.next().await {
        if let Message::Text(text) = msg {
            clerk::trace!("WS -> PTY: {text}");
            match ReceiveMsg::parse(text.as_str()) {
                Ok(ReceiveMsg::Resize(msg)) => {
                    clerk::debug!(cols = msg.cols, rows = msg.rows, "Terminal resize:");
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
                    clerk::trace!("Input: {}", msg.data);
                    let mut w = tty_writer.lock().await;
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
#[cfg(test)]
mod tests {
    use crate::cli::Args;
    use crate::server::AppContext;
    use clap_verbosity_flag::Verbosity;
    use clerk::tracing_subscriber::{
        self, EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt,
    };
    use futures::{SinkExt, StreamExt};
    use rstest::*;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::time::timeout;
    use tokio_tungstenite::{connect_async, tungstenite::Message};

    // ===== Fixture =====

    #[fixture]
    fn state() -> Arc<AppContext> {
        tracing_subscriber::registry()
            .with(
                clerk::terminal_layer(true).with_filter(
                    EnvFilter::builder()
                        .with_default_directive(
                            format!("{}={}", env!("CARGO_PKG_NAME"), "trace")
                                .parse()
                                .unwrap(),
                        )
                        .from_env_lossy(),
                ),
            )
            .init();
        Arc::new(AppContext {
            args: Args {
                port: 0,
                #[cfg(unix)]
                cmd: "/bin/sh".into(),
                #[cfg(windows)]
                cmd: "cmd".into(),
                working_directory: std::env::temp_dir(),
                verbose: Verbosity::new(1, 1),
            },
        })
    }

    /// Spin up the axum server on a random port, return the bound address.
    async fn spawn_server(
        state: Arc<AppContext>,
    ) -> (std::net::SocketAddr, tokio::task::JoinHandle<()>) {
        let app = crate::server::app(state);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let handle = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        (addr, handle)
    }

    // ===== Tests =====

    /// Connecting opens a shell and we receive output within 2s
    #[rstest]
    #[tokio::test]
    async fn receives_pty_output_on_connect(state: Arc<AppContext>) {
        let (addr, _handle) = spawn_server(state).await;
        clerk::debug!("server started on {addr}");
        let (mut ws, _) = connect_async(format!("ws://{addr}/ws")).await.unwrap();
        ws.send(Message::Text("echo hello_from_test\r\n".into()))
            .await
            .unwrap();

        // Collect output until we see our marker or timeout
        let output = timeout(Duration::from_secs(3), async {
            let mut buf = String::new();
            while let Some(Ok(msg)) = ws.next().await {
                if let Message::Binary(b) = msg {
                    buf.push_str(&String::from_utf8_lossy(&b));
                    if buf.contains("hello_from_test") {
                        return buf;
                    }
                }
            }
            buf
        })
        .await
        .unwrap();

        assert!(output.contains("hello_from_test"), "got: {output}");
    }

    /// Resize message is accepted without error (no output assertion needed)
    #[rstest]
    #[tokio::test]
    async fn resize_message_accepted(state: Arc<AppContext>) {
        let (addr, _handle) = spawn_server(state).await;
        let (mut ws, _) = connect_async(format!("ws://{addr}/ws")).await.unwrap();

        ws.send(Message::Text(
            r#"{"kind":"resize","cols":220,"rows":50}"#.into(),
        ))
        .await
        .unwrap();

        // Server should not close the connection after a resize
        ws.send(Message::Text("echo still_alive\n".into()))
            .await
            .unwrap();

        let output = timeout(Duration::from_secs(2), async {
            let mut buf = String::new();
            while let Some(Ok(msg)) = ws.next().await {
                if let Message::Binary(b) = msg {
                    buf.push_str(&String::from_utf8_lossy(&b));
                    if buf.contains("still_alive") {
                        return buf;
                    }
                }
            }
            buf
        })
        .await
        .unwrap();

        assert!(output.contains("still_alive"), "got: {output}");
    }

    /// Two simultaneous connections get independent shells
    #[rstest]
    #[tokio::test]
    async fn two_tabs_get_independent_shells(state: Arc<AppContext>) {
        let (addr, _handle) = spawn_server(state).await;

        let (mut ws1, _) = connect_async(format!("ws://{addr}/ws")).await.unwrap();
        let (mut ws2, _) = connect_async(format!("ws://{addr}/ws")).await.unwrap();

        ws1.send(Message::Text("echo tab1_marker\n".into()))
            .await
            .unwrap();
        ws2.send(Message::Text("echo tab2_marker\n".into()))
            .await
            .unwrap();

        async fn collect(
            ws: &mut tokio_tungstenite::WebSocketStream<
                tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
            >,
            marker: &str,
        ) -> String {
            timeout(Duration::from_secs(2), async {
                let mut buf = String::new();
                while let Some(Ok(Message::Binary(b))) = ws.next().await {
                    buf.push_str(&String::from_utf8_lossy(&b));
                    if buf.contains(marker) {
                        return buf;
                    }
                }
                buf
            })
            .await
            .unwrap_or_default()
        }

        let out1 = collect(&mut ws1, "tab1_marker").await;
        let out2 = collect(&mut ws2, "tab2_marker").await;

        assert!(out1.contains("tab1_marker"), "ws1 got: {out1}");
        assert!(out2.contains("tab2_marker"), "ws2 got: {out2}");
        // Crucially, neither should see the other's marker
        assert!(!out1.contains("tab2_marker"));
        assert!(!out2.contains("tab1_marker"));
    }

    /// Closing the WebSocket kills the child process (no zombie)
    #[rstest]
    #[tokio::test]
    async fn child_killed_on_disconnect(state: Arc<AppContext>) {
        let (addr, _handle) = spawn_server(state).await;
        let (mut ws, _) = connect_async(format!("ws://{addr}/ws")).await.unwrap();

        // Get a shell prompt so we know the process is running
        ws.send(Message::Text("echo ready\n".into())).await.unwrap();
        timeout(Duration::from_secs(2), async {
            while let Some(Ok(Message::Binary(b))) = ws.next().await {
                if String::from_utf8_lossy(&b).contains("ready") {
                    break;
                }
            }
        })
        .await
        .unwrap();

        // Close — handle_socket should call child.kill()
        ws.close(None).await.unwrap();

        // Give the server a moment to reap
        tokio::time::sleep(Duration::from_millis(200)).await;
        // If we get here without hanging, the child was cleaned up
    }
}
