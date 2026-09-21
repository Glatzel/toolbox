extern crate alloc;
use alloc::vec;

use num_traits::Float;

use crate::conversion::{spectrum_to_amplitude, spectrum_to_db, spectrum_to_magnitude};
use crate::spectrogram::Spectrogram;
use crate::{Data, Dtype};

#[derive(Debug)]
pub struct StftResult<T> {
    data: Data<T>,
    frame_count: usize,
    bin_count: usize,
}
unsafe impl<T> Send for StftResult<T> {}
unsafe impl<T> Sync for StftResult<T> {}

impl<T> StftResult<T>
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
                data: vec![T::zero(); frame_count * bin_count],
                frame_count,
                bin_count,
            }
        }
    }
    pub fn data(&self) -> &[Dtype<T>] { &self.data }
    pub fn bin_count(&self) -> usize { self.bin_count }
    pub fn frame_count(&self) -> usize { self.frame_count }
    pub fn frame(&self, index: usize) -> &[Dtype<T>] {
        let start = index * self.bin_count;
        let end = start + self.bin_count;
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

    #[cfg(feature = "parallel")]
    pub fn frames_mut(&mut self) -> rayon::slice::ChunksMut<'_, Dtype<T>>
    where
        Dtype<T>: Send,
    {
        use rayon::prelude::*;
        #[cfg(feature = "complex")]
        let chunk_size = self.bin_count;
        #[cfg(feature = "split")]
        let chunk_size = self.bin_count * 2;
        self.data.par_chunks_mut(chunk_size)
    }

    pub fn magnitude(&self) -> Spectrogram<T> {
        #[cfg(feature = "complex")]
        {
            Spectrogram::new(
                self.data
                    .iter()
                    .map(|c| spectrum_to_magnitude(c.re, c.im))
                    .collect(),
                self.frame_count,
                self.bin_count,
            )
        }
        #[cfg(feature = "split")]
        {
            let mut amplitude = Vec::with_capacity(self.frame_count);

            for f in 0..self.frame_count {
                for b in 0..self.bin_count {
                    amplitude.push(spectrum_to_magnitude(
                        *unsafe { self.data.get_unchecked(f * self.bin_count * 2 + b * 2) },
                        *unsafe { self.data.get_unchecked(f * self.bin_count * 2 + b * 2 + 1) },
                    ));
                }
            }
            Spectrogram::new(amplitude, self.frame_count, self.bin_count)
        }
    }

    pub fn amplitude(&self, scale: T) -> Spectrogram<T> {
        #[cfg(feature = "complex")]
        {
            Spectrogram::new(
                self.data
                    .iter()
                    .map(|c| spectrum_to_amplitude(c.re, c.im, scale))
                    .collect(),
                self.frame_count,
                self.bin_count,
            )
        }
        #[cfg(feature = "split")]
        {
            let mut amplitude = Vec::with_capacity(self.frame_count);

            for f in 0..self.frame_count {
                for b in 0..self.bin_count {
                    amplitude.push(spectrum_to_amplitude(
                        *unsafe { self.data.get_unchecked(f * self.bin_count * 2 + b * 2) },
                        *unsafe { self.data.get_unchecked(f * self.bin_count * 2 + b * 2 + 1) },
                        scale,
                    ));
                }
            }
            Spectrogram::new(amplitude, self.frame_count, self.bin_count)
        }
    }

    pub fn db(&self, reference: T) -> Spectrogram<T> {
        #[cfg(feature = "complex")]
        {
            Spectrogram::new(
                self.data
                    .iter()
                    .map(|c| spectrum_to_db(c.re, c.im, reference))
                    .collect(),
                self.frame_count,
                self.bin_count,
            )
        }
        #[cfg(feature = "split")]
        {
            let mut amplitude = Vec::with_capacity(self.frame_count);

            for f in 0..self.frame_count {
                for b in 0..self.bin_count {
                    amplitude.push(spectrum_to_db(
                        *unsafe { self.data.get_unchecked(f * self.bin_count * 2 + b * 2) },
                        *unsafe { self.data.get_unchecked(f * self.bin_count * 2 + b * 2 + 1) },
                        reference,
                    ));
                }
            }
            Spectrogram::new(amplitude, self.frame_count, self.bin_count)
        }
    }
}
