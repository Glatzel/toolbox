/// Logs a trace-level message (no-op if neither `tracing` nor `defmt` is
/// enabled).
#[cfg(not(any(feature = "tracing", feature = "defmt")))]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => { let _ = ::core::format_args!($($arg)*);};
}

/// Logs a debug-level message (no-op if neither `tracing` nor `defmt` is
/// enabled).
#[cfg(not(any(feature = "tracing", feature = "defmt")))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => { let _ = ::core::format_args!($($arg)*);};
}

/// Logs an info-level message (no-op if neither `tracing` nor `defmt` is
/// enabled).
#[cfg(not(any(feature = "tracing", feature = "defmt")))]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => { let _ = ::core::format_args!($($arg)*);};
}

/// Logs a warning message (no-op if neither `tracing` nor `defmt` is enabled).
#[cfg(not(any(feature = "tracing", feature = "defmt")))]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => { let _ = ::core::format_args!($($arg)*);};
}

/// Logs an error message (no-op if neither `tracing` nor `defmt` is enabled).
#[cfg(not(any(feature = "tracing", feature = "defmt")))]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => { let _ = ::core::format_args!($($arg)*);};
}

#[cfg(all(feature = "defmt", not(feature = "tracing")))]
pub use defmt::debug;
#[cfg(all(feature = "defmt", not(feature = "tracing")))]
pub use defmt::error;
#[cfg(all(feature = "defmt", not(feature = "tracing")))]
pub use defmt::info;
#[cfg(all(feature = "defmt", not(feature = "tracing")))]
pub use defmt::trace;
#[cfg(all(feature = "defmt", not(feature = "tracing")))]
pub use defmt::warn;
#[cfg(all(feature = "tracing", not(feature = "defmt")))]
pub use tracing::debug;
#[cfg(all(feature = "tracing", not(feature = "defmt")))]
pub use tracing::error;
#[cfg(all(feature = "tracing", not(feature = "defmt")))]
pub use tracing::info;
#[cfg(all(feature = "tracing", not(feature = "defmt")))]
pub use tracing::trace;
#[cfg(all(feature = "tracing", not(feature = "defmt")))]
pub use tracing::warn;
