//! NMEA 0183 sentence parsing.
//!
//! `rax-nmea` provides `no_std` parsers for a subset of the NMEA 0183
//! protocol, built on the [`rax`] text parsing primitives. Every supported
//! sentence is exposed as a struct that implements [`FromStr`], with optional
//! fields represented as [`Option`].
//!
//! # Example
//!
//! ```
//! use core::str::FromStr;
//! use rax_nmea::sentence::Gga;
//!
//! let gga = Gga::from_str(
//!     "$GPGGA,110256,5505.676996,N,03856.028884,E,2,08,0.7,2135.0,M,14.0,M,,*7D",
//! )?;
//! assert_eq!(gga.num_sv(), &Some(8));
//! assert!(gga.lat().is_some());
//! # Ok::<(), rax_nmea::RaxNmeaError>(())
//! ```
//!
//! # Supported sentences
//!
//! The [`sentence`] module contains parsers for DHV, DTM, GBQ, GBS, GGA, GLL,
//! GLQ, GNQ, GNS, GPQ, GRS, GSA, GST, GSV, RMC, THS, TXT, VLW, VTG and ZDA.
//! The [`common`] module holds the shared talker, identifier, and status
//! enumerations, while [`rules`] exposes the lower-level [`rax`] parsing
//! rules used to build the sentence parsers.
//!
//! # References
//!
//! * <https://gpsd.gitlab.io/gpsd/NMEA.html>
//! * <https://receiverhelp.trimble.com/alloy-gnss/en-us/NMEA-0183messages_MessageOverview.html>
//!
//! [`FromStr`]: core::str::FromStr
//! [`Option`]: core::option::Option

#![no_std]
#![warn(missing_docs)]

/// Shared NMEA enumerations such as talkers, sentence identifiers, and status
/// flags.
pub mod common;
mod error;
/// Low-level [`rax`] rules for parsing individual NMEA fields.
///
/// The rules in this module can be composed directly with
/// [`rax::text::StrParser`] when a full sentence type is not needed.
pub mod rules;
pub use error::RaxNmeaError;
/// Parsers for individual NMEA sentence types.
///
/// Every sentence type implements [`core::str::FromStr`], so it can be parsed
/// with `Sentence::from_str(input)`.
pub mod sentence;
mod utils;
