use std::io;

use owo_colors::OwoColorize;
use tracing::{Event, Level};
use tracing_core::Subscriber;
use tracing_core::field::Visit;
use tracing_subscriber::fmt::FmtContext;
use tracing_subscriber::fmt::format::{FormatEvent, FormatFields, Writer};
use tracing_subscriber::registry::LookupSpan;

const TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.6fZ";

/// Formatter for `tracing` events used by `tracing_subscriber::fmt`.
///
/// Output format:
///
/// ```text
/// [timestamp][LEVEL][target] message key=value
/// ```
///
/// In debug builds, source location is also included:
///
/// ```text
/// [timestamp][LEVEL][target][file:line] message key=value
/// ```
///
/// If `color` is enabled, log levels are colorized using `owo_colors`.
pub struct ClerkFormatter {
    /// Enable colored level output.
    pub color: bool,
}

impl ClerkFormatter {
    fn color_level(&self, level: Level) -> String {
        if !self.color {
            return level.to_string();
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
macro_rules! write_header {
    ($self:expr,$writer:expr, $meta:expr) => {{
        #[cfg(not(debug_assertions))]
        let _ = write!(
            $writer,
            "[{}][{}][{}]",
            chrono::Local::now().format(TIME_FORMAT),
            self.color_level(*event.metadata().level()),
            $meta.target(),
        );

        #[cfg(debug_assertions)]
        let _ = write!(
            $writer,
            "[{}][{}][{}][{}:{}] ",
            chrono::Local::now().format(TIME_FORMAT),
            $self.color_level(*$meta.level()),
            $meta.target(),
            $meta.file().unwrap_or("<file>"),
            $meta.line().unwrap_or(0),
        );
    }};
}
impl<S, N> FormatEvent<S, N> for ClerkFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        write_header!(self, writer, event.metadata());
        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}

/// Allows formatting a `tracing::Event` into any `io::Write`.
pub trait FormatEventToWriter {
    fn format_to_writer<W: io::Write>(&self, writer: &mut W, event: &Event<'_>);
}

impl FormatEventToWriter for ClerkFormatter {
    fn format_to_writer<W: io::Write>(&self, writer: &mut W, event: &Event<'_>) {
        let _ = write_header!(self, writer, event.metadata());
        let mut visitor = WriterFieldVisitor { writer };
        event.record(&mut visitor);
        let _ = writeln!(writer);
    }
}

struct WriterFieldVisitor<'a, W: io::Write> {
    writer: &'a mut W,
}

impl<'a, W: io::Write> Visit for WriterFieldVisitor<'a, W> {
    fn record_str(&mut self, field: &tracing_core::Field, value: &str) {
        if field.name() == "message" {
            let _ = write!(self.writer, " {}", value);
        } else {
            let _ = write!(self.writer, " {}={}", field.name(), value);
        }
    }

    fn record_debug(&mut self, field: &tracing_core::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            let _ = write!(self.writer, " {:?}", value);
        } else {
            let _ = write!(self.writer, " {}={:?}", field.name(), value);
        }
    }
}
