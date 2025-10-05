#[cfg(not(any(feature = "tracing", feature = "defmt")))]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {};
}
#[cfg(not(any(feature = "tracing", feature = "defmt")))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {};
}
#[cfg(not(any(feature = "tracing", feature = "defmt")))]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {};
}
#[cfg(not(any(feature = "tracing", feature = "defmt")))]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {};
}
#[cfg(not(any(feature = "tracing", feature = "defmt")))]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {};
}
#[cfg(all(feature = "tracing", not(feature = "defmt")))]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {

        $crate::tracing::trace!($($arg)*);

    };
}
#[cfg(all(feature = "tracing", not(feature = "defmt")))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {

        $crate::tracing::debug!($($arg)*);

    };
}
#[cfg(all(feature = "tracing", not(feature = "defmt")))]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::tracing::info!($($arg)*);
    };
}
#[cfg(all(feature = "tracing", not(feature = "defmt")))]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::tracing::warn!($($arg)*);
    };
}
#[cfg(all(feature = "tracing", not(feature = "defmt")))]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::tracing::error!($($arg)*);
    };
}

#[cfg(all(feature = "defmt", not(feature = "tracing")))]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::defmt::trace!($($arg)*);
    };
}
#[cfg(all(feature = "defmt", not(feature = "tracing")))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::defmt::debug!($($arg)*);
    };
}
#[cfg(all(feature = "defmt", not(feature = "tracing")))]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::defmt::info!($($arg)*);
    };
}
#[cfg(all(feature = "defmt", not(feature = "tracing")))]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::defmt::warn!($($arg)*);
    };
}
#[cfg(all(feature = "defmt", not(feature = "tracing")))]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::defmt::error!($($arg)*);
    };
}

#[cfg(all(feature = "defmt", feature = "tracing"))]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::tracing::trace!($($arg)*);
        $crate::defmt::trace!($($arg)*);
    };
}
#[cfg(all(feature = "defmt", feature = "tracing"))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::tracing::debug!($($arg)*);
        $crate::defmt::debug!($($arg)*);
    };
}
#[cfg(all(feature = "defmt", feature = "tracing"))]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::tracing::info!($($arg)*);
        $crate::defmt::info!($($arg)*);
    };
}
#[cfg(all(feature = "defmt", feature = "tracing"))]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::tracing::warn!($($arg)*);
        $crate::defmt::warn!($($arg)*);
    };
}
#[cfg(all(feature = "defmt", feature = "tracing"))]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::tracing::error!($($arg)*);
        $crate::defmt::error!($($arg)*);
    };
}
