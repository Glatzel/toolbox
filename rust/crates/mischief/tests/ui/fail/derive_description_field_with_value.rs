use mischief::IDiagnosis;

#[derive(IDiagnosis)]
struct DescriptionWithValue {
    #[diagnosis(description = "message")]
    message: String,
}

fn main() {}
