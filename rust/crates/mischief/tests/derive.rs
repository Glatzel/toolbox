#![cfg(feature = "macros")]
#![deny(unreachable_code)]

#[cfg(feature = "std")]
use mischief::render::*;
use mischief::{IDiagnosis, MischiefError, Severity};
use core::convert::Infallible;

#[cfg(feature = "std")]
struct NoTheme;

#[cfg(feature = "std")]
impl ITheme for NoTheme {
    fn default_style(&self) -> Option<&owo_colors::Style> { None }
    fn description_style(&self) -> Option<&owo_colors::Style> { None }
    fn severity_style(&self, _severity: Option<Severity>) -> Option<&owo_colors::Style> { None }
    fn help_style(&self) -> &(Option<owo_colors::Style>, Option<owo_colors::Style>) {
        &(None, None)
    }
    fn hyperlink_style(&self) -> &(Option<owo_colors::Style>, HyperlinkFormat) {
        &(None, HyperlinkFormat::Plain)
    }
}

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
struct DelegatedDiagnosis {
    #[diagnosis(description)]
    message: String,
    #[diagnosis(source)]
    source: MischiefError,
}

#[derive(Debug, IDiagnosis)]
#[diagnosis(display)]
struct FieldMetadataDiagnosis {
    #[diagnosis(description)]
    message: String,
    #[diagnosis(code)]
    code: String,
}

#[derive(IDiagnosis)]
struct OptionalCodeDiagnosis {
    #[diagnosis(description)]
    message: String,
    #[diagnosis(code)]
    code: Option<String>,
}

#[derive(IDiagnosis)]
#[diagnosis(description = "enum default", code = "E_DEFAULT")]
enum EnumFieldMetadataDiagnosis {
    WithFields {
        #[diagnosis(description)]
        message: String,
        #[diagnosis(code)]
        code: Option<&'static str>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, IDiagnosis)]
#[diagnosis(error, display, description = "Filter Error: {0}")]
struct FilterError(
    /// Description of the failure.
    pub String,
);

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error)]
struct TupleFieldMetadataDiagnosis(
    #[diagnosis(description)] String,
    #[diagnosis(code)] Option<String>,
);

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error, description = "tuple source")]
struct TupleSourceDiagnosis(#[diagnosis(source, from)] InnerError);

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error, description = "tuple second source")]
struct TupleSecondSourceDiagnosis(u8, #[diagnosis(source)] InnerError);

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error, transparent)]
struct TupleTransparentDiagnosis(InnerError);

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error)]
#[allow(dead_code, reason = "the uninhabited variant is a compile-time regression fixture")]
enum InfallibleDiagnosis {
    #[diagnosis(transparent(foreign))]
    Infallible(#[diagnosis(from)] Infallible),
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
    Tuple(
        #[diagnosis(description)] String,
        #[diagnosis(source)] MischiefError,
    ),
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

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error, description = "transparent default")]
enum TransparentDiagnosis {
    #[diagnosis(transparent)]
    Inner(InnerError),
}

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error, transparent)]
struct NamedTransparent {
    inner: InnerError,
}

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error, description = "foreign transparent")]
enum TransparentForeign {
    #[diagnosis(transparent(foreign))]
    Io(std::io::Error),
}

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error)]
enum TransparentForeignWithoutDescription {
    #[diagnosis(transparent(foreign))]
    Io(std::io::Error),
}

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error, description = "decode default", code = "E_DECODE")]
enum DecodeError {
    #[diagnosis(transparent(foreign))]
    Io(#[diagnosis(from)] std::io::Error),
    #[diagnosis(transparent)]
    Diagnostic(#[diagnosis(from)] InnerError),
    #[diagnosis(description = "invalid input")]
    Invalid,
}

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error, description = "verb error")]
struct GoldenVerbError {
    _marker: (),
}

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error, description = "integer parse error")]
struct GoldenParseIntError {
    _marker: (),
}

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error, description = "enum parse error")]
struct GoldenStrumError {
    _marker: (),
}

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error)]
enum GoldenNotationError {
    #[diagnosis(transparent)]
    Verb(#[diagnosis(from)] GoldenVerbError),
    #[diagnosis(transparent)]
    ParseInt(#[diagnosis(from)] GoldenParseIntError),
    #[diagnosis(transparent)]
    Strum(#[diagnosis(from)] GoldenStrumError),
    #[diagnosis(description = "Unknown pitch: {0}")]
    UnknownPitch(char),
    #[diagnosis(description = "Unknown accidental: {0}")]
    UnknownAccidental(char),
    #[diagnosis(description = "Invalid octave first char: {0}")]
    InvalidOctave(String),
    #[diagnosis(description = "Invalid note: {0}")]
    InvalidNote(String),
    #[diagnosis(description = "Invalid key: {0}")]
    InvalidKey(String),
    #[diagnosis(description = "Invalid key kind: {0}")]
    InvalidKeyKind(String),
}

#[derive(Debug, IDiagnosis)]
#[diagnosis(description = "structured {{value}} {value:?}")]
struct StructuredOnlyTemplate {
    value: u8,
}

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error)]
enum MixedTemplateError {
    #[diagnosis(transparent)]
    Transparent(OuterError),
    #[diagnosis(description = "number {0:?}")]
    Number(u8),
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
fn derives_description_field_and_source() {
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
fn derives_description_and_code_from_marked_fields() {
    let diagnosis = FieldMetadataDiagnosis {
        message: String::from("field description"),
        code: String::from("E_FIELD"),
    };

    assert_eq!(diagnosis.description(), "field description");
    assert_eq!(diagnosis.code(), Some("E_FIELD"));
    assert_eq!(diagnosis.to_string(), "field description");
}

#[test]
fn derives_optional_code_from_marked_field() {
    let with_code = OptionalCodeDiagnosis {
        message: String::from("optional code"),
        code: Some(String::from("E_OPTIONAL")),
    };
    assert_eq!(with_code.description(), "optional code");
    assert_eq!(with_code.code(), Some("E_OPTIONAL"));

    let without_code = OptionalCodeDiagnosis {
        message: String::from("no code"),
        code: None,
    };
    assert_eq!(without_code.code(), None);
}

#[test]
fn derives_enum_description_and_code_from_marked_fields() {
    let diagnosis = EnumFieldMetadataDiagnosis::WithFields {
        message: String::from("enum field"),
        code: Some("E_ENUM_FIELD"),
    };

    assert_eq!(diagnosis.description(), "enum field");
    assert_eq!(diagnosis.code(), Some("E_ENUM_FIELD"));
}

#[test]
fn derives_filter_error_for_tuple_struct() {
    let diagnosis = FilterError(String::from("invalid UTF-8"));

    assert_eq!(diagnosis.description(), "Filter Error: {0}");
    assert_eq!(diagnosis.to_string(), "Filter Error: invalid UTF-8");
    assert_eq!(
        diagnosis.clone(),
        FilterError(String::from("invalid UTF-8"))
    );
    assert!(core::error::Error::source(&diagnosis).is_none());

    let report = diagnosis.into_report();
    assert_eq!(report.error().description(), "Filter Error: invalid UTF-8");
}

#[test]
fn derives_description_and_code_from_tuple_fields() {
    let diagnosis = TupleFieldMetadataDiagnosis(
        String::from("tuple field description"),
        Some(String::from("E_TUPLE")),
    );

    assert_eq!(diagnosis.description(), "tuple field description");
    assert_eq!(diagnosis.code(), Some("E_TUPLE"));
    assert_eq!(diagnosis.to_string(), "tuple field description");
}

#[test]
fn derives_tuple_source_and_from_conversion() {
    let diagnosis = TupleSourceDiagnosis::from(InnerError::Inner);

    assert_eq!(diagnosis.description(), "tuple source");
    assert_eq!(
        diagnosis.source().map(IDiagnosis::description),
        Some("inner")
    );
    assert_eq!(
        core::error::Error::source(&diagnosis).map(ToString::to_string),
        Some(String::from("inner"))
    );
}

#[test]
fn derives_source_from_tuple_field_after_the_first() {
    let diagnosis = TupleSecondSourceDiagnosis(7, InnerError::Inner);

    assert_eq!(diagnosis.0, 7);
    assert_eq!(diagnosis.source().map(IDiagnosis::description), Some("inner"));
    assert_eq!(
        core::error::Error::source(&diagnosis).map(ToString::to_string),
        Some(String::from("inner"))
    );
}

#[test]
fn derives_transparent_tuple_struct() {
    let diagnosis = TupleTransparentDiagnosis(InnerError::Inner);

    assert_eq!(diagnosis.description(), "inner");
    assert_eq!(diagnosis.to_string(), "inner");
    assert_eq!(
        core::error::Error::source(&diagnosis).map(ToString::to_string),
        None
    );
    assert_eq!(diagnosis.into_report().error().description(), "inner");
}

#[test]
fn derives_infallible_error_without_unreachable_code() {
    fn assert_from<T: From<Infallible>>() {}

    assert_from::<InfallibleDiagnosis>();
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
fn transparent_variants_forward_display_and_source_without_wrapping() {
    let diagnosis = TransparentDiagnosis::Inner(InnerError::Inner);
    assert_eq!(diagnosis.to_string(), InnerError::Inner.to_string());
    assert_eq!(
        core::error::Error::source(&diagnosis).map(ToString::to_string),
        core::error::Error::source(&InnerError::Inner).map(ToString::to_string)
    );
    assert_report_render_snapshots("transparent_diagnosis", &diagnosis.into_report());

    let named = NamedTransparent {
        inner: InnerError::Inner,
    };
    assert_eq!(named.to_string(), InnerError::Inner.to_string());
    assert_eq!(
        core::error::Error::source(&named).map(ToString::to_string),
        core::error::Error::source(&InnerError::Inner).map(ToString::to_string)
    );
    assert_report_render_snapshots("transparent_named", &named.into_report());

    let foreign = TransparentForeign::Io(std::io::Error::other("foreign transparent"));
    assert_eq!(foreign.to_string(), "foreign transparent");
    assert!(core::error::Error::source(&foreign).is_none());
    assert_report_render_snapshots("transparent_foreign", &foreign.into_report());

    let fallback =
        TransparentForeignWithoutDescription::Io(std::io::Error::other("foreign fallback"));
    assert_eq!(fallback.description(), "Io");
    assert_eq!(fallback.to_string(), "foreign fallback");
    assert_eq!(
        format!("{}", mischief::DescriptionDisplay(&fallback)),
        "foreign fallback"
    );
    assert!(core::error::Error::source(&fallback).is_none());
    let report = fallback.into_report();
    assert_eq!(report.error().description(), "foreign fallback");
    assert_report_render_snapshots("transparent_foreign_fallback", &report);

    let decode_io = DecodeError::from(std::io::Error::other("decode io"));
    assert_eq!(decode_io.to_string(), "decode io");
    assert!(core::error::Error::source(&decode_io).is_none());
    let report = decode_io.into_report();
    assert_eq!(report.error().description(), "decode io");
    assert!(report.downcast_ref::<std::io::Error>().is_some());
    assert_report_render_snapshots("transparent_decode", &report);

    let decode_diagnosis = DecodeError::from(InnerError::Inner);
    assert_eq!(decode_diagnosis.to_string(), "inner");
    assert!(matches!(decode_diagnosis, DecodeError::Diagnostic(_)));

    let invalid = DecodeError::Invalid;
    assert_eq!(invalid.to_string(), "invalid input");
    assert!(core::error::Error::source(&invalid).is_none());
    assert_eq!(invalid.into_report().error().description(), "invalid input");
}

fn assert_report_render_snapshots(name: &str, report: &mischief::Report) {
    insta::assert_snapshot!(
        format!("{name}_simple"),
        format!("{}", SimpleRenderer(report.error()))
    );

    #[cfg(feature = "std")]
    insta::assert_snapshot!(
        format!("{name}_verbose"),
        format!(
            "{}",
            RenderBundle {
                diagnosis: report.error(),
                theme: NoTheme,
                indent: MischiefIndent::default(),
                #[cfg(feature = "textwrap")]
                width: 80,
            }
        )
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

#[test]
fn structured_description_does_not_require_display_flag() {
    let diagnosis = StructuredOnlyTemplate { value: 7 };

    assert_eq!(diagnosis.description(), "structured {{value}} {value:?}");
    assert_eq!(
        format!("{}", mischief::DescriptionDisplay(&diagnosis)),
        "structured {value} 7"
    );
    assert_eq!(
        diagnosis.into_report().error().description(),
        "structured {value} 7"
    );
}

#[test]
fn transparent_template_mix_preserves_forwarding() {
    let transparent = MixedTemplateError::Transparent(OuterError {
        source: Some(InnerError::Inner),
    });
    assert_eq!(transparent.to_string(), "outer");
    assert_eq!(
        core::error::Error::source(&transparent).map(ToString::to_string),
        Some(String::from("inner"))
    );
    assert_eq!(
        format!("{}", mischief::DescriptionDisplay(&transparent)),
        "outer"
    );

    let number = MixedTemplateError::Number(42);
    assert_eq!(number.description(), "number {0:?}");
    assert_eq!(number.to_string(), "number 42");
    assert_eq!(number.into_report().error().description(), "number 42");
}

#[test]
fn notation_templates_match_the_previous_public_behavior() {
    let cases = [
        (
            GoldenNotationError::Verb(GoldenVerbError { _marker: () }),
            "verb error",
            "verb error",
        ),
        (
            GoldenNotationError::ParseInt(GoldenParseIntError { _marker: () }),
            "integer parse error",
            "integer parse error",
        ),
        (
            GoldenNotationError::Strum(GoldenStrumError { _marker: () }),
            "enum parse error",
            "enum parse error",
        ),
        (
            GoldenNotationError::UnknownPitch('H'),
            "Unknown pitch: {0}",
            "Unknown pitch: H",
        ),
        (
            GoldenNotationError::UnknownAccidental('X'),
            "Unknown accidental: {0}",
            "Unknown accidental: X",
        ),
        (
            GoldenNotationError::InvalidOctave(String::from("9")),
            "Invalid octave first char: {0}",
            "Invalid octave first char: 9",
        ),
        (
            GoldenNotationError::InvalidNote(String::from("H#9")),
            "Invalid note: {0}",
            "Invalid note: H#9",
        ),
        (
            GoldenNotationError::InvalidKey(String::from("not a key")),
            "Invalid key: {0}",
            "Invalid key: not a key",
        ),
        (
            GoldenNotationError::InvalidKeyKind(String::from("mode")),
            "Invalid key kind: {0}",
            "Invalid key kind: mode",
        ),
    ];

    for (error, raw_description, display) in cases {
        assert_eq!(error.description(), raw_description);
        assert_eq!(error.to_string(), display);
        assert_eq!(format!("{}", mischief::DescriptionDisplay(&error)), display);
        assert!(core::error::Error::source(&error).is_none());
        assert_eq!(error.into_report().error().description(), display);
    }
}

#[test]
fn notation_template_reports_match_formatted_text() {
    assert_report_render_snapshots(
        "notation_template_verb",
        &GoldenNotationError::Verb(GoldenVerbError { _marker: () }).into_report(),
    );
    assert_report_render_snapshots(
        "notation_template_parse_int",
        &GoldenNotationError::ParseInt(GoldenParseIntError { _marker: () }).into_report(),
    );
    assert_report_render_snapshots(
        "notation_template_strum",
        &GoldenNotationError::Strum(GoldenStrumError { _marker: () }).into_report(),
    );
    assert_report_render_snapshots(
        "notation_template_unknown_pitch",
        &GoldenNotationError::UnknownPitch('H').into_report(),
    );
    assert_report_render_snapshots(
        "notation_template_unknown_accidental",
        &GoldenNotationError::UnknownAccidental('X').into_report(),
    );
    assert_report_render_snapshots(
        "notation_template_invalid_octave",
        &GoldenNotationError::InvalidOctave(String::from("9")).into_report(),
    );
    assert_report_render_snapshots(
        "notation_template_invalid_note",
        &GoldenNotationError::InvalidNote(String::from("H#9")).into_report(),
    );
    assert_report_render_snapshots(
        "notation_template_invalid_key",
        &GoldenNotationError::InvalidKey(String::from("not a key")).into_report(),
    );
    assert_report_render_snapshots(
        "notation_template_invalid_key_kind",
        &GoldenNotationError::InvalidKeyKind(String::from("mode")).into_report(),
    );
}

struct SimpleRenderer<'a, D>(&'a D);

impl<D: IDiagnosis> core::fmt::Display for SimpleRenderer<'_, D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        mischief_core::render_diagnosis(self.0, f)
    }
}
