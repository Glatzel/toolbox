use phastft::planner::{PlannerR2c32, PlannerR2c64};
use phastft::{c2r_fft_f64_with_planner_and_opts, r2c_fft_f64_with_planner_and_opts};

use super::{FftError, IFftBackend};
use crate::fft_backend::{check_size, check_size_at_least};

#[derive(Debug, Clone)]
pub struct PhastftBackend<P, const N_FFT: usize> {
    options: phastft::options::Options,
    planner: P,
}
impl<P, const N_FFT: usize> PhastftBackend<P, N_FFT> {}
impl<const N_FFT: usize> PhastftBackend<PlannerR2c32, N_FFT> {
    pub fn new() -> Self {
        Self {
            options: phastft::options::Options::guess_options(N_FFT),
            planner: PlannerR2c32::new(N_FFT),
        }
    }
}

impl<const N_FFT: usize> IFftBackend<f32, f32, f32, N_FFT> for PhastftBackend<PlannerR2c32, N_FFT> {
    fn signal_size(&self) -> usize { N_FFT }
    fn spectrum_size(&self) -> usize { N_FFT / 2 + 1 }
    fn forward_scratch_size(&self) -> usize { N_FFT / 2 }
    fn inverse_scratch_size(&self) -> usize { N_FFT / 2 }
    fn fft(
        &self,
        signal: &mut [f32],
        spectrum: &mut [f32],
        scratch: &mut [f32],
    ) -> Result<(), FftError> {
        check_size_at_least("signal", signal.len(), self.signal_size())?;
        check_size("spectrum", spectrum.len(), self.spectrum_size() * 2)?;
        self.fft_unchecked(signal, spectrum, scratch);
        Ok(())
    }
    fn ifft(
        &self,
        spectrum: &mut [f32],
        signal: &mut [f32],
        scratch: &mut [f32],
    ) -> Result<(), FftError> {
        check_size("spectrum", spectrum.len(), self.spectrum_size() * 2)?;
        check_size("signal", signal.len(), self.signal_size())?;
        check_size("scratch", scratch.len(), self.inverse_scratch_size() * 2)?;
        self.ifft_unchecked(spectrum, signal, scratch);
        Ok(())
    }
    fn fft_unchecked(&self, signal: &mut [f32], spectrum: &mut [f32], _scratch: &mut [f32]) {
        let (real, imag) = unsafe { spectrum.split_at_mut_unchecked(self.spectrum_size()) };
        phastft::r2c_fft_f32_with_planner_and_opts(signal, real, imag, &self.planner, &self.options)
    }
    fn ifft_unchecked(&self, spectrum: &mut [f32], signal: &mut [f32], scratch: &mut [f32]) {
        let (real, imag) = unsafe { spectrum.split_at_unchecked(self.spectrum_size()) };
        let (scratch_real, scratch_imag) =
            unsafe { scratch.split_at_mut_unchecked(self.inverse_scratch_size()) };
        phastft::c2r_fft_f32_with_planner_and_opts(
            real,
            imag,
            signal,
            &self.planner,
            &self.options,
            scratch_real,
            scratch_imag,
        )
    }

    fn new_spectrum(&self) -> Vec<f32> { vec![0.0; self.spectrum_size() * 2] }

    fn new_forward_scratch(&self) -> Vec<f32> { vec![0.0; self.forward_scratch_size() * 2] }

    fn new_inverse_scratch(&self) -> Vec<f32> { vec![0.0; self.inverse_scratch_size() * 2] }
}

impl<const N_FFT: usize> PhastftBackend<PlannerR2c64, N_FFT> {
    pub fn new() -> Self {
        Self {
            options: phastft::options::Options::guess_options(N_FFT),
            planner: PlannerR2c64::new(N_FFT),
        }
    }
}

impl<const N_FFT: usize> IFftBackend<f64, f64, f64, N_FFT> for PhastftBackend<PlannerR2c64, N_FFT> {
    fn fft(
        &self,
        signal: &mut [f64],
        spectrum: &mut [f64],
        scratch: &mut [f64],
    ) -> Result<(), FftError> {
        check_size_at_least("signal", signal.len(), self.signal_size())?;
        check_size("spectrum", spectrum.len(), self.spectrum_size() * 2)?;
        check_size("scratch", scratch.len(), self.inverse_scratch_size() * 2)?;
        self.fft_unchecked(signal, spectrum, scratch);
        Ok(())
    }
    fn fft_unchecked(&self, signal: &mut [f64], spectrum: &mut [f64], _scratch: &mut [f64]) {
        let (real, imag) = unsafe { spectrum.split_at_mut_unchecked(self.spectrum_size()) };
        r2c_fft_f64_with_planner_and_opts(signal, real, imag, &self.planner, &self.options)
    }
    fn ifft_unchecked(&self, spectrum: &mut [f64], signal: &mut [f64], scratch: &mut [f64]) {
        let (real, imag) = unsafe { spectrum.split_at_unchecked(self.spectrum_size()) };
        let (scratch_real, scratch_imag) =
            unsafe { scratch.split_at_mut_unchecked(self.inverse_scratch_size()) };
        c2r_fft_f64_with_planner_and_opts(
            real,
            imag,
            signal,
            &self.planner,
            &self.options,
            scratch_real,
            scratch_imag,
        )
    }
    fn ifft(
        &self,
        spectrum: &mut [f64],
        signal: &mut [f64],
        scratch: &mut [f64],
    ) -> Result<(), FftError> {
        check_size("spectrum", spectrum.len(), self.spectrum_size() * 2)?;
        check_size("signal", signal.len(), self.signal_size())?;
        check_size("scratch", scratch.len(), self.inverse_scratch_size() * 2)?;
        self.ifft_unchecked(spectrum, signal, scratch);
        Ok(())
    }

    fn signal_size(&self) -> usize { N_FFT }
    fn spectrum_size(&self) -> usize { N_FFT / 2 + 1 }
    fn forward_scratch_size(&self) -> usize { N_FFT / 2 }
    fn inverse_scratch_size(&self) -> usize { N_FFT / 2 }

    fn new_spectrum(&self) -> Vec<f64> { vec![0.0; self.spectrum_size() * 2] }

    fn new_forward_scratch(&self) -> Vec<f64> { vec![0.0; self.forward_scratch_size() * 2] }

    fn new_inverse_scratch(&self) -> Vec<f64> { vec![0.0; self.inverse_scratch_size() * 2] }
}
