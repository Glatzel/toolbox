pub mod phastft;
pub mod rustfft;

use num_complex::{Complex, ComplexFloat};
pub use phastft::RealPhastftBackend;
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum FftError {
    #[error("size not equal, real: {real}, imag: {imag}")]
    SizeNotEqual { real: usize, imag: usize },
    #[error("{name} size not correct, expected {expected}, got {actual}")]
    SizeNotCorrect {
        name: &'static str,
        expected: usize,
        actual: usize,
    },
    #[error("size not power of two, got {size}")]
    SizeNotPowerOfTwo { size: usize },
}
pub trait IRealFftBackend<T, const N_FFT: usize> {
    fn fft(&self, signal: &[T], real: &mut [T], imag: &mut [T]) -> Result<(), FftError> {
        let bins = N_FFT / 2 + 1;
        check_size("input", signal.len(), N_FFT)?;
        check_size("real", real.len(), bins)?;
        check_size("imag", imag.len(), bins)?;

        self.fft_unchecked(signal, real, imag);
        Ok(())
    }
    fn ifft(
        &self,
        real: &[T],
        imag: &[T],
        signal: &mut [T],
        scratch_real: &mut [T],
        scratch_imag: &mut [T],
    ) -> Result<(), FftError> {
        let bins = N_FFT / 2 + 1;
        let scratch_len = N_FFT / 2;
        check_size("real", real.len(), bins)?;
        check_size("imag", imag.len(), bins)?;
        check_size("output", signal.len(), N_FFT)?;
        check_size("scratch_real", scratch_real.len(), scratch_len)?;
        check_size("scratch_imag", scratch_imag.len(), scratch_len)?;

        self.ifft_unchecked(real, imag, signal, scratch_real, scratch_imag);
        Ok(())
    }
    fn fft_unchecked(&self, signal: &[T], real: &mut [T], imag: &mut [T]);
    fn ifft_unchecked(
        &self,
        real: &[T],
        imag: &[T],
        signal: &mut [T],
        scratch_real: &mut [T],
        scratch_imag: &mut [T],
    );
}

pub trait IComplexFftBackend<T, const N_FFT: usize>
where
    T: ComplexFloat,
{
    fn fft(&self, buffer: &mut [Complex<T>]) -> Result<(), FftError> {
        check_size("buffer", buffer.len(), N_FFT)?;
        self.fft_unchecked(buffer);
        Ok(())
    }
    fn ifft(&self, buffer: &mut [Complex<T>], scratch: &mut [Complex<T>]) -> Result<(), FftError> {
        check_size("buffer", buffer.len(), N_FFT)?;
        check_size("scratch", scratch.len(), N_FFT)?;
        self.ifft_unchecked(buffer, scratch);
        Ok(())
    }
    fn fft_unchecked(&self, buffer: &mut [Complex<T>]);
    fn ifft_unchecked(&self, buffer: &mut [Complex<T>], scratch: &mut [Complex<T>]);
}

const fn check_size(name: &'static str, input: usize, expected: usize) -> Result<(), FftError> {
    if input != expected {
        return Err(FftError::SizeNotCorrect {
            name,
            expected,
            actual: input,
        });
    }
    Ok(())
}
