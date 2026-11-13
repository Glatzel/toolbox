use std::error::Error;
use std::fmt;

use clerk::ClerkError;
use mischief::IDiagnosis;

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error)]
pub enum KioyuError {
    #[diagnosis(transparent)]
    Clerk(#[diagnosis(from)] ClerkError),
    #[diagnosis(description = "I/O error")]
    IO(#[diagnosis(from)] std::io::Error),
    #[diagnosis(description = "Dispatcher closed.")]
    DispatcherClosed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transparent_io_error_preserves_display_and_source() {
        let error = KioyuError::from(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "fixed text",
        ));

        assert_eq!(error.to_string(), "fixed text");
        assert!(std::error::Error::source(&error).is_none());
    }

    #[test]
    fn transparent_clerk_error_preserves_display_and_source() {
        let error = KioyuError::from(clerk::ClerkError::ParentDirectoryNotFound(
            std::path::PathBuf::from("/tmp/missing"),
        ));

        assert_eq!(error.to_string(), "Parent directory not found");
        assert!(std::error::Error::source(&error).is_none());
    }

    #[test]
    fn dispatcher_closed_display_is_unchanged() {
        let error = KioyuError::DispatcherClosed;

        assert_eq!(error.to_string(), "Dispatcher closed.");
        assert!(std::error::Error::source(&error).is_none());
    }
}
