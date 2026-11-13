use mischief::IDiagnosis;

#[derive(IDiagnosis)]
#[diagnosis(code = "E_MISSING")]
struct MissingDescription {
    value: u8,
}

fn main() {}
