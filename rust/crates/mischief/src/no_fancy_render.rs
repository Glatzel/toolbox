extern crate alloc;

use crate::IDiagnostic;

/// Produces an iterator over the diagnostic chain.
///
/// This function walks through a diagnostic and all of its sources,
/// yielding each diagnostic in sequence. The iteration begins with
/// the provided diagnostic and continues through the chain returned
/// by successive calls to [`IDiagnostic::source`].
///
/// The resulting iterator represents the full causal chain of the
/// diagnostic, ordered from the outermost diagnostic to the deepest
/// underlying source.
fn chain(diagnostic: &impl crate::IDiagnostic) -> impl Iterator<Item = &dyn crate::IDiagnostic> {
    core::iter::successors(Some(diagnostic as &dyn crate::IDiagnostic), |r| r.source())
}

/// Renders a diagnostic chain in a simple textual format.
///
/// The first diagnostic in the chain is rendered as the primary error,
/// prefixed with `"Error:"`. Any subsequent diagnostics in the chain
/// are treated as causes and displayed under a `"Caused by:"` section.
///
/// Each diagnostic in the causal chain is displayed on its own line
/// using the description returned by [`IDiagnostic::description`].
///
/// This renderer provides a minimal, dependency-free textual
/// representation of diagnostic chains and can serve as a fallback
/// when more advanced rendering systems are not required.
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
