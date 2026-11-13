use mischief::IDiagnosis;

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error)]
enum MissingDefault {
    #[diagnosis(transparent(foreign))]
    Io(std::io::Error),
}

fn main() {}
