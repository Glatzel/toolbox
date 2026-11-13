use std::error::Error;
use std::fmt;

use clerk::ClerkError;
use mischief::IDiagnosis;

#[derive(Debug, IDiagnosis)]
pub enum KioyuError {
    #[diagnosis(description = "logging setup error")]
    Clerk(#[diagnosis(source, from)] ClerkError),
    #[diagnosis(description = "I/O error")]
    IO(#[diagnosis(from)] std::io::Error),
    #[diagnosis(description = "Dispatcher closed.")]
    DispatcherClosed,
}

impl fmt::Display for KioyuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Clerk(source) => fmt::Display::fmt(source, f),
            Self::IO(source) => fmt::Display::fmt(source, f),
            Self::DispatcherClosed => f.write_str("Dispatcher closed."),
        }
    }
}

impl Error for KioyuError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Clerk(source) => Error::source(source),
            Self::IO(source) => Error::source(source),
            Self::DispatcherClosed => None,
        }
    }
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
