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
pub struct Theme {
    default_theme: Style,
    indent_theme: Style,
    description_theme: Style,
    severity_theme: (Style, Style, Style),
    help_theme: (Style, Style),
    url_theme: Style,
}
impl Default for Theme {
    fn default() -> Self {
        Self {
            default_theme: Style::default(),
            indent_theme: Style::new().red(),
            description_theme: Style::default(),
            severity_theme: (
                Style::new().green(),
                Style::new().yellow(),
                Style::new().red(),
            ),
            help_theme: (Style::new().cyan(), Style::default()),
            url_theme: Style::new().blue(),
        }
    }
}
impl ITheme for Theme {
    fn default_theme(&self) -> Style { self.default_theme }
    fn indent_theme(&self) -> Style { self.indent_theme }

    fn description_theme(&self) -> Style { self.description_theme }
    fn code_theme(&self, _severity: Option<Severity>) -> Style { unimplemented!() }
    fn severity_theme(&self, severity: Option<Severity>) -> Style {
        match severity {
            Some(Severity::Advice) => self.severity_theme.0,
            Some(Severity::Warning) => self.severity_theme.1,
            Some(Severity::Error) => self.severity_theme.2,
            None => self.default_theme,
        }
    }
    fn help_theme(&self) -> (Style, Style) { self.help_theme }
    fn url_theme(&self) -> Style { self.url_theme }
}
