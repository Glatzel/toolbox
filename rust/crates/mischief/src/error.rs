extern crate alloc;
use alloc::boxed::Box;
#[cfg(feature = "fancy")]
use alloc::format;
use alloc::string::{String, ToString};
use core::fmt::Display;
#[cfg(feature = "fancy")]
use core::fmt::Write;

#[cfg(feature = "fancy")]
use owo_colors::{OwoColorize, Style};

use crate::IDiagnostic;
#[cfg(feature = "fancy")]
use crate::presets::HyperlinkFormat;
#[cfg(feature = "fancy")]
use crate::presets::ITheme;

/// Represents a structured error with optional metadata such as source, code,
/// severity, help message, and URL.
#[derive(Clone)]
pub struct MischiefError {
    description: String,
    pub(crate) source: Option<Box<MischiefError>>,
    code: Option<String>,
    severity: Option<crate::Severity>,
    help: Option<String>,
    url: Option<String>,
}

impl MischiefError {
    /// Constructs a new `MischiefError` with optional metadata.
    ///
    /// All fields that implement `Display` can be passed as `description`,
    /// `code`, `help`, and `url`. `source` is an optional boxed inner error.
    pub fn new<D>(
        description: D,
        source: Option<Box<MischiefError>>,
        code: Option<D>,
        severity: Option<crate::Severity>,
        help: Option<D>,
        url: Option<D>,
    ) -> Self
    where
        D: Display,
    {
        Self {
            description: description.to_string(),
            source,
            code: code.map(|s| s.to_string()),
            severity,
            help: help.map(|s| s.to_string()),
            url: url.map(|s| s.to_string()),
        }
    }
    /// Returns the description as an optional string slice.
    pub fn description(&self) -> Option<&str> { Some(&self.description) }
    /// Returns the source error, if any.
    pub fn source(&self) -> Option<&MischiefError> { self.source.as_deref() }
    #[cfg(not(feature = "fancy"))]
    pub fn render_text(&self) -> String { self.description().unwrap_or_default().to_string() }
    #[cfg(feature = "fancy")]
    pub fn render_text(&self, theme: &impl ITheme) -> String {
        use core::fmt::Write;
        let mut buffer = String::new();
        let severity_color = *theme.severity_style(self.severity());
        if let Some(s) = self.severity() {
            self.apply_style(&mut buffer, &s.to_string(), &severity_color)
                .unwrap()
        }
        if let Some(s) = self.code() {
            self.apply_style(&mut buffer, &format!("[{}]", s), &severity_color)
                .unwrap()
        }
        if let Some(s) = self.url() {
            self.apply_hyperlink_style(&mut buffer, s, "(link)", theme.hyperlink_style())
                .unwrap();
        }
        if self.severity().is_some() || self.code().is_some() || self.url().is_some() {
            buffer.write_str(": ").unwrap();
        }
        if let Some(s) = self.description() {
            self.apply_style(&mut buffer, s, theme.description_style())
                .unwrap();
        }

        if let Some(s) = self.help() {
            writeln!(buffer).unwrap();
            self.apply_style(&mut buffer, "help: ", &theme.help_style().0)
                .unwrap();
            self.apply_style(&mut buffer, s, &theme.help_style().1)
                .unwrap();
        }
        buffer
    }
    #[cfg(feature = "fancy")]
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
    #[cfg(feature = "fancy")]
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

impl IDiagnostic for MischiefError {
    fn description(&self) -> &str { &self.description }
    fn source(&self) -> Option<&dyn IDiagnostic> { self.source().map(|f| f as &dyn IDiagnostic) }
    fn code(&self) -> Option<&str> { self.code.as_deref() }
    fn severity(&self) -> Option<crate::Severity> { self.severity }
    fn help(&self) -> Option<&str> { self.help.as_deref() }
    fn url(&self) -> Option<&str> { self.url.as_deref() }
}
