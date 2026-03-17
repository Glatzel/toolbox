use std::sync::OnceLock;
extern crate std;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::LogLevel;

static INIT_LOGGING: OnceLock<()> = OnceLock::new();

/// Initialize global logging with the given [`LogLevel`].
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
/// use clerk::{init_log_with_level, LogLevel};
///
/// // Initialize global logging (only the first call has an effect).
/// init_log_with_level(LogLevel::INFO);
///
/// // Now tracing events are logged:
/// info!(target: "example", "Hello from tracing!");
/// ```
pub fn init_log_with_level(level: LogLevel) {
    INIT_LOGGING.get_or_init(|| {
        tracing_subscriber::registry()
            .with(crate::layer::terminal_layer(level, true))
            .init();
    });
}
