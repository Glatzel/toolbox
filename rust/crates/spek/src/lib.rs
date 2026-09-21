#![cfg_attr(not(feature = "std"), no_std)]

pub mod conversion;
pub mod fft_backend;
pub mod pad;
pub mod spectrogram;
pub mod stft;
pub mod windows;

#[cfg(feature = "complex")]
use num_complex::Complex;
#[cfg(feature = "complex")]
type Data<T> = Vec<Complex<T>>;
#[cfg(feature = "complex")]
type Dtype<T> = Complex<T>;
#[cfg(feature = "split")]
type Data<T> = Vec<T>;
#[cfg(feature = "split")]
type Dtype<T> = T;
#[cfg(all(feature = "complex", feature = "split"))]
compile_error!("features `complex` and `split` are mutually exclusive");
#[cfg(not(any(feature = "complex", feature = "split")))]
compile_error!("one of `complex` or `split` must be enabled");
