#[cfg(any(feature = "color", feature = "pretty", feature = "hyperlink"))]
mod fancy_render;
#[cfg(not(any(feature = "color", feature = "pretty", feature = "hyperlink")))]
mod no_fancy_render;
#[cfg(any(feature = "color", feature = "pretty", feature = "hyperlink"))]
pub use fancy_render::*;
#[cfg(not(any(feature = "color", feature = "pretty", feature = "hyperlink")))]
pub use no_fancy_render::*;
