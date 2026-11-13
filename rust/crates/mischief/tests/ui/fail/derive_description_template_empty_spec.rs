use mischief::IDiagnosis;

#[derive(IDiagnosis)]
enum EmptySpec {
    #[diagnosis(description = "{0:}")]
    Value(u8),
}

fn main() {}
