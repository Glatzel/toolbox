//! Rich terminal rendering for `mischief` diagnostics.

#[cfg(feature = "backtrace")]
pub use backtrace::Backtrace;

#[cfg(any(feature = "color", feature = "pretty", feature = "hyperlink"))]
mod fancy_render;
#[cfg(any(feature = "color", feature = "pretty", feature = "hyperlink"))]
pub use fancy_render::*;

#[cfg(feature = "backtrace")]
mod simple_backtrace;
#[cfg(feature = "backtrace")]
pub use simple_backtrace::render_backtrace_plain;
