use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::string::ToString;
use core::error::Error;
use core::fmt::Display;

use crate::IDiagnosis;

/// Concrete diagnosis error type used by the crate.
///
/// `MischiefError` owns a private root node containing either a mischief
/// diagnosis or an owned foreign error. Its source chain can contain further
/// mischief diagnostics or foreign errors while preserving their descriptions
/// and foreign values for downcasting.
#[derive(Debug)]
pub struct MischiefError {
    root: ErrorNode,
}

/// Builder for constructing a [`MischiefError`] with independent metadata
/// values.
///
/// Static strings are stored without allocating. Dynamically formatted
/// values are owned by the resulting error.
#[derive(Debug)]
pub struct MischiefErrorBuilder {
    description: Cow<'static, str>,
    source: Option<Box<ErrorNode>>,
    code: Option<Cow<'static, str>>,
    severity: Option<crate::Severity>,
    help: Option<Cow<'static, str>>,
    url: Option<Cow<'static, str>>,
}

/// Conversion into a diagnosis suitable for a diagnostic source.
///
/// Implementations exist for [`MischiefError`] and owned foreign errors.
pub trait IIntoDiagnosis {
    /// Converts this value into a mischief diagnosis.
    fn into_diagnosis(self) -> MischiefError;
}

/// Exposes a field as an optional diagnosis source.
///
/// This is implemented for diagnosis values and `Option<T>` containers.
pub trait IAsDiagnosis {
    /// Returns this value as a diagnosis source, when present.
    fn as_diagnosis(&self) -> Option<&dyn IDiagnosis>;
}

/// Exposes a field as a core error source.
pub trait IErrorSource {
    /// Returns this value as a core error source, when present.
    fn as_error_source(&self) -> Option<&(dyn Error + 'static)>;
}

/// Exposes a field as a sendable foreign core error source.
pub trait IForeignErrorSource {
    /// Returns this value as a core error source, when present.
    fn as_foreign_error_source(&self) -> Option<&(dyn Error + 'static)>;
}

impl<T> IForeignErrorSource for T
where
    T: Error + Send + Sync + 'static,
{
    fn as_foreign_error_source(&self) -> Option<&(dyn Error + 'static)> { Some(self) }
}

impl<T> IErrorSource for T
where
    T: Error + 'static,
{
    fn as_error_source(&self) -> Option<&(dyn Error + 'static)> { Some(self) }
}

impl<T> IAsDiagnosis for T
where
    T: IDiagnosis,
{
    fn as_diagnosis(&self) -> Option<&dyn IDiagnosis> { Some(self) }
}

impl<T> IAsDiagnosis for Option<T>
where
    T: IAsDiagnosis,
{
    fn as_diagnosis(&self) -> Option<&dyn IDiagnosis> {
        self.as_ref().and_then(IAsDiagnosis::as_diagnosis)
    }
}

impl IIntoDiagnosis for MischiefError {
    fn into_diagnosis(self) -> MischiefError { self }
}

impl<E> IIntoDiagnosis for E
where
    E: Error + Send + Sync + 'static,
{
    fn into_diagnosis(self) -> MischiefError { MischiefError::from_error(self) }
}

impl MischiefError {
    /// Creates a new `MischiefError` using the original constructor API.
    ///
    /// The builder API is available through [`MischiefError::builder`] when
    /// metadata values have different textual types.
    pub fn new<D>(
        description: D,
        source: Option<Box<Self>>,
        code: Option<D>,
        severity: Option<crate::Severity>,
        help: Option<D>,
        url: Option<D>,
    ) -> Self
    where
        D: Display,
    {
        Self {
            root: ErrorNode::Diagnosis(DiagnosisNode {
                description: Cow::Owned(description.to_string()),
                source: source.map(|source| Box::new(source.root)),
                code: code.map(|value| Cow::Owned(value.to_string())),
                severity,
                help: help.map(|value| Cow::Owned(value.to_string())),
                url: url.map(|value| Cow::Owned(value.to_string())),
            }),
        }
    }

    /// Starts building a diagnosis without eagerly allocating static text.
    pub fn builder(description: impl Into<Cow<'static, str>>) -> MischiefErrorBuilder {
        MischiefErrorBuilder {
            description: description.into(),
            source: None,
            code: None,
            severity: None,
            help: None,
            url: None,
        }
    }

    /// Replaces this diagnosis's source with another diagnosis.
    pub fn with_source(mut self, source: Self) -> Self {
        self.root.set_source(Some(Box::new(source.root)));
        self
    }

    /// Creates a diagnosis root that retains a foreign error for
    /// downcasting while presenting its description and source chain.
    pub fn from_error<E>(error: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self {
            root: ErrorNode::from_owned_error(error),
        }
    }

    /// Returns the first foreign source in this diagnostic chain that can be
    /// downcast to `E`.
    pub fn downcast_ref<E>(&self) -> Option<&E>
    where
        E: Error + 'static,
    {
        self.root.downcast_ref::<E>()
    }
}

impl MischiefErrorBuilder {
    /// Replaces the diagnosis description.
    ///
    /// Static text is stored without allocating. Owned text is retained by
    /// the resulting diagnosis.
    pub fn description(mut self, description: impl Into<Cow<'static, str>>) -> Self {
        self.description = description.into();
        self
    }

    /// Sets the stable error code.
    pub fn code(mut self, code: impl Into<Cow<'static, str>>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Sets the diagnosis severity.
    pub fn severity(mut self, severity: crate::Severity) -> Self {
        self.severity = Some(severity);
        self
    }

    /// Sets the help text.
    pub fn help(mut self, help: impl Into<Cow<'static, str>>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Sets the documentation URL.
    pub fn url(mut self, url: impl Into<Cow<'static, str>>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Sets a mischief diagnostic as the source.
    pub fn source(mut self, source: MischiefError) -> Self {
        self.source = Some(Box::new(source.root));
        self
    }

    /// Sets a foreign error as the source while retaining it for downcasting.
    pub fn source_error<E>(mut self, error: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        self.source = Some(Box::new(ErrorNode::from_owned_error(error)));
        self
    }

    /// Finishes constructing the diagnosis.
    pub fn build(self) -> MischiefError {
        MischiefError {
            root: ErrorNode::Diagnosis(DiagnosisNode {
                description: self.description,
                source: self.source,
                code: self.code,
                severity: self.severity,
                help: self.help,
                url: self.url,
            }),
        }
    }
}

impl Display for MischiefError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.root.fmt_description(f)
    }
}

impl IDiagnosis for MischiefError {
    fn description(&self) -> &str { self.root.description() }

    fn source(&self) -> Option<&dyn IDiagnosis> { IDiagnosis::source(&self.root) }

    fn code(&self) -> Option<&str> { self.root.code() }

    fn severity(&self) -> Option<crate::Severity> { self.root.severity() }

    fn help(&self) -> Option<&str> { self.root.help() }

    fn url(&self) -> Option<&str> { self.root.url() }
}

#[derive(Debug)]
enum ErrorNode {
    Diagnosis(DiagnosisNode),
    Foreign(ForeignErrorNode),
}

#[derive(Debug)]
struct DiagnosisNode {
    description: Cow<'static, str>,
    source: Option<Box<ErrorNode>>,
    code: Option<Cow<'static, str>>,
    severity: Option<crate::Severity>,
    help: Option<Cow<'static, str>>,
    url: Option<Cow<'static, str>>,
}

#[derive(Debug)]
struct ForeignErrorNode {
    error: Option<Box<dyn Error + Send + Sync>>,
    description: Cow<'static, str>,
    source: Option<Box<ErrorNode>>,
}

impl ErrorNode {
    fn from_owned_error<E>(error: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        let source = error
            .source()
            .map(|source| Box::new(Self::from_borrowed_error(source)));
        let description = Cow::Owned(error.to_string());

        Self::Foreign(ForeignErrorNode {
            error: Some(Box::new(error)),
            description,
            source,
        })
    }

    fn from_borrowed_error(error: &dyn Error) -> Self {
        Self::Foreign(ForeignErrorNode {
            error: None,
            description: Cow::Owned(error.to_string()),
            source: error
                .source()
                .map(|source| Box::new(Self::from_borrowed_error(source))),
        })
    }

    fn source(&self) -> Option<&ErrorNode> {
        match self {
            Self::Diagnosis(error) => error.source.as_deref(),
            Self::Foreign(error) => error.source.as_deref(),
        }
    }

    fn set_source(&mut self, source: Option<Box<ErrorNode>>) {
        match self {
            Self::Diagnosis(error) => error.source = source,
            Self::Foreign(error) => error.source = source,
        }
    }

    fn downcast_ref<E>(&self) -> Option<&E>
    where
        E: Error + 'static,
    {
        match self {
            Self::Diagnosis(error) => error.source.as_deref()?.downcast_ref::<E>(),
            Self::Foreign(error) => {
                if let Some(source) = error.error.as_deref() {
                    if let Some(source) = source.downcast_ref::<E>() {
                        return Some(source);
                    }
                }
                error.source.as_deref()?.downcast_ref::<E>()
            }
        }
    }
}

impl IDiagnosis for ErrorNode {
    fn description(&self) -> &str {
        match self {
            Self::Diagnosis(error) => &error.description,
            Self::Foreign(error) => &error.description,
        }
    }

    fn source(&self) -> Option<&dyn IDiagnosis> {
        self.source().map(|source| source as &dyn IDiagnosis)
    }

    fn code(&self) -> Option<&str> {
        match self {
            Self::Diagnosis(error) => error.code.as_deref(),
            Self::Foreign(_) => None,
        }
    }

    fn severity(&self) -> Option<crate::Severity> {
        match self {
            Self::Diagnosis(error) => error.severity,
            Self::Foreign(_) => None,
        }
    }

    fn help(&self) -> Option<&str> {
        match self {
            Self::Diagnosis(error) => error.help.as_deref(),
            Self::Foreign(_) => None,
        }
    }

    fn url(&self) -> Option<&str> {
        match self {
            Self::Diagnosis(error) => error.url.as_deref(),
            Self::Foreign(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::boxed::Box;
    use alloc::format;
    use alloc::string::{String, ToString};
    use core::error::Error;
    use core::fmt;

    use super::{ErrorNode, MischiefError};
    use crate::{IDiagnosis, Severity};

    #[test]
    fn builder_borrows_static_text() {
        let error = MischiefError::builder("description")
            .code("E_BORROWED")
            .severity(Severity::Warning)
            .help("help")
            .url("https://example.com/errors/E_BORROWED")
            .build();

        let ErrorNode::Diagnosis(error) = &error.root else {
            panic!("builder should create a diagnosis root");
        };
        assert!(matches!(
            error.description,
            alloc::borrow::Cow::Borrowed("description")
        ));
        assert!(matches!(
            error.code,
            Some(alloc::borrow::Cow::Borrowed("E_BORROWED"))
        ));
        assert_eq!(error.severity, Some(Severity::Warning));
        assert!(matches!(
            error.help,
            Some(alloc::borrow::Cow::Borrowed("help"))
        ));
        assert!(matches!(
            error.url,
            Some(alloc::borrow::Cow::Borrowed(
                "https://example.com/errors/E_BORROWED"
            ))
        ));
    }

    #[test]
    fn builder_owns_dynamic_text() {
        let description: String = "dynamic description".to_string();
        let error = MischiefError::builder(description).build();

        let ErrorNode::Diagnosis(error) = &error.root else {
            panic!("builder should create a diagnosis root");
        };
        assert!(matches!(error.description, alloc::borrow::Cow::Owned(_)));
    }

    #[test]
    fn builder_replaces_description() {
        let error = MischiefError::builder("static")
            .description(String::from("formatted value"))
            .build();

        assert_eq!(error.description(), "formatted value");
        let ErrorNode::Diagnosis(error) = &error.root else {
            panic!("builder should create a diagnosis root");
        };
        assert!(matches!(error.description, alloc::borrow::Cow::Owned(_)));
    }

    #[test]
    fn display_uses_the_description() {
        let error = MischiefError::builder("display description").build();

        assert_eq!(format!("{error}"), "display description");
    }

    #[test]
    fn new_preserves_source_chain_behavior() {
        let inner = MischiefError::new("inner", None, None, None, None, None);
        let outer = MischiefError::new("outer", Some(Box::new(inner)), None, None, None, None);

        let source = outer.source().expect("outer should have a source");
        assert_eq!(source.description(), "inner");
    }

    #[derive(Debug)]
    struct RootError {
        source: LeafError,
    }

    impl fmt::Display for RootError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str("root error") }
    }

    impl Error for RootError {
        fn source(&self) -> Option<&(dyn Error + 'static)> { Some(&self.source) }
    }

    #[derive(Debug)]
    struct LeafError;

    impl fmt::Display for LeafError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str("leaf error") }
    }

    impl Error for LeafError {}

    #[test]
    fn foreign_source_is_retained_for_downcasting() {
        let error = MischiefError::builder("wrapper")
            .source_error(RootError { source: LeafError })
            .build();

        assert!(error.downcast_ref::<RootError>().is_some());

        let source = error.source().expect("wrapper should have a source");
        assert_eq!(source.description(), "root error");

        let source = source.source().expect("root should have a source");
        assert_eq!(source.description(), "leaf error");
    }
}
