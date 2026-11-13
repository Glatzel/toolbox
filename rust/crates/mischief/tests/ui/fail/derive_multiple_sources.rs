use mischief::{IDiagnosis, MischiefError};

#[derive(IDiagnosis)]
#[diagnosis(description = "multiple sources")]
struct MultipleSources {
    #[diagnosis(source)]
    primary: MischiefError,
    #[diagnosis(source)]
    secondary: MischiefError,
}

fn main() {}
