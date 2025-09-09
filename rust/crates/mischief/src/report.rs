use core::error::Error;
use core::fmt::{Debug, Display};
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::ToString;

use crate::error::MischiefError;
use crate::render;

/// Wrapper around a `MischiefError` for ergonomic error handling.
pub struct Report {
    inner: MischiefError,
}

impl Report {
    /// Creates a new `Report` from a `MischiefError`.
    pub fn new(error: MischiefError) -> Self {
        Report { inner: error }
    }
}

impl Debug for Report {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        render::Render::new(&self.inner).fmt(f)
    }
}

/// Converts any type implementing `Error` into a `Report`, recursively converting
/// source errors into `MischiefError`.
impl<E> From<E> for Report
where
    E: Error,
{
    fn from(value: E) -> Self {
        Self {
            inner: {
                fn convert(err: &dyn Error) -> MischiefError {
                    MischiefError::new(
                        err.to_string(),
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

impl<D, T> WrapErr<D, T> for Result<T, Report>
where
    D: Display,
{
    fn wrap_err(self, msg: D) -> Result<T, Report> {
        match self {
            Err(e) => Err(Report::new(MischiefError::new(
                &msg,
                Some(Box::new(e.inner)),
                None,
                None,
                None,
                None,
            ))),
            ok => ok,
        }
    }

    fn wrap_err_with<F>(self, msg: F) -> Result<T, Report>
    where
        F: FnOnce() -> D,
    {
        match self {
            Err(e) => Err(Report::new(MischiefError::new(
                &msg(),
                Some(Box::new(e.inner)),
                None,
                None,
                None,
                None,
            ))),
            ok => ok,
        }
    }
}

impl<T> WrapErr<Report, T> for Result<T, Report> {
    fn wrap_err(self, mut msg: Report) -> Result<T, Report> {
        match self {
            Err(e) => {
                msg.inner.source = Some(Box::new(e.inner));
                Err(msg)
            }
            ok => ok,
        }
    }

    fn wrap_err_with<F>(self, msg: F) -> Result<T, Report>
    where
        F: FnOnce() -> Report,
    {
        match self {
            Err(e) => {
                let mut msg = msg();
                msg.inner.source = Some(Box::new(e.inner));
                Err(msg)
            }
            ok => ok,
        }
    }
}
