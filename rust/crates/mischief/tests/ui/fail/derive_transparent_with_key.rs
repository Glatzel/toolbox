use mischief::IDiagnosis;

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error, description = "transparent")]
enum TransparentWithKey {
    #[diagnosis(transparent, description = "inner")]
    Inner(std::io::Error),
}

fn main() {}
