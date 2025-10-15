use std::fmt;
extern crate std;
use std::format;
use std::string::{String, ToString};

use owo_colors::OwoColorize;
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::fmt::format::{FormatEvent, FormatFields};
use tracing_subscriber::fmt::{FmtContext, format};
use tracing_subscriber::registry::LookupSpan;

/// A custom [`tracing`] event formatter for use with
/// [`tracing_subscriber`]'s `fmt` layer.
///
/// `ClerkFormatter` controls how log events are rendered:
/// - Timestamps are included in `[YYYY-MM-DD HH:MM:SS.sss]` format.
/// - Log levels are displayed, optionally colorized.
/// - In debug builds (`#[cfg(debug_assertions)]`), the target, file, and line
///   are also shown.
/// - Event fields are printed using the configured field formatter.
///
/// # Colorization
///
/// If `color` is `true`, log levels are highlighted using [`owo_colors`]:
/// - `TRACE` → purple
/// - `DEBUG` → blue
/// - `INFO` → green
/// - `WARN` → bold yellow
/// - `ERROR` → bold red
///
/// If `color` is `false`, plain text is used.
pub struct ClerkFormatter {
    /// Whether to enable colored output for log levels.
    pub(crate) color: bool,
}

impl ClerkFormatter {
    /// Format a log [`Level`] into a string, applying color if enabled.
    fn color_level(&self, level: tracing::Level) -> String {
        if !self.color {
            return format!("{}", level);
        }

        match level {
            Level::TRACE => "TRACE".purple().to_string(),
            Level::DEBUG => "DEBUG".blue().to_string(),
            Level::INFO => "INFO".green().to_string(),
            Level::WARN => "WARN".yellow().bold().to_string(),
            Level::ERROR => "ERROR".red().bold().to_string(),
        }
    }
}

impl<S, N> FormatEvent<S, N> for ClerkFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    /// Formats a single [`Event`] for output.
    ///
    /// The format is roughly:
    /// ```text
    /// [timestamp] [LEVEL] [target] [file:line] field1=value field2=value
    /// ```
    ///
    /// - The `[target] [file:line]` portion is included only in debug builds.
    /// - Timestamps use the local timezone.
    /// - Event fields are formatted via the active [`FormatFields`].
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

        write!(writer, "{}]", self.color_level(*event.metadata().level()))?;

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
