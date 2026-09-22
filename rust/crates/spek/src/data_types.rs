#[cfg(feature = "complex")]
use num_complex::Complex;
#[cfg(feature = "complex")]
pub(crate) type Data<T> = Vec<Complex<T>>;
#[cfg(feature = "complex")]
pub(crate) type Dtype<T> = Complex<T>;
#[cfg(feature = "split")]
pub(crate) type Data<T> = Vec<T>;
#[cfg(feature = "split")]
pub(crate) type Dtype<T> = T;
#[cfg(all(feature = "complex", feature = "split"))]
compile_error!("features `complex` and `split` are mutually exclusive");
#[cfg(not(any(feature = "complex", feature = "split")))]
compile_error!("one of `complex` or `split` must be enabled");
