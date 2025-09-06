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
        use alloc::string::String;
        use core::fmt::Write;

        use owo_colors::OwoColorize;

        let width = terminal_config.width.unwrap_or(80);
        let mut buffer = String::new(); // Reuse a single buffer

        let mut chain = self.chain().peekable();
        let mut is_first = true;

        while let Some(diagnostic) = chain.next() {
            let is_last = chain.peek().is_none();

            // 1. Determine prefixes without allocating
            let (prefix_str, sub_prefix_str) = if is_first {
                ("x", "│ ")
            } else if is_last {
                ("╰─▶", "  ") // Note: adjust spacing to your preference
            } else {
                ("├─▶", "│ ")
            };

            // 2. Write prefix (colored or not) directly to the formatter
            if terminal_config.support_color {
                write!(f, "{} ", prefix_str.red())?;
            } else {
                write!(f, "{} ", prefix_str)?;
            }

            // --- Build the main diagnostic line in the buffer ---
            buffer.clear();

            if let Some(sev) = diagnostic.severity() {
                write!(buffer, "{:?}", sev)?;
            }

            if let Some(code) = diagnostic.code() {
                if terminal_config.support_color {
                    match diagnostic.severity() {
                        Some(crate::Severity::Warning) => write!(buffer, "<{}>", code.yellow()),
                        Some(crate::Severity::Advice) => write!(buffer, "<{}>", code.cyan()),
                        _ => write!(buffer, "<{}>", code.red()),
                    }?;
                } else {
                    write!(buffer, "<{}>", code)?;
                }
            }

            if let Some(url) = diagnostic.url() {
                // This part is tricky to do without allocation if you need the link string
                // For now, let's keep it but it could be optimized further if needed.
                let link = if terminal_config.support_hyperlinks {
                    alloc::format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", url, "link")
                } else {
                    alloc::format!("{}", url)
                };

                if terminal_config.support_color {
                    write!(buffer, " ({})", link.blue())?;
                } else {
                    write!(buffer, " ({})", link)?;
                }
            }

            if !buffer.is_empty() {
                buffer.push_str(": ");
            }

            write!(buffer, "{}", diagnostic.description())?;

            // 3. Setup textwrap options with the correct indent
            let sub_prefix_colored = if terminal_config.support_color {
                alloc::format!("{}", sub_prefix_str.red())
            } else {
                sub_prefix_str.into()
            };

            let opts = textwrap::Options::new(width).subsequent_indent(&sub_prefix_colored);

            // Write the wrapped main line
            write!(f, "{}", textwrap::fill(&buffer, &opts))?;

            // --- Help line ---
            if let Some(help) = diagnostic.help() {
                buffer.clear();
                if terminal_config.support_color {
                    write!(buffer, "    {}: {}", "help".cyan(), help.blue())?;
                } else {
                    write!(buffer, "    help: {}", help)?;
                }
                // Write the wrapped help line
                write!(f, "\n{}", textwrap::fill(&buffer, &opts))?;
            }

            if !is_last {
                write!(f, "\n")?;
            }

            is_first = false;
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
