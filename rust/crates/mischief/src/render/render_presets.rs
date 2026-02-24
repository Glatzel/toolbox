use crate::IDiagnostic;
use crate::render_protocol::*;
extern crate alloc;
#[cfg(feature = "fancy")]
use alloc::format;
use alloc::string::String;
#[cfg(feature = "fancy")]
use alloc::string::ToString;
#[cfg(feature = "fancy")]
use core::fmt::Display;
use core::fmt::Write;

#[cfg(feature = "fancy")]
use owo_colors::{OwoColorize, Style};

#[cfg(feature = "fancy")]
use crate::Severity;
/// Trait for computing indentation strings based on layer layers and elements.
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

/// Default theme implementation using `owo_colors`.
#[cfg(feature = "fancy")]
#[derive(Debug, Clone, PartialEq)]
pub struct DefaultStyle;
#[cfg(feature = "fancy")]
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
#[cfg(feature = "fancy")]
#[derive(Debug, Clone, PartialEq)]
pub struct NoColorStyle;
#[cfg(feature = "fancy")]
impl IStyle for NoColorStyle {
    fn default_style(&self) -> Option<Style> { None }
    fn indent_style(&self) -> Option<Style> { None }
    fn description_style(&self) -> Option<Style> { None }
    fn severity_style(&self, _severity: Option<Severity>) -> Option<Style> { None }
    fn help_style(&self) -> (Option<Style>, Option<Style>) { (None, None) }
    fn hyperlink_style(&self) -> (Option<Style>, HyperlinkFormat) { (None, HyperlinkFormat::Link) }
}

/// Default theme implementation using `owo_colors`.
#[cfg(feature = "fancy")]
#[derive(Debug, Clone, PartialEq)]
pub struct DefaultTheme;
#[cfg(feature = "fancy")]
impl ITheme for DefaultTheme {
    fn width(&self) -> Option<usize> { None }
}
#[cfg(feature = "fancy")]
impl IIndent for DefaultTheme {
    fn get_indent(&self, layer: Layer, element: Item) -> (&'static str, &'static str) {
        DefaultIndent.get_indent(layer, element)
    }
}
#[cfg(feature = "fancy")]
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

/// Configuration for terminal capabilities.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TerminalConfig {
    width: Option<usize>,
    support_color: bool,
    support_hyperlinks: bool,
    supports_unicode: bool,
}
#[cfg(feature = "fancy")]
impl Default for TerminalConfig {
    /// Initializes a new `TerminalConfig` with detected terminal capabilities.
    fn default() -> Self {
        Self {
            width: if let Some((terminal_size::Width(w), _)) = terminal_size::terminal_size() {
                Some(w as usize)
            } else {
                None
            },
            support_color: supports_color::on(supports_color::Stream::Stdout).is_some(),
            support_hyperlinks: supports_hyperlinks::supports_hyperlinks(),
            supports_unicode: supports_unicode::supports_unicode(),
        }
    }
}
impl ITerminalConfig for TerminalConfig {
    /// Returns the width of the terminal in columns.
    fn terminal_width(&self) -> Option<usize> { self.width }

    /// Returns whether the terminal supports color output.
    fn support_color(&self) -> bool { self.support_color }

    /// Returns whether the terminal supports hyperlinks.
    fn support_hyperlinks(&self) -> bool { self.support_hyperlinks }

    /// Returns whether the terminal supports Unicode characters.
    fn supports_unicode(&self) -> bool { self.supports_unicode }
}
#[cfg(feature = "fancy")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DefaultShader;
#[cfg(feature = "fancy")]
impl IShader for DefaultShader {
    fn apply<D: Display, TC: ITerminalConfig>(
        &self,
        buffer: &mut String,
        text: D,
        style: &Option<owo_colors::Style>,
        terminal_config: &TC,
    ) -> core::fmt::Result {
        if terminal_config.support_color()
            && let Some(style) = style
        {
            buffer.write_str(&text.style(*style).to_string())
        } else {
            buffer.write_str(&text.to_string())
        }
    }
    fn apply_hyperlink<D: Display, T: ITheme, TC: ITerminalConfig>(
        &self,
        buffer: &mut String,
        hyperlink: D,
        text: D,
        theme: &T,
        terminal_config: &TC,
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
    fn wrap_string<T: ITheme, TC: ITerminalConfig>(
        &self,
        buffer: &str,
        terminal_config: &TC,
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
        let width = match (terminal_config.terminal_width(), theme.width()) {
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

pub struct DefaultRender;
impl DefaultRender {
    /// Produces an iterator over the diagnostic chain.
    fn chain(diagnostic: &impl IDiagnostic) -> impl Iterator<Item = &dyn IDiagnostic> {
        core::iter::successors(Some(diagnostic as &dyn IDiagnostic), |r| r.source())
    }
    fn render_plain(&self, text: &mut String, diagnostic: &impl IDiagnostic) -> core::fmt::Result {
        let mut chain = Self::chain(diagnostic);

        if let Some(first) = chain.next() {
            writeln!(text, "Error: {}", first.description())?;
        }
        let mut first = true;
        for diagnostic in chain {
            if first {
                writeln!(text, "\nCaused by:")?;
                first = false;
            }
            writeln!(text, "    {}", diagnostic.description())?;
        }
        Ok(())
    }
}
impl IRender for DefaultRender {
    fn render(&self, text: &mut String, diagnostic: &impl IDiagnostic) -> core::fmt::Result {
        self.render_plain(text, diagnostic)
    }
}
/// Wrapper struct to render diagnostics.
#[cfg(feature = "fancy")]
pub struct DefaultFancyRender<S: IShader, T: ITheme, TC: ITerminalConfig> {
    shader: S,
    terminal_config: TC,
    theme: T,
}
#[cfg(feature = "fancy")]
impl<S: IShader, T: ITheme, TC: ITerminalConfig> DefaultFancyRender<S, T, TC> {
    /// Creates a new render wrapper for a diagnostic.
    pub fn new(shader: S, theme: T, terminal_config: TC) -> Self {
        Self {
            shader,
            theme,
            terminal_config,
        }
    }

    fn render_fancy(&self, text: &mut String, diagnostic: &impl IDiagnostic) -> core::fmt::Result {
        let mut buffer = String::new();
        let mut chain = DefaultRender::chain(diagnostic).peekable();
        let mut layer = Layer::Bottom;

        while let Some(diagnostic) = chain.next() {
            if chain.peek().is_none() {
                layer = Layer::Top;
            }
            buffer.clear();

            let severity_color = self.theme.severity_style(diagnostic.severity());
            if let Some(s) = diagnostic.severity() {
                self.shader
                    .apply(&mut buffer, s, &severity_color, &self.terminal_config)?
            }

            if let Some(s) = diagnostic.code() {
                self.shader.apply(
                    &mut buffer,
                    format!("[{}]", s),
                    &severity_color,
                    &self.terminal_config,
                )?
            }
            if let Some(s) = diagnostic.url() {
                self.shader.apply_hyperlink(
                    &mut buffer,
                    s,
                    "(link)",
                    &self.theme,
                    &self.terminal_config,
                )?
            }
            if diagnostic.severity().is_some()
                || diagnostic.code().is_some()
                || diagnostic.url().is_some()
            {
                buffer.write_str(": ")?;
            }

            self.shader.apply(
                &mut buffer,
                diagnostic.description(),
                &self.theme.description_style(),
                &self.terminal_config,
            )?;
            buffer = self.shader.wrap_string(
                &buffer,
                &self.terminal_config,
                &self.theme,
                layer,
                Item::First,
            );
            text.write_str(&buffer)?;
            buffer.clear();

            if let Some(s) = diagnostic.help() {
                writeln!(text)?;
                let help_theme = self.theme.help_style();
                self.shader
                    .apply(&mut buffer, "help: ", &help_theme.0, &self.terminal_config)?;
                self.shader
                    .apply(&mut buffer, s, &help_theme.1, &self.terminal_config)?;
                buffer = self.shader.wrap_string(
                    &buffer,
                    &self.terminal_config,
                    &self.theme,
                    layer,
                    Item::Other,
                );
                text.write_str(&buffer)?;
            }

            writeln!(text)?;
            layer = Layer::Middle;
        }
        Ok(())
    }
}
#[cfg(feature = "fancy")]
impl<S: IShader, T: ITheme, TC: ITerminalConfig> IRender for DefaultFancyRender<S, T, TC> {
    fn render(&self, text: &mut String, diagnostic: &impl IDiagnostic) -> core::fmt::Result {
        if self.terminal_config.supports_unicode() {
            self.render_fancy(text, diagnostic)
        } else {
            DefaultRender.render_plain(text, diagnostic)
        }
    }
}
