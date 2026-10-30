//! Composable text-parsing rules for `no_std` targets.
//!
//! The crate parses UTF-8 text by applying small, reusable rules. A *flow
//! rule* consumes a prefix of the input and reports how many bytes it used; a
//! *global rule* inspects the whole input without consuming it.
//! [`text::StrParser`] drives flow rules over an input and [`text::rules`]
//! provides the concrete rules. See [`text`] and [`error`] for the main entry
//! points.

#![no_std]

pub mod error;
pub mod text;
