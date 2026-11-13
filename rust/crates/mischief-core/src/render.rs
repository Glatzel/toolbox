use crate::{DescriptionDisplay, IDiagnosis};

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
/// using [`IDiagnosis::fmt_description`].
///
/// This renderer provides a minimal, dependency-free textual
/// representation of diagnosis chains and can serve as a fallback
/// when more advanced rendering systems are not required.
pub fn render_diagnosis<D>(diagnosis: &D, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
where
    D: IDiagnosis,
{
    let mut chain = chain(diagnosis);

    if let Some(first) = chain.next() {
        write!(f, "Error: {}", DescriptionDisplay(first))?;
        writeln!(f)?;
    }

    let mut first = true;

    for diagnosis in chain {
        if first {
            f.write_str("\nCaused by:")?;
            writeln!(f)?;
            first = false;
        }

        write!(f, "    {}", DescriptionDisplay(diagnosis))?;
        writeln!(f)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;
    use crate::Severity;

    #[derive(Debug)]
    struct FormattedDiagnosis;

    impl IDiagnosis for FormattedDiagnosis {
        fn description(&self) -> &str { "borrowed description" }

        fn fmt_description(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str("formatted description")
        }

        fn source(&self) -> Option<&dyn IDiagnosis> { None }

        fn code(&self) -> Option<&str> { None }

        fn severity(&self) -> Option<Severity> { None }

        fn help(&self) -> Option<&str> { None }

        fn url(&self) -> Option<&str> { None }
    }

    struct Renderer(FormattedDiagnosis);

    impl core::fmt::Display for Renderer {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            render_diagnosis(&self.0, f)
        }
    }

    #[test]
    fn simple_renderer_uses_fmt_description() {
        assert_eq!(
            Renderer(FormattedDiagnosis).to_string(),
            "Error: formatted description\n"
        );
    }
}
