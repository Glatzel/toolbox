#![cfg_attr(feature = "defmt", no_std)]
#![allow(unexpected_cfgs)]
#[cfg(feature = "defmt")]
pub use defmt;

#[cfg(feature = "tracing")]
mod log_normal;
#[cfg(feature = "tracing")]
pub use log_normal::*;

mod macros;
