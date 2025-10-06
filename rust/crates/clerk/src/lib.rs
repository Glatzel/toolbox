#![cfg_attr(feature = "defmt", no_std)]
#![allow(unexpected_cfgs)]
#[cfg(feature = "defmt")]
pub use defmt;

#[cfg(feature = "tracing")]
mod log_normal;
#[cfg(feature = "tracing")]
pub use log_normal::*;
#[cfg(all(feature = "defmt", feature = "tracing", not(doc), not(clippy)))]
compile_error!("Features `defmt` and `tracing` cannot be enabled at the same time");

mod macros;
