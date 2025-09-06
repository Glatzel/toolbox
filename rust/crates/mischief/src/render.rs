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
#[cfg(feature = "fancy")]
impl<'a, T> Debug for Render<'a, T>
where
    T: IDiagnostic,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use alloc::string::String;
        use alloc::vec::Vec;

        use owo_colors::OwoColorize;

        let chain: Vec<&dyn IDiagnostic> = self.chain().collect();
        let mut output = String::new();

        output.push('\n');
        for (i, diagnostic) in chain.iter().enumerate() {
            if i == 0 {
                #[cfg(feature = "fancy")]
                write!(output, "{} ", "x".red())?;
            } else if i == chain.len() - 1 {
                #[cfg(feature = "fancy")]
                write!(output, "{} ", "╰─▶".red())?;
            } else {
                #[cfg(feature = "fancy")]
                write!(output, "{} ", "├─▶".red())?;
            }

            use core::fmt::Write as _;
            if let Some(severity) = diagnostic.severity() {
                write!(output, "{:?}", severity).unwrap();
            }
            if let Some(code) = diagnostic.code() {
                use crate::Severity;

                match diagnostic.severity() {
                    Some(Severity::Warning) => write!(output, " <{}>", code.red()).unwrap(),
                    Some(Severity::Advice) => write!(output, " <{}>", code.yellow()).unwrap(),
                    _ => write!(output, " <{}>", code.green()).unwrap(),
                }
            }
            if let Some(url) = diagnostic.url() {
                let mut link_url = String::new();
                write!(link_url, "\x1b]8;;{}\x1b\\link\x1b]8;;\x1b\\", url).unwrap();
                write!(output, " ({})", link_url.blue()).unwrap();
            }
            if diagnostic.severity().is_some()
                || diagnostic.code().is_some()
                || diagnostic.url().is_some()
            {
                output.push_str(": ");
            }
            write!(output, "{}", diagnostic.description()).unwrap();
            if let Some(help) = diagnostic.help() {
                output.push('\n');
                if i == 0 {
                    write!(output, "{}", "  ╰─".red())?;
                } else if i == chain.len() - 1 {
                    write!(output, "{}", "    ╰─".red())?;
                } else {
                    write!(output, "{}", "│   ╰─".red())?;
                }
                write!(output, " {}", "help: ".cyan())?;
                write!(output, "{}", help.blue())?;
            }

            output.push('\n');
        }

        write!(f, "{}", output)
    }
}
#[cfg(not(feature = "fancy"))]
impl<'a, T: IDiagnostic> Debug for Render<'a, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
