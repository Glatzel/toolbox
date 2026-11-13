use core::error::Error;
use core::fmt::{Debug, Display};
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::ToString;

use mischief_core::{IDiagnosis, IIntoDiagnosis, MischiefError};
#[cfg(all(feature = "backtrace", debug_assertions))]
use mischief_render::Backtrace;

use crate::render::render_diagnosis;

struct ReportInner {
    error: MischiefError,

    #[cfg(all(feature = "backtrace", debug_assertions))]
    backtrace: Backtrace,
}
/// High-level wrapper around [`MischiefError`] used for ergonomic error
/// handling.
///
/// `Report` acts as the primary error type exposed by the crate. It wraps a
/// [`MischiefError`] and provides convenient conversions, formatting, and
/// utilities for integrating with Rust’s standard error ecosystem.
///
/// The type is designed to behave similarly to application-oriented error
/// containers such as `anyhow::Error`, while preserving structured diagnosis
/// information compatible with [`crate::IDiagnosis`].
///
/// Formatting a `Report` renders the full diagnosis chain. With the `std`
/// feature, a structured tree renderer is used; otherwise a minimal textual
/// fallback renderer prints descriptions only.
pub struct Report {
    inner: Box<ReportInner>,
}

impl Report {
    /// Creates a new `Report` from a [`MischiefError`].
    ///
    /// This function wraps the provided diagnosis as the root error
    /// contained by the report.
    pub fn new(error: MischiefError) -> Self {
        #[cfg(all(feature = "backtrace", debug_assertions))]
        let backtrace = Backtrace::new();

        Self {
            inner: Box::new(ReportInner {
                error,

                #[cfg(all(feature = "backtrace", debug_assertions))]
                backtrace,
            }),
        }
    }

    /// Returns a reference to the underlying diagnosis.
    ///
    /// This allows callers to inspect structured metadata such as
    /// error codes, severity levels, and help messages.
    pub fn error(&self) -> &MischiefError { &self.inner.error }

    /// Returns a shared reference to a foreign error retained in this report's
    /// source chain that can be downcast to `E`.
    pub fn downcast_ref<E>(&self) -> Option<&E>
    where
        E: Error + 'static,
    {
        self.inner.error.downcast_ref()
    }

    /// Renders the report using the configured rendering backend.
    ///
    /// If the `std` feature is enabled, a tree renderer based on `arbor`
    /// is used. Otherwise a minimal textual renderer is used.
    fn render_report(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        render_diagnosis(&self.inner.error, f)?;

        #[cfg(all(feature = "backtrace", debug_assertions))]
        crate::render_backtrace(&self.inner.backtrace, f)?;
        Ok(())
    }
}

impl IIntoDiagnosis for Report {
    fn into_diagnosis(self) -> MischiefError { self.inner.error }
}

impl IDiagnosis for Report {
    fn description(&self) -> &str { self.inner.error.description() }

    fn source(&self) -> Option<&dyn IDiagnosis> { self.inner.error.source() }

    fn code(&self) -> Option<&str> { self.inner.error.code() }

    fn severity(&self) -> Option<mischief_core::Severity> { self.inner.error.severity() }

    fn help(&self) -> Option<&str> { self.inner.error.help() }

    fn url(&self) -> Option<&str> { self.inner.error.url() }
}

/// Formats the report using the configured diagnosis renderer.
///
/// The `Debug` representation intentionally matches the `Display`
/// representation to produce readable diagnosis output.
impl Debug for Report {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.render_report(f) }
}

/// Formats the report using the configured diagnosis renderer.
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
    E: Error + Send + Sync + 'static,
{
    fn from(value: E) -> Self { Self::new(MischiefError::from_error(value)) }
}

/// Convenient `Result` alias using [`Report`] as the default error type.
///
/// This alias simplifies function signatures when working with
/// diagnosis-aware errors.
pub type Result<T, E = Report> = core::result::Result<T, E>;

/// Trait providing conversion into [`Report`].
///
/// This trait enables ergonomic conversion of existing `Result` values
/// whose error types implement [`Error`] into results using [`Report`].
pub trait IntoMischief<T> {
    /// Converts the error type into a [`Report`].
    fn into_mischief(self) -> Result<T, Report>;
}

impl<T, E> IntoMischief<T> for Result<T, E>
where
    E: Error + Send + Sync + 'static,
{
    fn into_mischief(self) -> Result<T, Report> {
        match self {
            Err(e) => Err(Report::from(e)),
            Ok(v) => Ok(v),
        }
    }
}

/// Trait for attaching additional diagnosis context to existing errors.
///
/// These methods allow callers to extend an error chain with
/// higher-level context while preserving the original cause.
pub trait WrapErr<D, T> {
    /// Attaches a context message to the error if the result is `Err`.
    ///
    /// The message becomes a new diagnosis layer above the original error.
    fn wrap_err(self, msg: D) -> Result<T, Report>;

    /// Lazily attaches a context message to the error if the result is `Err`.
    ///
    /// The provided closure is evaluated only when an error occurs.
    fn wrap_err_with<F>(self, msg: F) -> Result<T, Report>
    where
        F: FnOnce() -> D;
}

/// Internal helper used to attach contextual diagnosiss.
fn wrap_inner<D>(e: Report, msg: D) -> Report
where
    D: Display + 'static,
{
    let message: Box<dyn core::any::Any> = Box::new(msg);
    match message.downcast::<Report>() {
        Ok(report) => {
            #[cfg(all(feature = "backtrace", debug_assertions))]
            let ReportInner { error, .. } = *report.inner;
            #[cfg(not(all(feature = "backtrace", debug_assertions)))]
            let ReportInner { error } = *report.inner;
            Report::new(error.with_source(e.inner.error))
        }
        Err(message) => {
            let message = message
                .downcast::<D>()
                .expect("the original wrap message type should be preserved");
            Report::new(
                MischiefError::builder(message.to_string())
                    .source(e.inner.error)
                    .build(),
            )
        }
    }
}

impl<D, T> WrapErr<D, T> for Result<T, Report>
where
    D: Display + 'static,
{
    fn wrap_err(self, msg: D) -> Self { self.wrap_err_with(|| msg) }

    fn wrap_err_with<F>(self, msg: F) -> Self
    where
        F: FnOnce() -> D,
    {
        match self {
            Err(e) => Err(wrap_inner(e, msg())),
            Ok(v) => Ok(v),
        }
    }
}
