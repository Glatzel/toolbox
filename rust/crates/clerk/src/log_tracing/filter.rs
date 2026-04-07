use tracing::Level;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::LevelFilter;

pub fn level_filter<L: Into<Level>>(level: L) -> EnvFilter {
    let level_filter = LevelFilter::from_level(level.into());
    EnvFilter::builder()
        .with_default_directive(level_filter.into())
        .from_env_lossy()
}
