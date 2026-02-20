use crate::IDiagnostic;
extern crate alloc;
use alloc::string::String;
use core::fmt::Write;
#[cfg(feature = "fancy")]
mod shader;
#[cfg(feature = "fancy")]
mod theme;
#[cfg(feature = "fancy")]
use alloc::format;

#[cfg(feature = "fancy")]
pub use shader::*;
#[cfg(feature = "fancy")]
pub use theme::*;

/// Trait defining rendering behavior for diagnostic types.
pub trait IRender {
    fn render(&self, text: &mut String, diagnostic: &impl IDiagnostic) -> core::fmt::Result;
}

/// Wrapper struct to render diagnostics.
#[cfg(feature = "fancy")]
pub struct Render<T: ITheme> {
    #[cfg(feature = "fancy")]
    shader: Shader,
    #[cfg(feature = "fancy")]
    terminal_config: TerminalConfig,
    #[cfg(feature = "fancy")]
    theme: T,
}
#[cfg(not(feature = "fancy"))]
pub struct Render;
#[cfg(feature = "fancy")]
impl<T: ITheme> Render<T> {
    /// Creates a new render wrapper for a diagnostic.
    pub fn new(#[cfg(feature = "fancy")] theme: T) -> Self {
        #[cfg(feature = "fancy")]
        let terminal_config = TerminalConfig::init();
        Self {
            #[cfg(feature = "fancy")]
            shader: Shader,
            #[cfg(feature = "fancy")]
            terminal_config,
            #[cfg(feature = "fancy")]
            theme,
        }
    }
    #[cfg(feature = "fancy")]
    fn render_fancy(&self, text: &mut String, diagnostic: &impl IDiagnostic) -> core::fmt::Result {
        let mut buffer = String::new();
        let mut chain = Self::chain(diagnostic).peekable();
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
#[cfg(not(feature = "fancy"))]
impl Render {
    pub fn new() -> Self { Self }
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
#[cfg(feature = "fancy")]
impl<T: ITheme> IRender for Render<T> {
    fn render(&self, text: &mut String, diagnostic: &impl IDiagnostic) -> core::fmt::Result {
        if self.terminal_config.supports_unicode() {
            self.render_fancy(text, diagnostic)
        } else {
            self.render_plain(text, diagnostic)
        }
    }
}
#[cfg(not(feature = "fancy"))]
impl IRender for Render<T> {
    fn render(&self, text: &mut String, diagnostic: &impl IDiagnostic) -> core::fmt::Result {
        self.render_plain(text, diagnostic)
    }
}
