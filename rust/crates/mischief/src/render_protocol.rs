use core::fmt::Display;
extern crate alloc;
use alloc::string::String;

use owo_colors::Style;

use crate::{IDiagnostic, Severity};

/// Represents the position of a layer in a hierarchical layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Layer {
    /// The bottom layer (start of the tree branch).
    Bottom,
    /// A middle layer (continuation of a branch).
    Middle,
    /// The top layer (end of a branch).
    Top,
}

/// Represents the position of an item within a layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Item {
    /// The first item in the layer.
    First,
    /// Any subsequent item in the layer.
    Other,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HyperlinkFormat {
    Plain,
    Link,
}
pub trait IIndent {
    /// Returns a tuple of `(prefix, continuation)` strings for the given layer
    /// and element.
    fn get_indent(&self, layer: Layer, element: Item) -> (&'static str, &'static str);
}
/// Trait defining styling for different components of diagnostic output.
pub trait IStyle {
    /// Style for default text.
    fn default_style(&self) -> Option<Style>;
    /// Style for indentation lines.
    fn indent_style(&self) -> Option<Style>;
    /// Style for error or diagnostic descriptions.
    fn description_style(&self) -> Option<Style>;
    /// Style for severity labels.
    fn severity_style(&self, severity: Option<Severity>) -> Option<Style>;
    /// Style for help text: returns a tuple of `(prefix_style, message_style)`.
    fn help_style(&self) -> (Option<Style>, Option<Style>);
    /// Style for URLs.
    fn hyperlink_style(&self) -> (Option<Style>, HyperlinkFormat);
}
pub trait ITheme: IIndent + IStyle {
    fn width(&self) -> Option<usize>;
}
pub trait ITerminalConfig {
    /// Returns the width of the terminal in columns.
    fn terminal_width(&self) -> Option<usize>;
    /// Returns whether the terminal supports color output.
    fn support_color(&self) -> bool;
    /// Returns whether the terminal supports hyperlinks.
    fn support_hyperlinks(&self) -> bool;
    /// Returns whether the terminal supports Unicode characters.
    fn supports_unicode(&self) -> bool;
}
pub trait IShader {
    fn apply<D: Display, TC: ITerminalConfig>(
        &self,
        buffer: &mut String,
        text: D,
        style: &Option<owo_colors::Style>,
        terminal_config: &TC,
    ) -> core::fmt::Result;
    fn apply_hyperlink<D: Display, T: ITheme, TC: ITerminalConfig>(
        &self,
        buffer: &mut String,
        hyperlink: D,
        text: D,
        theme: &T,
        terminal_config: &TC,
    ) -> core::fmt::Result;

    fn wrap_string<T: ITheme, TC: ITerminalConfig>(
        &self,
        buffer: &str,
        terminal_config: &TC,
        theme: &T,
        layer: Layer,
        element: Item,
    ) -> String;
}
/// Trait defining rendering behavior for diagnostic types.
pub trait IRender {
    fn render(&self, text: &mut String, diagnostic: &impl IDiagnostic) -> core::fmt::Result;
}
