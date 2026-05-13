extern crate std;

use std::path::Path;

use tracing_subscriber::Layer;
use tracing_subscriber::registry::LookupSpan;

use crate::log_tracing::error::ClerkError;

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
/// use clerk::LevelFilter;
/// use tracing::{debug, error, info, trace, warn};
/// use tracing_subscriber::layer::SubscriberExt;
/// use tracing_subscriber::util::SubscriberInitExt;
/// use tracing_subscriber::{EnvFilter, Layer};
/// tracing_subscriber::registry()
///     .with(clerk::terminal_layer(true).with_filter(LevelFilter::TRACE))
///     .init();
///
/// trace!("Trace message");
/// debug!("Debug message");
/// info!("Informational message");
/// warn!("Warning message");
/// error!("Error message");
/// ```
pub fn terminal_layer<S>(color: bool) -> impl Layer<S> + Send + Sync
where
    S: tracing_core::Subscriber,
    for<'a> S: LookupSpan<'a>,
{
    tracing_subscriber::fmt::layer()
        .event_format(crate::ClerkFormatter { color })
        .with_writer(std::io::stderr)
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
/// use clerk::LevelFilter;
/// use tracing::{debug, error, info, trace, warn};
/// use tracing_subscriber::layer::SubscriberExt;
/// use tracing_subscriber::util::SubscriberInitExt;
/// use tracing_subscriber::{EnvFilter, Layer};
///
/// let f = format!(
///     "./temp/{}.log",
///     chrono::Local::now().format("%Y-%m-%d-%H-%M-%S")
/// );
/// let f = PathBuf::from(f);
///
/// tracing_subscriber::registry()
///     .with(clerk::file_layer(f, true).with_filter(LevelFilter::TRACE))
///     .init();
///
/// trace!("Trace message");
/// debug!("Debug message");
/// info!("Informational message");
/// warn!("Warning message");
/// error!("Error message");
/// ```
pub fn file_layer<F, S>(
    filepath: F,
    overwrite: bool,
) -> Result<impl Layer<S> + Send + Sync, ClerkError>
where
    F: AsRef<Path>,
    S: tracing_core::Subscriber,
    for<'a> S: LookupSpan<'a>,
{
    let filepath = filepath.as_ref();
    if !filepath
        .parent()
        .ok_or_else(|| ClerkError::ParentDirectoryNotFound(filepath.to_owned()))?
        .exists()
    {
        std::fs::create_dir_all(
            filepath
                .parent()
                .ok_or_else(|| ClerkError::ParentDirectoryNotFound(filepath.to_owned()))?,
        )?;
    }
    let file = std::fs::File::options()
        .write(true)
        .truncate(overwrite)
        .append(!overwrite)
        .create(true)
        .open(filepath)?;

    let layer = tracing_subscriber::fmt::layer()
        .event_format(crate::ClerkFormatter { color: false })
        .with_writer(file);
    Ok(layer)
}
#[cfg(test)]
mod tests {
    use tracing::{debug, error, info, trace, warn};
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    use super::*;

    #[test]
    fn test_log_file() -> mischief::Result<()> {
        let f1 = std::path::PathBuf::from("./temp/a.log");
        let f2 = std::path::PathBuf::from("./temp/b.log");
        tracing_subscriber::registry()
            .with(file_layer(f1, true)?.with_filter(crate::LevelFilter::TRACE))
            .with(file_layer(f2, false)?.with_filter(crate::LevelFilter::TRACE))
            .init();
        trace!("Trace message");
        debug!("Debug message");
        info!("Informational message");
        warn!("Warning message");
        error!("Error message");
        Ok(())
    }
    #[test]
    fn test_log_term() {
        tracing_subscriber::registry()
            .with(terminal_layer(true).with_filter(crate::LevelFilter::TRACE))
            .init();
        trace!("Trace message");
        debug!("Debug message");
        info!("Informational message");
        warn!("Warning message");
        error!("Error message");
    }
}
