#[cfg(feature = "fancy")]
use mischief::fancy_render::*;
use mischief::{IntoMischief, WrapErr, mischief};

#[cfg(feature = "fancy")]
struct NoTheme;

#[cfg(feature = "fancy")]
impl ITheme for NoTheme {
    fn default_style(&self) -> &Option<owo_colors::Style> { &None }
    fn description_style(&self) -> &Option<owo_colors::Style> { &None }
    fn severity_style(&self, _severity: Option<mischief::Severity>) -> &Option<owo_colors::Style> {
        &None
    }
    fn help_style(&self) -> &(Option<owo_colors::Style>, Option<owo_colors::Style>) {
        &(None, None)
    }
    fn hyperlink_style(
        &self,
    ) -> &(
        Option<owo_colors::Style>,
        mischief::fancy_render::HyperlinkFormat,
    ) {
        &(None, HyperlinkFormat::Plain)
    }
}

#[cfg(feature = "fancy")]
fn assert_fancy_snapshot(name: &str, report: &mischief::Report) {
    let bundle = RenderBundle {
        report,
        theme: MischiefTheme::default(),
        indent: MischiefIndent::default(),
        width: 80,
    };
    println!("{}", bundle);
    let bundle = RenderBundle {
        report,
        theme: NoTheme,
        indent: MischiefIndent::default(),
        width: 80,
    };
    insta::assert_snapshot!(name, format!("{}", bundle));
}

#[cfg(not(feature = "fancy"))]
fn assert_no_fancy_snapshot(name: &str, report: &mischief::Report) {
    println!("{}", report);
    insta::assert_snapshot!(name, format!("{}", report));
}

fn check_report(name: &str, result: Result<i32, mischief::Report>) {
    let report = result.unwrap_err();
    #[cfg(feature = "fancy")]
    assert_fancy_snapshot(&format!("{}_fancy", name), &report);
    #[cfg(not(feature = "fancy"))]
    assert_no_fancy_snapshot(&format!("{}_no_fancy", name), &report);
}

#[test]
fn report_error() {
    let result = Err("first error")
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
    check_report("report_error", result);
}

#[test]
fn report_error_long() {
    let result = Err("Failed to parse configuration file due to invalid syntax near line 42; unexpected token found that prevents correct interpretation of the settings provided.")
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
    check_report("report_error_long", result);
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

    let report = Err::<(), FakeError>(FakeError)
        .into_mischief()
        .wrap_err("error wrapper")
        .unwrap_err();
    #[cfg(feature = "fancy")]
    assert_fancy_snapshot("report_from_error_fancy", &report);
    #[cfg(not(feature = "fancy"))]
    assert_no_fancy_snapshot("report_from_error_no_fancy", &report);

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
