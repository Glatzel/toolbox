use std ::fmt;
use std::sync::LazyLock;

use owo_colors::{OwoColorize, Styled};
use tracing::{Event, Subscriber};
use tracing_subscriber::fmt::format::{FormatEvent, FormatFields};
use tracing_subscriber::fmt::{FmtContext, format};
use tracing_subscriber::registry::LookupSpan;
pub(crate) struct ClerkFormatter {
    pub(crate) color: bool,
}

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





impl<S, N> FormatEvent<S, N> for ClerkFormatter
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
            if self.color {
                color_level(event.metadata().level()).to_string()
            } else {
                event.metadata().level().to_string()
            },
            event.metadata().target(),
            event.metadata().file().unwrap_or("<file>"),
            event.metadata().line().unwrap_or(0),
        )?;

        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}
