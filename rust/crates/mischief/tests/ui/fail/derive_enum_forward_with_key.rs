use mischief::IDiagnosis;

#[derive(IDiagnosis)]
enum ForwardWithKey {
    #[diagnosis(forward, description = "also set")]
    Variant(u8),
}

fn main() {}
