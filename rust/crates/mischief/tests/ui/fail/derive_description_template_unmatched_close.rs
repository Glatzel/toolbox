use mischief::IDiagnosis;

#[derive(IDiagnosis)]
enum UnmatchedClose {
    #[diagnosis(description = "value }")]
    Value,
}

fn main() {}
