use std::iter::Sum;

use num_traits::Float;

pub type Signal<T> = Vec<T>;
pub type SignalRef<'a, T> = &'a [T];
pub type SignalRefMut<'a, T> = &'a mut [T];

pub trait ISignal<T> {
    fn frame_count(&self, win_size: usize, hop_size: usize) -> usize
    where
        Self: AsRef<[T]>,
    {
        let len = self.as_ref().len();

        if len < win_size {
            1
        } else {
            (len - win_size) / hop_size + 1
        }
    }

    fn frame(
        &self,
        frame_idx: usize,
        win_size: usize,
        hop_size: usize,
        fft_size: usize,
        window: &[T],
    ) -> Signal<T>
    where
        Self: AsRef<[T]>,
        T: Float + Sum,
    {
        debug_assert!(win_size > 0, "win_size must be greater than zero");
        debug_assert!(hop_size > 0, "hop_size must be greater than zero");
        debug_assert!(fft_size >= win_size, "fft_size must be >= win_size");
        debug_assert!(
            window.len() >= win_size,
            "window length must be >= win_size"
        );
        debug_assert!(
            frame_idx < self.frame_count(win_size, hop_size),
            "frame_idx out of bounds"
        );

        let signal = self.as_ref();
        let start = frame_idx * hop_size;
        let available = signal.len().saturating_sub(start).min(win_size);
        let frame_start = (fft_size - available) / 2;

        let mut frame = vec![T::zero(); fft_size];

        for (i, (&sample, &weight)) in signal[start..start + available]
            .iter()
            .zip(window.iter())
            .enumerate()
        {
            frame[i + frame_start] = sample * weight;
        }

        frame
    }
}

impl<T> ISignal<T> for Vec<T> {}
impl<T> ISignal<T> for [T] {}
