#![no_std]
#![deny(clippy::unwrap_used)]

pub mod data;
mod error;
pub mod rules;
pub use error::RaxNmeaError;
