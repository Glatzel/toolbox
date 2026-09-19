use num_complex::{Complex, ComplexFloat};
use rustfft::{Direction, Fft, FftNum, Length};

use crate::fft_backend::IComplexFftBackend;

pub struct RustfftBackend<T, A, const N_FFT: usize>
where
    T: FftNum,
    A: Direction + Fft<T> + Length,
{
    algorithm: A,
    phantom: core::marker::PhantomData<T>,
}
impl<T, A, const N_FFT: usize> RustfftBackend<T, A, N_FFT>
where
    T: FftNum,
    A: Direction + Fft<T> + Length,
{
    pub const fn new(algorithm: A) -> Self {
        Self {
            algorithm,
            phantom: core::marker::PhantomData,
        }
    }
}
impl<T, A, const N_FFT: usize> IComplexFftBackend<T, N_FFT> for RustfftBackend<T, A, N_FFT>
where
    T: FftNum + ComplexFloat,
    A: Direction + Fft<T> + Length,
{
    fn fft_unchecked(&self, buffer: &mut [Complex<T>]) { self.algorithm.process(buffer); }

    fn ifft_unchecked(&self, buffer: &mut [Complex<T>], scratch: &mut [Complex<T>]) {
        self.algorithm.process_with_scratch(buffer, scratch);
    }
}
