#[cfg(not(feature = "std"))]
pub use mischief_core::render_diagnosis;
#[cfg(all(feature = "std", feature = "backtrace"))]
pub use mischief_render::render_backtrace;
#[cfg(feature = "std")]
pub use mischief_render::{
    HyperlinkFormat, ITheme, MischiefIndent, RenderBundle, render_diagnosis,
};
