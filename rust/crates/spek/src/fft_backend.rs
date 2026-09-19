mod phastft;
use num_traits::Float;
pub use phastft::Phastft;
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum FftError {
    #[error("size not match, expected {expected}, got {actual}")]
    SizeNotMatch { expected: usize, actual: usize },
    #[error("size not power of two, got {size}")]
    SizeNotPowerOfTwo { size: usize },
}
pub trait IFftBackend<T, const N_FFT: usize>
where
    T: Float,
{
    fn fft(&self, input: &[T], real: &mut [T], imag: &mut [T]) -> Result<(), FftError>;
    fn ifft(
        &self,
        real: &[T],
        imag: &[T],
        output: &mut [T],
        scratch_real: &mut [T],
        scratch_imag: &mut [T],
    ) -> Result<(), FftError>;
    fn fft_unchecked(&self, input: &[T], real: &mut [T], imag: &mut [T]);
    fn ifft_unchecked(
        &self,
        real: &[T],
        imag: &[T],
        output: &mut [T],
        scratch_real: &mut [T],
        scratch_imag: &mut [T],
    );
}
