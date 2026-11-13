use mischief::IDiagnosis;

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error, description = "transparent")]
enum TransparentWrongArity {
    #[diagnosis(transparent)]
    Unit,
}

fn main() {}
