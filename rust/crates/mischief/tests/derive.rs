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

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error, description = "foreign source")]
struct ForeignSource {
    #[diagnosis(source(foreign), from)]
    source: std::io::Error,
}

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error, description = "from only")]
struct FromOnly {
    #[diagnosis(from)]
    value: u8,
}

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error, description = "foreign enum")]
enum ForeignEnum {
    #[diagnosis(description = "io variant")]
    Io(#[diagnosis(source(foreign), from)] std::io::Error),
    #[diagnosis(description = "parse variant")]
    Parse(#[diagnosis(from)] std::num::ParseIntError),
}

#[derive(Debug, IDiagnosis)]
#[diagnosis(description = "report inner")]
enum ReportInner {
    Inner,
}

#[derive(Debug, IDiagnosis)]
#[diagnosis(
    description = "report outer",
    code = "E_REPORT_OUTER",
    severity = Severity::Warning,
    help = "retry",
    url = "https://example.com/E_REPORT_OUTER"
)]
struct ReportableError {
    #[diagnosis(source)]
    source: ReportInner,
}

#[derive(Debug, IDiagnosis)]
#[diagnosis(description = "foreign report")]
struct ForeignReportableError {
    #[diagnosis(source(foreign))]
    source: std::io::Error,
}

#[derive(Debug, IDiagnosis)]
#[diagnosis(description = "report enum")]
enum ReportableEnum {
    #[diagnosis(description = "diagnosis variant")]
    Diagnosis(#[diagnosis(source)] ReportInner),
    #[diagnosis(description = "foreign variant")]
    Foreign(#[diagnosis(source(foreign))] std::io::Error),
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
fn foreign_sources_and_from_conversions_work_independently() {
    let foreign = ForeignSource::from(std::io::Error::other("io failure"));
    assert!(IDiagnosis::source(&foreign).is_none());
    assert!(core::error::Error::source(&foreign).is_some());

    let from_only = FromOnly::from(7);
    assert_eq!(from_only.value, 7);
    assert!(core::error::Error::source(&from_only).is_none());

    let io = ForeignEnum::Io(std::io::Error::other("enum io"));
    assert!(IDiagnosis::source(&io).is_none());
    assert!(core::error::Error::source(&io).is_some());

    let parse = ForeignEnum::from("not a number".parse::<u8>().expect_err("invalid integer"));
    assert!(matches!(parse, ForeignEnum::Parse(_)));
}

#[test]
fn into_report_preserves_metadata_and_sources() {
    let report = ReportableError {
        source: ReportInner::Inner,
    }
    .into_report();
    assert_eq!(report.error().description(), "report outer");
    assert_eq!(report.error().code(), Some("E_REPORT_OUTER"));
    assert_eq!(
        report.error().source().map(IDiagnosis::description),
        Some("report inner")
    );

    let foreign = ForeignReportableError {
        source: std::io::Error::other("foreign report"),
    }
    .into_report();
    assert!(foreign.downcast_ref::<std::io::Error>().is_some());

    let enum_report = ReportableEnum::Foreign(std::io::Error::other("enum report")).into_report();
    assert_eq!(enum_report.error().description(), "foreign variant");
    assert!(enum_report.downcast_ref::<std::io::Error>().is_some());

    let diagnosis_report = ReportableEnum::Diagnosis(ReportInner::Inner).into_report();
    assert_eq!(diagnosis_report.error().description(), "diagnosis variant");
    assert_eq!(
        diagnosis_report
            .error()
            .source()
            .map(IDiagnosis::description),
        Some("report inner")
    );
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
