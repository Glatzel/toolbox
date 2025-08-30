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
