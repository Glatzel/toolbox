//! Clerk — small logging adapters and helpers.
//!
//! This crate provides lightweight glue to integrate different logging backends
//! in a convenient, feature-gated way:
//!
//! - With the `defmt` feature enabled the crate is `no_std` and re-exports
//!   `defmt` for use in embedded environments.
//! - With the `tracing` feature enabled it provides integration for `tracing`.
//!
//! Only one of `defmt` or `tracing` may be enabled at a time.
//!
//! # Examples
//!
//! ```no_run
//! // Enable the desired feature in Cargo.toml and then use the re-exports:
//! // For `defmt`:
//! //   use clerk::defmt;
//! //
//! // For `tracing` the crate exposes its tracing helpers when the feature is enabled.
//! ```
#![cfg_attr(feature = "defmt", no_std)]
#![allow(unexpected_cfgs)]
#[cfg(feature = "defmt")]
pub use defmt;

#[cfg(feature = "tracing")]
mod log_normal;
#[cfg(feature = "tracing")]
pub use log_normal::*;

mod macros;
#[cfg(all(feature = "defmt", feature = "tracing",))]
compile_error!("Features `defmt` and `tracing` cannot be enabled at the same time");
