/// Configuration for terminal capabilities.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TerminalConfig {
    width: usize,
    support_color: bool,
    support_hyperlinks: bool,
    supports_unicode: bool,
}

impl TerminalConfig {
    /// Initializes a new `TerminalConfig` with detected terminal capabilities.
    pub fn init() -> Self {
        Self {
            width: Self::get_terminal_width(),
            support_color: supports_color::on(supports_color::Stream::Stdout).is_some(),
            support_hyperlinks: supports_hyperlinks::supports_hyperlinks(),
            supports_unicode: supports_unicode::supports_unicode(),
        }
    }

    /// Returns the width of the terminal in columns.
    fn get_terminal_width() -> usize {
        if let Some((terminal_size::Width(w), _)) = terminal_size::terminal_size() {
            w as usize
        } else {
            80
        }
    }

    /// Returns the detected terminal width.
    pub fn width(&self) -> usize { self.width }

    /// Returns whether the terminal supports color output.
    pub fn support_color(&self) -> bool { self.support_color }

    /// Returns whether the terminal supports hyperlinks.
    pub fn support_hyperlinks(&self) -> bool { self.support_hyperlinks }

    /// Returns whether the terminal supports Unicode characters.
    pub fn supports_unicode(&self) -> bool { self.supports_unicode }
}
