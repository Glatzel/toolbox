use phastft::planner::{PlannerR2c32, PlannerR2c64};

use super::{FftError, IFftBackend};

#[derive(Debug, Clone)]
pub struct Phastft<P, const N_FFT: usize> {
    options: phastft::options::Options,
    planar: P,
}

fn check_fft_size<const N_FFT: usize>(
    input: usize,
    real: usize,
    imag: usize,
) -> Result<(), FftError> {
    let bins = N_FFT / 2 + 1;

    for (actual, expected) in [(input, N_FFT), (real, bins), (imag, bins)] {
        if actual != expected {
            return Err(FftError::SizeNotMatch { expected, actual });
        }
    }

    Ok(())
}

fn check_ifft_size<const N_FFT: usize>(
    real: usize,
    imag: usize,
    output: usize,
    scratch_real: usize,
    scratch_imag: usize,
) -> Result<(), FftError> {
    let bins = N_FFT / 2 + 1;
    let scratch = N_FFT / 2;

    for (actual, expected) in [
        (real, bins),
        (imag, bins),
        (output, N_FFT),
        (scratch_real, scratch),
        (scratch_imag, scratch),
    ] {
        if actual != expected {
            return Err(FftError::SizeNotMatch { expected, actual });
        }
    }

    Ok(())
}

macro_rules! impl_phastft {
    (
        $ty:ty,
        $planner:ty,
        $r2c:ident,
        $c2r:ident
    ) => {
        impl<const N_FFT: usize> Phastft<$planner, N_FFT> {
            pub fn new() -> Self {
                Self {
                    options: phastft::options::Options::guess_options(N_FFT),
                    planar: <$planner>::new(N_FFT),
                }
            }
        }

        impl<const N_FFT: usize> IFftBackend<$ty, N_FFT> for Phastft<$planner, N_FFT> {
            fn fft_unchecked(&self, input: &[$ty], real: &mut [$ty], imag: &mut [$ty]) {
                phastft::$r2c(input, real, imag, &self.planar, &self.options)
            }

            fn ifft_unchecked(
                &self,
                real: &[$ty],
                imag: &[$ty],
                output: &mut [$ty],
                scratch_real: &mut [$ty],
                scratch_imag: &mut [$ty],
            ) {
                phastft::$c2r(
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
                input: &[$ty],
                real: &mut [$ty],
                imag: &mut [$ty],
            ) -> Result<(), FftError> {
                check_fft_size::<N_FFT>(input.len(), real.len(), imag.len())?;

                self.fft_unchecked(input, real, imag);
                Ok(())
            }

            fn ifft(
                &self,
                real: &[$ty],
                imag: &[$ty],
                output: &mut [$ty],
                scratch_real: &mut [$ty],
                scratch_imag: &mut [$ty],
            ) -> Result<(), FftError> {
                check_ifft_size::<N_FFT>(
                    real.len(),
                    imag.len(),
                    output.len(),
                    scratch_real.len(),
                    scratch_imag.len(),
                )?;

                self.ifft_unchecked(real, imag, output, scratch_real, scratch_imag);

                Ok(())
            }
        }
    };
}

impl_phastft!(
    f32,
    PlannerR2c32,
    r2c_fft_f32_with_planner_and_opts,
    c2r_fft_f32_with_planner_and_opts
);

impl_phastft!(
    f64,
    PlannerR2c64,
    r2c_fft_f64_with_planner_and_opts,
    c2r_fft_f64_with_planner_and_opts
);
