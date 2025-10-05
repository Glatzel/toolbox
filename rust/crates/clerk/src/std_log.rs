#[cfg(feature = "log")]
mod formatter;
#[cfg(feature = "log")]
pub mod layer;
#[cfg(feature = "log")]
pub use fast_setup::init_log_with_level;
#[cfg(feature = "log")]
pub use formatter::ClerkFormatter;
#[cfg(feature = "log")]
pub use tracing;
#[cfg(feature = "log")]
pub use tracing::level_filters::LevelFilter;
#[cfg(feature = "log")]
mod fast_setup;
#[cfg(feature = "log")]
mod log_level;
#[cfg(feature = "log")]
pub use log_level::LogLevel;
