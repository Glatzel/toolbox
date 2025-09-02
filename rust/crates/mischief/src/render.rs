use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Debug, Write};

use crate::IDiagnostic;
extern crate alloc;
#[cfg(feature = "fancy")]
use owo_colors::OwoColorize;

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
        let chain: Vec<&dyn IDiagnostic> = self.chain().collect();
        let mut output = String::new();

        for (i, diagnostic) in chain.iter().enumerate() {
            if i == 0 {
                #[cfg(feature = "fancy")]
                write!(output, "{} ", "x".red())?;
                #[cfg(not(feature = "fancy"))]
                output.push_str("x ");
            } else if i == chain.len() - 1 {
                #[cfg(feature = "fancy")]
                write!(output, "{} ", "╰─▶".red())?;
                #[cfg(not(feature = "fancy"))]
                output.push_str("╰─▶ ");
            } else {
                #[cfg(feature = "fancy")]
                write!(output, "{} ", "├─▶".red())?;
                #[cfg(not(feature = "fancy"))]
                output.push_str("├─▶ ");
            }

            if let Some(desc) = diagnostic.description() {
                use core::fmt::Write as _;
                let _ = write!(output, "{}", desc);
            }
            output.push('\n');
        }

        write!(f, "{}", output)
    }
}
