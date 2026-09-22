use core::slice::ChunksExactMut;

#[cfg(feature = "parallel")]
use num_traits::Float;

#[derive(Debug)]
pub struct Spectrogram<T> {
    data: Vec<T>,
    frame_count: usize,
    bin_count: usize,
}
impl<T> Spectrogram<T> {
    pub const fn new(data: Vec<T>, frame_count: usize, bin_count: usize) -> Self {
        Self {
            data,
            frame_count,
            bin_count,
        }
    }
    pub fn data(&self) -> &[T] { &self.data }
    pub const fn frame_count(&self) -> usize { self.frame_count }
    pub const fn bin_count(&self) -> usize { self.bin_count }
    pub fn frame(&self, index: usize) -> &[T] {
        let start = index * self.bin_count;
        let end = start + self.bin_count;
        unsafe { self.data.get_unchecked(start..end) }
    }
    pub fn frame_mut(&mut self, index: usize) -> &mut [T] {
        let start = index * self.bin_count;
        let end = start + self.bin_count;
        unsafe { self.data.get_unchecked_mut(start..end) }
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ { self.data.iter() }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> + '_ { self.data.iter_mut() }

    pub fn iter_frame(&self) -> impl Iterator<Item = &[T]> + '_ {
        self.data.chunks_exact(self.bin_count)
    }
    pub fn iter_frame_mut(&mut self) -> ChunksExactMut<'_, T> {
        self.data.chunks_exact_mut(self.bin_count)
    }

    /// Transpose of `iter_frame`: outer axis over bins, inner axis over
    /// frames. Storage is frame-major, so a bin's values across frames are
    /// strided rather than contiguous — hence the nested-iterator shape
    /// instead of a slice.
    pub fn iter_bin(&self) { todo!() }
    pub fn iter_bin_mut(&mut self) { todo!() }
}
#[cfg(feature = "parallel")]
impl<T> Spectrogram<T>
where
    T: Float + Sync + Send,
{
    pub fn par_iter(&self) -> impl rayon::iter::IndexedParallelIterator<Item = &T> + '_ {
        use rayon::prelude::*;
        self.data.par_iter()
    }

    pub fn par_iter_mut(
        &mut self,
    ) -> impl rayon::iter::IndexedParallelIterator<Item = &mut T> + '_ {
        use rayon::prelude::*;
        self.data.par_iter_mut()
    }
    pub fn par_iter_frame(&self) -> impl rayon::iter::IndexedParallelIterator<Item = &[T]> + '_ {
        use rayon::prelude::*;
        self.data.par_chunks_exact(self.bin_count)
    }
    pub fn par_iter_frame_mut(
        &mut self,
    ) -> impl rayon::iter::IndexedParallelIterator<Item = &mut [T]> + '_ {
        use rayon::prelude::*;
        self.data.par_chunks_exact_mut(self.bin_count)
    }
    pub fn par_iter_bin(&self) { todo!() }
    pub fn par_iter_bin_mut(&self) { todo!() }
}
