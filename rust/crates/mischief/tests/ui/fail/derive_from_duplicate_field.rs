use mischief::IDiagnosis;

#[derive(IDiagnosis)]
#[diagnosis(description = "duplicate from")]
enum DuplicateFrom {
    Variant(
        #[diagnosis(from)] u8,
        #[diagnosis(from)] u16,
    ),
}

fn main() {}
