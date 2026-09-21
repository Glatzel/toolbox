extern crate alloc;
use alloc::sync::Arc;

use generic_num::num;
use num_complex::Complex;
use num_traits::Float;
use rustfft::{Fft, FftNum};

use crate::fft_backend::{FftError, IFftBackend, check_size};
use crate::spectogram::{ISpectrogram, Spectrogram};

pub struct RustfftBackend<T>
where
    T: FftNum,
{
    forward_planner: Arc<dyn Fft<T>>,
    inverse_planner: Arc<dyn Fft<T>>,
    phantom: core::marker::PhantomData<T>,
}
impl<T> RustfftBackend<T>
where
    T: FftNum,
{
    pub fn new(
        forward_planner: Arc<dyn Fft<T>>,
        inverse_planner: Arc<dyn Fft<T>>,
    ) -> Result<Self, FftError> {
        if forward_planner.len() != inverse_planner.len() {
            return Err(FftError::SizeNotEqual {
                name: "planner",
                a: forward_planner.len(),
                b: inverse_planner.len(),
            });
        }

        Ok(Self {
            forward_planner,
            inverse_planner,
            phantom: core::marker::PhantomData,
        })
    }
}
impl<T> IFftBackend<T, Complex<T>> for RustfftBackend<T>
where
    T: FftNum + Float,
    Spectrogram<Complex<T>>: ISpectrogram<Complex<T>>,
{
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
        for (dst, &src) in spectrum.iter_mut().zip(signal.iter()) {
            *dst = Complex::new(src, T::zero());
        }
        self.forward_planner.process_with_scratch(spectrum, scratch);
    }

    fn ifft_unchecked(
        &self,
        spectrum: &mut [Complex<T>],
        signal: &mut [T],
        scratch: &mut [Complex<T>],
    ) {
        self.inverse_planner.process_with_scratch(spectrum, scratch);
        let n = num!(self.fft_size());

        for (dst, src) in signal.iter_mut().zip(spectrum.iter()) {
            *dst = src.re / n;
        }
    }

    fn spectrum_size(&self) -> usize { self.fft_size() }

    fn forward_scratch_size(&self) -> usize { self.forward_planner.get_inplace_scratch_len() }

    fn inverse_scratch_size(&self) -> usize { self.inverse_planner.get_inplace_scratch_len() }

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
    fn new_spectrogram(&self, frame_count: usize) -> Spectrogram<Complex<T>> {
        crate::spectogram::Spectrogram::new(frame_count, self.spectrum_size())
    }

    fn fft_size(&self) -> usize { self.forward_planner.len() }

    fn signal_size(&self) -> usize { self.fft_size() }
}
#[cfg(test)]
mod tests {

    use super::*;
    use crate::test_fft_backend;

    test_fft_backend!(
        test_rustfft_backend_fft4,
        f32,
        RustfftBackend<f32>,
        RustfftBackend::new(
            rustfft::FftPlanner::new().plan_fft_forward(4),
            rustfft::FftPlanner::new().plan_fft_inverse(4)
        )
        .unwrap()
    );
    test_fft_backend!(
        test_rustfft_backend_fft8,
        f32,
        RustfftBackend<f32>,
        RustfftBackend::new(
            rustfft::FftPlanner::new().plan_fft_forward(8),
            rustfft::FftPlanner::new().plan_fft_inverse(8)
        )
        .unwrap()
    );
    test_fft_backend!(
        test_rustfft_backend_fft7,
        f32,
        RustfftBackend<f32>,
        RustfftBackend::new(
            rustfft::FftPlanner::new().plan_fft_forward(7),
            rustfft::FftPlanner::new().plan_fft_inverse(7)
        )
        .unwrap()
    );
}
