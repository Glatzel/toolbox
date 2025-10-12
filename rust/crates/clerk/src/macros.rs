//! Unified logging macros for `tracing`, `defmt`, or no-op fallback.
//!
//! These macros automatically adapt depending on which Cargo feature is
//! enabled:
//!
//! - `tracing` → Uses the [`tracing`] crate’s macros.
//! - `defmt` → Uses the [`defmt`] crate’s macros.
//! - Neither → Expands to a no-op (empty macro).
//!
//! This allows your code to use `trace!`, `debug!`, `info!`, `warn!`, and
//! `error!` consistently without needing to import feature-specific crates.
//!
//! # Feature Matrix
//! | Feature combination | Behavior |
//! |---------------------|-----------|
//! | none                | No-op (logs are disabled) |
//! | `tracing` only      | Uses `tracing::*` macros |
//! | `defmt` only        | Uses `defmt::*` macros |
//! | both `tracing` + `defmt` | Compile error (should not be enabled together) |

// ============================================================================
// No-op fallback (no logging backend enabled)
// ============================================================================

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

/// Logs an error message using [`defmt::error!`].
#[cfg(all(feature = "defmt", not(feature = "tracing")))]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::defmt::error!($($arg)*);
    };
}
