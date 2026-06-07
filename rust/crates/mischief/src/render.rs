#[cfg(feature = "fancy")]
mod fancy_render;
#[cfg(not(feature = "fancy"))]
mod no_fancy_render;
#[cfg(feature = "fancy")]
pub use fancy_render::*;
#[cfg(not(feature = "fancy"))]
pub use no_fancy_render::*;
