#![cfg(feature = "macros")]

use mischief::{IDiagnosis, MischiefError, Severity};

#[derive(IDiagnosis)]
#[diagnosis(
    description = "static description",
    code = "E_STATIC",
    severity = Severity::Warning,
    help = "static help",
    url = "https://example.com/errors/E_STATIC",
)]
struct StaticDiagnosis {
    #[diagnosis(source)]
    source: Option<MischiefError>,
}

#[derive(IDiagnosis)]
#[diagnosis(description = message)]
struct DelegatedDiagnosis {
    message: String,
    #[diagnosis(source)]
    source: MischiefError,
}

#[derive(IDiagnosis)]
#[diagnosis(description = "description only")]
struct DescriptionOnly {
    value: u8,
}

#[derive(IDiagnosis)]
#[diagnosis(
    description = "enum default",
    code = "E_ENUM_DEFAULT",
    severity = Severity::Advice,
    help = "default help",
    url = "https://example.com/enum"
)]
enum EnumDiagnosis {
    #[diagnosis(description = "unit")]
    Unit,
    #[diagnosis(description_from = 0)]
    Tuple(String, #[diagnosis(source)] MischiefError),
    #[diagnosis(description = "named")]
    Named {
        #[diagnosis(source)]
        source: Option<MischiefError>,
    },
    #[diagnosis(forward)]
    Forward(MischiefError),
}

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error, description = "inner")]
enum InnerError {
    Inner,
}

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error, description = "outer")]
struct OuterError {
    #[diagnosis(source)]
    source: Option<InnerError>,
}

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error, description = "default")]
enum FlaggedError {
    Default,
    #[diagnosis(description = "variant")]
    Variant(#[diagnosis(source)] InnerError),
}

#[test]
fn derives_static_description_and_metadata() {
    let source = MischiefError::builder("source").build();
    let diagnosis = StaticDiagnosis {
        source: Some(source),
    };

    assert_eq!(diagnosis.description(), "static description");
    assert_eq!(diagnosis.code(), Some("E_STATIC"));
    assert_eq!(diagnosis.severity(), Some(Severity::Warning));
    assert_eq!(diagnosis.help(), Some("static help"));
    assert_eq!(diagnosis.url(), Some("https://example.com/errors/E_STATIC"));
    assert_eq!(
        diagnosis.source().map(IDiagnosis::description),
        Some("source")
    );
}

#[test]
fn derives_delegated_description_and_source() {
    let diagnosis = DelegatedDiagnosis {
        message: String::from("delegated description"),
        source: MischiefError::builder("source").build(),
    };

    assert_eq!(diagnosis.description(), "delegated description");
    assert_eq!(diagnosis.code(), None);
    assert_eq!(diagnosis.severity(), None);
    assert_eq!(diagnosis.help(), None);
    assert_eq!(diagnosis.url(), None);
    assert_eq!(
        diagnosis.source().map(IDiagnosis::description),
        Some("source")
    );
}

#[test]
fn derives_no_source_when_no_field_is_marked() {
    let diagnosis = DescriptionOnly { value: 1 };

    assert_eq!(diagnosis.value, 1);
    assert_eq!(diagnosis.description(), "description only");
    assert!(diagnosis.source().is_none());
}

#[test]
fn display_and_error_flags_preserve_source_chain() {
    let outer = OuterError {
        source: Some(InnerError::Inner),
    };
    assert_eq!(outer.to_string(), "outer");
    let source = core::error::Error::source(&outer).expect("outer source");
    assert_eq!(source.to_string(), "inner");

    let default = FlaggedError::Default;
    assert_eq!(default.to_string(), "default");
    assert!(core::error::Error::source(&default).is_none());

    let variant = FlaggedError::Variant(InnerError::Inner);
    assert_eq!(variant.to_string(), "variant");
    let source = core::error::Error::source(&variant).expect("variant source");
    assert_eq!(source.to_string(), "inner");
}

#[test]
fn derives_enum_variants_and_renders_with_simple_renderer() {
    let unit = EnumDiagnosis::Unit;
    assert_eq!(unit.description(), "unit");
    assert_eq!(unit.code(), Some("E_ENUM_DEFAULT"));
    assert_eq!(unit.severity(), Some(Severity::Advice));
    assert_eq!(unit.help(), Some("default help"));
    assert_eq!(unit.url(), Some("https://example.com/enum"));
    assert!(unit.source().is_none());

    let diagnosis = EnumDiagnosis::Tuple(
        String::from("tuple description"),
        MischiefError::builder("tuple source").build(),
    );
    assert_eq!(diagnosis.description(), "tuple description");
    assert_eq!(
        diagnosis.source().map(IDiagnosis::description),
        Some("tuple source")
    );

    let rendered = format!("{}", SimpleRenderer(&diagnosis));
    assert_eq!(
        rendered,
        "Error: tuple description\n\nCaused by:\n    tuple source\n"
    );

    let named = EnumDiagnosis::Named { source: None };
    assert_eq!(named.description(), "named");
    assert_eq!(named.code(), Some("E_ENUM_DEFAULT"));
    assert!(named.source().is_none());

    let forwarded = EnumDiagnosis::Forward(
        MischiefError::builder("forwarded")
            .code("E_FORWARDED")
            .build(),
    );
    assert_eq!(forwarded.description(), "forwarded");
    assert_eq!(forwarded.code(), Some("E_FORWARDED"));
}

struct SimpleRenderer<'a, D>(&'a D);

impl<D: IDiagnosis> core::fmt::Display for SimpleRenderer<'_, D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        mischief_core::render_diagnosis(self.0, f)
    }
}
