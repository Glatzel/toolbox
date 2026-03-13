use arbor::protocol::{IIndent, Layer, Line};
#[cfg(feature = "fancy")]
use owo_colors::Style;

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
    #[cfg(not(feature = "fancy"))]
    fn default() -> Self {
        Self {
            root_first: "",
            root_other: "",
            top_middle_first: "    ",
            bottom_first: "    ",
            bottom_other: "    ",
            other: "    ",
        }
    }
    #[cfg(feature = "fancy")]
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

/// Default theme implementation using `owo_colors`.
#[cfg(feature = "fancy")]
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
