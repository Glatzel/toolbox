use core::fmt;

use mischief::{IDiagnosis, MischiefError, Severity};

#[derive(IDiagnosis)]
#[diagnosis(
    description = "static",
    code = "E_PASS",
    severity = Severity::Advice,
    help = "help",
    url = "https://example.com/errors/E_PASS",
)]
struct StaticDiagnosis {
    #[diagnosis(source)]
    source: Option<MischiefError>,
}

#[derive(IDiagnosis)]
#[diagnosis(description = message)]
struct DelegatedDiagnosis {
    message: String,
}

#[derive(IDiagnosis)]
#[diagnosis(description = "nested description")]
struct NestedDiagnosis {
    #[diagnosis(source)]
    source: StaticDiagnosis,
}

#[derive(IDiagnosis)]
#[diagnosis(description = "default", code = "E_ENUM")]
enum EnumDiagnosis {
    Unit,
    #[diagnosis(description_from = 0)]
    Tuple(String, #[diagnosis(source)] MischiefError),
    #[diagnosis(description_from = 0)]
    BoxedDescription(Box<str>),
    #[diagnosis(description_from = message)]
    StrDescription { message: &'static str },
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
#[diagnosis(error, description = "manual display")]
struct ManualDisplayError {
    #[diagnosis(source)]
    source: InnerError,
}

impl fmt::Display for ManualDisplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str("manual display") }
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
#[diagnosis(description = "reportable")]
struct Reportable {
    #[diagnosis(source(foreign))]
    source: std::io::Error,
}

fn main() {
    let static_diagnosis = StaticDiagnosis { source: None };
    let _ = (static_diagnosis.code(), static_diagnosis.source());

    let delegated = DelegatedDiagnosis {
        message: String::from("delegated"),
    };
    let _ = delegated.description();

    let nested = NestedDiagnosis {
        source: StaticDiagnosis { source: None },
    };
    let _ = nested.source();

    let variants = [
        EnumDiagnosis::Unit,
        EnumDiagnosis::Tuple(
            String::from("tuple"),
            MischiefError::builder("source").build(),
        ),
        EnumDiagnosis::BoxedDescription(String::from("boxed").into_boxed_str()),
        EnumDiagnosis::StrDescription { message: "str" },
        EnumDiagnosis::Named { source: None },
        EnumDiagnosis::Forward(MischiefError::builder("forwarded").build()),
    ];
    for variant in &variants {
        let _ = (
            variant.description(),
            variant.source(),
            variant.code(),
            variant.severity(),
            variant.help(),
            variant.url(),
        );
    }

    let outer = OuterError {
        source: Some(InnerError::Inner),
    };
    assert_eq!(outer.to_string(), "outer");
    assert!(core::error::Error::source(&outer).is_some());

    let manual = ManualDisplayError {
        source: InnerError::Inner,
    };
    assert_eq!(manual.to_string(), "manual display");
    assert!(core::error::Error::source(&manual).is_some());

    let foreign = ForeignSource::from(std::io::Error::other("io"));
    assert!(core::error::Error::source(&foreign).is_some());

    let from_only = FromOnly::from(1);
    assert!(core::error::Error::source(&from_only).is_none());

    let report = Reportable {
        source: std::io::Error::other("report"),
    }
    .into_report();
    assert!(report.downcast_ref::<std::io::Error>().is_some());
}
