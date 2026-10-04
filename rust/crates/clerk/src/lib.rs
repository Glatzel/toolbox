#![cfg_attr(feature = "defmt", no_std)]

#[cfg(feature = "defmt")]
pub use defmt;

#[cfg(feature = "tracing")]
mod log_tracing;
#[cfg(feature = "tracing")]
pub use log_tracing::*;

mod macros;
