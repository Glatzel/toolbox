extern crate alloc;
use alloc::sync::Arc;

use num_complex::Complex;
use realfft::{ComplexToReal, RealFftPlanner, RealToComplex};

use crate::fft_backend::IRealFftBackend;

pub struct RealfftBackend<T, const N_FFT: usize>
where
    T: realfft::FftNum,
{
    forward_planner: Arc<dyn RealToComplex<T>>,
    inverse_planner: Arc<dyn ComplexToReal<T>>,
}

impl<T, const N_FFT: usize> RealfftBackend<T, N_FFT>
where
    T: realfft::FftNum,
{
    pub fn new() -> Self {
        let mut planner = RealFftPlanner::new();
        Self {
            forward_planner: planner.plan_fft_forward(N_FFT),
            inverse_planner: planner.plan_fft_inverse(N_FFT),
        }
    }
}
impl<T, const N_FFT: usize> IRealFftBackend<T, N_FFT> for RealfftBackend<T, N_FFT>
where
    T: realfft::FftNum,
{
    fn fft_unchecked(&self, signal: &[T], spectrum: &mut [Complex<T>]) {
        self.forward_planner.process(&mut signal.as_mut(), spectrum).unwrap();
    }

    fn ifft_unchecked(
        &self,
        spectrum: &[Complex<T>],
        signal: &mut [T],
        scratch: &mut [Complex<T>],
    ) {
        self.inverse_planner
            .process_with_scratch(spectrum, signal, scratch)
            .unwrap();
    }
}
