use core::error::Error;
use core::fmt::{Debug, Display};
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::ToString;

use crate::error::MischiefError;
#[cfg(feature = "fancy")]
use crate::fancy_render::*;
#[cfg(not(feature = "fancy"))]
use crate::no_fancy_render::*;

/// High-level wrapper around [`MischiefError`] used for ergonomic error
/// handling.
///
/// `Report` acts as the primary error type exposed by the crate. It wraps a
/// [`MischiefError`] and provides convenient conversions, formatting, and
/// utilities for integrating with Rust’s standard error ecosystem.
///
/// The type is designed to behave similarly to application-oriented error
/// containers such as `anyhow::Error`, while preserving structured diagnostic
/// information compatible with [`IDiagnostic`].
///
/// Formatting a `Report` will render the full diagnostic chain. If the
/// `fancy` feature is enabled, a structured tree-based renderer is used.
/// Otherwise a minimal textual fallback renderer is used.
#[derive(Clone)]
pub struct Report {
    /// Inner structured diagnostic.
    pub inner: MischiefError,
}

impl Report {
    /// Creates a new `Report` from a [`MischiefError`].
    ///
    /// This function wraps the provided diagnostic as the root error
    /// contained by the report.
    pub fn new(error: MischiefError) -> Self { Report { inner: error } }

    /// Returns a reference to the underlying diagnostic.
    ///
    /// This allows callers to inspect structured metadata such as
    /// error codes, severity levels, and help messages.
    pub fn diagnostic(&self) -> &MischiefError { &self.inner }

    /// Renders the report using the configured rendering backend.
    ///
    /// If the `fancy` feature is enabled, a themed tree renderer based
    /// on `arbor` is used. Otherwise a minimal textual renderer is used.
    fn render_report(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        #[cfg(feature = "fancy")]
        {
            let bundle = RenderBundle {
                report: self,
                theme: MischiefTheme::default(),
                indent: MischiefIndent::default(),
                width: match terminal_size::terminal_size() {
                    Some((w, _)) => w.0 as usize,
                    None => 0,
                },
            };
            write!(f, "{}", bundle)
        }

        #[cfg(not(feature = "fancy"))]
        {
            render_diagnostic(&self.inner, f)
        }
    }
}

/// Formats the report using the configured diagnostic renderer.
///
/// The `Debug` representation intentionally matches the `Display`
/// representation to produce readable diagnostic output.
impl Debug for Report {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.render_report(f) }
}

/// Formats the report using the configured diagnostic renderer.
impl Display for Report {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.render_report(f) }
}

/// Converts any type implementing [`Error`] into a [`Report`].
///
/// During conversion the full error source chain is recursively
/// transformed into a hierarchy of [`MischiefError`] values,
/// preserving the causal structure of the original error.
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
                        #[cfg(feature = "fancy")]
                        None,
                        #[cfg(feature = "fancy")]
                        None,
                        #[cfg(feature = "fancy")]
                        None,
                        #[cfg(feature = "fancy")]
                        None,
                    )
                }
                convert(&value)
            },
        }
    }
}

/// Convenient `Result` alias using [`Report`] as the default error type.
///
/// This alias simplifies function signatures when working with
/// diagnostic-aware errors.
pub type Result<T, E = Report> = core::result::Result<T, E>;

/// Trait providing conversion into [`Report`].
///
/// This trait enables ergonomic conversion of existing `Result` values
/// whose error types implement [`Error`] into results using [`Report`].
pub trait IntoMischief<T> {
    /// Converts the error type into a [`Report`].
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

/// Trait for attaching additional diagnostic context to existing errors.
///
/// These methods allow callers to extend an error chain with
/// higher-level context while preserving the original cause.
pub trait WrapErr<D, T> {
    /// Attaches a context message to the error if the result is `Err`.
    ///
    /// The message becomes a new diagnostic layer above the original error.
    fn wrap_err(self, msg: D) -> Result<T, Report>;

    /// Lazily attaches a context message to the error if the result is `Err`.
    ///
    /// The provided closure is evaluated only when an error occurs.
    fn wrap_err_with<F>(self, msg: F) -> Result<T, Report>
    where
        F: FnOnce() -> D;
}

/// Internal helper used to attach contextual diagnostics.
fn wrap_inner<D>(e: Report, msg: D) -> Report
where
    D: Display + 'static,
{
    let new_inner = if let Some(r) = (&msg as &dyn core::any::Any).downcast_ref::<Report>() {
        let mut r = r.clone();
        r.inner.source = Some(Box::new(e.inner));
        r.inner
    } else {
        MischiefError::new(
            &msg,
            Some(Box::new(e.inner)),
            #[cfg(feature = "fancy")]
            None,
            #[cfg(feature = "fancy")]
            None,
            #[cfg(feature = "fancy")]
            None,
            #[cfg(feature = "fancy")]
            None,
        )
    };

    Report::new(new_inner)
}

impl<D, T> WrapErr<D, T> for Result<T, Report>
where
    D: Display + 'static,
{
    fn wrap_err(self, msg: D) -> Result<T, Report> { self.wrap_err_with(|| msg) }

    fn wrap_err_with<F>(self, msg: F) -> Result<T, Report>
    where
        F: FnOnce() -> D,
    {
        match self {
            Err(e) => Err(wrap_inner(e, msg())),
            Ok(v) => Ok(v),
        }
    }
}
