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
}
