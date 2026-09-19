use phastft::planner::{Direction, PlannerDit32, PlannerDit64, PlannerR2c32, PlannerR2c64};

use super::IRealFftBackend;
use crate::fft_backend::IComplexFftBackend;

#[derive(Debug, Clone)]
pub struct RealPhastftBackend<P, const N_FFT: usize> {
    options: phastft::options::Options,
    planar: P,
}

impl<P, const N_FFT: usize> RealPhastftBackend<PlannerR2c32, N_FFT> {
    pub fn new() -> Self {
        Self {
            options: phastft::options::Options::guess_options(N_FFT),
            planar: PlannerR2c32::new(N_FFT),
        }
    }
}
impl<const N_FFT: usize> RealPhastftBackend<PlannerR2c64, N_FFT> {
    pub fn new() -> Self {
        Self {
            options: phastft::options::Options::guess_options(N_FFT),
            planar: PlannerR2c64::new(N_FFT),
        }
    }
}

impl<const N_FFT: usize> IRealFftBackend<f32, N_FFT> for RealPhastftBackend<PlannerR2c32, N_FFT> {
    fn fft_unchecked(&self, signal: &[f32], real: &mut [f32], imag: &mut [f32]) {
        phastft::r2c_fft_f32_with_planner_and_opts(signal, real, imag, &self.planar, &self.options)
    }
    fn ifft_unchecked(
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
            &self.planar,
            &self.options,
            scratch_real,
            scratch_imag,
        )
    }
}

impl<const N_FFT: usize> IRealFftBackend<f64, N_FFT> for RealPhastftBackend<PlannerR2c64, N_FFT> {
    fn fft_unchecked(&self, signal: &[f64], real: &mut [f64], imag: &mut [f64]) {
        phastft::r2c_fft_f64_with_planner_and_opts(signal, real, imag, &self.planar, &self.options)
    }
    fn ifft_unchecked(
        &self,
        real: &[f64],
        imag: &[f64],
        signal: &mut [f64],
        scratch_real: &mut [f64],
        scratch_imag: &mut [f64],
    ) {
        phastft::c2r_fft_f64_with_planner_and_opts(
            real,
            imag,
            signal,
            &self.planar,
            &self.options,
            scratch_real,
            scratch_imag,
        )
    }
}

#[derive(Debug, Clone)]
pub struct ComplexPhastftBackend<P, const N_FFT: usize> {
    options: phastft::options::Options,
    planar: P,
}

impl<const N_FFT: usize> ComplexPhastftBackend<PlannerDit32, N_FFT> {
    pub fn new() -> Self {
        Self {
            options: phastft::options::Options::guess_options(N_FFT),
            planar: PlannerDit32::new(N_FFT),
        }
    }
}
impl<const N_FFT: usize> ComplexPhastftBackend<PlannerDit64, N_FFT> {
    pub fn new() -> Self {
        Self {
            options: phastft::options::Options::guess_options(N_FFT),
            planar: PlannerDit64::new(N_FFT),
        }
    }
}

impl<const N_FFT: usize> IComplexFftBackend<f32, N_FFT> for ComplexPhastftBackend<f32, N_FFT> {
    fn fft_unchecked(&self, buffer: &mut [num_complex::Complex<f32>]) {
        fft_f32_dit_interleaved_with_planner_and_opts(
            buffer,
            Direction::Forward,
            &self.planar,
            &self.options,
        )
    }

    fn ifft_unchecked(
        &self,
        buffer: &mut [num_complex::Complex<f32>],
        scratch: &mut [num_complex::Complex<f32>],
    ) {
        fft_f32_dit_interleaved_with_planner_and_opts(
            buffer,
            Direction::Inverse,
            &self.planar,
            &self.options,
        )
    }
}
