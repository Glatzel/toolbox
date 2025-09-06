#![cfg_attr(not(feature="std"), no_std)]

#[cfg(feature = "device")]
pub mod device;
#[cfg(feature = "std")]
pub mod io;

pub mod str_parser;
