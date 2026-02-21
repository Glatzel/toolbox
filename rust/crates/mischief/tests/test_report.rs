use mischief::render_presets::*;
use mischief::render_protocol::*;
use mischief::{IntoMischief, Report, WrapErr, mischief};
struct Theme;
impl ITheme for Theme {
    fn width(&self) -> Option<usize> { Some(80) }
}
impl IIndent for Theme {
    fn get_indent(&self, layer: Layer, element: Item) -> (&'static str, &'static str) {
        DefaultIndent.get_indent(layer, element)
    }
}
impl IStyle for Theme {
    fn default_style(&self) -> Option<owo_colors::Style> { NoColorStyle.default_style() }
    fn indent_style(&self) -> Option<owo_colors::Style> { NoColorStyle.indent_style() }
    fn description_style(&self) -> Option<owo_colors::Style> { NoColorStyle.description_style() }
    fn severity_style(&self, severity: Option<mischief::Severity>) -> Option<owo_colors::Style> {
        NoColorStyle.severity_style(severity)
    }
    fn help_style(&self) -> (Option<owo_colors::Style>, Option<owo_colors::Style>) {
        NoColorStyle.help_style()
    }
    fn hyperlink_style(
        &self,
    ) -> (
        Option<owo_colors::Style>,
        mischief::render_protocol::HyperlinkFormat,
    ) {
        (None, HyperlinkFormat::Plain)
    }
}

fn render_report(report: &Report) -> String {
    let mut result = String::new();
    let render = DefaultRender::new(DefaultShader, Theme, TerminalConfig::default());
    render.render(&mut result, report.diagnostic()).unwrap();
    result
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
            insta::assert_snapshot!(render_report(&report))
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
            insta::assert_snapshot!(render_report(&report))
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
            insta::assert_snapshot!(render_report(&report))
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
