use core::fmt::{Debug, Display, Write};
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "fancy")]
use owo_colors::OwoColorize;

use crate::diagnostic::{IDiagnostic, MischiefError};

pub struct Report {
    inner: Box<dyn IDiagnostic>,
}

impl Report {
    pub(crate) fn new(diagnostic: Box<dyn IDiagnostic>) -> Self { Report { inner: diagnostic } }

    pub(crate) fn chain(&self) -> impl Iterator<Item = &dyn IDiagnostic> {
        core::iter::successors(Some(&*self.inner), |r| r.source())
    }
}

impl Debug for Report {
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

pub trait IntoMischief<T> {
    fn into_mischief(self) -> Result<T>;
}
pub type Result<T, E = Report> = core::result::Result<T, E>;

impl<T, E> IntoMischief<T> for core::result::Result<T, E>
where
    E: IDiagnostic + 'static,
{
    fn into_mischief(self) -> Result<T> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(Report::new(Box::new(e))),
        }
    }
}
pub trait WrapErr<T> {
    fn wrap_err<D>(self, msg: D) -> Result<T, Report>
    where
        D: Display + Send + Sync + 'static;
    fn wrap_err_with<D, F>(self, msg: F) -> Result<T, Report>
    where
        D: Display + Send + Sync + 'static,
        F: FnOnce() -> D;
}

impl<T> WrapErr<T> for Result<T, Report> {
    /// Wraps the error with a custom message and returns a `Result`.
    fn wrap_err<D>(self, msg: D) -> Result<T, Report>
    where
        D: Display + Send + Sync + 'static,
    {
        match self {
            Err(e) => Err(Report::new(Box::new(MischiefError::new(
                msg,
                Some(e.inner),
            )))),
            ok => ok,
        }
    }

    fn wrap_err_with<D, F>(self, msg: F) -> Result<T, Report>
    where
        D: Display + Send + Sync + 'static,
        F: FnOnce() -> D,
    {
        match self {
            Err(e) => Err(Report::new(Box::new(MischiefError::new(
                msg(),
                Some(e.inner),
            )))),
            ok => ok,
        }
    }
}
