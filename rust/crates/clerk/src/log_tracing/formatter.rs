use std::io;

use owo_colors::OwoColorize;
use tracing::{Event, Level};
use tracing_core::Subscriber;
use tracing_subscriber::fmt::FmtContext;
use tracing_subscriber::fmt::format::{FormatEvent, FormatFields, Writer};
use tracing_subscriber::registry::LookupSpan;
const TIME_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.6fZ";
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
    pub color: bool,
}

impl ClerkFormatter {
    /// Format a log [`Level`] into a string, applying color if enabled.
    fn color_level(&self, level: Level) -> String {
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
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        #[cfg(not(debug_assertions))]
        write!(
            writer,
            "[{}][{}][{}]",
            chrono::Local::now().format(TIME_FORMAT),
            self.color_level(*event.metadata().level()),
            event.metadata().target(),
        )?;

        #[cfg(debug_assertions)]
        write!(
            writer,
            "[{}][{}][{}][{}:{}] ",
            chrono::Local::now().format(TIME_FORMAT),
            self.color_level(*event.metadata().level()),
            event.metadata().target(),
            event.metadata().file().unwrap_or("<file>"),
            event.metadata().line().unwrap_or(0),
        )?;

        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}

pub trait FormatEventToWriter {
    fn format_to_writer<W: io::Write>(&self, writer: &mut W, event: &Event<'_>);
}

impl FormatEventToWriter for ClerkFormatter {
    fn format_to_writer<W: io::Write>(&self, writer: &mut W, event: &Event<'_>) {
        let meta = event.metadata();
        #[cfg(not(debug_assertions))]
        write!(
            writer,
            "[{}][{}][{}]",
            chrono::Local::now().format(TIME_FORMAT),
            self.color_level(*meta.level()),
            event.metadata().target(),
        )
        .ok();

        #[cfg(debug_assertions)]
        write!(
            writer,
            "[{}][{}][{}][{}:{}] ",
            chrono::Local::now().format(TIME_FORMAT),
            self.color_level(*event.metadata().level()),
            meta.target(),
            meta.file().unwrap_or("<file>"),
            meta.line().unwrap_or(0),
        )
        .ok();

        let mut visitor = WriterFieldVisitor { writer };
        event.record(&mut visitor);
        writeln!(writer).ok();
    }
}

struct WriterFieldVisitor<'a, W: io::Write> {
    writer: &'a mut W,
}

impl<'a, W: io::Write> tracing_core::field::Visit for WriterFieldVisitor<'a, W> {
    fn record_str(&mut self, field: &tracing_core::Field, value: &str) {
        if field.name() == "message" {
            write!(self.writer, " {}", value).ok();
        } else {
            write!(self.writer, " {}={}", field.name(), value).ok();
        }
    }

    fn record_debug(&mut self, field: &tracing_core::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            write!(self.writer, " {:?}", value).ok();
        } else {
            write!(self.writer, " {}={:?}", field.name(), value).ok();
        }
    }
}
