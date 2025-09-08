use core::error::Error;
use core::fmt::{Debug, Display};
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::ToString;

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

default impl<E> From<E> for Report
where
    E: ToString,
{
    fn from(value: E) -> Self {
        Self {
            inner: { MischiefError::new(value.to_string(), None, None, None, None, None) },
        }
    }
}

impl<E> From<E> for Report
where
    E: Error,
{
    fn from(value: E) -> Self {
        Self {
            inner: {
                // convert recursively
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

pub type Result<T, E = Report> = core::result::Result<T, E>;
pub trait IntoMischief<T> {
    fn into_mischief(self) -> Result<T, Report>;
}
default impl<T, E> IntoMischief<T> for Result<T, E>
where
    E: ToString + Debug,
{
    fn into_mischief(self) -> Result<T, Report> {
        match self {
            Err(e) => Err(Report::new(MischiefError::new(
                e.to_string(),
                None,
                None,
                None,
                None,
                None,
            ))),
            Ok(v) => Ok(v),
        }
    }
}
impl<T, E: Error> IntoMischief<T> for Result<T, E> {
    fn into_mischief(self) -> Result<T, Report> {
        match self {
            Err(e) => Err(Report::from(e)),
            Ok(v) => Ok(v),
        }
    }
}
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
