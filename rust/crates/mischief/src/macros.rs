#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => {
       return Err(mischief_macros::mischief!($($arg)*));
    };
}
