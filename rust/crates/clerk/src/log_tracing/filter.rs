use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::LevelFilter;
pub fn level_filter<L: Into<LevelFilter>>(level: L) -> EnvFilter {
    let level_filter = level.into();
    EnvFilter::builder()
        .with_default_directive(level_filter.into())
        .from_env_lossy()
}
