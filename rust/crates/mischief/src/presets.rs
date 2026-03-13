use core::fmt;

use arbor::protocol::{IIndent, Layer, Line};
use arbor::renders::Render;
use arbor::trees::Tree;
use owo_colors::Style;

use crate::{Report, Severity};

#[derive(Debug, Clone)]
pub struct MischiefIndent {
    pub root_first: &'static str,
    pub root_other: &'static str,
    pub top_middle_first: &'static str,
    pub bottom_first: &'static str,
    pub bottom_other: &'static str,
    pub other: &'static str,
}
impl IIndent for MischiefIndent {
    fn get_indent(&self, layer: Layer, line: Line) -> &'static str {
        match (layer, line) {
            (Layer::Root, Line::First) => self.root_first,
            (Layer::Root, Line::Other) => self.root_other,
            (Layer::Top | Layer::Middle, Line::First) => self.top_middle_first,
            (Layer::Bottom, Line::First) => self.bottom_first,
            (Layer::Bottom, Line::Other) => self.bottom_other,
            (_, Line::Other) => self.other,
        }
    }
}
impl Default for MischiefIndent {
    fn default() -> Self {
        {
            if supports_unicode::on(supports_unicode::Stream::Stdout) {
                Self {
                    root_first: "x ",
                    root_other: "│ ",
                    top_middle_first: "├─▶ ",
                    bottom_first: "╰─▶ ",
                    bottom_other: "    ",
                    other: "│   ",
                }
            } else {
                Self {
                    root_first: "x",
                    root_other: "| ",
                    top_middle_first: "|-- ",
                    bottom_first: "`-- ",
                    bottom_other: "    ",
                    other: "|   ",
                }
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HyperlinkFormat {
    Plain,
    Link,
}
/// Trait defining styling for different components of diagnostic output.
pub trait ITheme {
    /// Style for default text.
    fn default_style(&self) -> &Option<Style>;
    /// Style for error or diagnostic descriptions.
    fn description_style(&self) -> &Option<Style>;
    /// Style for severity labels.
    fn severity_style(&self, severity: Option<Severity>) -> &Option<Style>;
    /// Style for help text: returns a tuple of `(prefix_style, message_style)`.
    fn help_style(&self) -> &(Option<Style>, Option<Style>);
    /// Style for URLs.
    fn hyperlink_style(&self) -> &(Option<Style>, HyperlinkFormat);
}
/// Default theme implementation using `owo_colors`.

#[derive(Debug, Clone)]
pub struct MischiefTheme {
    pub default_style: Option<Style>,
    pub description_style: Option<Style>,
    pub severity_advice_style: Option<Style>,
    pub severity_warning_style: Option<Style>,
    pub severity_error_style: Option<Style>,
    pub help_style: (Option<Style>, Option<Style>),
    pub hyperlink_style: (Option<Style>, HyperlinkFormat),
}

impl Default for MischiefTheme {
    fn default() -> Self {
        if supports_color::on(supports_color::Stream::Stdout).is_some() {
            Self {
                default_style: Default::default(),
                description_style: Some(Style::default()),
                severity_advice_style: Some(Style::new().green()),
                severity_warning_style: Some(Style::new().yellow()),
                severity_error_style: Default::default(),
                help_style: Default::default(),
                hyperlink_style: (
                    Some(Style::new().blue()),
                    if supports_hyperlinks::on(supports_hyperlinks::Stream::Stdout) {
                        HyperlinkFormat::Link
                    } else {
                        HyperlinkFormat::Plain
                    },
                ),
            }
        } else {
            Self {
                default_style: None,
                description_style: None,
                severity_advice_style: None,
                severity_warning_style: None,
                severity_error_style: None,
                help_style: (None, None),
                hyperlink_style: (None, HyperlinkFormat::Plain),
            }
        }
    }
}

impl ITheme for MischiefTheme {
    fn default_style(&self) -> &Option<Style> { &self.default_style }
    fn description_style(&self) -> &Option<Style> { &self.description_style }
    fn severity_style(&self, severity: Option<Severity>) -> &Option<Style> {
        match severity {
            Some(Severity::Advice) => &self.severity_advice_style,
            Some(Severity::Warning) => &self.severity_warning_style,
            Some(Severity::Error) => &self.severity_error_style,
            None => &None,
        }
    }
    fn help_style(&self) -> &(Option<Style>, Option<Style>) { &self.help_style }
    fn hyperlink_style(&self) -> &(Option<Style>, HyperlinkFormat) { &self.hyperlink_style }
}

pub struct RenderBundle<'a, I, T> {
    pub report: &'a Report,

    pub theme: T,
    pub indent: I,

    pub width: usize,
}

impl<I: IIndent, T: ITheme> fmt::Display for RenderBundle<'_, I, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tree = Tree::new(self.report.inner.render_text(&self.theme));
        let mut source = &self.report.inner.source;
        while let Some(e) = source {
            tree.push(e.render_text(&self.theme));
            source = &e.source
        }
        let render = Render {
            tree: &tree,
            indent: self.indent.clone(),

            width: self.width,
        };
        write!(f, "{}", render)
    }
}
