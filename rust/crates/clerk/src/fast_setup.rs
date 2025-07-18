#[cfg(feature = "log")]
use std::sync::OnceLock;

#[cfg(feature = "log")]
use tracing_subscriber::layer::SubscriberExt;
#[cfg(feature = "log")]
use tracing_subscriber::util::SubscriberInitExt;

#[cfg(feature = "log")]
use crate::LogLevel;
#[cfg(feature = "log")]
static INIT_LOGGING: OnceLock<()> = OnceLock::new();
#[cfg(feature = "log")]
pub fn init_log_with_level(level: LogLevel) {
    INIT_LOGGING.get_or_init(|| {
        tracing_subscriber::registry()
            .with(crate::terminal_layer(level, true))
            .init();
    });
}
#[cfg(not(feature = "log"))]
pub fn init_log_with_level<T>(_level: T) {}
