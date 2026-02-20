use std::fs::File;

#[cfg(feature = "fancy")]
use mischief::render::{DefaultIndent, HyperlinkFormat, IIndent, IStyle, ITheme, NoColorStyle};
use mischief::render::{IRender, Render};
use mischief::{IntoMischief, Report, WrapErr, mischief};
#[cfg(feature = "fancy")]
struct Theme;
#[cfg(feature = "fancy")]
impl ITheme for Theme {
    fn width(&self) -> Option<usize> { Some(80) }
}
#[cfg(feature = "fancy")]
impl IIndent for Theme {
    fn get_indent(
        &self,
        layer: mischief::render::Layer,
        element: mischief::render::Item,
    ) -> (&'static str, &'static str) {
        DefaultIndent.get_indent(layer, element)
    }
}
#[cfg(feature = "fancy")]
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
    fn hyperlink_style(&self) -> (Option<owo_colors::Style>, mischief::render::HyperlinkFormat) {
        (None, HyperlinkFormat::Plain)
    }
}

fn render_report(report: &Report) -> String {
    let mut result = String::new();
    #[cfg(feature = "fancy")]
    {
        let render = Render::new(Theme);
        render.render(&mut result, report.diagnostic()).unwrap();
        result
    }
    #[cfg(not(feature = "fancy"))]
    {
        let render = Render::new();
        render.render(&mut result, report.diagnostic()).unwrap();
        result
    }
}
fn snapshot_file_name(name: &str) -> String {
    if cfg!(feature = "fancy") {
        format!("{name}_fancy")
    } else {
        format!("{name}_no_fancy")
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
            insta::assert_snapshot!(snapshot_file_name("report_error"), render_report(&report))
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
            insta::assert_snapshot!(
                snapshot_file_name("report_error_long"),
                render_report(&report)
            )
        }
    }
}
#[test]
fn report_from_error() -> mischief::Result<()> {
    let f = File::open("fake").into_mischief().wrap_err("error wrapper");
    match f {
        Ok(_) => unreachable!(),
        Err(report) => {
            insta::assert_snapshot!(
                snapshot_file_name("report_from_error"),
                render_report(&report)
            )
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
