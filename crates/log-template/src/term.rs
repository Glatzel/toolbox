use owo_colors::{OwoColorize, Styled};
use std::fmt;
use tracing::{Event, Subscriber};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::format::{FormatEvent, FormatFields};
use tracing_subscriber::fmt::{format, FmtContext};
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Layer;

pub fn terminal_layer<S>(level: LevelFilter) -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: tracing_core::Subscriber,
    for<'a> S: LookupSpan<'a>,
{
    let layer = tracing_subscriber::fmt::layer()
        .event_format(TerminalFormatter)
        .with_writer(std::io::stderr)
        .with_filter(
            EnvFilter::builder()
                .with_default_directive(level.into())
                .from_env_lossy(),
        );

    Box::new(layer)
}

struct TerminalFormatter;

fn color_level(level: &tracing::Level) -> Styled<&&str> {
    match *level {
        tracing::Level::TRACE => "TRACE".style(*crate::TRACE_STYLE),
        tracing::Level::DEBUG => "DEBUG".style(*crate::DEBUG_STYLE),
        tracing::Level::INFO => "INFO".style(*crate::INFO_STYLE),
        tracing::Level::WARN => "WARN".style(*crate::WARN_STYLE),
        tracing::Level::ERROR => "ERROR".style(*crate::ERROR_STYLE),
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
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
