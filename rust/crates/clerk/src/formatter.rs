use std::fmt;

use owo_colors::OwoColorize;
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::fmt::format::{FormatEvent, FormatFields};
use tracing_subscriber::fmt::{FmtContext, format};
use tracing_subscriber::registry::LookupSpan;

pub struct ClerkFormatter {
    pub(crate) color: bool,
}

impl ClerkFormatter {
    fn color_level(&self, level: &tracing::Level) -> String {
        if !self.color {
            return format!("{}", level);
        }

        match level {
            &Level::TRACE => "TRACE".purple().to_string(),
            &Level::DEBUG => "DEBUG".blue().to_string(),
            &Level::INFO => "INFO".green().to_string(),
            &Level::WARN => "WARN".yellow().bold().to_string(),
            &Level::ERROR => "ERROR".red().bold().to_string(),
        }
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
            "[{}] [",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
        )?;

        write!(writer, "{}]", self.color_level(event.metadata().level()))?;

        #[cfg(debug_assertions)]
        write!(
            writer,
            "[{}] [{}:{}] ",
            event.metadata().target(),
            event.metadata().file().unwrap_or("<file>"),
            event.metadata().line().unwrap_or(0),
        )?;

        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}
