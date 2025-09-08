extern crate alloc;
use alloc::string::{String, ToString};
use core::fmt::{Display, Write};

use owo_colors::OwoColorize;

use crate::render::indent::IIndent;
use crate::render::layer;
use crate::render::terminal_config::TerminalConfig;
use crate::render::theme::ITheme;

pub trait IShader {
    fn apply<T>(
        &self,
        buffer: &mut String,
        s: T,
        style: &owo_colors::Style,
        terminal_config: &TerminalConfig,
    ) where
        T: Display;
    fn write_wrapped<INDENT, THEME>(
        &self,
        buffer: &str,
        terminal_config: &TerminalConfig,
        theme: &THEME,
        indent: &INDENT,
        node: &layer::Layer,
    ) -> String
    where
        INDENT: IIndent,
        THEME: ITheme;
}

pub struct Shader;
impl IShader for Shader {
    fn apply<T>(
        &self,
        buffer: &mut String,
        s: T,
        style: &owo_colors::Style,
        terminal_config: &TerminalConfig,
    ) where
        T: Display,
    {
        if terminal_config.support_color() {
            buffer.write_str(&s.style(*style).to_string()).unwrap();
        } else {
            buffer.write_str(&s.to_string()).unwrap();
        }
    }
    fn write_wrapped<INDENT, THEME>(
        &self,
        buffer: &str,
        terminal_config: &TerminalConfig,
        theme: &THEME,
        indent: &INDENT,
        node: &layer::Layer,
    ) -> String
    where
        INDENT: IIndent,
        THEME: ITheme,
    {
        let (indent, sub_indent): (String, String) = if terminal_config.support_color() {
            let (indent, sub_indent) = indent.get(node);
            // let indent_theme = theme.indent_theme().clone();
            let indent = indent.style(theme.indent_theme()).to_string();
            let sub_indent = sub_indent.style(theme.indent_theme()).to_string();

            (indent, sub_indent)
        } else {
            let (indent, sub_indent) = indent.get(node);
            (indent.to_string(), sub_indent.to_string())
        };
        let opt = textwrap::Options::new(terminal_config.width())
            .initial_indent(&indent)
            .subsequent_indent(&sub_indent);
        textwrap::fill(buffer, &opt)
    }
}
