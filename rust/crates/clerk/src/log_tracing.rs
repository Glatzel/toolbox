mod fast_setup;
mod filter;
mod formatter;
mod layer;
pub use fast_setup::init_log_with_level;
pub use filter::level_filter;
pub use formatter::ClerkFormatter;
pub use layer::{file_layer, terminal_layer};
// re-export tracing
pub use tracing::{self, Level};
pub use tracing_core;
pub use tracing_subscriber;
