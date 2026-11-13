use mischief::IDiagnosis;

#[derive(IDiagnosis)]
#[diagnosis(description = "unknown source mode")]
struct UnknownSource {
    #[diagnosis(source(other))]
    source: std::io::Error,
}

fn main() {}
