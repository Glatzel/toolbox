use std::sync::OnceLock;

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::LogLevel;

static INIT_LOGGING: OnceLock<()> = OnceLock::new();

pub fn init_log_with_level(level: LogLevel) {
    INIT_LOGGING.get_or_init(|| {
        tracing_subscriber::registry()
            .with(crate::layer::terminal_layer(level, true))
            .init();
    });
}
