extern crate alloc;
use alloc::vec;
use core::slice::{ChunksExact, ChunksExactMut};

use num_traits::Float;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::conversion::{power_to_db, spectrum_to_amplitude, spectrum_to_power};
use crate::spectrogram::Spectrogram;

#[derive(Debug, Clone)]
pub struct Spectrum2D<T> {
    real: Vec<T>,
    imag: Vec<T>,
    frame_count: usize,
    bin_count: usize,
}
unsafe impl<T> Send for Spectrum2D<T> {}
unsafe impl<T> Sync for Spectrum2D<T> {}

impl<T> Spectrum2D<T>
where
    T: Float,
{
    pub fn new(frame_count: usize, bin_count: usize) -> Self {
        Self {
            real: vec![T::zero(); frame_count * bin_count],
            imag: vec![T::zero(); frame_count * bin_count],
            frame_count,
            bin_count,
        }
    }
    pub fn real(&self) -> &[T] { &self.real }
    pub fn imag(&self) -> &[T] { &self.imag }
    pub const fn bin_count(&self) -> usize { self.bin_count }
    pub const fn frame_count(&self) -> usize { self.frame_count }

    pub fn frame(&self, index: usize) -> (&[T], &[T]) {
        let start = index * self.bin_count;

        let end = start + self.bin_count;

        unsafe {
            (
                self.real.get_unchecked(start..end),
                self.imag.get_unchecked(start..end),
            )
        }
    }
    pub fn frame_mut(&mut self, index: usize) -> (&mut [T], &mut [T]) {
        let start = index * self.bin_count;
        let end = start + self.bin_count;
        unsafe {
            (
                self.real.get_unchecked_mut(start..end),
                self.imag.get_unchecked_mut(start..end),
            )
        }
    }
    pub fn frame_iter(&self, index: usize) -> impl Iterator<Item = (&T, &T)> {
        let (real, imag) = self.frame(index);
        real.iter().zip(imag.iter())
    }

    pub fn frame_iter_mut(&mut self, index: usize) -> impl Iterator<Item = (&mut T, &mut T)> {
        let (real, imag) = self.frame_mut(index);
        real.iter_mut().zip(imag.iter_mut())
    }
    pub fn iter(&self) -> impl Iterator<Item = (&T, &T)> + '_ {
        self.real.iter().zip(self.imag.iter()).map(|c| c)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&mut T, &mut T)> + '_ {
        self.real.iter_mut().zip(self.imag.iter_mut()).map(|c| c)
    }

    pub fn frames_iter(&self) -> std::iter::Zip<ChunksExact<'_, T>, ChunksExact<'_, T>> {
        self.real
            .chunks_exact(self.bin_count)
            .zip(self.imag.chunks_exact(self.bin_count))
    }
    pub fn frames_iter_mut(
        &mut self,
    ) -> std::iter::Zip<ChunksExactMut<'_, T>, ChunksExactMut<'_, T>> {
        self.real
            .chunks_exact_mut(self.bin_count)
            .zip(self.imag.chunks_exact_mut(self.bin_count))
    }

    pub fn iter_bin(&self) { todo!() }
    pub fn iter_bin_mut(&mut self) { todo!() }
    pub fn power(&self) -> Spectrogram<T> {
        Spectrogram::new(
            self.iter()
                .map(|(r, i)| spectrum_to_power(*r, *i))
                .collect(),
            self.frame_count,
            self.bin_count,
        )
    }
    pub fn amplitude(&self) -> Spectrogram<T> {
        Spectrogram::new(
            self.iter()
                .map(|(r, i)| spectrum_to_amplitude(*r, *i))
                .collect(),
            self.frame_count,
            self.bin_count,
        )
    }
    pub fn db(&self, reference: impl Fn(&[T]) -> T, amin: T, top_db: T) -> Spectrogram<T> {
        let power: Vec<_> = self
            .iter()
            .map(|(r, i)| spectrum_to_power(*r, *i))
            .collect();
        let reference_value = reference(&power);
        Spectrogram::new(
            power
                .into_iter()
                .map(|p| power_to_db(p, reference_value, amin, top_db))
                .collect(),
            self.frame_count,
            self.bin_count,
        )
    }
    pub fn to_spectrogram<F>(&self, process: F) -> Spectrogram<T>
    where
        F: Fn(T, T) -> T,
    {
        Spectrogram::new(
            self.iter().map(|(r, i)| process(*r, *i)).collect(),
            self.frame_count,
            self.bin_count,
        )
    }
}
#[cfg(feature = "parallel")]
impl<T> Spectrum2D<T>
where
    T: Float + Sync + Send,
{
    pub fn par_iter(&self) -> impl rayon::iter::ParallelIterator<Item = (&T, &T)> + '_ {
        use rayon::prelude::*;
        self.real.par_iter().zip(self.imag.par_iter()).map(|c| c)
    }

    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = (&mut T, &mut T)> + '_ {
        self.real
            .par_iter_mut()
            .zip(self.imag.par_iter_mut())
            .map(|c| c)
    }
    pub fn frame_par_iter(&self, index: usize) -> impl ParallelIterator<Item = (&T, &T)> {
        let (real, imag) = self.frame(index);
        real.par_iter().zip(imag.par_iter())
    }

    pub fn frame_par_iter_mut(
        &mut self,
        index: usize,
    ) -> impl ParallelIterator<Item = (&mut T, &mut T)> {
        let (real, imag) = self.frame_mut(index);
        real.par_iter_mut().zip(imag.par_iter_mut())
    }
    pub fn frames_par_iter(
        &self,
    ) -> rayon::iter::Zip<rayon::slice::ChunksExact<'_, T>, rayon::slice::ChunksExact<'_, T>> {
        self.real
            .par_chunks_exact(self.bin_count)
            .zip(self.imag.par_chunks_exact(self.bin_count))
    }
    pub fn frames_par_iter_mut(
        &mut self,
    ) -> rayon::iter::Zip<rayon::slice::ChunksExactMut<'_, T>, rayon::slice::ChunksExactMut<'_, T>>
    {
        self.real
            .par_chunks_exact_mut(self.bin_count)
            .zip(self.imag.par_chunks_exact_mut(self.bin_count))
    }
    pub fn par_iter_bin(&self) { todo!() }
    pub fn par_iter_bin_mut(&self) { todo!() }
    pub fn power_par(&self) -> Spectrogram<T> {
        Spectrogram::new(
            self.par_iter()
                .map(|(r, i)| spectrum_to_power(*r, *i))
                .collect(),
            self.frame_count,
            self.bin_count,
        )
    }
    pub fn amplitude_par(&self) -> Spectrogram<T> {
        Spectrogram::new(
            self.par_iter()
                .map(|(r, i)| spectrum_to_amplitude(*r, *i))
                .collect(),
            self.frame_count,
            self.bin_count,
        )
    }
    pub fn db_par(&self, reference: impl Fn(&[T]) -> T, amin: T, top_db: T) -> Spectrogram<T> {
        let power: Vec<_> = self
            .par_iter()
            .map(|(r, i)| spectrum_to_power(*r, *i))
            .collect();
        let reference_value = reference(&power);
        Spectrogram::new(
            power
                .into_par_iter()
                .map(|p| power_to_db(p, reference_value, amin, top_db))
                .collect(),
            self.frame_count,
            self.bin_count,
        )
    }
    pub fn to_spectrogram_par<F>(&self, process: F) -> Spectrogram<T>
    where
        F: Fn(T, T) -> T + Sync + Send,
    {
        Spectrogram::new(
            self.par_iter().map(|(r, i)| process(*r, *i)).collect(),
            self.frame_count,
            self.bin_count,
        )
    }
}
