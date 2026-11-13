use mischief::IDiagnosis;

#[derive(IDiagnosis)]
#[diagnosis(description_from = 0)]
enum RemovedDescriptionFrom {
    Invalid(String),
}

fn main() {}
