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

        //description
        {
            output.push_str("\n\n");
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
                let _ = write!(output, "{}", diagnostic.description());

                output.push('\n');
            }
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
