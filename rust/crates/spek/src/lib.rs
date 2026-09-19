#![cfg_attr(not(feature = "std"), no_std)]
mod error;
mod stft;
pub mod windows;
pub use error::SpekError;
