use tracing_core::LevelFilter;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::{EnvFilter, Layer};

use crate::LogLevel;
/// Generate a terminal log layer for tracing.
///
/// # Arguments
///
/// - `level`: The desired log level filter to set.
///
/// # Example
///
/// ```
/// use tracing::{debug, error, info, trace, warn};
/// use tracing_subscriber::EnvFilter;
/// use tracing_subscriber::filter::LevelFilter;
/// use tracing_subscriber::layer::SubscriberExt;
/// use tracing_subscriber::util::SubscriberInitExt;
/// tracing_subscriber::registry()
///     .with(clerk::terminal_layer(LevelFilter::TRACE, true))
///     .init();
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

#[cfg(test)]
mod tests {
    use tracing::{debug, error, info, trace, warn};
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    use super::*;
    #[test]
    fn test_log() {
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
