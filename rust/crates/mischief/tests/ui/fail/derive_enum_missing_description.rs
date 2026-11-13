use mischief::IDiagnosis;

#[derive(IDiagnosis)]
#[diagnosis(code = "E_DEFAULT")]
enum MissingDescription {
    Missing,
}

fn main() {}
