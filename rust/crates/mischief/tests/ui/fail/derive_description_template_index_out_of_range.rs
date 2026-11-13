use mischief::IDiagnosis;

#[derive(IDiagnosis)]
enum IndexOutOfRange {
    #[diagnosis(description = "{1}")]
    Value(u8),
}

fn main() {}
