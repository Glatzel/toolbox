#[cfg(feature = "log")]
mod constants;
#[cfg(feature = "log")]
mod file;
#[cfg(feature = "log")]
mod formatter;
#[cfg(feature = "log")]
mod term;
#[cfg(feature = "log")]
use constants::*;
#[cfg(feature = "log")]
pub use file::file_layer;
#[cfg(feature = "log")]
use formatter::ClerkFormatter;
#[cfg(feature = "log")]
pub use term::terminal_layer;
#[cfg(feature = "log")]
 
mod macros;
pub use fast_setup::init_log_with_level;
mod fast_setup;
