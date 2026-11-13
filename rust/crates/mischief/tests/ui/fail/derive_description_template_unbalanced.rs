use mischief::IDiagnosis;

#[derive(IDiagnosis)]
enum Unbalanced {
    #[diagnosis(description = "{0")]
    Value(u8),
}

fn main() {}
