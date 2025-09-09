use crate::IDiagnostic;
extern crate alloc;
use core::fmt::Debug;
#[cfg(feature = "fancy")]
mod indent;
#[cfg(feature = "fancy")]
mod position;
#[cfg(feature = "fancy")]
mod shader;
#[cfg(feature = "fancy")]
mod terminal_config;
#[cfg(feature = "fancy")]
mod theme;
#[cfg(feature = "fancy")]
use alloc::format;
#[cfg(feature = "fancy")]
use alloc::string::String;
#[cfg(feature = "fancy")]
use core::fmt::Write;

#[cfg(feature = "fancy")]
use indent::Indent;
#[cfg(feature = "fancy")]
use position::Layer;
#[cfg(feature = "fancy")]
use shader::{IShader, Shader};
#[cfg(feature = "fancy")]
use terminal_config::TerminalConfig;
#[cfg(feature = "fancy")]
use theme::{ITheme, Theme};
pub trait IRender: Debug {
    #[cfg(feature = "fancy")]
    fn render_fancy(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result;
    fn render_plain(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result;
}

pub struct Render<'a, T>
where
    T: IDiagnostic,
{
    diagnostic: &'a T,
    #[cfg(feature = "fancy")]
    indent: Indent,
    #[cfg(feature = "fancy")]
    shader: Shader,
    #[cfg(feature = "fancy")]
    terminal_config: TerminalConfig,
    #[cfg(feature = "fancy")]
    theme: Theme,
}
impl<'a, T> Render<'a, T>
where
    T: IDiagnostic,
{
    pub fn new(diagnostic: &'a T) -> Self {
        #[cfg(feature = "fancy")]
        let terminal_config = TerminalConfig::init();
        Self {
            diagnostic,
            #[cfg(feature = "fancy")]
            indent: Indent,
            #[cfg(feature = "fancy")]
            shader: Shader,
            #[cfg(feature = "fancy")]
            terminal_config,
            #[cfg(feature = "fancy")]
            theme: Theme,
        }
    }
    fn chain(&self) -> impl Iterator<Item = &dyn IDiagnostic> {
        core::iter::successors(Some(self.diagnostic as &dyn IDiagnostic), |r| r.source())
    }
}
impl<'a, T> Debug for Render<'a, T>
where
    T: IDiagnostic,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        #[cfg(not(feature = "fancy"))]
        self.render_plain(f)?;

        #[cfg(feature = "fancy")]
        {
            if self.terminal_config.supports_unicode() {
                self.render_fancy(f)?
            } else {
                self.render_plain(f)?
            }
        }

        Ok(())
    }
}

impl<'a, T> IRender for Render<'a, T>
where
    T: IDiagnostic,
{
    #[cfg(feature = "fancy")]
    fn render_fancy(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut buffer = String::new(); // Reuse a single buffer

        let mut chain = self.chain().peekable();
        let mut node: Layer = Layer::Bottom;

        while let Some(diagnostic) = chain.next() {
            if chain.peek().is_none() {
                node = Layer::Top;
            }
            buffer.clear();

            let severity_theme = self.theme.severity_theme(diagnostic.severity());
            diagnostic.severity().map(|s| {
                self.shader
                    .apply(&mut buffer, s, &severity_theme, &self.terminal_config)
            });
            diagnostic.code().map(|s| {
                self.shader.apply(
                    &mut buffer,
                    format!("[{}]", s),
                    &severity_theme,
                    &self.terminal_config,
                )
            });
            diagnostic.url().map(|s| {
                self.shader.apply_hyperlink(
                    &mut buffer,
                    s,
                    "(link)",
                    &self.theme.url_theme(),
                    &self.terminal_config,
                )
            });
            if diagnostic.severity().is_some()
                || diagnostic.code().is_some()
                || diagnostic.url().is_some()
            {
                buffer.write_str(": ")?;
            }

            self.shader.apply(
                &mut buffer,
                diagnostic.description(),
                &self.theme.description_theme(),
                &self.terminal_config,
            );
            buffer = self.shader.write_wrapped(
                &buffer,
                &self.terminal_config,
                &self.theme,
                &self.indent,
                &node,
                &position::Element::First,
            );
            f.write_str(&buffer).unwrap();
            buffer.clear();

            diagnostic.help().map(|s| {
                writeln!(f).unwrap();
                let help_theme = self.theme.help_theme();
                self.shader
                    .apply(&mut buffer, "help: ", &help_theme.0, &self.terminal_config);
                self.shader
                    .apply(&mut buffer, s, &help_theme.1, &self.terminal_config);
                buffer = self.shader.write_wrapped(
                    &buffer,
                    &self.terminal_config,
                    &self.theme,
                    &self.indent,
                    &node,
                    &position::Element::Other,
                );
                f.write_str(&buffer).unwrap();
            });

            writeln!(f)?;

            node = Layer::Middle;
        }

        Ok(())
    }

    fn render_plain(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut chain = self.chain();

        // Top-level error
        if let Some(first) = chain.next() {
            writeln!(f, "Error: {}", first.description())?;
        }

        // Causes
        let mut first = true;
        for diagnostic in chain {
            if first {
                writeln!(f, "\nCaused by:")?;
                first = false;
            }
            writeln!(f, "    {}", diagnostic.description())?;
        }

        Ok(())
    }
}
