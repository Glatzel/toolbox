use mischief::{IDiagnosis, MischiefError};

#[derive(IDiagnosis)]
enum MultipleSources {
    #[diagnosis(description = "multiple")]
    Variant(
        #[diagnosis(source)] MischiefError,
        #[diagnosis(source)] MischiefError,
    ),
}

fn main() {}
