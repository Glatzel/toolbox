#![cfg_attr(not(feature = "std"), no_std)]
pub mod data;
mod dispatcher;
mod macros;
pub mod rules;
#[cfg(feature = "std")]
pub use dispatcher::*;
