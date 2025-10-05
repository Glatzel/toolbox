extern crate std;
use std::boxed::Box;
use std::path::PathBuf;

use tracing_core::LevelFilter;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::{EnvFilter, Layer};

use crate::LogLevel;

/// Generate a terminal log layer for tracing.
///
/// # Arguments
///
/// - `level`: The desired log level filter to set.
/// - `color`: Whether to colorize log levels in terminal output.
///
/// # Example
///
/// ```
/// use tracing::{debug, error, info, trace, warn};
/// use tracing_subscriber::layer::SubscriberExt;
/// use tracing_subscriber::util::SubscriberInitExt;
///
/// tracing_subscriber::registry()
///     .with(clerk::layer::terminal_layer(clerk::LogLevel::TRACE, true))
///     .init();
///
/// trace!("Trace message");
/// debug!("Debug message");
/// info!("Informational message");
/// warn!("Warning message");
/// error!("Error message");
/// ```
pub fn terminal_layer<S>(level: LogLevel, color: bool) -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: tracing_core::Subscriber,
    for<'a> S: LookupSpan<'a>,
{
    tracing_subscriber::fmt::layer()
        .event_format(crate::ClerkFormatter { color })
        .with_writer(std::io::stderr)
        .with_filter(
            EnvFilter::builder()
                .with_default_directive(Into::<LevelFilter>::into(level).into())
                .from_env_lossy(),
        )
        .boxed()
}

/// Generate a file log layer for tracing.
///
/// # Arguments
///
/// - `level`: The desired log level filter to set.
/// - `filepath`: The path of the log file.
/// - `overwrite`: Whether to overwrite the log file if it already exists.
///
/// # Example
///
/// ```
/// use std::path::PathBuf;
///
/// use tracing::{debug, error, info, trace, warn};
/// use tracing_subscriber::layer::SubscriberExt;
/// use tracing_subscriber::util::SubscriberInitExt;
///
/// let f = format!(
///     "./temp/{}.log",
///     chrono::Local::now().format("%Y-%m-%d-%H-%M-%S")
/// );
/// let f = PathBuf::from(f);
///
/// tracing_subscriber::registry()
///     .with(clerk::layer::file_layer(clerk::LogLevel::TRACE, f, true))
///     .init();
///
/// trace!("Trace message");
/// debug!("Debug message");
/// info!("Informational message");
/// warn!("Warning message");
/// error!("Error message");
/// ```
pub fn file_layer<S>(
    level: LogLevel,
    filepath: PathBuf,
    overwrite: bool,
) -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: tracing_core::Subscriber,
    for<'a> S: LookupSpan<'a>,
{
    if !filepath.parent().unwrap().exists() {
        std::fs::create_dir_all(filepath.parent().unwrap()).unwrap();
    }
    let a = std::fs::File::options()
        .write(!filepath.exists() || overwrite)
        .append(filepath.exists() && !overwrite)
        .create(true)
        .open(filepath)
        .unwrap();

    tracing_subscriber::fmt::layer()
        .event_format(crate::ClerkFormatter { color: false })
        .with_writer(a)
        .with_filter(
            EnvFilter::builder()
                .with_default_directive(Into::<LevelFilter>::into(level).into())
                .from_env_lossy(),
        )
        .boxed()
}
#[cfg(test)]
mod tests {
    use tracing::{debug, error, info, trace, warn};
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    use super::*;
    #[test]
    fn test_log_file() {
        let f1 = std::path::PathBuf::from("./temp/a.log");
        let f2 = std::path::PathBuf::from("./temp/b.log");
        tracing_subscriber::registry()
            .with(file_layer(LogLevel::TRACE, f1, true))
            .with(file_layer(LogLevel::TRACE, f2, false))
            .init();
        trace!("Trace message");
        debug!("Debug message");
        info!("Informational message");
        warn!("Warning message");
        error!("Error message");
    }
    #[test]
    fn test_log_term() {
        tracing_subscriber::registry()
            .with(terminal_layer(LogLevel::TRACE, true))
            .init();
        trace!("Trace message");
        debug!("Debug message");
        info!("Informational message");
        warn!("Warning message");
        error!("Error message");
    }
}
