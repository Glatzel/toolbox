use mischief::IDiagnosis;

#[derive(IDiagnosis)]
enum InvalidSpec {
    #[diagnosis(description = "{0:not-a-format}")]
    Value(u8),
}

fn main() {}
