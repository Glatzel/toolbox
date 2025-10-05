// ===== Internal hidden macros =====
#[macro_export]
#[doc(hidden)]
macro_rules! __trace {
    ($($arg:tt)*) => {
        #[cfg(feature = "tracing")]
        $crate::tracing::trace!($($arg)*);

        #[cfg(feature = "defmt")]
        $crate::defmt::trace!($($arg)*);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __debug {
    ($($arg:tt)*) => {
        #[cfg(feature = "tracing")]
        $crate::tracing::debug!($($arg)*);

        #[cfg(feature = "defmt")]
        $crate::defmt::debug!($($arg)*);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __info {
    ($($arg:tt)*) => {
        #[cfg(feature = "tracing")]
        $crate::tracing::info!($($arg)*);

        #[cfg(feature = "defmt")]
        $crate::defmt::info!($($arg)*);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __warn {
    ($($arg:tt)*) => {
        #[cfg(feature = "tracing")]
        $crate::tracing::warn!($($arg)*);

        #[cfg(feature = "defmt")]
        $crate::defmt::warn!($($arg)*);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __error {
    ($($arg:tt)*) => {
        #[cfg(feature = "tracing")]
        $crate::tracing::error!($($arg)*);

        #[cfg(feature = "defmt")]
        $crate::defmt::error!($($arg)*);
    };
}

// ===== Public macros =====
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::__trace!($($arg)*);
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::__debug!($($arg)*);
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::__info!($($arg)*);
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::__warn!($($arg)*);
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::__error!($($arg)*);
    };
}
