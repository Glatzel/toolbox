use std::error::Error;
use std::fmt;
use std::path::PathBuf;

use mischief::IDiagnosis;

#[derive(Debug, IDiagnosis)]
pub enum ClerkError {
    #[diagnosis(description = "I/O error")]
    IO(#[diagnosis(from)] std::io::Error),
    #[diagnosis(description = "Parent directory not found")]
    ParentDirectoryNotFound(PathBuf),
}

impl fmt::Display for ClerkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IO(source) => fmt::Display::fmt(source, f),
            Self::ParentDirectoryNotFound(_) => f.write_str("Parent directory not found"),
        }
    }
}

impl Error for ClerkError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::IO(source) => Error::source(source),
            Self::ParentDirectoryNotFound(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transparent_io_error_preserves_display_and_source() {
        let error = ClerkError::from(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "fixed text",
        ));

        assert_eq!(error.to_string(), "fixed text");
        assert!(std::error::Error::source(&error).is_none());
    }

    #[test]
    fn parent_directory_display_is_unchanged() {
        let error = ClerkError::ParentDirectoryNotFound(PathBuf::from("/tmp/missing"));

        assert_eq!(error.to_string(), "Parent directory not found");
        assert!(std::error::Error::source(&error).is_none());
    }
}
