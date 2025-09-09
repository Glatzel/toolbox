use owo_colors::Style;

use crate::Severity;
pub trait ITheme {
    fn default_theme(&self) -> Style;
    fn indent_theme(&self) -> Style;

    fn description_theme(&self) -> Style;
    fn code_theme(&self, severity: Option<Severity>) -> Style;
    fn severity_theme(&self, severity: Option<Severity>) -> Style;
    fn help_theme(&self) -> (Style, Style);
    fn url_theme(&self) -> Style;
}
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
