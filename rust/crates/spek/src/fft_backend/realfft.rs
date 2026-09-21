extern crate alloc;
use alloc::sync::Arc;

use generic_num::num;
use num_complex::Complex;
use num_traits::{Float, FloatConst};
use realfft::{ComplexToReal, FftNum, RealFftPlanner, RealToComplex};

use crate::fft_backend::{IFftBackend, check_size};
use crate::spectogram::{ISpectrogram, Spectrogram};

pub struct RealfftBackend<T>
where
    T: FftNum,
{
    forward_planner: Arc<dyn RealToComplex<T>>,
    inverse_planner: Arc<dyn ComplexToReal<T>>,
}

impl<T> RealfftBackend<T>
where
    T: FftNum,
{
    pub fn new(fft_size: usize) -> Self {
        let mut planner = RealFftPlanner::new();
        Self {
            forward_planner: planner.plan_fft_forward(fft_size),
            inverse_planner: planner.plan_fft_inverse(fft_size),
        }
    }
}

impl<T> IFftBackend<T, Complex<T>> for RealfftBackend<T>
where
    T: FftNum + FloatConst + Float,
    Spectrogram<Complex<T>>: ISpectrogram<Complex<T>>,
{
    fn signal_size(&self) -> usize { self.fft_size() }

    fn spectrum_size(&self) -> usize { self.fft_size() / 2 + 1 }

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
        let n = num!(self.fft_size());
        signal.iter_mut().for_each(|s| *s = *s / n);
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

    fn new_forward_scratch(&self) -> Vec<Complex<T>> { self.forward_planner.make_scratch_vec() }

    fn new_inverse_scratch(&self) -> Vec<Complex<T>> { self.inverse_planner.make_scratch_vec() }

    fn new_spectrogram(&self, frame_count: usize) -> Spectrogram<Complex<T>> {
        crate::spectogram::Spectrogram::new(frame_count, self.spectrum_size())
    }

    fn fft_size(&self) -> usize { self.forward_planner.len() }
    fn new_signal(&self) -> Vec<T> { vec![T::zero(); self.signal_size()] }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_fft_backend;
    test_fft_backend!(
        test_realfft_backend_fft4,
        f32,
        RealfftBackend<f32>,
        RealfftBackend::new(4)
    );
    test_fft_backend!(
        test_realfft_backend_fft8,
        f32,
        RealfftBackend<f32>,
        RealfftBackend::new(8)
    );
    test_fft_backend!(
        test_realfft_backend_fft7,
        f32,
        RealfftBackend<f32>,
        RealfftBackend::new(7)
    );
}
