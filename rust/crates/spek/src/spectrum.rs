extern crate alloc;
use alloc::vec;
use core::slice::ChunksExactMut;

use num_traits::Float;

use crate::conversion::{spectrum_to_amplitude, spectrum_to_db, spectrum_to_magnitude};
use crate::spectrogram::Spectrogram;
use crate::{Data, Dtype};

#[derive(Debug, Clone)]
pub struct Spectrum2D<T> {
    data: Data<T>,
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
        #[cfg(feature = "complex")]
        {
            Self {
                data: vec![
                    num_complex::Complex {
                        re: T::zero(),
                        im: T::zero()
                    };
                    frame_count * bin_count
                ],
                frame_count,
                bin_count,
            }
        }

        #[cfg(feature = "split")]
        {
            Self {
                data: vec![T::zero(); frame_count * bin_count * 2],
                frame_count,
                bin_count,
            }
        }
    }
    pub fn data(&self) -> &[Dtype<T>] { &self.data }
    pub fn bin_count(&self) -> usize { self.bin_count }
    pub fn frame_count(&self) -> usize { self.frame_count }

    /// FIX: previously missing the `split`-aware stride that `frame_mut`
    /// already had. Under `split`, each frame occupies `bin_count * 2`
    /// elements (a real block followed by an imag block), so the flat
    /// `complex`-style stride silently read the wrong window.
    pub fn frame(&self, index: usize) -> &[Dtype<T>] {
        #[cfg(feature = "complex")]
        let start = index * self.bin_count;
        #[cfg(feature = "complex")]
        let end = start + self.bin_count;
        #[cfg(feature = "split")]
        let start = index * self.bin_count * 2;
        #[cfg(feature = "split")]
        let end = start + self.bin_count * 2;

        unsafe { self.data.get_unchecked(start..end) }
    }
    pub fn frame_mut(&mut self, index: usize) -> &mut [Dtype<T>] {
        #[cfg(feature = "complex")]
        let start = index * self.bin_count;
        #[cfg(feature = "complex")]
        let end = start + self.bin_count;
        #[cfg(feature = "split")]
        let start = index * self.bin_count * 2;
        #[cfg(feature = "split")]
        let end = start + self.bin_count * 2;

        unsafe { self.data.get_unchecked_mut(start..end) }
    }
    pub fn iter(&self) -> impl Iterator<Item = (&T, &T)> + '_ {
        #[cfg(feature = "complex")]
        {
            self.data.iter().map(|c| (&c.re, &c.im))
        }

        #[cfg(feature = "split")]
        {
            self.data
                .chunks_exact(self.bin_count * 2)
                .flat_map(|frame| {
                    let (real, imag) = unsafe { frame.split_at_unchecked(self.bin_count) };
                    real.iter().zip(imag.iter())
                })
        }
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&mut T, &mut T)> + '_ {
        #[cfg(feature = "complex")]
        {
            self.data.iter_mut().map(|c| (&mut c.re, &mut c.im))
        }
        #[cfg(feature = "split")]
        {
            self.data
                .chunks_exact_mut(self.bin_count * 2)
                .flat_map(|frame| {
                    let (real, imag) = unsafe { frame.split_at_mut_unchecked(self.bin_count) };
                    real.iter_mut().zip(imag.iter_mut())
                })
        }
    }

    pub fn iter_frame(&self) -> impl Iterator<Item = &[Dtype<T>]> + '_ {
        #[cfg(feature = "complex")]
        {
            self.data.chunks_exact(self.bin_count)
        }

        #[cfg(feature = "split")]
        {
            self.data.chunks_exact(self.bin_count * 2)
        }
    }
    pub fn iter_frame_mut(&mut self) -> ChunksExactMut<'_, Dtype<T>> {
        #[cfg(feature = "complex")]
        {
            self.data.chunks_exact_mut(self.bin_count)
        }

        #[cfg(feature = "split")]
        {
            self.data.chunks_exact_mut(self.bin_count * 2)
        }
    }

    /// Transpose of `iter_frame`: outer axis over bins, inner axis over
    /// frames. Storage is frame-major, so a bin's values across frames are
    /// strided rather than contiguous — hence the nested-iterator shape
    /// instead of a slice.
    pub fn iter_bin(&self) { todo!() }
    pub fn iter_bin_mut(&mut self) { todo!() }
    pub fn magnitude(&self) -> Spectrogram<T> {
        Spectrogram::new(
            self.iter()
                .map(|(r, i)| spectrum_to_magnitude(*r, *i))
                .collect(),
            self.frame_count,
            self.bin_count,
        )
    }
    pub fn amplitude(&self, scale: T) -> Spectrogram<T> {
        Spectrogram::new(
            self.iter()
                .map(|(r, i)| spectrum_to_amplitude(*r, *i, scale))
                .collect(),
            self.frame_count,
            self.bin_count,
        )
    }
    pub fn db(&self, reference: T) -> Spectrogram<T> {
        Spectrogram::new(
            self.iter()
                .map(|(r, i)| spectrum_to_db(*r, *i, reference))
                .collect(),
            self.frame_count,
            self.bin_count,
        )
    }
    pub fn to_spectrogram(&self, process: impl Fn(T, T) -> T) -> Spectrogram<T> {
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
    Dtype<T>: Send + Sync,
{
    pub fn par_iter(&self) -> impl rayon::iter::IndexedParallelIterator<Item = (&T, &T)> + '_ {
        use rayon::prelude::*;
        #[cfg(feature = "complex")]
        {
            self.data.par_iter().map(|c| (&c.re, &c.im))
        }
        #[cfg(feature = "split")]
        {
            let bin_count = self.bin_count;
            (0..self.frame_count * self.bin_count)
                .into_par_iter()
                .map(move |i| {
                    let frame = i / bin_count;
                    let bin = i % bin_count;
                    let offset = frame * bin_count * 2;
                    unsafe {
                        (
                            self.data.get_unchecked(offset + bin),
                            self.data.get_unchecked(offset + bin_count + bin),
                        )
                    }
                })
        }
    }

    pub fn par_iter_mut(
        &mut self,
    ) -> impl rayon::iter::IndexedParallelIterator<Item = (&mut T, &mut T)> + '_ {
        use rayon::prelude::*;
        #[cfg(feature = "complex")]
        {
            self.data.par_iter_mut().map(|c| (&mut c.re, &mut c.im))
        }
        #[cfg(feature = "split")]
        {
            let bin_count = self.bin_count;
            let len = self.frame_count * bin_count;

            struct SyncPtr<T>(*mut T);

            unsafe impl<T: Send> Send for SyncPtr<T> {}
            unsafe impl<T: Sync> Sync for SyncPtr<T> {}

            impl<T> SyncPtr<T> {
                #[inline]
                fn get(&self) -> *mut T { self.0 }
            }

            let ptr = SyncPtr(self.data.as_mut_ptr());

            (0..len).into_par_iter().map(move |i| {
                let frame = i / bin_count;
                let bin = i % bin_count;

                let offset = frame * bin_count * 2;
                let p = ptr.get();

                // SAFETY:
                // `i` uniquely identifies one `(real, imag)` pair.
                // Real and imaginary regions are disjoint, and different `i`
                // values never produce overlapping mutable references.
                unsafe {
                    (
                        &mut *p.add(offset + bin),
                        &mut *p.add(offset + bin_count + bin),
                    )
                }
            })
        }
    }
    pub fn par_iter_frame(
        &self,
    ) -> impl rayon::iter::IndexedParallelIterator<Item = &[Dtype<T>]> + '_ {
        use rayon::prelude::*;
        #[cfg(feature = "complex")]
        {
            self.data.par_chunks_exact(self.bin_count)
        }

        #[cfg(feature = "split")]
        {
            self.data.par_chunks_exact(self.bin_count * 2)
        }
    }
    pub fn par_iter_frame_mut(
        &mut self,
    ) -> impl rayon::iter::IndexedParallelIterator<Item = &mut [Dtype<T>]> + '_ {
        use rayon::prelude::*;
        #[cfg(feature = "complex")]
        {
            self.data.par_chunks_exact_mut(self.bin_count)
        }

        #[cfg(feature = "split")]
        {
            self.data.par_chunks_exact_mut(self.bin_count * 2)
        }
    }
    pub fn par_iter_bin(&self) { todo!() }
    pub fn par_iter_bin_mut(&self) { todo!() }
    pub fn par_magnitude(&self) -> Spectrogram<T> {
        use rayon::prelude::*;
        Spectrogram::new(
            self.par_iter()
                .map(|(r, i)| spectrum_to_magnitude(*r, *i))
                .collect(),
            self.frame_count,
            self.bin_count,
        )
    }
    pub fn par_amplitude(&self, scale: T) -> Spectrogram<T> {
        use rayon::prelude::*;
        Spectrogram::new(
            self.par_iter()
                .map(|(r, i)| spectrum_to_amplitude(*r, *i, scale))
                .collect(),
            self.frame_count,
            self.bin_count,
        )
    }
    pub fn par_db(&self, reference: T) -> Spectrogram<T> {
        use rayon::prelude::*;
        Spectrogram::new(
            self.par_iter()
                .map(|(r, i)| spectrum_to_db(*r, *i, reference))
                .collect(),
            self.frame_count,
            self.bin_count,
        )
    }
    pub fn par_to_spectrogram<F>(&self, process: F) -> Spectrogram<T>
    where
        F: Fn(T, T) -> T + Sync + Send,
    {
        use rayon::prelude::*;
        Spectrogram::new(
            self.par_iter().map(|(r, i)| process(*r, *i)).collect(),
            self.frame_count,
            self.bin_count,
        )
    }
}
