use core::slice;

use crate::data_types::Signal;
use crate::data_types::signal::SignalRef;

#[derive(Debug, PartialEq, Eq)]
pub struct SignalRefMut<'a, T>(&'a mut [T]);

impl<'a, T> SignalRefMut<'a, T> {
    pub fn new(data: &'a mut [T]) -> Self { Self(data) }

    pub fn as_slice(&self) -> &[T] { self.0 }

    pub fn as_ref(&'a self) -> SignalRef<'a, T> { SignalRef::new(self.0) }

    pub fn as_mut_slice(&mut self) -> &mut [T] { self.0 }

    pub fn into_slice(self) -> &'a mut [T] { self.0 }

    pub fn len(&self) -> usize { self.0.len() }

    pub fn is_empty(&self) -> bool { self.0.is_empty() }

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
