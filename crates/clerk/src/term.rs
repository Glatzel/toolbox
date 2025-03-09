use std::fmt;
use std::sync::LazyLock;

use owo_colors::{OwoColorize, Styled};
use tracing::{Event, Subscriber};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::format::{FormatEvent, FormatFields};
use tracing_subscriber::fmt::{format, FmtContext};
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::{EnvFilter, Layer};
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
/// use tracing_subscriber::layer::SubscriberExt;
/// use tracing_subscriber::util::SubscriberInitExt;
/// use tracing_subscriber::filter::LevelFilter;
/// tracing_subscriber::registry()
///     .with(log_template::terminal_layer(LevelFilter::TRACE))
///     .init();
/// trace!("Trace message");
/// debug!("Debug message");
/// info!("Informational message");
/// warn!("Warning message");
/// error!("Error message");
/// ```
pub fn terminal_layer<S>(level: LevelFilter) -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: tracing_core::Subscriber,
    for<'a> S: LookupSpan<'a>,
{
    tracing_subscriber::fmt::layer()
        .event_format(TerminalFormatter)
        .with_writer(std::io::stderr)
        .with_filter(
            EnvFilter::builder()
                .with_default_directive(level.into())
                .from_env_lossy(),
        )
        .boxed()
}

struct TerminalFormatter;
impl TerminalFormatter {}
static TRACE_TEXT: LazyLock<Styled<&&str>> = LazyLock::new(|| "TRACE".style(*crate::TRACE_STYLE));
static DEBUG_TEXT: LazyLock<Styled<&&str>> = LazyLock::new(|| "DEBUG".style(*crate::DEBUG_STYLE));
static INFO_TEXT: LazyLock<Styled<&&str>> = LazyLock::new(|| "INFO".style(*crate::INFO_STYLE));
static WARN_TEXT: LazyLock<Styled<&&str>> = LazyLock::new(|| "WARN".style(*crate::WARN_STYLE));
static ERROR_TEXT: LazyLock<Styled<&&str>> = LazyLock::new(|| "ERROR".style(*crate::ERROR_STYLE));
fn color_level(level: &tracing::Level) -> &Styled<&&str> {
    match *level {
        tracing::Level::TRACE => &TRACE_TEXT,
        tracing::Level::DEBUG => &DEBUG_TEXT,
        tracing::Level::INFO => &INFO_TEXT,
        tracing::Level::WARN => &WARN_TEXT,
        tracing::Level::ERROR => &ERROR_TEXT,
    }
}

impl<S, N> FormatEvent<S, N> for TerminalFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        write!(
            writer,
            "[{}] [{:}] [{}] [{}:{}] ",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"), // Custom timestamp format
            color_level(event.metadata().level()),
            event.metadata().target(),
            event.metadata().file().unwrap_or("<file>"),
            event.metadata().line().unwrap_or(0),
        )?;

        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
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
            .with(terminal_layer(LevelFilter::TRACE))
            .init();
        trace!("Trace message");
        debug!("Debug message");
        info!("Informational message");
        warn!("Warning message");
        error!("Error message");
    }
}
