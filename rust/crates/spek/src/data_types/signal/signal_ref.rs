use core::slice;

use crate::data_types::Signal;
use crate::data_types::signal::SignalRefMut;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignalRef<'a, T>(&'a [T]);

impl<'a, T> SignalRef<'a, T> {
    pub fn new(data: &'a [T]) -> Self { Self(data) }

    pub fn as_slice(self) -> &'a [T] { self.0 }

    pub fn len(self) -> usize { self.0.len() }

    pub fn is_empty(self) -> bool { self.0.is_empty() }

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
