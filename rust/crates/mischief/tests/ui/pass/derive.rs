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
}
