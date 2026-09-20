use num_complex::Complex;
use phastft::planner::{Direction, PlannerDit32, PlannerDit64, PlannerR2c32, PlannerR2c64};
use phastft::{
    fft_f32_dit_interleaved_with_planner_and_opts, fft_f32_dit_with_planner_and_opts,
    fft_f64_dit_interleaved_with_planner_and_opts, fft_f64_dit_with_planner_and_opts,
};

use super::IRealSplitFftBackend;
use crate::fft_backend::{IComplexFftBackend, IComplexSplitFftBackend};

#[derive(Debug, Clone)]
pub struct PhastftBackend<P, const N_FFT: usize> {
    options: phastft::options::Options,
    planner: P,
}

impl<const N_FFT: usize> PhastftBackend<PlannerR2c32, N_FFT> {
    pub fn new() -> Self {
        Self {
            options: phastft::options::Options::guess_options(N_FFT),
            planner: PlannerR2c32::new(N_FFT),
        }
    }
}

impl<const N_FFT: usize> IRealSplitFftBackend<f32, N_FFT> for PhastftBackend<PlannerR2c32, N_FFT> {
    fn fft_unchecked(&self, signal: &[f32], real: &mut [f32], imag: &mut [f32]) {
        phastft::r2c_fft_f32_with_planner_and_opts(signal, real, imag, &self.planner, &self.options)
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
            &self.planner,
            &self.options,
            scratch_real,
            scratch_imag,
        )
    }
}

impl<const N_FFT: usize> PhastftBackend<PlannerR2c64, N_FFT> {
    pub fn new() -> Self {
        Self {
            options: phastft::options::Options::guess_options(N_FFT),
            planner: PlannerR2c64::new(N_FFT),
        }
    }
}

impl<const N_FFT: usize> IRealSplitFftBackend<f64, N_FFT> for PhastftBackend<PlannerR2c64, N_FFT> {
    fn fft_unchecked(&self, signal: &[f64], real: &mut [f64], imag: &mut [f64]) {
        phastft::r2c_fft_f64_with_planner_and_opts(signal, real, imag, &self.planner, &self.options)
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
            &self.planner,
            &self.options,
            scratch_real,
            scratch_imag,
        )
    }
}
impl<const N_FFT: usize> PhastftBackend<PlannerDit32, N_FFT> {
    pub fn new() -> Self {
        Self {
            options: phastft::options::Options::guess_options(N_FFT),
            planner: PlannerDit32::new(N_FFT),
        }
    }
}

impl<const N_FFT: usize> IComplexSplitFftBackend<f32, N_FFT>
    for PhastftBackend<PlannerDit32, N_FFT>
{
    fn fft_unchecked(&self, real: &mut [f32], imag: &mut [f32]) {
        fft_f32_dit_with_planner_and_opts(
            real,
            imag,
            Direction::Forward,
            &self.planner,
            &self.options,
        )
    }

    fn ifft_unchecked(
        &self,
        real: &mut [f32],
        imag: &mut [f32],
        _scratch_real: &mut [f32],
        _scratch_imag: &mut [f32],
    ) {
        fft_f32_dit_with_planner_and_opts(
            real,
            imag,
            Direction::Inverse,
            &self.planner,
            &self.options,
        )
    }
}
impl<const N_FFT: usize> PhastftBackend<PlannerDit64, N_FFT> {
    pub fn new() -> Self {
        Self {
            options: phastft::options::Options::guess_options(N_FFT),
            planner: PlannerDit64::new(N_FFT),
        }
    }
}
impl<const N_FFT: usize> IComplexSplitFftBackend<f64, N_FFT>
    for PhastftBackend<PlannerDit64, N_FFT>
{
    fn fft_unchecked(&self, real: &mut [f64], imag: &mut [f64]) {
        fft_f64_dit_with_planner_and_opts(
            real,
            imag,
            Direction::Forward,
            &self.planner,
            &self.options,
        )
    }

    fn ifft_unchecked(
        &self,
        real: &mut [f64],
        imag: &mut [f64],
        _scratch_real: &mut [f64],
        _scratch_imag: &mut [f64],
    ) {
        fft_f64_dit_with_planner_and_opts(
            real,
            imag,
            Direction::Inverse,
            &self.planner,
            &self.options,
        )
    }
}

impl<const N_FFT: usize> IComplexFftBackend<f32, N_FFT> for PhastftBackend<PlannerDit32, N_FFT> {
    fn fft_unchecked(&self, buffer: &mut [Complex<f32>]) {
        fft_f32_dit_interleaved_with_planner_and_opts(
            buffer,
            Direction::Forward,
            &self.planner,
            &self.options,
        )
    }

    fn ifft_unchecked(&self, buffer: &mut [Complex<f32>], _scratch: &mut [Complex<f32>]) {
        fft_f32_dit_interleaved_with_planner_and_opts(
            buffer,
            Direction::Inverse,
            &self.planner,
            &self.options,
        )
    }
}

impl<const N_FFT: usize> IComplexFftBackend<f64, N_FFT> for PhastftBackend<PlannerDit64, N_FFT> {
    fn fft_unchecked(&self, buffer: &mut [Complex<f64>]) {
        fft_f64_dit_interleaved_with_planner_and_opts(
            buffer,
            Direction::Forward,
            &self.planner,
            &self.options,
        )
    }

    fn ifft_unchecked(&self, buffer: &mut [Complex<f64>], _scratch: &mut [Complex<f64>]) {
        fft_f64_dit_interleaved_with_planner_and_opts(
            buffer,
            Direction::Inverse,
            &self.planner,
            &self.options,
        )
    }
}
