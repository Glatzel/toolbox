use std::path::PathBuf;

use tracing_core::LevelFilter;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::{EnvFilter, Layer};
/// Generate a file log layer for tracing.
///
/// # Arguments
///
/// - `level`: The desired log level filter to set.
/// - `filepath`: The Path of log file.
/// - `overwrite`: whether to Overwrite log file if it is existed.
///
/// # Example
///
/// ```
/// use tracing::{debug, error, info, trace, warn};
/// use tracing_subscriber::layer::SubscriberExt;
/// use tracing_subscriber::util::SubscriberInitExt;
/// use tracing_subscriber::filter::LevelFilter;
/// use tracing_subscriber::EnvFilter;
/// let f = format!(
///            "./temp/{}.log",
///            chrono::Local::now().format("%Y-%m-%d-%H-%M-%S")
///        );
/// let f = std::path::PathBuf::from(f);
/// tracing_subscriber::registry()
///     .with(clerk::file_layer( LevelFilter::TRACE, f, true))
///     .init();
/// trace!("Trace message");
/// debug!("Debug message");
/// info!("Informational message");
/// warn!("Warning message");
/// error!("Error message");
/// ```
pub fn file_layer<S>(
    level: LevelFilter,
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
                .with_default_directive(level.into())
                .from_env_lossy(),
        )
        .boxed()
}

#[cfg(test)]
mod tests {
    use tracing::{debug, error, info, trace, warn};
    use tracing_core::LevelFilter;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    use super::*;
    #[test]
    fn test_log() {
        let f1 = std::path::PathBuf::from("./temp/a.log");
        let f2 = std::path::PathBuf::from("./temp/b.log");
        tracing_subscriber::registry()
            .with(file_layer(LevelFilter::TRACE, f1, true))
            .with(file_layer(LevelFilter::TRACE, f2, false))
            .init();
        trace!("Trace message");
        debug!("Debug message");
        info!("Informational message");
        warn!("Warning message");
        error!("Error message");
    }
}
