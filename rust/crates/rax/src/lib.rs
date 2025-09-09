#![cfg_attr(not(feature = "std"), no_std)]

mod error;
#[cfg(feature = "std")]
pub mod io;
pub use error::RaxError;
pub mod str_parser;
