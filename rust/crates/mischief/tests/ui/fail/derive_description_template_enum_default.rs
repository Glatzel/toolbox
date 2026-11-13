use mischief::IDiagnosis;

#[derive(IDiagnosis)]
#[diagnosis(description = "default {0}")]
enum EnumDefaultPlaceholder {
    Value(u8),
}

fn main() {}
