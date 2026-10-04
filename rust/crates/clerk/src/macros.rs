// ============================================================================
// backend dispatch (the only place that depends on features)
// ============================================================================

#[cfg(not(any(feature = "tracing", feature = "defmt")))]
#[doc(hidden)]
#[macro_export]
macro_rules! __log {
    ($lvl:ident, $($arg:tt)*) => {{ let _ = ::core::format_args!($($arg)*); }};
}

#[cfg(all(feature = "tracing", not(feature = "defmt")))]
#[doc(hidden)]
#[macro_export]
macro_rules! __log {
    ($lvl:ident, $($arg:tt)*) => { $crate::tracing::$lvl!($($arg)*) };
}

#[cfg(all(feature = "defmt", not(feature = "tracing")))]
#[doc(hidden)]
#[macro_export]
macro_rules! __log {
    ($lvl:ident, $($arg:tt)*) => { $crate::defmt::$lvl!($($arg)*) };
}

#[cfg(all(feature = "defmt", feature = "tracing"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __log {
    ($lvl:ident, $($arg:tt)*) => {{
        $crate::defmt::$lvl!($($arg)*);
        $crate::tracing::$lvl!($($arg)*);
    }};
}

// ============================================================================
// public macros
// ============================================================================

/// Logs a trace-level message via the enabled backend(s).
#[macro_export]
macro_rules! trace { ($($arg:tt)*) => { $crate::__log!(trace, $($arg)*) }; }

/// Logs a debug-level message via the enabled backend(s).
#[macro_export]
macro_rules! debug { ($($arg:tt)*) => { $crate::__log!(debug, $($arg)*) }; }

/// Logs an info-level message via the enabled backend(s).
#[macro_export]
macro_rules! info { ($($arg:tt)*) => { $crate::__log!(info, $($arg)*) }; }

/// Logs a warning message via the enabled backend(s).
#[macro_export]
macro_rules! warn { ($($arg:tt)*) => { $crate::__log!(warn, $($arg)*) }; }

/// Logs an error message via the enabled backend(s).
#[macro_export]
macro_rules! error { ($($arg:tt)*) => { $crate::__log!(error, $($arg)*) }; }
