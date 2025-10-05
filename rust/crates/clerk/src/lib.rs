#![cfg_attr(feature = "defmt", no_std)]

#[cfg(feature = "defmt")]
pub use fmt;

#[cfg(feature = "tracing")]
mod log_normal;
#[cfg(feature = "tracing")]
pub use log_normal::*;

mod macros;
