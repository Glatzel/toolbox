use std::f64;

use phastft::planner::{PlannerR2c32, PlannerR2c64};
use phastft::{c2r_fft_f64_with_planner_and_opts, r2c_fft_f64_with_planner_and_opts};

use super::{FftError, IFftBackend};
use crate::fft_backend::check_size;
use crate::stft::{IStftResult, StftResult};

#[derive(Debug, Clone)]
pub struct PhastftBackend<P> {
    fft_size: usize,
    options: phastft::options::Options,
    planner: P,
}

impl PhastftBackend<PlannerR2c32> {
    pub fn new(fft_size: usize) -> Self {
        Self {
            fft_size,
            options: phastft::options::Options::guess_options(fft_size),
            planner: PlannerR2c32::new(fft_size),
        }
    }
}

impl IFftBackend<f32, f32> for PhastftBackend<PlannerR2c32>
where
    StftResult<f32>: IStftResult<f32, f32>,
{
    fn signal_size(&self) -> usize { self.fft_size }
    fn spectrum_size(&self) -> usize { (self.fft_size / 2 + 1) * 2 }
    fn forward_scratch_size(&self) -> usize { (self.fft_size / 2) * 2 }
    fn inverse_scratch_size(&self) -> usize { (self.fft_size / 2) * 2 }
    fn fft(
        &self,
        signal: &mut [f32],
        spectrum: &mut [f32],
        scratch: &mut [f32],
    ) -> Result<(), FftError> {
        check_size("signal", signal.len(), self.signal_size())?;
        check_size("spectrum", spectrum.len(), self.spectrum_size())?;
        self.fft_unchecked(signal, spectrum, scratch);
        Ok(())
    }
    fn ifft(
        &self,
        spectrum: &mut [f32],
        signal: &mut [f32],
        scratch: &mut [f32],
    ) -> Result<(), FftError> {
        check_size("spectrum", spectrum.len(), self.spectrum_size())?;
        check_size("signal", signal.len(), self.signal_size())?;
        check_size("scratch", scratch.len(), self.inverse_scratch_size())?;
        self.ifft_unchecked(spectrum, signal, scratch);
        Ok(())
    }
    fn fft_unchecked(&self, signal: &mut [f32], spectrum: &mut [f32], _scratch: &mut [f32]) {
        let (real, imag) = unsafe { spectrum.split_at_mut_unchecked(self.spectrum_size() / 2) };
        phastft::r2c_fft_f32_with_planner_and_opts(
            signal,
            real,
            imag,
            &self.planner,
            &self.options,
        );
    }
    fn ifft_unchecked(&self, spectrum: &mut [f32], signal: &mut [f32], scratch: &mut [f32]) {
        let (real, imag) = unsafe { spectrum.split_at_unchecked(self.spectrum_size() / 2) };
        let (scratch_real, scratch_imag) =
            unsafe { scratch.split_at_mut_unchecked(self.inverse_scratch_size() / 2) };
        phastft::c2r_fft_f32_with_planner_and_opts(
            real,
            imag,
            signal,
            &self.planner,
            &self.options,
            scratch_real,
            scratch_imag,
        );
    }

    fn new_spectrum(&self) -> Vec<f32> { vec![0.0; self.spectrum_size()] }

    fn new_forward_scratch(&self) -> Vec<f32> { vec![0.0; self.forward_scratch_size()] }

    fn new_inverse_scratch(&self) -> Vec<f32> { vec![0.0; self.inverse_scratch_size()] }

    fn new_stft_result_buffer(&self, frame_count: usize) -> StftResult<f32> {
        StftResult::new(frame_count, self.spectrum_size() / 2)
    }

    fn fft_size(&self) -> usize { self.fft_size }
    fn new_signal(&self) -> Vec<f32> { vec![0_f32; self.signal_size()] }
}

impl PhastftBackend<PlannerR2c64> {
    pub fn new(fft_size: usize) -> Self {
        Self {
            options: phastft::options::Options::guess_options(fft_size),
            planner: PlannerR2c64::new(fft_size),
            fft_size,
        }
    }
}

impl IFftBackend<f64, f64> for PhastftBackend<PlannerR2c64>
where
    StftResult<f64>: IStftResult<f64, f64>,
{
    fn fft(
        &self,
        signal: &mut [f64],
        spectrum: &mut [f64],
        scratch: &mut [f64],
    ) -> Result<(), FftError> {
        check_size("signal", signal.len(), self.signal_size())?;
        check_size("spectrum", spectrum.len(), self.spectrum_size())?;
        self.fft_unchecked(signal, spectrum, scratch);
        Ok(())
    }
    fn fft_unchecked(&self, signal: &mut [f64], spectrum: &mut [f64], _scratch: &mut [f64]) {
        let (real, imag) = unsafe { spectrum.split_at_mut_unchecked(self.spectrum_size() / 2) };
        r2c_fft_f64_with_planner_and_opts(signal, real, imag, &self.planner, &self.options);
    }
    fn ifft_unchecked(&self, spectrum: &mut [f64], signal: &mut [f64], scratch: &mut [f64]) {
        let (real, imag) = unsafe { spectrum.split_at_unchecked(self.spectrum_size() / 2) };
        let (scratch_real, scratch_imag) =
            unsafe { scratch.split_at_mut_unchecked(self.inverse_scratch_size() / 2) };
        c2r_fft_f64_with_planner_and_opts(
            real,
            imag,
            signal,
            &self.planner,
            &self.options,
            scratch_real,
            scratch_imag,
        );
    }
    fn ifft(
        &self,
        spectrum: &mut [f64],
        signal: &mut [f64],
        scratch: &mut [f64],
    ) -> Result<(), FftError> {
        check_size("spectrum", spectrum.len(), self.spectrum_size())?;
        check_size("signal", signal.len(), self.signal_size())?;
        check_size("scratch", scratch.len(), self.inverse_scratch_size())?;
        self.ifft_unchecked(spectrum, signal, scratch);
        Ok(())
    }

    fn spectrum_size(&self) -> usize { (self.fft_size / 2 + 1) * 2 }
    fn forward_scratch_size(&self) -> usize { (self.fft_size / 2) * 2 }
    fn inverse_scratch_size(&self) -> usize { (self.fft_size / 2) * 2 }

    fn new_spectrum(&self) -> Vec<f64> { vec![0.0; self.spectrum_size()] }

    fn new_forward_scratch(&self) -> Vec<f64> { vec![0.0; self.forward_scratch_size()] }

    fn new_inverse_scratch(&self) -> Vec<f64> { vec![0.0; self.inverse_scratch_size()] }

    fn new_stft_result_buffer(&self, frame_count: usize) -> StftResult<f64> {
        StftResult::new(frame_count, self.spectrum_size() / 2)
    }

    fn fft_size(&self) -> usize { self.fft_size }

    fn signal_size(&self) -> usize { self.fft_size }
    fn new_signal(&self) -> Vec<f64> { vec![0_f64; self.signal_size()] }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_fft_backend;
    test_fft_backend!(
        test_phastft_backendf32_fft4,
        f32,
        PhastftBackend<PlannerR2c32>,
        PhastftBackend::<PlannerR2c32>::new(4)
    );
    test_fft_backend!(
        test_phastft_backendf32_fft8,
        f32,
        PhastftBackend<PlannerR2c32>,
        PhastftBackend::<PlannerR2c32>::new(8)
    );
    test_fft_backend!(
        #[should_panic]
        test_phastft_backendf32_fft7,
        f32,
        PhastftBackend<PlannerR2c32>,
        PhastftBackend::<PlannerR2c32>::new(7)
    );
    test_fft_backend!(
        test_phastft_backendf64_fft4,
        f64,
        PhastftBackend<PlannerR2c64>,
        PhastftBackend::<PlannerR2c64>::new(4)
    );
    test_fft_backend!(
        test_phastft_backendf64_fft8,
        f64,
        PhastftBackend<PlannerR2c64>,
        PhastftBackend::<PlannerR2c64>::new(8)
    );
    test_fft_backend!(
        #[should_panic]
        test_phastft_backendf64_fft7,
        f64,
        PhastftBackend<PlannerR2c64>,
        PhastftBackend::<PlannerR2c64>::new(7)
    );
}
