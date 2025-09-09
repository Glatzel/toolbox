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
#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => {
        return Err($crate::mischief!($($arg)*));
    };
}
