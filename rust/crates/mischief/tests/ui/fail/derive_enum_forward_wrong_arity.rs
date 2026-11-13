use mischief::IDiagnosis;

#[derive(IDiagnosis)]
enum ForwardWrongArity {
    #[diagnosis(forward)]
    Unit,
}

fn main() {}
