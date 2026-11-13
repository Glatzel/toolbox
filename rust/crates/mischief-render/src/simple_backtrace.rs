use core::fmt;

use backtrace::Backtrace;

pub fn render_backtrace_plain(backtrace: &Backtrace, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    writeln!(f, "Backtrace:\n{backtrace:?}")
}
