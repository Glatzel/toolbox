use mischief::IDiagnosis;

#[derive(IDiagnosis)]
struct OptionalDescription {
    #[diagnosis(description)]
    message: Option<String>,
}

fn main() {}
