use std::fmt;
use std::sync::LazyLock;

use owo_colors::{OwoColorize, Style, Styled};
use tracing::{Event, Subscriber};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::format::{FormatEvent, FormatFields};
use tracing_subscriber::fmt::{format, FmtContext};
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::EnvFilter;
// console style
static TRACE_STYLE: LazyLock<Style> = LazyLock::new(|| Style::new().purple());
static DEBUG_STYLE: LazyLock<Style> = LazyLock::new(|| Style::new().blue());
static INFO_STYLE: LazyLock<Style> = LazyLock::new(|| Style::new().green());
static WARN_STYLE: LazyLock<Style> = LazyLock::new(|| Style::new().yellow().bold());
static ERROR_STYLE: LazyLock<Style> = LazyLock::new(|| Style::new().red().bold());

pub fn init_terminal_logger(level: LevelFilter) {
    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(level.into())
                .from_env_lossy(),
        )
        .with_writer(std::io::stderr)
        .event_format(VinayaLogFormatter)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
struct VinayaLogFormatter;

fn color_level(level: &tracing::Level) -> Styled<&&str> {
    match *level {
        tracing::Level::TRACE => "TRACE".style(*TRACE_STYLE),
        tracing::Level::DEBUG => "DEBUG".style(*DEBUG_STYLE),
        tracing::Level::INFO => "INFO".style(*INFO_STYLE),
        tracing::Level::WARN => "WARN".style(*WARN_STYLE),
        tracing::Level::ERROR => "ERROR".style(*ERROR_STYLE),
    }
}

impl<S, N> FormatEvent<S, N> for VinayaLogFormatter
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

    use super::*;
    #[test]
    fn test_log() {
        init_terminal_logger(LevelFilter::TRACE);
        trace!("Trace message");
        debug!("Debug message");
        info!("Informational message");
        warn!("Warning message");
        error!("Error message");
    }
}
