/// Represents the log levels supported by the application.
///
/// This enum is used for configuring both terminal and file logging layers.
/// It maps directly to [`tracing_core::LevelFilter`] via `From<LogLevel>`.
///
/// # Variants
///
/// - `ERROR` — Error messages, typically critical failures.
/// - `WARN`  — Warnings about potential issues.
/// - `INFO`  — General informational messages.
/// - `DEBUG` — Debugging information, usually verbose.
/// - `TRACE` — Very fine-grained tracing information.
/// - `OFF`   — Disable all logging.
///
/// # Example
///
/// ```
/// use clerk::LogLevel;
/// use tracing_core::LevelFilter;
///
/// let level: LogLevel = LogLevel::DEBUG;
/// let filter: LevelFilter = level.into();
/// assert_eq!(filter, LevelFilter::DEBUG);
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LogLevel {
    ERROR,
    WARN,
    INFO,
    DEBUG,
    TRACE,
    OFF,
}

impl From<LogLevel> for tracing_core::LevelFilter {
    fn from(value: LogLevel) -> Self {
        use tracing_core::LevelFilter;

        match value {
            LogLevel::ERROR => LevelFilter::ERROR,
            LogLevel::WARN => LevelFilter::WARN,
            LogLevel::INFO => LevelFilter::INFO,
            LogLevel::DEBUG => LevelFilter::DEBUG,
            LogLevel::TRACE => LevelFilter::TRACE,
            LogLevel::OFF => LevelFilter::OFF,
        }
    }
}
