use core::slice;
use std::ops::{Index, IndexMut};

use num_traits::Float;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Signal<T>(Vec<T>);
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SignalRef<'a, T>(pub &'a [T]);
#[derive(Debug, PartialEq, Default)]
pub struct SignalRefMut<'a, T>(pub &'a mut [T]);

impl<T> Signal<T>
where
    T: Float,
{
    pub fn new(data: Vec<T>) -> Self { Self(data) }

    pub fn as_slice(&self) -> &[T] { &self.0 }

    pub fn as_mut_slice(&mut self) -> &mut [T] { self.0.as_mut_slice() }

    pub fn len(&self) -> usize { self.0.len() }

    pub fn frame_count(&self, win_size: usize, hop_size: usize) -> usize {
        if self.len() < win_size {
            1
        } else {
            ((self.len() - win_size) / hop_size) + 1
        }
    }
    pub fn frame(
        &self,
        frame_idx: usize,
        win_size: usize,
        hop_size: usize,
        fft_size: usize,
    ) -> Signal<T> {
        let start = frame_idx * hop_size;
        let samples = &self.as_slice()[start..start + win_size];
        let mut frame = vec![T::zero(); fft_size];
        frame[..win_size].copy_from_slice(samples);
        frame.into()
    }
}

impl<T> From<Vec<T>> for Signal<T> {
    fn from(data: Vec<T>) -> Self { Self(data) }
}
impl<T> From<[T]> for Signal<T>
where
    [T]: Sized,
    T: Clone,
{
    fn from(data: [T]) -> Self { Self(data.to_vec()) }
}
impl<T> From<&[T]> for Signal<T>
where
    T: Clone,
{
    fn from(data: &[T]) -> Self { Self(data.to_vec()) }
}
impl<T> From<Signal<T>> for Vec<T> {
    fn from(signal: Signal<T>) -> Self { signal.0 }
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

impl<T> Index<usize> for Signal<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output { &self.0[index] }
}

impl<T> IndexMut<usize> for Signal<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.0[index] }
}
