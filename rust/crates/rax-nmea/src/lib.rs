#![no_std]
#![deny(clippy::unwrap_used)]

pub mod common;
mod error;
pub mod rules;
pub use error::RaxNmeaError;
pub mod sentence;
