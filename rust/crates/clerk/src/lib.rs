#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(all(feature = "std", feature = "log"))]
mod std_log;
#[cfg(all(feature = "std", feature = "log"))]
pub use std_log::*;
#[cfg(not(feature = "log"))]
mod macros;
#[cfg(all(feature = "log", feature = "std"))]
pub use defmt::{debug, error, info, trace, warn};
#[cfg(not(feature = "log"))]
pub use macros::*;
#[cfg(all(feature = "log", not(feature = "std")))]
pub use tracing::{debug, error, info, trace, warn};
