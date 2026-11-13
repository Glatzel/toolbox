use mischief::IDiagnosis;

#[derive(IDiagnosis)]
#[diagnosis(description = "duplicate from type")]
enum DuplicateFromType {
    First(#[diagnosis(from)] u8),
    Second(#[diagnosis(from)] u8),
}

fn main() {}
