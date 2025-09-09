/// Trace-level logging macro.
///
/// Delegates to [`tracing::trace!`] when the `log` feature is enabled.
/// Otherwise, expands to a no-op.
#[cfg(feature = "log")]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::tracing::trace!($($arg)*);
    };
}

#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {{}};
}

/// Debug-level logging macro.
///
/// Delegates to [`tracing::debug!`] when the `log` feature is enabled.
/// Otherwise, expands to a no-op.
#[cfg(feature = "log")]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::tracing::debug!($($arg)*);
    };
}

#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{}};
}

/// Info-level logging macro.
///
/// Delegates to [`tracing::info!`] when the `log` feature is enabled.
/// Otherwise, expands to a no-op.
#[cfg(feature = "log")]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::tracing::info!($($arg)*);
    };
}

#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{}};
}

/// Warn-level logging macro.
///
/// Delegates to [`tracing::warn!`] when the `log` feature is enabled.
/// Otherwise, expands to a no-op.
#[cfg(feature = "log")]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::tracing::warn!($($arg)*);
    };
}

#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{}};
}

/// Error-level logging macro.
///
/// Delegates to [`tracing::error!`] when the `log` feature is enabled.
/// Otherwise, expands to a no-op.
#[cfg(feature = "log")]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::tracing::error!($($arg)*);
    };
}

#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{}};
}
