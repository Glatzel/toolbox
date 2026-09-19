use phastft::planner::{PlannerR2c32, PlannerR2c64};

use super::IFftBackend;

#[derive(Debug, Clone)]
pub struct Phastft<P, const N_FFT: usize> {
    options: phastft::options::Options,
    planar: P,
}
impl<const N_FFT: usize> Phastft<PlannerR2c32, N_FFT> {
    pub fn new() -> Self {
        Self {
            options: phastft::options::Options::guess_options(N_FFT),
            planar: PlannerR2c32::new(N_FFT),
        }
    }
}
impl<const N_FFT: usize> Phastft<PlannerR2c64, N_FFT> {
    pub fn new() -> Self {
        Self {
            options: phastft::options::Options::guess_options(N_FFT),
            planar: PlannerR2c64::new(N_FFT),
        }
    }
}

impl<const N_FFT: usize> IFftBackend<f32, N_FFT> for Phastft<PlannerR2c32, N_FFT> {
    fn fft_unchecked(&self, input: &[f32], real: &mut [f32], imag: &mut [f32]) {
        phastft::r2c_fft_f32_with_planner_and_opts(input, real, imag, &self.planar, &self.options)
    }
    fn ifft_unchecked(
        &self,
        real: &[f32],
        imag: &[f32],
        output: &mut [f32],
        scratch_real: &mut [f32],
        scratch_imag: &mut [f32],
    ) {
        phastft::c2r_fft_f32_with_planner_and_opts(
            real,
            imag,
            output,
            &self.planar,
            &self.options,
            scratch_real,
            scratch_imag,
        )
    }

    fn fft(
        &self,
        input: &[f32],
        real: &mut [f32],
        imag: &mut [f32],
    ) -> Result<(), super::FftError> {
        let expected_bins = N_FFT / 2 + 1;

        if input.len() != N_FFT {
            return Err(super::FftError::SizeNotMatch {
                expected: N_FFT,
                actual: input.len(),
            });
        }

        if real.len() != expected_bins {
            return Err(super::FftError::SizeNotMatch {
                expected: expected_bins,
                actual: real.len(),
            });
        }

        if imag.len() != expected_bins {
            return Err(super::FftError::SizeNotMatch {
                expected: expected_bins,
                actual: imag.len(),
            });
        }

        self.fft_unchecked(input, real, imag);
        Ok(())
    }

    fn ifft(
        &self,
        real: &[f32],
        imag: &[f32],
        output: &mut [f32],
        scratch_real: &mut [f32],
        scratch_imag: &mut [f32],
    ) -> Result<(), super::FftError> {
        let expected_bins = N_FFT / 2 + 1;
        let expected_scratch = N_FFT / 2;

        if real.len() != expected_bins {
            return Err(super::FftError::SizeNotMatch {
                expected: expected_bins,
                actual: real.len(),
            });
        }

        if imag.len() != expected_bins {
            return Err(super::FftError::SizeNotMatch {
                expected: expected_bins,
                actual: imag.len(),
            });
        }

        if output.len() != N_FFT {
            return Err(super::FftError::SizeNotMatch {
                expected: N_FFT,
                actual: output.len(),
            });
        }

        if scratch_real.len() != expected_scratch {
            return Err(super::FftError::SizeNotMatch {
                expected: expected_scratch,
                actual: scratch_real.len(),
            });
        }

        if scratch_imag.len() != expected_scratch {
            return Err(super::FftError::SizeNotMatch {
                expected: expected_scratch,
                actual: scratch_imag.len(),
            });
        }

        self.ifft_unchecked(real, imag, output, scratch_real, scratch_imag);

        Ok(())
    }
}
impl<const N_FFT: usize> IFftBackend<f64, N_FFT> for Phastft<PlannerR2c64, N_FFT> {
    fn fft_unchecked(&self, input: &[f64], real: &mut [f64], imag: &mut [f64]) {
        phastft::r2c_fft_f64_with_planner_and_opts(input, real, imag, &self.planar, &self.options)
    }
    fn ifft_unchecked(
        &self,
        real: &[f64],
        imag: &[f64],
        output: &mut [f64],
        scratch_real: &mut [f64],
        scratch_imag: &mut [f64],
    ) {
        phastft::c2r_fft_f64_with_planner_and_opts(
            real,
            imag,
            output,
            &self.planar,
            &self.options,
            scratch_real,
            scratch_imag,
        )
    }

    fn fft(
        &self,
        input: &[f64],
        real: &mut [f64],
        imag: &mut [f64],
    ) -> Result<(), super::FftError> {
        let expected_bins = N_FFT / 2 + 1;

        if input.len() != N_FFT {
            return Err(super::FftError::SizeNotMatch {
                expected: N_FFT,
                actual: input.len(),
            });
        }

        if real.len() != expected_bins {
            return Err(super::FftError::SizeNotMatch {
                expected: expected_bins,
                actual: real.len(),
            });
        }

        if imag.len() != expected_bins {
            return Err(super::FftError::SizeNotMatch {
                expected: expected_bins,
                actual: imag.len(),
            });
        }

        self.fft_unchecked(input, real, imag);
        Ok(())
    }

    fn ifft(
        &self,
        real: &[f64],
        imag: &[f64],
        output: &mut [f64],
        scratch_real: &mut [f64],
        scratch_imag: &mut [f64],
    ) -> Result<(), super::FftError> {
        let expected_bins = N_FFT / 2 + 1;
        let expected_scratch = N_FFT / 2;

        if real.len() != expected_bins {
            return Err(super::FftError::SizeNotMatch {
                expected: expected_bins,
                actual: real.len(),
            });
        }

        if imag.len() != expected_bins {
            return Err(super::FftError::SizeNotMatch {
                expected: expected_bins,
                actual: imag.len(),
            });
        }

        if output.len() != N_FFT {
            return Err(super::FftError::SizeNotMatch {
                expected: N_FFT,
                actual: output.len(),
            });
        }

        if scratch_real.len() != expected_scratch {
            return Err(super::FftError::SizeNotMatch {
                expected: expected_scratch,
                actual: scratch_real.len(),
            });
        }

        if scratch_imag.len() != expected_scratch {
            return Err(super::FftError::SizeNotMatch {
                expected: expected_scratch,
                actual: scratch_imag.len(),
            });
        }

        self.ifft_unchecked(real, imag, output, scratch_real, scratch_imag);

        Ok(())
    }
}
