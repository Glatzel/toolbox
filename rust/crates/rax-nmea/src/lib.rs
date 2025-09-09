#![cfg_attr(not(feature = "std"), no_std)]
pub mod data;
#[cfg(feature = "std")]
mod dispatcher;
mod macros;
pub mod rules;
#[cfg(feature = "std")]
pub use dispatcher::*;
mod error;
pub use error::RaxNmeaError;
