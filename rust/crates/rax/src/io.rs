mod reader;
pub use reader::*;

#[cfg(feature = "async")]
mod reader_async;
#[cfg(feature = "async")]
pub use reader_async::*;
