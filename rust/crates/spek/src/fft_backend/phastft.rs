use phastft::planner::{PlannerR2c32, PlannerR2c64};

use super::IRealFftBackend;

#[derive(Debug, Clone)]
pub struct PhastftBackend<P, const N_FFT: usize> {
    options: phastft::options::Options,
    planar: P,
}

macro_rules! impl_phastft {
    (
        $ty:ty,
        $planner:ty,
        $r2c:ident,
        $c2r:ident
    ) => {
        impl<const N_FFT: usize> PhastftBackend<$planner, N_FFT> {
            pub fn new() -> Self {
                Self {
                    options: phastft::options::Options::guess_options(N_FFT),
                    planar: <$planner>::new(N_FFT),
                }
            }
        }
        impl<const N_FFT: usize> Default for PhastftBackend<$planner, N_FFT> {
            fn default() -> Self { Self::new() }
        }
        impl<const N_FFT: usize> IRealFftBackend<$ty, N_FFT> for PhastftBackend<$planner, N_FFT> {
            fn fft_unchecked(&self, signal: &[$ty], real: &mut [$ty], imag: &mut [$ty]) {
                phastft::$r2c(signal, real, imag, &self.planar, &self.options)
            }

            fn ifft_unchecked(
                &self,
                real: &[$ty],
                imag: &[$ty],
                signal: &mut [$ty],
                scratch_real: &mut [$ty],
                scratch_imag: &mut [$ty],
            ) {
                phastft::$c2r(
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
