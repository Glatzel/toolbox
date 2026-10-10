use core::slice;
use std::iter::FromIterator;

use num_traits::Float;

use crate::data_types::signal::{SignalRef, SignalRefMut};

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Signal<T>(Vec<T>);

impl<T> Signal<T> {
    pub fn new(data: Vec<T>) -> Self { Self(data) }

    pub fn with_capacity(capacity: usize) -> Self { Self(Vec::with_capacity(capacity)) }

    pub fn as_slice(&self) -> &[T] { &self.0 }

    pub fn as_mut_slice(&mut self) -> &mut [T] { &mut self.0 }

    pub fn as_ref(&self) -> SignalRef<'_, T> { SignalRef::new(&self.0) }

    pub fn as_mut(&mut self) -> SignalRefMut<'_, T> { SignalRefMut::new(&mut self.0) }

    pub fn into_inner(self) -> Vec<T> { self.0 }

    pub fn len(&self) -> usize { self.0.len() }

    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    pub fn capacity(&self) -> usize { self.0.capacity() }

    pub fn reserve(&mut self, additional: usize) { self.0.reserve(additional); }

    pub fn push(&mut self, value: T) { self.0.push(value); }

    pub fn pop(&mut self) -> Option<T> { self.0.pop() }

    pub fn clear(&mut self) { self.0.clear(); }

    pub fn truncate(&mut self, len: usize) { self.0.truncate(len); }

    pub fn get(&self, index: usize) -> Option<&T> { self.0.get(index) }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> { self.0.get_mut(index) }

    pub fn iter(&self) -> slice::Iter<'_, T> { self.0.iter() }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> { self.0.iter_mut() }
}

impl<T> From<Vec<T>> for Signal<T> {
    fn from(data: Vec<T>) -> Self { Self(data) }
}

impl<T, const N: usize> From<[T; N]> for Signal<T> {
    fn from(data: [T; N]) -> Self { Self(Vec::from(data)) }
}

impl<T: Clone> From<&[T]> for Signal<T> {
    fn from(data: &[T]) -> Self { Self(data.to_vec()) }
}

impl<T: Clone, const N: usize> From<&[T; N]> for Signal<T> {
    fn from(data: &[T; N]) -> Self { Self(data.to_vec()) }
}

impl<T> From<Signal<T>> for Vec<T> {
    fn from(signal: Signal<T>) -> Self { signal.0 }
}

impl<T> FromIterator<T> for Signal<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self { Self(iter.into_iter().collect()) }
}

impl<T> IntoIterator for Signal<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

impl<'a, T> IntoIterator for &'a Signal<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter { self.0.iter() }
}

impl<'a, T> IntoIterator for &'a mut Signal<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter { self.0.iter_mut() }
}

impl<T: Float> Signal<T> {
    pub fn frame_count(&self, win_size: usize, hop_size: usize) -> usize {
        self.as_ref().frame_count(win_size, hop_size)
    }
    pub fn frame(
        &self,
        frame_idx: usize,
        win_size: usize,
        hop_size: usize,
        fft_size: usize,
    ) -> Signal<T> {
        self.as_ref().frame(frame_idx, win_size, hop_size, fft_size)
    }
}
