use core::error::Error;
use core::fmt::{Debug, Display};
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::ToString;

#[cfg(feature = "fancy")]
use terminal_size::terminal_size;

use crate::error::MischiefError;
#[cfg(feature = "fancy")]
use crate::presets::*;

/// Wrapper around a `MischiefError` for ergonomic error handling.
#[derive(Clone)]
pub struct Report {
    pub inner: MischiefError,
}

impl Report {
    /// Creates a new `Report` from a `MischiefError`.
    pub fn new(error: MischiefError) -> Self { Report { inner: error } }
    pub fn diagnostic(&self) -> &MischiefError { &self.inner }
}
#[cfg(not(feature = "fancy"))]
impl Report {
    /// Produces an iterator over the diagnostic chain.
    fn chain(
        diagnostic: &impl crate::IDiagnostic,
    ) -> impl Iterator<Item = &dyn crate::IDiagnostic> {
        core::iter::successors(Some(diagnostic as &dyn crate::IDiagnostic), |r| r.source())
    }
    fn render_plain(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut chain = Self::chain(&self.inner);

        if let Some(first) = chain.next() {
            f.write_str(&alloc::format!("Error: {}", first.description()))?;
            writeln!(f)?;
        }
        let mut first = true;
        for diagnostic in chain {
            if first {
                f.write_str("\nCaused by:")?;
                writeln!(f)?;
                first = false;
            }
            f.write_str(&alloc::format!("    {}", diagnostic.description()))?;
        }
        Ok(())
    }
}
impl Debug for Report {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        #[cfg(feature = "fancy")]
        {
            let bundle = RenderBundle {
                report: self,
                theme: MischiefTheme::default(),
                indent: MischiefIndent::default(),
                width: match terminal_size() {
                    Some((w, _)) => w.0 as usize,
                    None => 0,
                },
            };
            write!(f, "{}", bundle)
        }
        #[cfg(not(feature = "fancy"))]
        self.render_plain(f)
    }
}
impl Display for Report {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        #[cfg(feature = "fancy")]
        {
            let bundle = RenderBundle {
                report: self,
                theme: MischiefTheme::default(),
                indent: MischiefIndent::default(),
                width: match terminal_size() {
                    Some((w, _)) => w.0 as usize,
                    None => 0,
                },
            };
            write!(f, "{}", bundle)
        }
        #[cfg(not(feature = "fancy"))]
        self.render_plain(f)
    }
}
/// Converts any type implementing `Error` into a `Report`, recursively
/// converting source errors into `MischiefError`.
impl<E> From<E> for Report
where
    E: Error,
{
    fn from(value: E) -> Self {
        Self {
            inner: {
                fn convert(err: &dyn Error) -> MischiefError {
                    MischiefError::new(
                        &err.to_string(),
                        err.source().map(|src| Box::new(convert(src))),
                        None,
                        None,
                        None,
                        None,
                    )
                }
                convert(&value)
            },
        }
    }
}

/// Convenient `Result` type with `Report` as the default error.
pub type Result<T, E = Report> = core::result::Result<T, E>;

/// Converts results or other error types into `Report`.
pub trait IntoMischief<T> {
    fn into_mischief(self) -> Result<T, Report>;
}

impl<T, E: Error> IntoMischief<T> for Result<T, E> {
    fn into_mischief(self) -> Result<T, Report> {
        match self {
            Err(e) => Err(Report::from(e)),
            Ok(v) => Ok(v),
        }
    }
}

/// Adds context to existing `Report` errors.
pub trait WrapErr<D, T> {
    fn wrap_err(self, msg: D) -> Result<T, Report>;
    fn wrap_err_with<F>(self, msg: F) -> Result<T, Report>
    where
        F: FnOnce() -> D;
}

///TODO: use specialization to impl for [`Report`]
impl<D, T> WrapErr<D, T> for Result<T, Report>
where
    D: Display + 'static,
{
    fn wrap_err(self, msg: D) -> Result<T, Report> {
        match self {
            Err(e) => {
                let new_inner =
                    if let Some(r) = (&msg as &dyn core::any::Any).downcast_ref::<Report>() {
                        let mut r = r.clone();
                        r.inner.source = Some(Box::new(e.inner));
                        r.inner
                    } else {
                        MischiefError::new(&msg, Some(Box::new(e.inner)), None, None, None, None)
                    };
                Err(Report::new(new_inner))
            }
            ok => ok,
        }
    }

    fn wrap_err_with<F>(self, msg: F) -> Result<T, Report>
    where
        F: FnOnce() -> D,
    {
        match self {
            Err(e) => {
                let msg = msg();
                let new_inner =
                    if let Some(r) = (&msg as &dyn core::any::Any).downcast_ref::<Report>() {
                        let mut r = r.clone();
                        r.inner.source = Some(Box::new(e.inner));
                        r.inner
                    } else {
                        MischiefError::new(&msg, Some(Box::new(e.inner)), None, None, None, None)
                    };
                Err(Report::new(new_inner))
            }
            ok => ok,
        }
    }
}
