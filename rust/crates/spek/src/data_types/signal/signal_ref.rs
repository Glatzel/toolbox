use core::slice;
use std::iter::Sum;

use num_traits::Float;

use crate::data_types::Signal;
use crate::data_types::signal::SignalRefMut;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignalRef<'a, T>(&'a [T]);

impl<'a, T> SignalRef<'a, T> {
    pub const fn new(data: &'a [T]) -> Self { Self(data) }

    pub const fn as_slice(self) -> &'a [T] { self.0 }

    pub const fn len(self) -> usize { self.0.len() }

    pub const fn is_empty(self) -> bool { self.0.is_empty() }

    pub fn get(self, index: usize) -> Option<&'a T> { self.0.get(index) }

    pub fn iter(self) -> slice::Iter<'a, T> { self.0.iter() }
}

impl<'a, T> From<&'a [T]> for SignalRef<'a, T> {
    fn from(data: &'a [T]) -> Self { Self(data) }
}
impl<'a, T> From<&'a Signal<T>> for SignalRef<'a, T> {
    fn from(signal: &'a Signal<T>) -> Self { signal.as_ref() }
}

impl<'a, T> From<&'a SignalRefMut<'a, T>> for SignalRef<'a, T> {
    fn from(signal: &'a SignalRefMut<'a, T>) -> Self { signal.as_ref() }
}

impl<'a, T> IntoIterator for SignalRef<'a, T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter { self.0.iter() }
}

impl<T: Float> SignalRef<'_, T> {
    /// Returns the number of frames for a non-centered, non-padded frame grid.
    ///
    /// If the signal is shorter than the window, returns one frame.
    /// A zero-length window or hop size is invalid.
    pub const fn frame_count(&self, win_size: usize, hop_size: usize) -> usize {
        if self.len() < win_size {
            1
        } else {
            (self.len() - win_size) / hop_size + 1
        }
    }

    pub fn frame(
        &self,
        frame_idx: usize,
        win_size: usize,
        hop_size: usize,
        fft_size: usize,
        window: &[T],
    ) -> Signal<T>
    where
        T: Sum,
    {
        debug_assert!(win_size > 0, "win_size must be greater than zero");
        debug_assert!(hop_size > 0, "hop_size must be greater than zero");
        debug_assert!(fft_size >= win_size, "fft_size must be >= win_size");
        debug_assert!(
            frame_idx < self.frame_count(win_size, hop_size),
            "frame_idx out of bounds"
        );
        let start = frame_idx * hop_size;
        let available = self.len().saturating_sub(start).min(win_size);
        let frame_start = (fft_size - available) / 2;

        let mut frame = vec![T::zero(); fft_size];

        for (i, (&sample, &weight)) in self.0[start..start + available]
            .iter()
            .zip(window.iter())
            .enumerate()
        {
            frame[i + frame_start] = sample * weight;
        }

        Signal::new(frame)
    }
}
