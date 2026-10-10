use core::slice;

use num_traits::Float;

use crate::data_types::Signal;
use crate::data_types::signal::SignalRef;
use crate::pad::IPad;
use crate::windows::IWindow;

#[derive(Debug, PartialEq, Eq)]
pub struct SignalRefMut<'a, T>(&'a mut [T]);

impl<'a, T> SignalRefMut<'a, T> {
    pub const fn new(data: &'a mut [T]) -> Self { Self(data) }

    pub const fn as_slice(&self) -> &[T] { self.0 }

    pub const fn as_ref(&'a self) -> SignalRef<'a, T> { SignalRef::new(self.0) }

    pub const fn as_mut_slice(&mut self) -> &mut [T] { self.0 }

    pub const fn into_slice(self) -> &'a mut [T] { self.0 }

    pub const fn len(&self) -> usize { self.0.len() }

    pub const fn is_empty(&self) -> bool { self.0.is_empty() }

    pub fn get(&self, index: usize) -> Option<&T> { self.0.get(index) }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> { self.0.get_mut(index) }

    pub fn iter(&self) -> slice::Iter<'_, T> { self.0.iter() }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> { self.0.iter_mut() }
}

impl<'a, T> From<&'a mut [T]> for SignalRefMut<'a, T> {
    fn from(data: &'a mut [T]) -> Self { Self(data) }
}

impl<'a, T> From<&'a mut Signal<T>> for SignalRefMut<'a, T> {
    fn from(signal: &'a mut Signal<T>) -> Self { signal.as_mut() }
}

impl<'a, T> IntoIterator for SignalRefMut<'a, T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter { self.0.iter_mut() }
}

impl<T: Float> SignalRefMut<'_, T> {
    pub const fn frame_count(&self, win_size: usize, hop_size: usize) -> usize {
        self.as_ref().frame_count(win_size, hop_size)
    }
    pub fn frame<P, W>(
        &self,
        frame_idx: usize,
        win_size: usize,
        hop_size: usize,
        fft_size: usize,
        window: &[T],
        pad: &P,
        center: bool,
    ) -> Signal<T>
    where
        P: IPad<T>,
        W: IWindow<T>,
    {
        self.as_ref()
            .frame(frame_idx, win_size, hop_size, fft_size, window, pad, center)
    }
}
