pub use mischief_macros::mischief;
#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => {

       return Err($crate::mischief!($($arg)*));
    };
}
