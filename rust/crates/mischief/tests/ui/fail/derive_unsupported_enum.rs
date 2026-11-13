use mischief::IDiagnosis;

#[derive(IDiagnosis)]
#[diagnosis(description = "enum")]
enum Unsupported {
    Variant,
}

fn main() {}
