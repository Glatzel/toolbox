use crate::IDiagnostic;
extern crate alloc;
use core::fmt::Debug;
pub struct Render<'a, T>
where
    T: IDiagnostic,
{
    diagnostic: &'a T,
}
impl<'a, T> Render<'a, T>
where
    T: IDiagnostic,
{
    pub fn new(diagnostic: &'a T) -> Self { Self { diagnostic } }
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
        self.render_fancy(f, TerminalConfig::init())?;
        Ok(())
    }
}
impl<'a, T> Render<'a, T>
where
    T: IDiagnostic,
{
    #[cfg(feature = "fancy")]
    fn render_fancy(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        terminal_config: TerminalConfig,
    ) -> core::fmt::Result {
        use alloc::string::{String, ToString};
        use core::fmt::Write;

        use owo_colors::OwoColorize;

        let chain: alloc::vec::Vec<&dyn IDiagnostic> = self.chain().collect();
        let width = terminal_config.width.unwrap_or(80);
        let mut buffer = String::new();

        for (i, diagnostic) in chain.iter().enumerate() {
            buffer.clear();
            // --- Prefixes ---
            let (prefix, sub_prefix) = if terminal_config.support_color {
                if i == 0 {
                    ("x".red().to_string(), "│ ".red().to_string())
                } else if i == chain.len() - 1 {
                    ("╰─▶".red().to_string(), "     ".red().to_string())
                } else {
                    ("├─▶".red().to_string(), "│    ".red().to_string())
                }
            } else if i == 0 {
                ("x".to_string(), "│  ".to_string())
            } else if i == chain.len() - 1 {
                ("╰─▶".to_string(), "     ".to_string())
            } else {
                ("├─▶".to_string(), "│    ".to_string())
            };

            write!(f, "{} ", prefix)?;

            // --- Severity + Code  ---
            if let Some(sev) = diagnostic.severity() {
                write!(buffer, "{:?}", sev)?
            };

            if let Some(code) = diagnostic.code() {
                if terminal_config.support_color {
                    let code_colored = match diagnostic.severity() {
                        Some(crate::Severity::Warning) => code.red().to_string(),
                        Some(crate::Severity::Advice) => code.yellow().to_string(),
                        _ => code.green().to_string(),
                    };
                    write!(buffer, "<{}>", code_colored)?;
                } else {
                    write!(buffer, "<{}>", code)?;
                }
            }

            // --- URL ---
            if let Some(url) = diagnostic.url() {
                let mut link = String::new();
                if terminal_config.support_hyperlinks {
                    write!(link, "\x1b]8;;{}\x1b\\link\x1b]8;;\x1b\\", url)?;
                } else {
                    write!(link, "{}", url)?;
                }

                if terminal_config.support_color {
                    write!(buffer, "({})", link.blue())?;
                } else {
                    write!(buffer, "({})", link)?;
                }
            }

            if diagnostic.severity().is_some()
                || diagnostic.code().is_some()
                || diagnostic.url().is_some()
            {
                write!(buffer, ": ")?;
            }

            // --- Description with wrapping ---
            write!(buffer, "{}", diagnostic.description())?;
            write!(
                f,
                "{}",
                textwrap::fill(
                    &buffer,
                    textwrap::Options::new(width).subsequent_indent(&sub_prefix)
                )
            )?;

            // --- Help line ---
            if let Some(help) = diagnostic.help() {
                buffer.clear();
                write!(buffer, "    {}: {}", "help".cyan(), help.blue())?;
                write!(
                    f,
                    "\n{}",
                    textwrap::fill(
                        &buffer,
                        textwrap::Options::new(width).subsequent_indent(&sub_prefix)
                    )
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }

    #[cfg(not(feature = "fancy"))]
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

#[cfg(feature = "fancy")]
struct TerminalConfig {
    width: Option<usize>,
    support_color: bool,
    support_hyperlinks: bool,
    _supports_unicode: bool,
}

#[cfg(feature = "fancy")]
impl TerminalConfig {
    pub fn init() -> Self {
        Self {
            width: Self::get_terminal_width(),
            support_color: supports_color::on(supports_color::Stream::Stdout).is_some(),
            support_hyperlinks: supports_hyperlinks::supports_hyperlinks(),
            _supports_unicode: supports_unicode::supports_unicode(),
        }
    }
    fn get_terminal_width() -> Option<usize> {
        if let Some((terminal_size::Width(w), _)) = terminal_size::terminal_size() {
            Some(w as usize)
        } else {
            None
        }
    }
}
