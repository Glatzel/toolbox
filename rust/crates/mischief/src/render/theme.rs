use owo_colors::Style;

use crate::Severity;

/// Trait defining styling for different components of diagnostic output.
pub trait ITheme {
    /// Style for default text.
    fn default_theme(&self) -> Style;

    /// Style for indentation lines.
    fn indent_theme(&self) -> Style;

    /// Style for error or diagnostic descriptions.
    fn description_theme(&self) -> Style;

    /// Style for code snippets, optionally depending on severity.
    fn code_theme(&self, severity: Option<Severity>) -> Style;

    /// Style for severity labels.
    fn severity_theme(&self, severity: Option<Severity>) -> Style;

    /// Style for help text: returns a tuple of `(prefix_style, message_style)`.
    fn help_theme(&self) -> (Style, Style);

    /// Style for URLs.
    fn url_theme(&self) -> Style;
}

/// Default theme implementation using `owo_colors`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Theme;

impl ITheme for Theme {
    fn default_theme(&self) -> Style { Style::default() }

    fn indent_theme(&self) -> Style { Style::new().red() }

    fn description_theme(&self) -> Style { Style::default() }

    fn code_theme(&self, _severity: Option<Severity>) -> Style { unimplemented!() }

    fn severity_theme(&self, severity: Option<Severity>) -> Style {
        match severity {
            Some(Severity::Advice) => Style::new().green(),
            Some(Severity::Warning) => Style::new().yellow(),
            Some(Severity::Error) => Style::new().red(),
            None => Style::default(),
        }
    }

    fn help_theme(&self) -> (Style, Style) { (Style::new().cyan(), Style::default()) }

    fn url_theme(&self) -> Style { Style::new().blue() }
}
