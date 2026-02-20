extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use core::fmt::{Display, Write};

use owo_colors::OwoColorize;

use crate::render::{HyperlinkFormat, ITheme};

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
/// Configuration for terminal capabilities.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TerminalConfig {
    width: Option<usize>,
    support_color: bool,
    support_hyperlinks: bool,
    supports_unicode: bool,
}

impl TerminalConfig {
    /// Initializes a new `TerminalConfig` with detected terminal capabilities.
    pub fn init() -> Self {
        Self {
            width: Self::terminal_width(),
            support_color: supports_color::on(supports_color::Stream::Stdout).is_some(),
            support_hyperlinks: supports_hyperlinks::supports_hyperlinks(),
            supports_unicode: supports_unicode::supports_unicode(),
        }
    }

    /// Returns the width of the terminal in columns.
    fn terminal_width() -> Option<usize> {
        if let Some((terminal_size::Width(w), _)) = terminal_size::terminal_size() {
            Some(w as usize)
        } else {
            None
        }
    }

    /// Returns whether the terminal supports color output.
    pub fn support_color(&self) -> bool { self.support_color }

    /// Returns whether the terminal supports hyperlinks.
    pub fn support_hyperlinks(&self) -> bool { self.support_hyperlinks }

    /// Returns whether the terminal supports Unicode characters.
    pub fn supports_unicode(&self) -> bool { self.supports_unicode }
}

pub trait IShader {
    fn apply<S: Display>(
        &self,
        buffer: &mut String,
        text: S,
        style: &Option<owo_colors::Style>,
        terminal_config: &TerminalConfig,
    ) -> core::fmt::Result;
    fn apply_hyperlink<S: Display, T: ITheme>(
        &self,
        buffer: &mut String,
        hyperlink: S,
        text: S,
        theme: &T,
        terminal_config: &TerminalConfig,
    ) -> core::fmt::Result;

    fn wrap_string<T: ITheme>(
        &self,
        buffer: &str,
        terminal_config: &TerminalConfig,
        theme: &T,
        layer: Layer,
        element: Item,
    ) -> String;
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Shader;
impl IShader for Shader {
    fn apply<S: Display>(
        &self,
        buffer: &mut String,
        text: S,
        style: &Option<owo_colors::Style>,
        terminal_config: &TerminalConfig,
    ) -> core::fmt::Result {
        if terminal_config.support_color()
            && let Some(style) = style
        {
            buffer.write_str(&text.style(*style).to_string())
        } else {
            buffer.write_str(&text.to_string())
        }
    }
    fn apply_hyperlink<S: Display, T: ITheme>(
        &self,
        buffer: &mut String,
        hyperlink: S,
        text: S,
        theme: &T,
        terminal_config: &TerminalConfig,
    ) -> core::fmt::Result {
        match (
            terminal_config.support_color() && theme.hyperlink_style().0.is_some(),
            terminal_config.support_hyperlinks()
                && theme.hyperlink_style().1 == HyperlinkFormat::Link,
        ) {
            (true, true) => buffer.write_str(
                &format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", hyperlink, text)
                    .style(theme.hyperlink_style().0.unwrap())
                    .to_string(),
            ),
            (true, false) => buffer.write_str(
                &format!("<{}>", hyperlink)
                    .style(theme.hyperlink_style().0.unwrap())
                    .to_string(),
            ),
            (false, true) => buffer.write_str(&format!(
                "\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\",
                hyperlink, text
            )),
            (false, false) => buffer.write_str(&format!("<{}>", hyperlink)),
        }
    }
    fn wrap_string<T: ITheme>(
        &self,
        buffer: &str,
        terminal_config: &TerminalConfig,
        theme: &T,
        layer: Layer,
        element: Item,
    ) -> String {
        let (indent, sub_indent): (String, String) = if terminal_config.support_color() {
            let (indent, sub_indent) = theme.get_indent(layer, element);
            // let indent_theme = theme.indent_theme().clone();
            match theme.indent_style() {
                Some(indent_color) => {
                    let indent = indent.style(indent_color).to_string();
                    let sub_indent = sub_indent.style(indent_color).to_string();
                    (indent, sub_indent)
                }
                None => (indent.to_string(), sub_indent.to_string()),
            }
        } else {
            let (indent, sub_indent) = theme.get_indent(layer, element);
            (indent.to_string(), sub_indent.to_string())
        };
        let width = match (terminal_config.width, theme.width()) {
            (None, None) => 80,
            (None, Some(w)) => w,
            (Some(w), None) => w,
            (Some(_), Some(w)) => w,
        };
        let opt = textwrap::Options::new(width)
            .initial_indent(&indent)
            .subsequent_indent(&sub_indent);
        textwrap::fill(buffer, &opt)
    }
}
