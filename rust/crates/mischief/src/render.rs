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
        use alloc::vec::Vec;
        use core::fmt::Write;

        use owo_colors::OwoColorize;

        let chain: Vec<&dyn IDiagnostic> = self.chain().collect();
        let width: usize = terminal_config.width.unwrap_or(80);

        for (i, diagnostic) in chain.iter().enumerate() {
            let (prefix, sub_prefix) = if terminal_config.support_color {
                if i == 0 {
                    ("x".red().to_string(), "│  ".red().to_string())
                } else if i == chain.len() - 1 {
                    ("╰─▶".red().to_string(), "    ".to_string())
                } else {
                    ("├─▶".red().to_string(), "│    ".red().to_string())
                }
            } else {
                if i == 0 {
                    ("x".to_string(), "│  ".to_string())
                } else if i == chain.len() - 1 {
                    ("╰─▶".to_string(), "    ".to_string())
                } else {
                    ("├─▶".to_string(), "│    ".to_string())
                }
            };
            write!(f, "{} ", prefix)?;
            let mut line = String::new();
            //line 1
            {
                // severity + code
                if let Some(severity) = diagnostic.severity() {
                    write!(line, "{:?}", severity)?;
                }
                if let Some(code) = diagnostic.code() {
                    if terminal_config.support_color {
                        match diagnostic.severity() {
                            Some(crate::Severity::Warning) => write!(line, " <{}>", code.red())?,
                            Some(crate::Severity::Advice) => write!(line, " <{}>", code.yellow())?,
                            _ => write!(line, " <{}>", code.green())?,
                        }
                    } else {
                        write!(line, " <{}>", code)?
                    }
                }

                // url
                if let Some(url) = diagnostic.url() {
                    let mut link_url = String::new();
                    if terminal_config.support_hyperlinks {
                        write!(link_url, "\x1b]8;;{}\x1b\\link\x1b]8;;\x1b\\", url)?;
                    } else {
                        write!(link_url, "{}", url)?;
                    }
                    if terminal_config.support_color {
                        write!(line, " ({})", link_url.blue())?;
                    } else {
                        write!(line, " ({})", link_url)?;
                    }
                }
                if diagnostic.severity().is_some()
                    || diagnostic.code().is_some()
                    || diagnostic.url().is_some()
                {
                    write!(line, ": ")?;
                }
                write!(line, "{}", diagnostic.description())?;
                write!(
                    f,
                    "{}",
                    textwrap::fill(
                        &line,
                        textwrap::Options::new(width).subsequent_indent(&sub_prefix)
                    )
                )?;
                line.clear();
            }

            //line 2
            {
                if let Some(help) = diagnostic.help() {
                    write!(f, "\n\n")?;
                    write!(line, "    {}: {}\n", "help".cyan(), help.blue())?;
                    write!(
                        f,
                        "{}",
                        textwrap::fill(
                            &line,
                            textwrap::Options::new(width).subsequent_indent(&sub_prefix)
                        )
                    )?;
                    line.clear();
                }
            }
            write!(f, "\n")?;
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
