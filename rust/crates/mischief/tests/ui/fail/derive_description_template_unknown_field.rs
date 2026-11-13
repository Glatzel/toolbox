use mischief::IDiagnosis;

#[derive(IDiagnosis)]
enum UnknownField {
    #[diagnosis(description = "{missing}")]
    Record { value: u8 },
}

fn main() {}
