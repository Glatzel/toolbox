//! Envoy — helpers for interoperating with C strings in no_std contexts.
//!
//! This crate provides tiny, focused utilities to convert between C-style
//! strings (CStr / *const c_char / null-terminated lists) and Rust
//! heap-allocated types (`String`, `CString`) while remaining `#![no_std]` by
//! relying on the `alloc` crate.
//!
//! Public modules:
//! - `cstr_to_string`         — convert a C string pointer/CStr to `String`.
//! - `to_cstring`             — create `CString` for FFI use.
//! - `vec_cstring`            — helpers for vectors of `CString`.
//! - `cstr_list_to_vec_string`— convert null-terminated lists of C strings.
//!
//! # Examples
//!
//! ```no_run
//! use envoy::cstr_to_string::cstr_to_string;
//! // unsafe { let s = cstr_to_string(ptr); }
//! ```
//!
//! Note: this crate is `#![no_std]` but requires `alloc` for heap allocations.
#![no_std]
extern crate alloc;

mod cstr_list_to_vec_string;
mod cstr_to_string;
mod to_cstring;
mod vec_cstring;

pub use cstr_list_to_vec_string::*;
pub use cstr_to_string::*;
pub use to_cstring::*;
pub use vec_cstring::*;
