use std::sync::OnceLock;

use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

static INIT_LOGGING: OnceLock<()> = OnceLock::new();

pub fn init_log_with_level(level: LevelFilter) {
    INIT_LOGGING.get_or_init(|| {
        tracing_subscriber::registry()
            .with(crate::terminal_layer(level, true))
            .init();
    });
}
