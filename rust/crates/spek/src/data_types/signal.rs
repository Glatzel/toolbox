use std::ops::{Deref, DerefMut};

use num_traits::Float;

pub type Signal<T> = Vec<T>;

pub struct SignalRef<'a, T>(&'a [T]);
pub struct SignalRefMut<'a, T>(&'a mut [T]);

// Manual impls: derive(Clone, Copy) would add an unnecessary `T: Copy` bound.
impl<T> Clone for SignalRef<'_, T> {
    fn clone(&self) -> Self { *self }
}
impl<T> Copy for SignalRef<'_, T> {}

impl<T> SignalRefMut<'_, T> {
    /// Cheap mut -> shared view without consuming `self`.
    pub const fn as_signal_ref(&self) -> SignalRef<'_, T> { SignalRef(&*self.0) }

    /// Explicit reborrow, since a struct holding `&mut` is moved, not
    /// implicitly reborrowed, when passed by value.
    pub const fn reborrow(&mut self) -> SignalRefMut<'_, T> { SignalRefMut(&mut *self.0) }
}

impl<'a, T> From<&'a [T]> for SignalRef<'a, T> {
    fn from(s: &'a [T]) -> Self { Self(s) }
}
impl<'a, T> From<&'a Vec<T>> for SignalRef<'a, T> {
    fn from(v: &'a Vec<T>) -> Self { Self(v.as_slice()) }
}
impl<'a, T> From<&'a mut [T]> for SignalRefMut<'a, T> {
    fn from(s: &'a mut [T]) -> Self { Self(s) }
}
impl<'a, T> From<&'a mut Vec<T>> for SignalRefMut<'a, T> {
    fn from(v: &'a mut Vec<T>) -> Self { Self(v.as_mut_slice()) }
}
impl<'a, T> From<SignalRefMut<'a, T>> for SignalRef<'a, T> {
    fn from(s: SignalRefMut<'a, T>) -> Self { Self(s.0) }
}

impl<T> Deref for SignalRef<'_, T> {
    type Target = [T];
    fn deref(&self) -> &[T] { self.0 }
}
impl<T> Deref for SignalRefMut<'_, T> {
    type Target = [T];
    fn deref(&self) -> &[T] { self.0 }
}
impl<T> DerefMut for SignalRefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut [T] { self.0 }
}

impl<T> AsRef<[T]> for SignalRef<'_, T> {
    fn as_ref(&self) -> &[T] { self.0 }
}
impl<T> AsRef<[T]> for SignalRefMut<'_, T> {
    fn as_ref(&self) -> &[T] { self.0 }
}

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

    /// Allocation-free variant: writes the windowed, zero-padded, centered
    /// frame into `out` (which must have length `fft_size`).
    fn frame_into<'a>(
        &self,
        out: impl Into<SignalRefMut<'a, T>>,
        frame_idx: usize,
        win_size: usize,
        hop_size: usize,
        window: &[T],
    ) where
        Self: AsRef<[T]>,
        T: 'a + Float,
    {
        let mut out = out.into();
        let fft_size = out.len();
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
        let frame_end = frame_start + available;

        // Zero only the padding; the middle is fully overwritten below.
        out[..frame_start].fill(T::zero());
        out[frame_end..].fill(T::zero());

        // Equal-length slices let the compiler drop bounds checks and
        // vectorize.
        let src = &signal[start..start + available];
        let win = &window[..available];
        let dst = &mut out[frame_start..frame_end];

        for ((d, &s), &w) in dst.iter_mut().zip(src).zip(win) {
            *d = s * w;
        }
    }

    /// Convenience wrapper that allocates. Prefer `frame_into` in hot loops.
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
        T: Float,
    {
        let mut out = vec![T::zero(); fft_size];
        self.frame_into(
            SignalRefMut::from(&mut out),
            frame_idx,
            win_size,
            hop_size,
            window,
        );
        out
    }
}

impl<T> ISignal<T> for SignalRef<'_, T> {}
impl<T> ISignal<T> for SignalRefMut<'_, T> {}
