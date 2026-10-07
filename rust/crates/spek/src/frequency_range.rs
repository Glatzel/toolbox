use generic_num::num;
use num_traits::Float;

pub fn fft_frequencies<T>(sr: usize, n_fft: usize) -> impl ExactSizeIterator<Item = T>
where
    T: Float,
{
    (0..=( n_fft / 2)).map(move |n| num!(sr * n / n_fft))
}
pub fn cqt_frequencies() { todo!() }
pub fn mel_frequencies() { todo!() }
pub fn tempo_frequencies() { todo!() }
pub fn fourier_tempo_frequencies() { todo!() }
