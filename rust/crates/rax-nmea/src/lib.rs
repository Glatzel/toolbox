#![cfg_attr(not(feature = "std"), no_std)]
pub mod data;
mod dispatcher;
mod macros;
pub mod rules;
pub use dispatcher::*;
mod error;
pub use error::RaxNmeaError;
