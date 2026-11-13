#[cfg(not(any(feature = "color", feature = "pretty", feature = "hyperlink")))]
pub use mischief_core::render_diagnosis;
#[cfg(all(
    any(feature = "color", feature = "pretty", feature = "hyperlink"),
    feature = "backtrace"
))]
pub use mischief_render::render_backtrace;
#[cfg(all(
    not(any(feature = "color", feature = "pretty", feature = "hyperlink")),
    feature = "backtrace"
))]
pub use mischief_render::render_backtrace_plain as render_backtrace;
#[cfg(any(feature = "color", feature = "pretty", feature = "hyperlink"))]
pub use mischief_render::{
    HyperlinkFormat, ITheme, MischiefIndent, RenderBundle, render_diagnosis,
};
