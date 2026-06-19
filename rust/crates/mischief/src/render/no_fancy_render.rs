extern crate alloc;
use crate::IDiagnosis;

/// Produces an iterator over the diagnosis chain.
///
/// This function walks through a diagnosis and all of its sources,
/// yielding each diagnosis in sequence. The iteration begins with
/// the provided diagnosis and continues through the chain returned
/// by successive calls to [`IDiagnosis::source`].
///
/// The resulting iterator represents the full causal chain of the
/// diagnosis, ordered from the outermost diagnosisto the deepest
/// underlying source.
fn chain(diagnosis: &impl crate::IDiagnosis) -> impl Iterator<Item = &dyn crate::IDiagnosis> {
    core::iter::successors(Some(diagnosis as &dyn crate::IDiagnosis), |r| r.source())
}

/// Renders a diagnosis chain in a simple textual format.
///
/// The first diagnosis in the chain is rendered as the primary error,
/// prefixed with `"Error:"`. Any subsequent diagnosis in the chain
/// are treated as causes and displayed under a `"Caused by:"` section.
///
/// Each diagnosis in the causal chain is displayed on its own line
/// using the description returned by [`IDiagnosis::description`].
///
/// This renderer provides a minimal, dependency-free textual
/// representation of diagnosis chains and can serve as a fallback
/// when more advanced rendering systems are not required.
pub fn render_diagnosis<D: IDiagnosis>(
    diagnosis: &D,
    f: &mut core::fmt::Formatter<'_>,
) -> core::fmt::Result {
    let mut chain = chain(diagnosis);

    if let Some(first) = chain.next() {
        f.write_str(&alloc::format!("Error: {}", first.description()))?;
        writeln!(f)?;
    }

    let mut first = true;

    for diagnosis in chain {
        if first {
            f.write_str("\nCaused by:")?;
            writeln!(f)?;
            first = false;
        }

        f.write_str(&alloc::format!("    {}", diagnosis.description()))?;
        writeln!(f)?;
    }

    Ok(())
}
#[cfg(all(feature = "backtrace", debug_assertions))]
pub fn render_backtrace(
    backtrace: &backtrace::Backtrace,
    f: &mut core::fmt::Formatter<'_>,
) -> core::fmt::Result {
    writeln!(f, "Backtrace:\n{:?}", backtrace)
}
