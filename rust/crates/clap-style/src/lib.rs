use clap::builder::styling::{AnsiColor, Effects, Styles};

pub fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .usage(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Green.on_default())
        .error(AnsiColor::Red.on_default() | Effects::BOLD)
        .valid(AnsiColor::Green.on_default() | Effects::BOLD)
        .invalid(AnsiColor::Red.on_default() | Effects::BOLD)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_styles_snapshot() {
        let styles = styles();
        insta::assert_debug_snapshot!(styles);
    }
}
