extern crate alloc;
use crate::IDiagnostic;
/// Produces an iterator over the diagnostic chain.
fn chain(diagnostic: &impl crate::IDiagnostic) -> impl Iterator<Item = &dyn crate::IDiagnostic> {
    core::iter::successors(Some(diagnostic as &dyn crate::IDiagnostic), |r| r.source())
}
pub fn render_diagnostic<D: IDiagnostic>(
    diagnostic: &D,
    f: &mut core::fmt::Formatter<'_>,
) -> core::fmt::Result {
    let mut chain = chain(diagnostic);

    if let Some(first) = chain.next() {
        f.write_str(&alloc::format!("Error: {}", first.description()))?;
        writeln!(f)?;
    }
    let mut first = true;
    for diagnostic in chain {
        if first {
            f.write_str("\nCaused by:")?;
            writeln!(f)?;
            first = false;
        }
        f.write_str(&alloc::format!("    {}", diagnostic.description()))?;
        writeln!(f)?;
    }
    Ok(())
}
