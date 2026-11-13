use std::path::PathBuf;

use mischief::IDiagnosis;

#[derive(Debug, IDiagnosis)]
#[diagnosis(error, display)]
pub enum ClerkError {
    #[diagnosis(transparent(foreign))]
    IO(#[diagnosis(from)] std::io::Error),
    #[diagnosis(description = "Parent directory not found")]
    ParentDirectoryNotFound(PathBuf),
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
