use mischief::render::*;
use mischief::{IntoMischief, WrapErr, mischief};

#[cfg(feature = "std")]
struct NoTheme;

#[cfg(feature = "std")]
impl ITheme for NoTheme {
    fn default_style(&self) -> Option<&owo_colors::Style> { None }
    fn description_style(&self) -> Option<&owo_colors::Style> { None }
    fn severity_style(&self, _severity: Option<mischief::Severity>) -> Option<&owo_colors::Style> {
        None
    }
    fn help_style(&self) -> &(Option<owo_colors::Style>, Option<owo_colors::Style>) {
        &(None, None)
    }
    fn hyperlink_style(&self) -> &(Option<owo_colors::Style>, HyperlinkFormat) {
        &(None, HyperlinkFormat::Plain)
    }
}

#[cfg(feature = "std")]
fn assert_verbose_snapshot(name: &str, report: &mischief::Report) {
    let bundle = RenderBundle {
        diagnosis: report.error(),
        theme: NoTheme,
        indent: MischiefIndent::default(),
        #[cfg(feature = "textwrap")]
        width: 80,
    };
    let rendered = format!("{bundle}");
    assert!(
        rendered.starts_with("Error: "),
        "verbose rendering should label the root: {rendered}"
    );
    let report_rendered = format!("{report}");
    assert!(
        !report_rendered.contains("Caused by:"),
        "Report Display should select verbose rendering with std: {report_rendered}"
    );

    insta::assert_snapshot!(name, rendered);
}

#[cfg(not(feature = "std"))]
fn assert_simple_snapshot(name: &str, report: &mischief::Report) {
    println!("{}", report);
    insta::assert_snapshot!(name, format!("{}", report));
}

#[test]
fn report_error() {
    let result: mischief::Result<()> = Err("first error")
        .map_err(|e| {
            mischief!(
                "{}",
                e,
                severity = mischief::Severity::Warning,
                code = "E404",
                url = "https://github.com/Glatzel/toolbox",
                help = "Try again."
            )
        })
        .wrap_err("Second error")
        .wrap_err_with(|| "Third error");
    match result {
        Ok(_) => unreachable!(),
        Err(report) => {
            #[cfg(feature = "std")]
            {
                assert_verbose_snapshot(
                    if cfg!(feature = "fancy") {
                        "report_error_fancy"
                    } else {
                        "report_error_std"
                    },
                    &report,
                );
            }
            #[cfg(not(feature = "std"))]
            {
                assert_simple_snapshot("report_error_no_fancy", &report);
            }
        }
    }
}

#[test]
fn report_error_long() {
    let result: mischief::Result<()> = Err("Failed to parse configuration file due to invalid syntax near line 42; unexpected token found that prevents correct interpretation of the settings provided.")
        .map_err(|e| {
            mischief!(
                "{}",
                e,
                severity = mischief::Severity::Warning,
                code = "E404",
                url = "https://github.com/Glatzel/toolbox",
                help = "To resolve this issue, please ensure all required dependencies are installed, network connectivity is stable, and configuration files are correctly formatted; you can enable verbose logging for more detailed diagnostic information."
            )
        })
        .wrap_err(mischief!(
            "Connection to the database server timed out after multiple retries, possibly caused by network instability, firewall restrictions, or an unreachable endpoint.",
            help = "Network resource unavailable. Please check your connection or file permissions, and try again. Contact support with code 404-B.",
            severity = mischief::Severity::Error,
            code = "E502",
        ))
        .wrap_err_with(|| "Attempted to access a resource that is not available in the current execution environment, which may indicate missing dependencies, restricted permissions, or an incorrect build target.");
    match result {
        Ok(_) => unreachable!(),
        Err(report) => {
            #[cfg(feature = "std")]
            {
                assert_verbose_snapshot(
                    if cfg!(feature = "fancy") {
                        "report_error_long_fancy"
                    } else {
                        "report_error_long_std"
                    },
                    &report,
                );
            }
            #[cfg(not(feature = "std"))]
            {
                assert_simple_snapshot("report_error_long_no_fancy", &report);
            }
        }
    }
}

#[test]
fn report_from_error() -> mischief::Result<()> {
    use std::fmt;

    #[derive(Debug)]
    pub struct FakeError;

    impl fmt::Display for FakeError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "fake error") }
    }

    impl std::error::Error for FakeError {}
    let f: Result<(), FakeError> = Err(FakeError);
    let f = f.into_mischief().wrap_err("error wrapper");
    match f {
        Ok(_) => unreachable!(),
        Err(report) => {
            assert!(report.downcast_ref::<FakeError>().is_some());

            #[cfg(feature = "std")]
            {
                assert_verbose_snapshot(
                    if cfg!(feature = "fancy") {
                        "report_from_error_fancy"
                    } else {
                        "report_from_error_std"
                    },
                    &report,
                );
            }
            #[cfg(not(feature = "std"))]
            {
                assert_simple_snapshot("report_from_error_no_fancy", &report);
            }
        }
    }
    Ok(())
}

#[test]
fn report_ok() -> mischief::Result<()> {
    Ok::<i32, mischief::Result<()>>(2i32)
        .map_err(|e| mischief!("{:?}", e))
        .wrap_err("Second error")
        .wrap_err_with(|| "Third error")?;
    Ok(())
}
