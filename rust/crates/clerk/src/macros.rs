#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        #[cfg(feature="tracing")]
        $crate::tracing::trace!($($arg)*);
        #[cfg(feature="defmt")]
        $crate::tracing::trace!($($arg)*);
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(feature="tracing")]
        $crate::tracing::debug!($($arg)*);
        #[cfg(feature="defmt")]
        $crate::tracing::trace!($($arg)*);
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        #[cfg(feature="tracing")]
        $crate::tracing::info!($($arg)*);
        #[cfg(feature="defmt")]
        $crate::tracing::trace!($($arg)*);
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        #[cfg(feature="tracing")]
        $crate::tracing::warn!($($arg)*);
        #[cfg(feature="defmt")]
        $crate::tracing::trace!($($arg)*);
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        #[cfg(feature="tracing")]
        $crate::tracing::error!($($arg)*);
        #[cfg(feature="defmt")]
        $crate::tracing::trace!($($arg)*);
    };
}
