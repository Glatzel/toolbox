#![cfg_attr(feature = "embedded", no_std)]
#[cfg(all(not(feature = "embedded"), feature = "log"))]
mod std_log;
#[cfg(all(not(feature = "embedded"), feature = "log"))]
pub use std_log::*;

mod macros;
pub use macros::*;
