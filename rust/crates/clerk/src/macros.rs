#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {{}};
}

#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{}};
}

#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{}};
}

#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{}};
}

#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{}};
}
