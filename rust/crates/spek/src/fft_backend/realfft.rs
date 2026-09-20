extern crate alloc;
use alloc::sync::Arc;

use num_complex::Complex;
use num_traits::FloatConst;
use realfft::{ComplexToReal, FftNum, RealFftPlanner, RealToComplex};

use crate::fft_backend::{IFftBackend, check_size};
use crate::spectogram::{ISpectrogram, Spectrogram};

pub struct RealfftBackend<T, const N: usize>
where
    T: FftNum,
{
    forward_planner: Arc<dyn RealToComplex<T>>,
    inverse_planner: Arc<dyn ComplexToReal<T>>,
}

impl<T, const N: usize> RealfftBackend<T, N>
where
    T: FftNum,
{
    pub fn new() -> Self {
        let mut planner = RealFftPlanner::new();
        Self {
            forward_planner: planner.plan_fft_forward(N),
            inverse_planner: planner.plan_fft_inverse(N),
        }
    }
}
impl<T, const N: usize> Default for RealfftBackend<T, N>
where
    T: FftNum,
{
    fn default() -> Self { Self::new() }
}
impl<T, const N: usize> IFftBackend<T, Complex<T>, N> for RealfftBackend<T, N>
where
    T: FftNum + FloatConst,
    Spectrogram<Complex<T>>: ISpectrogram<Complex<T>>,
{
    fn signal_size(&self) -> usize { N }

    fn spectrum_size(&self) -> usize { N / 2 + 1 }

    fn forward_scratch_size(&self) -> usize { self.forward_planner.get_scratch_len() }

    fn inverse_scratch_size(&self) -> usize { self.inverse_planner.get_scratch_len() }

    fn fft(
        &self,
        signal: &mut [T],
        spectrum: &mut [Complex<T>],
        scratch: &mut [Complex<T>],
    ) -> Result<(), super::FftError> {
        check_size("signal", signal.len(), self.signal_size())?;
        check_size("spectrum", spectrum.len(), self.spectrum_size())?;
        check_size("scratch", scratch.len(), self.forward_scratch_size())?;

        self.fft_unchecked(signal, spectrum, scratch);
        Ok(())
    }

    fn ifft(
        &self,
        spectrum: &mut [Complex<T>],
        signal: &mut [T],
        scratch: &mut [Complex<T>],
    ) -> Result<(), super::FftError> {
        check_size("spectrum", spectrum.len(), self.spectrum_size())?;
        check_size("signal", signal.len(), self.signal_size())?;
        check_size("scratch", scratch.len(), self.inverse_scratch_size())?;
        self.ifft_unchecked(spectrum, signal, scratch);
        Ok(())
    }

    fn fft_unchecked(
        &self,
        signal: &mut [T],
        spectrum: &mut [Complex<T>],
        scratch: &mut [Complex<T>],
    ) {
        self.forward_planner
            .process_with_scratch(signal, spectrum, scratch)
            .unwrap();
    }

    fn ifft_unchecked(
        &self,
        spectrum: &mut [Complex<T>],
        signal: &mut [T],
        scratch: &mut [Complex<T>],
    ) {
        self.inverse_planner
            .process_with_scratch(spectrum, signal, scratch)
            .unwrap();
    }

    fn new_spectrum(&self) -> Vec<Complex<T>> {
        vec![
            Complex {
                re: T::zero(),
                im: T::zero()
            };
            self.spectrum_size()
        ]
    }

    fn new_forward_scratch(&self) -> Vec<Complex<T>> {
        vec![
            Complex {
                re: T::zero(),
                im: T::zero()
            };
            self.forward_scratch_size()
        ]
    }

    fn new_inverse_scratch(&self) -> Vec<Complex<T>> {
        vec![
            Complex {
                re: T::zero(),
                im: T::zero()
            };
            self.inverse_scratch_size()
        ]
    }

    fn new_spectrogram(&self, frame_len: usize) -> Spectrogram<Complex<T>> {
         crate::spectogram::Spectrogram::new(frame_len, self.spectrum_size())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_fft_backend;
    test_fft_backend!(test_realfft_backend_fft4, f32, RealfftBackend<f32, 4>);
    test_fft_backend!(test_realfft_backend_fft8, f32, RealfftBackend<f32, 8>);
}
