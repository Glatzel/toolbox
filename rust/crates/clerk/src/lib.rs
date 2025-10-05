#![cfg_attr(not(feature = "log"), no_std)]
#[cfg(all(not(feature = "embedded"), feature = "log"))]
mod std_log;
#[cfg(all(not(feature = "embedded"), feature = "log"))]
pub use std_log::*;
#[cfg(all(not(feature = "embedded"), not(feature = "log")))]
mod macros;
#[cfg(all(feature = "embedded", feature = "log"))]
pub use defmt::{debug, error, info, trace, warn};
#[cfg(all(not(feature = "embedded"), feature = "log"))]
pub use tracing::{debug, error, info, trace, warn};
