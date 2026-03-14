use core::fmt::{self, Write};
extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};

use arbor::protocol::{IIndent, Layer, Line};
use arbor::renders::Render;
use arbor::trees::Tree;
use owo_colors::{OwoColorize, Style};

use crate::{IDiagnostic, Report, Severity};

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
impl<I: IIndent, T: ITheme> RenderBundle<'_, I, T> {
    pub fn render_diagnostic<D: IDiagnostic>(&self, diagnostic: &D, theme: &impl ITheme) -> String {
        use core::fmt::Write;
        let mut buffer = String::new();
        let severity_color = *theme.severity_style(diagnostic.severity());
        if let Some(s) = diagnostic.severity() {
            self.apply_style(&mut buffer, &(s).to_string(), &severity_color)
                .unwrap()
        }
        if let Some(s) = diagnostic.code() {
            self.apply_style(&mut buffer, &format!("[{}]", s), &severity_color)
                .unwrap()
        }
        if let Some(s) = diagnostic.url() {
            self.apply_hyperlink_style(&mut buffer, s, "(link)", theme.hyperlink_style())
                .unwrap();
        }
        if diagnostic.severity().is_some()
            || diagnostic.code().is_some()
            || diagnostic.url().is_some()
        {
            buffer.write_str(": ").unwrap();
        }

        self.apply_style(
            &mut buffer,
            diagnostic.description(),
            theme.description_style(),
        )
        .unwrap();

        if let Some(s) = diagnostic.help() {
            writeln!(buffer).unwrap();
            self.apply_style(&mut buffer, "help: ", &theme.help_style().0)
                .unwrap();
            self.apply_style(&mut buffer, s, &theme.help_style().1)
                .unwrap();
        }
        buffer
    }

    pub fn apply_style(
        &self,
        buffer: &mut String,
        text: &str,
        style: &Option<owo_colors::Style>,
    ) -> core::fmt::Result {
        use core::fmt::Write;
        if let Some(style) = style {
            use owo_colors::OwoColorize;
            buffer.write_str(&text.style(*style).to_string())
        } else {
            buffer.write_str(text)
        }
    }

    fn apply_hyperlink_style(
        &self,
        buffer: &mut String,
        hyperlink: &str,
        text: &str,
        style: &(Option<Style>, HyperlinkFormat),
    ) -> core::fmt::Result {
        match style {
            (Some(s), HyperlinkFormat::Link) => buffer.write_str(
                &format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", hyperlink, text)
                    .style(*s)
                    .to_string(),
            ),
            (Some(s), HyperlinkFormat::Plain) => {
                buffer.write_str(&format!("<{}>", hyperlink.style(*s)))
            }
            (None, HyperlinkFormat::Link) => buffer.write_str(&format!(
                "\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\",
                hyperlink, text
            )),
            (None, HyperlinkFormat::Plain) => buffer.write_str(&format!("<{}>", hyperlink)),
        }
    }
}
impl<I: IIndent, T: ITheme> fmt::Display for RenderBundle<'_, I, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tree = Tree::new(self.render_diagnostic(&self.report.inner, &self.theme));
        let mut source = &self.report.inner.source;
        while let Some(e) = source {
            tree.push(self.render_diagnostic(e.as_ref(), &self.theme));
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
