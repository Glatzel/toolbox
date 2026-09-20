use num_complex::Complex;
use num_traits::Float;
use rustfft::FftDirection::{Forward, Inverse};
use rustfft::{Fft, FftNum};

use crate::fft_backend::{FftError, IFftBackend, check_size, check_size_at_least};

pub struct RustfftBackend<T, P, const N_FFT: usize>
where
    T: FftNum,
    P: Fft<T>,
{
    planner: P,
    phantom: core::marker::PhantomData<T>,
}
impl<T, P, const N_FFT: usize> RustfftBackend<T, P, N_FFT>
where
    T: FftNum,
    P: Fft<T>,
{
    pub const fn new(planner: P) -> Self {
        Self {
            planner,
            phantom: core::marker::PhantomData,
        }
    }
}
impl<T, P, const N_FFT: usize> IFftBackend<T, Complex<T>, Complex<T>, N_FFT>
    for RustfftBackend<T, P, N_FFT>
where
    T: FftNum + Float,
    P: Fft<T>,
{
    fn fft(
        &self,
        signal: &mut [T],
        spectrum: &mut [Complex<T>],
        scratch: &mut [Complex<T>],
    ) -> Result<(), super::FftError> {
        check_size_at_least("signal", signal.len(), self.signal_size())?;
        check_size("spectrum", spectrum.len(), self.spectrum_size())?;
        check_size("scratch", scratch.len(), self.forward_scratch_size())?;
        if self.planner.fft_direction() != Forward {
            return Err(FftError::FftDirection {
                expected: super::FftDirection::Forward,
                actual: super::FftDirection::Inverse,
            });
        }
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
        if self.planner.fft_direction() != Inverse {
            return Err(FftError::FftDirection {
                expected: super::FftDirection::Inverse,
                actual: super::FftDirection::Forward,
            });
        }
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
        self.planner.process_with_scratch(spectrum, scratch);
    }

    fn ifft_unchecked(
        &self,
        spectrum: &mut [Complex<T>],
        signal: &mut [T],
        scratch: &mut [Complex<T>],
    ) {
        self.planner.process_with_scratch(spectrum, scratch);
        for (dst, src) in signal.iter_mut().zip(spectrum.iter()) {
            *dst = src.re;
        }
    }

    fn signal_size(&self) -> usize { N_FFT }

    fn spectrum_size(&self) -> usize { N_FFT / 2 + 1 }

    fn forward_scratch_size(&self) -> usize { self.planner.get_inplace_scratch_len() }

    fn inverse_scratch_size(&self) -> usize { self.planner.get_inplace_scratch_len() }

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
}
