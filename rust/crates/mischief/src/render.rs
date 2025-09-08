use crate::IDiagnostic;
extern crate alloc;
use core::fmt::Debug;
#[cfg(feature = "fancy")]
mod indent;
#[cfg(feature = "fancy")]
mod layer;
#[cfg(feature = "fancy")]
mod shader;
#[cfg(feature = "fancy")]
mod terminal_config;
#[cfg(feature = "fancy")]
mod theme;
#[cfg(feature = "fancy")]
use indent::Indent;
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
    pub fn new(diagnostic: &'a T) -> Self {#[cfg(feature = "fancy")]
        let terminal_config = TerminalConfig::init();
        Self {
            diagnostic,#[cfg(feature = "fancy")]
            indent: Indent,#[cfg(feature = "fancy")]
            shader: Shader,#[cfg(feature = "fancy")]
            terminal_config,#[cfg(feature = "fancy")]
            theme: Theme::default(),
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
        use alloc::string::String;

        use crate::render::layer::Layer;

        let mut buffer = String::new(); // Reuse a single buffer

        let mut chain = self.chain().peekable();
        let mut node: Layer = Layer::Bottom;

        while let Some(diagnostic) = chain.next() {
            if chain.peek().is_none() {
                node = Layer::Top;
            }
            buffer.clear();
            diagnostic.severity().map(|s| {
                self.shader.apply(
                    &mut buffer,
                    s,
                    &self.theme.severity_theme(),
                    &self.terminal_config,
                )
            });
            diagnostic.code().map(|s| {
                self.shader.apply(
                    &mut buffer,
                    s,
                    &self.theme.code_theme(diagnostic.severity()),
                    &self.terminal_config,
                )
            });
            diagnostic.url().map(|s| {
                self.shader.apply(
                    &mut buffer,
                    s,
                    &self.theme.url_theme(),
                    &self.terminal_config,
                )
            });

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
            );
            write!(f, "{}", buffer)?;
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
