pub use defmt;
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
       $crate::defmt::trace!($($arg)*);
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
      $crate::defmt::debug!($($arg)*);
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
       $crate::defmt::info!($($arg)*);
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
       $crate::defmt::warn!($($arg)*);
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
     $crate::defmt::error!($($arg)*);
    };
}
