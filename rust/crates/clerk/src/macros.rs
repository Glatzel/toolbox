// Default: tracing backend (std)
#[cfg(all(feature = "log", not(feature = "embedded")))]
mod log_backend {
    pub use tracing::{trace, debug, info, warn, error};
}

// Embedded: defmt backend
#[cfg(all(feature = "log", feature = "embedded"))]
mod log_backend {
    pub use defmt::{trace, debug, info, warn, error};
}

// No log feature: define no-op macros (zero cost)
#[cfg(not(feature = "log"))]
mod log_backend {
    #[macro_export]
    macro_rules! trace {
        ($($arg:tt)*) => {{}};
    }
    #[macro_export]
    macro_rules! debug {
        ($($arg:tt)*) => {{}};
    }
    #[macro_export]
    macro_rules! info {
        ($($arg:tt)*) => {{}};
    }
    #[macro_export]
    macro_rules! warn {
        ($($arg:tt)*) => {{}};
    }
    #[macro_export]
    macro_rules! error {
        ($($arg:tt)*) => {{}};
    }
}

// Re-export unified interface
pub use log_backend::*;
