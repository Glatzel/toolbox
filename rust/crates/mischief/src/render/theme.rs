use owo_colors::Style;

use crate::Severity;
use crate::render::{Item, Layer};
/// Trait for computing indentation strings based on layer layers and elements.
pub trait IIndent {
    /// Returns a tuple of `(prefix, continuation)` strings for the given layer
    /// and element.
    fn get_indent(&self, layer: Layer, element: Item) -> (&'static str, &'static str);
}

/// Trait defining styling for different components of diagnostic output.
pub trait IColorPalette {
    /// Style for default text.
    fn default_color(&self) -> Option<Style>;
    /// Style for indentation lines.
    fn indent_color(&self) -> Option<Style>;
    /// Style for error or diagnostic descriptions.
    fn description_color(&self) -> Option<Style>;
    /// Style for severity labels.
    fn severity_color(&self, severity: Option<Severity>) -> Option<Style>;
    /// Style for help text: returns a tuple of `(prefix_style, message_style)`.
    fn help_color(&self) -> (Option<Style>, Option<Style>);
    /// Style for URLs.
    fn url_color(&self) -> Option<Style>;
}
/// Default theme implementation using `owo_colors`.
#[derive(Debug, Clone, PartialEq)]
pub struct ColorPalette {
    default: Option<Style>,
    indent: Option<Style>,
    description: Option<Style>,
    severity_error: Option<Style>,
    severity_warning: Option<Style>,
    severity_advice: Option<Style>,
    help: (Option<Style>, Option<Style>),
    url: Option<Style>,
}
impl ColorPalette {
    pub fn new() -> Self {
        Self {
            default: None,
            indent: None,
            description: None,
            severity_error: None,
            severity_warning: None,
            severity_advice: None,
            help: (None, None),
            url: None,
        }
    }
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            default: Some(Style::default()),
            indent: Some(Style::new().red()),
            description: Some(Style::default()),
            severity_error: Some(Style::new().green()),
            severity_warning: Some(Style::new().yellow()),
            severity_advice: Some(Style::new().red()),
            help: (Some(Style::default()), Some(Style::new().cyan())),
            url: Some(Style::new().blue()),
        }
    }
}
impl IColorPalette for ColorPalette {
    fn default_color(&self) -> Option<Style> { self.default }
    fn indent_color(&self) -> Option<Style> { self.indent }
    fn description_color(&self) -> Option<Style> { self.description }
    fn severity_color(&self, severity: Option<Severity>) -> Option<Style> {
        match severity {
            Some(Severity::Advice) => self.severity_advice,
            Some(Severity::Warning) => self.severity_warning,
            Some(Severity::Error) => self.severity_error,
            None => None,
        }
    }
    fn help_color(&self) -> (Option<Style>, Option<Style>) { self.help }
    fn url_color(&self) -> Option<Style> { self.url }
}

pub trait ITheme: IIndent + IColorPalette {}
/// Default theme implementation using `owo_colors`.
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    indent: (
        (&'static str, &'static str),
        (&'static str, &'static str),
        (&'static str, &'static str),
        (&'static str, &'static str),
        (&'static str, &'static str),
        (&'static str, &'static str),
    ),
    color_palette: ColorPalette,
}
impl Theme {
    pub fn new(
        indent: (
            (&'static str, &'static str),
            (&'static str, &'static str),
            (&'static str, &'static str),
            (&'static str, &'static str),
            (&'static str, &'static str),
            (&'static str, &'static str),
        ),
        color_palette: ColorPalette,
    ) -> Self {
        Self {
            indent,
            color_palette,
        }
    }
}
impl Default for Theme {
    fn default() -> Self {
        Self {
            indent: (
                ("x ", "│ "),
                ("│ ", "│ "),
                ("├─▶ ", "│   "),
                ("│   ", "│   "),
                ("╰─▶ ", "    "),
                ("    ", "    "),
            ),
            color_palette: Default::default(),
        }
    }
}
impl ITheme for Theme {}
impl IIndent for Theme {
    fn get_indent(&self, node: Layer, element: Item) -> (&'static str, &'static str) {
        match (node, element) {
            (Layer::Bottom, Item::First) => (self.indent.0.0, self.indent.0.1),
            (Layer::Bottom, Item::Other) => (self.indent.1.0, self.indent.1.1),
            (Layer::Middle, Item::First) => (self.indent.2.0, self.indent.2.1),
            (Layer::Middle, Item::Other) => (self.indent.3.0, self.indent.3.1),
            (Layer::Top, Item::First) => (self.indent.4.0, self.indent.4.1),
            (Layer::Top, Item::Other) => (self.indent.5.0, self.indent.5.1),
        }
    }
}
impl IColorPalette for Theme {
    fn default_color(&self) -> Option<Style> { IColorPalette::default_color(&self.color_palette) }
    fn indent_color(&self) -> Option<Style> { self.color_palette.indent_color() }
    fn description_color(&self) -> Option<Style> { self.color_palette.description_color() }
    fn severity_color(&self, severity: Option<Severity>) -> Option<Style> {
        self.color_palette.severity_color(severity)
    }
    fn help_color(&self) -> (Option<Style>, Option<Style>) { self.color_palette.help_color() }
    fn url_color(&self) -> Option<Style> { self.color_palette.url_color() }
}
