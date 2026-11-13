use mischief::IDiagnosis;

#[derive(Debug, IDiagnosis)]
#[diagnosis(description = "transparent")]
enum MissingFlags {
    #[diagnosis(transparent)]
    Inner(std::io::Error),
}

fn main() {}
