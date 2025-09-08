pub struct TerminalConfig {
    width: usize,
    support_color: bool,
    support_hyperlinks: bool,
    supports_unicode: bool,
}

impl TerminalConfig {
    pub fn init() -> Self {
        Self {
            width: Self::get_terminal_width(),
            support_color: supports_color::on(supports_color::Stream::Stdout).is_some(),
            support_hyperlinks: supports_hyperlinks::supports_hyperlinks(),
            supports_unicode: supports_unicode::supports_unicode(),
        }
    }
    fn get_terminal_width() -> usize {
        if let Some((terminal_size::Width(w), _)) = terminal_size::terminal_size() {
            w as usize
        } else {
            80
        }
    }
    pub fn width(&self) -> usize { self.width }
    pub fn support_color(&self) -> bool { self.support_color }
    pub fn support_hyperlinks(&self) -> bool { self.support_hyperlinks }
    pub fn supports_unicode(&self) -> bool { self.supports_unicode }
}
