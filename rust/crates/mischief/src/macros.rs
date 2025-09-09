/// Re-exports the `mischief!` procedural macro.
pub use mischief_macros::mischief;

/// Early-return macro for error handling.
///
/// Similar to `anyhow::bail!`, this macro:
/// - Constructs a `MischiefError` wrapped in a `Report` using `mischief!`.
/// - Immediately returns it as an `Err` from the current function.
///
/// # Syntax
///
/// ```text
/// bail!(<mischief! arguments>);
/// ```
///
/// # Example
///
/// ```
/// use mischief::{bail, Severity};
///
/// fn open_file(path: &str) -> Result<(), mischief::Report> {
///     if path != "config.toml" {
///         bail!("Failed to open file: {}", path, severity = Severity::ERROR);
///     }
///     Ok(())
/// }
///
/// let result = open_file("missing.txt");
/// assert!(result.is_err());
/// ```
#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => {
        return Err($crate::mischief!($($arg)*));
    };
}
