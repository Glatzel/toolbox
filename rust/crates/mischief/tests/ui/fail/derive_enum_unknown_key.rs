use mischief::IDiagnosis;

#[derive(IDiagnosis)]
enum UnknownKey {
    #[diagnosis(description = "variant", detail = "unsupported")]
    Variant,
}

fn main() {}
