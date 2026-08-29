#[macro_export]
macro_rules! num {
    ($value:expr) => {
        <T>::from($value).unwrap()
    };
}
