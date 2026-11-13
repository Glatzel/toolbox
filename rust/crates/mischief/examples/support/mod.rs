use core::fmt;

use mischief::IDiagnosis;

pub struct DiagnosisDisplay<'a, D>(pub &'a D);

impl<D: IDiagnosis> fmt::Display for DiagnosisDisplay<'_, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        mischief::render::render_diagnosis(self.0, f)
    }
}

pub fn print_diagnosis(label: &str, diagnosis: &impl IDiagnosis) {
    eprintln!("{label}:");
    eprint!("{}", DiagnosisDisplay(diagnosis));
}
