use core::fmt::{Debug, Display, Write};
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "fancy")]
use owo_colors::OwoColorize;

use crate::diagnostic::Diagnostic;

pub struct Report {
    diagnostic: Diagnostic,
    source: Option<Box<Report>>,
}

impl Report {
    pub(crate) fn new(diagnostic: Diagnostic) -> Self {
        Report {
            diagnostic,
            source: None,
        }
    }

    pub(crate) fn append_error(self, diagnostic: Diagnostic) -> Self {
        Self {
            diagnostic,
            source: Some(Box::new(self)),
        }
    }
    pub(crate) fn chain(&self) -> impl Iterator<Item = &Report> {
        core::iter::successors(Some(self), |r| r.source.as_deref())
    }
}

impl Debug for Report {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let chain: Vec<&Report> = self.chain().collect();
        let mut output = String::new();

        for (i, report) in chain.iter().enumerate() {
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

            output.push_str(report.diagnostic.msg());
            output.push('\n');
        }

        write!(f, "{}", output)
    }
}
pub trait IntoMischief<T> {
    fn into_mischief(self) -> Result<T>;
}

pub type Result<T, E = Report> = core::result::Result<T, E>;

impl<T, E: core::fmt::Debug> IntoMischief<T> for core::result::Result<T, E> {
    fn into_mischief(self) -> Result<T> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                let mut msg: String = String::new();
                write!(msg, "{:?}", e).ok();
                let diagnostic = Diagnostic::new(msg);
                Err(Report::new(diagnostic))
            }
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
            Err(e) => Err(e.append_error(Diagnostic::new(msg))),
            ok => ok,
        }
    }

    fn wrap_err_with<D, F>(self, msg: F) -> Result<T, Report>
    where
        D: Display + Send + Sync + 'static,
        F: FnOnce() -> D,
    {
        match self {
            Err(e) => Err(e.append_error(Diagnostic::new(msg()))),
            ok => ok,
        }
    }
}
