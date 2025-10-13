/// Logs a trace-level message (no-op if neither `tracing` nor `defmt` is
/// enabled).
#[cfg(not(any(feature = "tracing", feature = "defmt")))]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {};
}

/// Logs a debug-level message (no-op if neither `tracing` nor `defmt` is
/// enabled).
#[cfg(not(any(feature = "tracing", feature = "defmt")))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {};
}

/// Logs an info-level message (no-op if neither `tracing` nor `defmt` is
/// enabled).
#[cfg(not(any(feature = "tracing", feature = "defmt")))]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {};
}

/// Logs a warning message (no-op if neither `tracing` nor `defmt` is enabled).
#[cfg(not(any(feature = "tracing", feature = "defmt")))]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {};
}

/// Logs an error message (no-op if neither `tracing` nor `defmt` is enabled).
#[cfg(not(any(feature = "tracing", feature = "defmt")))]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {};
}

// ============================================================================
// tracing backend
// ============================================================================

/// Logs a trace-level message using [`tracing::trace!`].
#[cfg(all(feature = "tracing", not(feature = "defmt")))]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::tracing::trace!($($arg)*);
    };
}

/// Logs a debug-level message using [`tracing::debug!`].
#[cfg(all(feature = "tracing", not(feature = "defmt")))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::tracing::debug!($($arg)*);
    };
}

/// Logs an info-level message using [`tracing::info!`].
#[cfg(all(feature = "tracing", not(feature = "defmt")))]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::tracing::info!($($arg)*);
    };
}

/// Logs a warning message using [`tracing::warn!`].
#[cfg(all(feature = "tracing", not(feature = "defmt")))]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::tracing::warn!($($arg)*);
    };
}

/// Logs an error message using [`tracing::error!`].
#[cfg(all(feature = "tracing", not(feature = "defmt")))]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::tracing::error!($($arg)*);
    };
}

// ============================================================================
// defmt backend
// ============================================================================

/// Logs a trace-level message using [`defmt::trace!`].
#[cfg(all(feature = "defmt", not(feature = "tracing")))]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::defmt::trace!($($arg)*);
    };
}

/// Logs a debug-level message using [`defmt::debug!`].
#[cfg(all(feature = "defmt", not(feature = "tracing")))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::defmt::debug!($($arg)*);
    };
}

/// Logs an info-level message using [`defmt::info!`].
#[cfg(all(feature = "defmt", not(feature = "tracing")))]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::defmt::info!($($arg)*);
    };
}

/// Logs a warning message using [`defmt::warn!`].
#[cfg(all(feature = "defmt", not(feature = "tracing")))]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::defmt::warn!($($arg)*);
    };
}

/// fake log
#[cfg(all(feature = "defmt", feature = "tracing"))]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::defmt::error!($($arg)*);
    };
}
/// fake log
#[cfg(all(feature = "defmt", feature = "tracing"))]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::defmt::trace!($($arg)*);
    };
}
/// fake log
#[cfg(all(feature = "defmt", feature = "tracing"))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::defmt::debug!($($arg)*);
    };
}
/// fake log
#[cfg(all(feature = "defmt", feature = "tracing"))]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::defmt::info!($($arg)*);
    };
}
/// fake log
#[cfg(all(feature = "defmt", feature = "tracing"))]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::defmt::warn!($($arg)*);
    };
}
/// fake log
#[cfg(all(feature = "defmt", feature = "tracing"))]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::defmt::error!($($arg)*);
    };
}

