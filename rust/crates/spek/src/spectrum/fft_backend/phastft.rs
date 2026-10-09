use phastft::planner::{PlannerR2c32, PlannerR2c64};
use phastft::{c2r_fft_f64_with_planner_and_opts, r2c_fft_f64_with_planner_and_opts};

use super::IFftBackend;
use crate::data_types::Spectrum2D;

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

impl IFftBackend<f32> for PhastftBackend<PlannerR2c32> {
    fn fft_size(&self) -> usize { self.fft_size }
    fn signal_size(&self) -> usize { self.fft_size }
    fn spectrum_size(&self) -> usize { self.fft_size / 2 + 1 }
    fn scratch_size(&self) -> usize { self.fft_size / 2 }
    fn new_signal(&self) -> Vec<f32> { vec![0_f32; self.signal_size()] }

    fn new_spectrum(&self) -> (Vec<f32>, Vec<f32>) {
        (
            vec![0.0; self.spectrum_size()],
            vec![0.0; self.spectrum_size()],
        )
    }

    fn new_scratch(&self) -> (Vec<f32>, Vec<f32>) {
        (
            vec![0.0; self.scratch_size()],
            vec![0.0; self.scratch_size()],
        )
    }

    fn new_spectrum2d(&self, frame_count: usize) -> Spectrum2D<f32> {
        Spectrum2D::new(frame_count, self.spectrum_size())
    }

    fn fft(&self, signal: &[f32], real: &mut [f32], imag: &mut [f32]) {
        phastft::r2c_fft_f32_with_planner_and_opts(
            signal,
            real,
            imag,
            &self.planner,
            &self.options,
        );
    }
    fn ifft(
        &self,
        real: &[f32],
        imag: &[f32],
        signal: &mut [f32],
        scratch_real: &mut [f32],
        scratch_imag: &mut [f32],
    ) {
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

impl IFftBackend<f64> for PhastftBackend<PlannerR2c64> {
    fn fft_size(&self) -> usize { self.fft_size }
    fn signal_size(&self) -> usize { self.fft_size }
    fn spectrum_size(&self) -> usize { self.fft_size / 2 + 1 }

    fn scratch_size(&self) -> usize { self.fft_size / 2 }
    fn new_signal(&self) -> Vec<f64> { vec![0_f64; self.signal_size()] }

    fn new_spectrum(&self) -> (Vec<f64>, Vec<f64>) {
        (
            vec![0.0; self.spectrum_size()],
            vec![0.0; self.spectrum_size()],
        )
    }

    fn new_scratch(&self) -> (Vec<f64>, Vec<f64>) {
        (
            vec![0.0; self.scratch_size()],
            vec![0.0; self.scratch_size()],
        )
    }

    fn new_spectrum2d(&self, frame_count: usize) -> Spectrum2D<f64> {
        Spectrum2D::new(frame_count, self.spectrum_size())
    }

    fn fft(&self, signal: &[f64], real: &mut [f64], imag: &mut [f64]) {
        r2c_fft_f64_with_planner_and_opts(signal, real, imag, &self.planner, &self.options);
    }
    fn ifft(
        &self,
        real: &[f64],
        imag: &[f64],
        signal: &mut [f64],
        scratch_real: &mut [f64],
        scratch_imag: &mut [f64],
    ) {
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
}
