use core::slice;
use std::iter::Sum;
use std::ops::MulAssign;

use num_traits::Float;

use crate::data_types::Signal;
use crate::data_types::signal::SignalRefMut;
use crate::pad::Pad;

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
        pad: &Pad<T>,
        center: bool,
    ) -> Signal<T>
    where
        T: Sum + MulAssign,
    {
        debug_assert!(win_size > 0, "win_size must be greater than zero");
        debug_assert!(hop_size > 0, "hop_size must be greater than zero");
        debug_assert!(fft_size >= win_size, "fft_size must be >= win_size");
        debug_assert!(
            frame_idx < self.frame_count(win_size, hop_size),
            "frame_idx out of bounds"
        );
        let start = frame_idx * hop_size;
        let frame = SignalRef(&self.0[start..(start + win_size).max(self.len())]);
        let (pad_before, pad_after) = if center {
            let rest = fft_size - win_size;
            let before = rest / 2;
            let after = rest - before;
            (before, after)
        } else {
            (0, fft_size - win_size)
        };
        let mut frame = frame.pad(pad, pad_before, pad_after).unwrap().into_inner();
        for i in 0..win_size {
            frame[pad_before + i] *= window[i];
        }
        frame.into()
    }
}
