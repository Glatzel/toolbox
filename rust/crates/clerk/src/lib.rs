#![cfg_attr(not(feature = "log"), no_std)]
#[cfg(feature = "log-embedded")]
mod log_embedded;
#[cfg(feature = "log-embedded")]
pub use log_embedded::*;

#[cfg(feature = "log")]
mod log_normal;
#[cfg(feature = "log")]
pub use log_normal::*;
#[cfg(all(not(feature = "log-embedded"), not(feature = "log")))]
mod macros;
