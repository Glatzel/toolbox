//! Rich terminal rendering for `mischief` diagnostics.

#[cfg(feature = "backtrace")]
pub use backtrace::Backtrace;

mod fancy_render;
pub use fancy_render::*;
