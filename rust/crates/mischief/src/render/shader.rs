extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use core::fmt::{Display, Write};

use owo_colors::OwoColorize;

use crate::render::indent::IIndent;
use crate::render::position;
use crate::render::terminal_config::TerminalConfig;
use crate::render::theme::ITheme;
pub trait IShader {
    fn apply<T>(
        &self,
        buffer: &mut String,
        s: T,
        style: &owo_colors::Style,
        terminal_config: &TerminalConfig,
    ) -> core::fmt::Result
    where
        T: Display;
    fn apply_hyperlink<T>(
        &self,
        buffer: &mut String,
        hyperlink: T,
        text: T,
        style: &owo_colors::Style,
        terminal_config: &TerminalConfig,
    ) -> core::fmt::Result
    where
        T: Display;
    fn write_wrapped<INDENT, THEME>(
        &self,
        buffer: &str,
        terminal_config: &TerminalConfig,
        theme: &THEME,
        indent: &INDENT,
        node: position::Layer,
        element: position::Item,
    ) -> String
    where
        INDENT: IIndent,
        THEME: ITheme;
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Shader;
impl IShader for Shader {
    fn apply<T>(
        &self,
        buffer: &mut String,
        s: T,
        style: &owo_colors::Style,
        terminal_config: &TerminalConfig,
    ) -> core::fmt::Result
    where
        T: Display,
    {
        if terminal_config.support_color() {
            buffer.write_str(&s.style(*style).to_string())
        } else {
            buffer.write_str(&s.to_string())
        }
    }
    fn apply_hyperlink<T>(
        &self,
        buffer: &mut String,
        hyperlink: T,
        text: T,
        style: &owo_colors::Style,
        terminal_config: &TerminalConfig,
    ) -> core::fmt::Result
    where
        T: Display,
    {
        match (
            terminal_config.support_color(),
            terminal_config.support_hyperlinks(),
        ) {
            (true, true) => buffer.write_str(
                &format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", hyperlink, text)
                    .style(*style)
                    .to_string(),
            ),
            (true, false) => buffer.write_str(&format!("{}", hyperlink).style(*style).to_string()),
            (false, true) => buffer.write_str(&format!(
                "\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\",
                hyperlink, text
            )),
            (false, false) => Ok(()),
        }
    }
    fn write_wrapped<INDENT, THEME>(
        &self,
        buffer: &str,
        terminal_config: &TerminalConfig,
        theme: &THEME,
        indent: &INDENT,
        layer: position::Layer,
        element: position::Item,
    ) -> String
    where
        INDENT: IIndent,
        THEME: ITheme,
    {
        let (indent, sub_indent): (String, String) = if terminal_config.support_color() {
            let (indent, sub_indent) = indent.get(layer, element);
            // let indent_theme = theme.indent_theme().clone();
            let indent = indent.style(theme.indent_theme()).to_string();
            let sub_indent = sub_indent.style(theme.indent_theme()).to_string();

            (indent, sub_indent)
        } else {
            let (indent, sub_indent) = indent.get(layer, element);
            (indent.to_string(), sub_indent.to_string())
        };
        let opt = textwrap::Options::new(terminal_config.width())
            .initial_indent(&indent)
            .subsequent_indent(&sub_indent);
        textwrap::fill(buffer, &opt)
    }
}
