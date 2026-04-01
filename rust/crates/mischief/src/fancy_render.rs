use core::fmt::{self, Write};
extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};

use arbor::protocol::{IIndent, Layer, Line};
use arbor::renders::OwnedRender;
use arbor::trees::OwnedTree;
use owo_colors::{OwoColorize, Style};

use crate::{IDiagnostic, Report, Severity};

/// Indentation configuration used when rendering diagnostic trees.
///
/// `MischiefIndent` defines the textual prefixes used to render the
/// hierarchical structure of diagnostics. The indentation strings are
/// selected depending on the node's [`Layer`] within the tree and
/// whether the line is the first or a continuation.
///
/// The default configuration automatically selects Unicode or ASCII
/// characters depending on terminal capabilities.
#[derive(Debug, Clone)]
pub struct MischiefIndent {
    /// Prefix used for the first line of the root diagnostic.
    pub root_first: &'static str,

    /// Prefix used for continuation lines of the root diagnostic.
    pub root_other: &'static str,

    /// Prefix used for the first line of top or middle child diagnostics.
    pub top_middle_first: &'static str,

    /// Prefix used for the first line of the final child diagnostic.
    pub bottom_first: &'static str,

    /// Prefix used for continuation lines of the final child diagnostic.
    pub bottom_other: &'static str,

    /// Prefix used for continuation lines of intermediate diagnostics.
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
    /// Creates a default indentation configuration.
    ///
    /// The implementation automatically chooses Unicode tree glyphs
    /// when the terminal supports Unicode output. Otherwise, an ASCII
    /// fallback representation is used.
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

/// Describes how hyperlinks should be rendered in diagnostic output.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HyperlinkFormat {
    /// Render the hyperlink as plain text.
    Plain,

    /// Render the hyperlink using terminal hyperlink escape sequences.
    Link,
}

/// Trait defining styling behavior for rendered diagnostics.
///
/// Implementations of this trait provide styling information for the
/// various components of a diagnostic report. Styling is typically
/// applied through terminal color libraries such as `owo_colors`,
/// but the abstraction allows custom rendering strategies.
///
/// The theme is responsible only for styling decisions; the rendering
/// logic itself is handled separately.
pub trait ITheme {
    /// Returns the style applied to general text.
    fn default_style(&self) -> &Option<Style>;

    /// Returns the style used for diagnostic descriptions.
    fn description_style(&self) -> &Option<Style>;

    /// Returns the style applied to severity labels.
    fn severity_style(&self, severity: Option<Severity>) -> &Option<Style>;

    /// Returns the styles used for help messages.
    ///
    /// The tuple represents `(prefix_style, message_style)`.
    fn help_style(&self) -> &(Option<Style>, Option<Style>);

    /// Returns the style and rendering format for hyperlinks.
    fn hyperlink_style(&self) -> &(Option<Style>, HyperlinkFormat);
}

/// Default styling implementation for diagnostic output.
///
/// `MischiefTheme` defines the visual appearance of rendered diagnostics,
/// including colors for severity labels, help messages, and hyperlinks.
/// Styling is implemented using the `owo_colors` crate.
///
/// The default configuration automatically adapts to terminal capabilities,
/// enabling color and hyperlink support only when supported by the output
/// stream.
#[derive(Debug, Clone)]
pub struct MischiefTheme {
    /// Style applied to general text.
    pub default_style: Option<Style>,

    /// Style applied to diagnostic descriptions.
    pub description_style: Option<Style>,

    /// Style used for diagnostics with [`Severity::Advice`].
    pub severity_advice_style: Option<Style>,

    /// Style used for diagnostics with [`Severity::Warning`].
    pub severity_warning_style: Option<Style>,

    /// Style used for diagnostics with [`Severity::Error`].
    pub severity_error_style: Option<Style>,

    /// Styles applied to help messages `(prefix, message)`.
    pub help_style: (Option<Style>, Option<Style>),

    /// Style and format used when rendering hyperlinks.
    pub hyperlink_style: (Option<Style>, HyperlinkFormat),
}

impl Default for MischiefTheme {
    /// Creates a theme configured according to terminal capabilities.
    ///
    /// Color and hyperlink support are enabled only when the terminal
    /// reports compatibility.
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

/// Rendering context used to format a [`Report`] as a diagnostic tree.
///
/// `RenderBundle` bundles together the configuration required to render
/// a diagnostic report, including the theme, indentation strategy,
/// and wrapping width.
///
/// The renderer converts a diagnostic chain into a hierarchical tree
/// representation using the `arbor` crate.
pub struct RenderBundle<'a, I, T> {
    /// The report being rendered.
    pub report: &'a Report,

    /// Theme used for styling output.
    pub theme: T,

    /// Indentation strategy used for the diagnostic tree.
    pub indent: I,

    /// Maximum line width used during rendering.
    pub width: usize,
}

impl<I: IIndent, T: ITheme> RenderBundle<'_, I, T> {
    /// Formats a single diagnostic entry as a styled string.
    ///
    /// The output may include severity labels, error codes, hyperlinks,
    /// descriptions, and optional help messages depending on the
    /// metadata provided by the diagnostic.
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

    /// Applies a text style to a string and writes it into the buffer.
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

    /// Writes a hyperlink into the buffer using the configured format.
    ///
    /// Depending on the theme configuration, hyperlinks may be rendered
    /// as terminal hyperlinks or plain text representations.
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
    /// Renders the entire diagnostic report as a formatted tree.
    ///
    /// Each diagnostic in the causal chain is converted into a tree
    /// node and rendered using the configured indentation and theme.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tree = OwnedTree::new(self.render_diagnostic(&self.report.inner, &self.theme));

        let mut source = &self.report.inner.source;
        while let Some(e) = source {
            tree.push(self.render_diagnostic(e.as_ref(), &self.theme));
            source = &e.source
        }

        let render = OwnedRender {
            tree: &tree,
            indent: self.indent.clone(),
            width: self.width,
        };

        write!(f, "{}", render)
    }
}
