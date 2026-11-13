use mischief::IDiagnosis;

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error, description = "transparent")]
enum UnknownArgument {
    #[diagnosis(transparent(other))]
    Inner(std::io::Error),
}

fn main() {}
