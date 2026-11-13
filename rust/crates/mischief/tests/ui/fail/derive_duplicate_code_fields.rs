use mischief::IDiagnosis;

#[derive(IDiagnosis)]
struct DuplicateCode {
    #[diagnosis(description)]
    message: String,
    #[diagnosis(code)]
    first: String,
    #[diagnosis(code)]
    second: String,
}

fn main() {}
