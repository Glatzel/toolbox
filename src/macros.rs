
#[cfg(feature = "tracing")]
#[macro_export]
macro_rules! trace_log {
    ($($arg:tt)*) => {
        tracing::info!($($arg)*);
    };
}

#[cfg(not(feature = "tracing"))]
#[macro_export]
macro_rules! trace_log {
    ($($arg:tt)*) => {};
}