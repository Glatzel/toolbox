use mischief::IDiagnosis;

#[derive(IDiagnosis)]
enum DuplicateKey {
    #[diagnosis(description = "first", description = "second")]
    Variant,
}

fn main() {}
