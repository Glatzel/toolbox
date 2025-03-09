use std::sync::LazyLock;

use owo_colors::Style;

// console style
pub(crate) static TRACE_STYLE: LazyLock<Style> = LazyLock::new(|| Style::new().purple());
pub(crate) static DEBUG_STYLE: LazyLock<Style> = LazyLock::new(|| Style::new().blue());
pub(crate) static INFO_STYLE: LazyLock<Style> = LazyLock::new(|| Style::new().green());
pub(crate) static WARN_STYLE: LazyLock<Style> = LazyLock::new(|| Style::new().yellow().bold());
pub(crate) static ERROR_STYLE: LazyLock<Style> = LazyLock::new(|| Style::new().red().bold());
