pub mod phastft;

use crate::data_types::Spectrum2D;

pub trait IFftBackend<T> {
    fn fft_size(&self) -> usize;
    fn signal_size(&self) -> usize;
    fn spectrum_size(&self) -> usize;
    fn scratch_size(&self) -> usize;
    fn new_signal(&self) -> Vec<T>;
    fn new_spectrum(&self) -> (Vec<T>, Vec<T>);
    fn new_scratch(&self) -> (Vec<T>, Vec<T>);
    fn new_spectrum2d(&self, frame_count: usize) -> Spectrum2D<T>;
    fn fft(&self, signal: &[T], real: &mut [T], imag: &mut [T]);
    fn ifft(
        &self,
        real: &[T],
        imag: &[T],
        signal: &mut [T],
        scratch_real: &mut [T],
        scratch_imag: &mut [T],
    );
}
