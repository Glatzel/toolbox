use core::error::Error;
use core::fmt::{Debug, Display};
extern crate alloc;
use alloc::boxed::Box;

use crate::error::MischiefError;
use crate::render;

pub struct Report {
    inner: MischiefError,
}

impl Report {
    pub fn new(error: MischiefError) -> Self { Report { inner: error } }
}

impl Debug for Report {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        render::Render::new(&self.inner).fmt(f)
    }
}
impl Display for Report {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        render::Render::new(&self.inner).fmt(f)
    }
}
impl<E> From<E> for Report
where
    E: Error,
{
    fn from(value: E) -> Self {
        Self {
            inner: MischiefError::from(value),
        }
    }
}
pub type Result<T, E = Report> = core::result::Result<T, E>;

pub trait WrapErr<T> {
    fn wrap_err<D>(self, msg: D) -> Result<T, Report>
    where
        D: Display + 'static;
    fn wrap_err_with<D, F>(self, msg: F) -> Result<T, Report>
    where
        D: Display + 'static,
        F: FnOnce() -> D;
}

impl<T> WrapErr<T> for Result<T, Report> {
    fn wrap_err<D>(self, msg: D) -> Result<T, Report>
    where
        D: Display + 'static,
    {
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

    fn wrap_err_with<D, F>(self, msg: F) -> Result<T, Report>
    where
        D: Display + 'static,
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

impl<T, E> WrapErr<T> for Result<T, E>
where
    E: Error,
{
    fn wrap_err<D>(self, msg: D) -> Result<T, Report>
    where
        D: Display + 'static,
    {
        match self {
            Err(e) => Err(Report::new(MischiefError::new(
                &msg,
                Some(Box::new(MischiefError::from(e))),
                None,
                None,
                None,
                None,
            ))),
            ok => Ok(ok?),
        }
    }

    fn wrap_err_with<D, F>(self, msg: F) -> Result<T, Report>
    where
        D: Display + 'static,
        F: FnOnce() -> D,
    {
        match self {
            Err(e) => Err(Report::new(MischiefError::new(
                &msg(),
                Some(Box::new(MischiefError::from(e))),
                None,
                None,
                None,
                None,
            ))),
            ok => Ok(ok?),
        }
    }
}
