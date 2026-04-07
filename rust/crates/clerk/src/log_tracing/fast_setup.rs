use std::sync::OnceLock;
extern crate std;

use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};
static INIT_LOGGING: OnceLock<()> = OnceLock::new();

/// Initialize global logging with the given [`Level`].
///
/// This function:
/// - Installs a [`tracing_subscriber`] registry with your custom terminal
///   layer.
/// - Runs only once per process; further calls are no-ops.
/// - Is safe to call from multiple threads concurrently.
///
/// # Example
///
/// ```
/// use tracing::info;
/// use clerk::{init_log_with_level, Level};
///
/// // Initialize global logging (only the first call has an effect).
/// init_log_with_level(LevelFilter::INFO);
///
/// // Now tracing events are logged:
/// info!(target: "example", "Hello from tracing!");
/// ```
pub fn init_log_with_level(level: LevelFilter) {
    INIT_LOGGING.get_or_init(|| {
        tracing_subscriber::registry()
            .with(
                crate::terminal_layer(true).with_filter(
                    EnvFilter::builder()
                        .with_default_directive(level.into())
                        .from_env_lossy(),
                ),
            )
            .init();
    });
}
