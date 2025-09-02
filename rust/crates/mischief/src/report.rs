use core::error::Error;
use core::fmt::{Debug, Display};
extern crate alloc;
use alloc::boxed::Box;

use crate::diagnostic::MischiefError;
use crate::render;

pub struct Report {
    inner: MischiefError,
}

impl Report {
    pub(crate) fn new(error: MischiefError) -> Self { Report { inner: error } }
}

impl Debug for Report {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        render::Render::new(&self.inner).fmt(f)
    }
}

pub trait IntoMischief<T> {
    fn into_mischief(self) -> Result<T>;
}
pub type Result<T, E = Report> = core::result::Result<T, E>;

impl<T, E> IntoMischief<T> for core::result::Result<T, E>
where
    E: Error,
{
    fn into_mischief(self) -> Result<T> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(Report::new(MischiefError::from(e))),
        }
    }
}
impl<T> IntoMischief<T> for core::result::Result<T, MischiefError> {
    fn into_mischief(self) -> Result<T> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(Report::new(e)),
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
            Err(e) => Err(Report::new(MischiefError::new(
                msg,
                Some(Box::new(e.inner)),
            ))),
            ok => ok,
        }
    }

    fn wrap_err_with<D, F>(self, msg: F) -> Result<T, Report>
    where
        D: Display + Send + Sync + 'static,
        F: FnOnce() -> D,
    {
        match self {
            Err(e) => Err(Report::new(MischiefError::new(
                msg(),
                Some(Box::new(e.inner)),
            ))),
            ok => ok,
        }
    }
}
