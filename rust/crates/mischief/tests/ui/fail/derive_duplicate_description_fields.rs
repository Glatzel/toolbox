use mischief::IDiagnosis;

#[derive(IDiagnosis)]
struct DuplicateDescription {
    #[diagnosis(description)]
    first: String,
    #[diagnosis(description)]
    second: String,
}

fn main() {}
