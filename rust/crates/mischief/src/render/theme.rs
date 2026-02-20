use owo_colors::Style;

use crate::Severity;
use crate::render::{Item, Layer};
/// Trait for computing indentation strings based on layer layers and elements.
pub trait IIndent {
    /// Returns a tuple of `(prefix, continuation)` strings for the given layer
    /// and element.
    fn get_indent(&self, layer: Layer, element: Item) -> (&'static str, &'static str);
}
pub struct DefaultIndent;
impl IIndent for DefaultIndent {
    fn get_indent(&self, layer: Layer, element: Item) -> (&'static str, &'static str) {
        match (layer, element) {
            (Layer::Bottom, Item::First) => ("x ", "│ "),
            (Layer::Bottom, Item::Other) => ("│ ", "│ "),
            (Layer::Middle, Item::First) => ("├─▶ ", "│   "),
            (Layer::Middle, Item::Other) => ("│   ", "│   "),
            (Layer::Top, Item::First) => ("╰─▶ ", "    "),
            (Layer::Top, Item::Other) => ("    ", "    "),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HyperlinkFormat {
    Plain,
    Link,
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
/// Default theme implementation using `owo_colors`.
#[derive(Debug, Clone, PartialEq)]
pub struct DefaultStyle;

impl IStyle for DefaultStyle {
    fn default_style(&self) -> Option<Style> { Some(Style::default()) }
    fn indent_style(&self) -> Option<Style> { Some(Style::new().red()) }
    fn description_style(&self) -> Option<Style> { Some(Style::default()) }
    fn severity_style(&self, severity: Option<Severity>) -> Option<Style> {
        match severity {
            Some(Severity::Advice) => Some(Style::new().green()),
            Some(Severity::Warning) => Some(Style::new().yellow()),
            Some(Severity::Error) => Some(Style::new().red()),
            None => Some(Style::default()),
        }
    }
    fn help_style(&self) -> (Option<Style>, Option<Style>) {
        (Some(Style::default()), Some(Style::new().cyan()))
    }
    fn hyperlink_style(&self) -> (Option<Style>, HyperlinkFormat) {
        (Some(Style::new().blue()), HyperlinkFormat::Link)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NoColorStyle;
impl IStyle for NoColorStyle {
    fn default_style(&self) -> Option<Style> { None }
    fn indent_style(&self) -> Option<Style> { None }
    fn description_style(&self) -> Option<Style> { None }
    fn severity_style(&self, _severity: Option<Severity>) -> Option<Style> { None }
    fn help_style(&self) -> (Option<Style>, Option<Style>) { (None, None) }
    fn hyperlink_style(&self) -> (Option<Style>, HyperlinkFormat) { (None, HyperlinkFormat::Link) }
}
pub trait ITheme: IIndent + IStyle {
    fn width(&self) -> Option<usize>;
}
/// Default theme implementation using `owo_colors`.
#[derive(Debug, Clone, PartialEq)]
pub struct DefaultTheme;
impl ITheme for DefaultTheme {
    fn width(&self) -> Option<usize> { None }
}
impl IIndent for DefaultTheme {
    fn get_indent(&self, layer: Layer, element: Item) -> (&'static str, &'static str) {
        DefaultIndent.get_indent(layer, element)
    }
}
impl IStyle for DefaultTheme {
    fn default_style(&self) -> Option<Style> { DefaultStyle.default_style() }
    fn indent_style(&self) -> Option<Style> { DefaultStyle.indent_style() }
    fn description_style(&self) -> Option<Style> { DefaultStyle.description_style() }
    fn severity_style(&self, severity: Option<Severity>) -> Option<Style> {
        DefaultStyle.severity_style(severity)
    }
    fn help_style(&self) -> (Option<Style>, Option<Style>) { DefaultStyle.help_style() }
    fn hyperlink_style(&self) -> (Option<Style>, HyperlinkFormat) { DefaultStyle.hyperlink_style() }
}
