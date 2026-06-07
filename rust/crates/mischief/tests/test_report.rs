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
#[test]
fn report_error() {
    let e: Result<i32, mischief::Report> = Err("first error")
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
    match e {
        Ok(_) => unreachable!(),
        Err(report) => {
            #[cfg(feature = "fancy")]
            {
                let bundle = RenderBundle {
                    report: &report,
                    theme: MischiefTheme::default(),
                    indent: MischiefIndent::default(),
                    width: 80,
                };
                println!("{}", bundle);
                let bundle = RenderBundle {
                    report: &report,
                    theme: NoTheme,
                    indent: MischiefIndent::default(),
                    width: 80,
                };
                insta::assert_snapshot!(("report_error_fancy"), format!("{}", bundle))
            }
            #[cfg(not(feature = "fancy"))]
            {
                println!("{}", report);
                insta::assert_snapshot!(("report_error_no_fancy"), format!("{}", report))
            }
        }
    }
}

#[test]
fn report_error_long() {
    let e: Result<i32, mischief::Report> = Err("Failed to parse configuration file due to invalid syntax near line 42; unexpected token found that prevents correct interpretation of the settings provided.")
        .map_err(|e| {
            mischief!(
                "{}",
                e,
                severity = mischief::Severity::Warning,
                code = "E404",
                url = "https://github.com/Glatzel/toolbox",
                help = "To resolve this issue, please ensure all required dependencies are installed, network connectivity is stable, and configuration files are correctly formatted; you can enable verbose logging for more detailed diagnostic information."
            )
        }).wrap_err(mischief::mischief!("Connection to the database server timed out after multiple retries, possibly caused by network instability, firewall restrictions, or an unreachable endpoint.",
        help="Network resource unavailable. Please check your connection or file permissions, and try again. Contact support with code 404-B.",severity = mischief::Severity::Error, code = "E502",))
        .wrap_err_with(|| "Attempted to access a resource that is not available in the current execution environment, which may indicate missing dependencies, restricted permissions, or an incorrect build target.");
    match e {
        Ok(_) => unreachable!(),
        Err(report) => {
            #[cfg(feature = "fancy")]
            {
                let bundle = RenderBundle {
                    report: &report,
                    theme: MischiefTheme::default(),
                    indent: MischiefIndent::default(),
                    width: 80,
                };
                println!("{}", bundle);
                let bundle = RenderBundle {
                    report: &report,
                    theme: NoTheme,
                    indent: MischiefIndent::default(),
                    width: 80,
                };
                insta::assert_snapshot!(("report_error_long_fancy"), format!("{}", bundle))
            }
            #[cfg(not(feature = "fancy"))]
            {
                println!("{}", report);
                insta::assert_snapshot!(("report_error_long_no_fancy"), format!("{}", report))
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
            #[cfg(feature = "fancy")]
            {
                let bundle = RenderBundle {
                    report: &report,
                    theme: MischiefTheme::default(),
                    indent: MischiefIndent::default(),
                    width: 80,
                };
                println!("{}", bundle);
                let bundle = RenderBundle {
                    report: &report,
                    theme: NoTheme,
                    indent: MischiefIndent::default(),
                    width: 80,
                };
                insta::assert_snapshot!(("report_from_error_fancy"), format!("{}", bundle))
            }
            #[cfg(not(feature = "fancy"))]
            {
                println!("{}", report);
                insta::assert_snapshot!(("report_from_error_no_fancy"), format!("{}", report))
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
