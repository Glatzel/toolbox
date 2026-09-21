extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

use num_complex::Complex;

use crate::conversion::{spectrum_to_amplitude, spectrum_to_db, spectrum_to_magnitude};
use crate::spectrogram::Spectrogram;
use crate::stft::StftError;

pub trait IStftResult<T, D> {
    fn new(frame_count: usize, bin_count: usize) -> Self;
    fn bin_count(&self) -> usize;
    fn data(&self) -> &[T];
    fn frame_count(&self) -> usize;
    fn frame_unchecked(&self, index: usize) -> &[T];
    fn frame_mut_unchecked(&mut self, index: usize) -> &mut [T];
    fn frame(&self, index: usize) -> Result<&[T], StftError>;
    fn frame_mut(&mut self, index: usize) -> Result<&mut [T], StftError>;
    /// Mutable parallel iterator over each frame's bin_count, in frame order.
    /// Used by the `parallel` STFT path so each frame can be FFT'd on its
    /// own thread without indexing back into `data` per-call.
    #[cfg(feature = "parallel")]
    fn frames_mut_unchecked(&mut self) -> rayon::slice::ChunksMut<'_, T>
    where
        T: Send;
    fn magnitude(&self) -> Spectrogram<D>;
    #[cfg(feature = "parallel")]
    fn magnitude_parallel(&self) -> Spectrogram<D>;
    fn amplitude(&self, scale: D) -> Spectrogram<D>;
    #[cfg(feature = "parallel")]
    fn amplitude_parallel(&self, scale: D) -> Spectrogram<D>;
    fn db(&self, reference: D) -> Spectrogram<D>;
    #[cfg(feature = "parallel")]
    fn db_parallel(&self, reference: D) -> Spectrogram<D>;
}
#[derive(Debug)]
pub struct StftResult<T> {
    data: Vec<T>,
    frame_count: usize,
    bin_count: usize,
}
unsafe impl<T> Send for StftResult<T> {}
unsafe impl<T> Sync for StftResult<T> {}

impl IStftResult<Complex<f32>, f32> for StftResult<Complex<f32>> {
    fn new(frame_count: usize, bin_count: usize) -> Self {
        Self {
            data: vec![<Complex<f32> as Default>::default(); frame_count * bin_count],
            frame_count,
            bin_count,
        }
    }
    fn data(&self) -> &[Complex<f32>] { &self.data }
    fn bin_count(&self) -> usize { self.bin_count }
    fn frame_count(&self) -> usize { self.frame_count }
    fn frame_unchecked(&self, index: usize) -> &[Complex<f32>] {
        let start = index * self.bin_count;
        let end = start + self.bin_count;
        unsafe { self.data.get_unchecked(start..end) }
    }
    fn frame_mut_unchecked(&mut self, index: usize) -> &mut [Complex<f32>] {
        let start = index * self.bin_count;
        let end = start + self.bin_count;
        unsafe { self.data.get_unchecked_mut(start..end) }
    }
    fn frame(&self, index: usize) -> Result<&[Complex<f32>], StftError> {
        if index >= self.frame_count {
            return Err(StftError::FrameIndexOutOfBounds {
                index,
                frame_count: self.frame_count,
            });
        }
        let start = index * self.bin_count;
        let end = start + self.bin_count;
        Ok(&self.data[start..end])
    }
    fn frame_mut(&mut self, index: usize) -> Result<&mut [Complex<f32>], StftError> {
        if index >= self.frame_count {
            return Err(StftError::FrameIndexOutOfBounds {
                index,
                frame_count: self.frame_count,
            });
        }
        let start = index * self.bin_count;
        let end = start + self.bin_count;
        Ok(unsafe { self.data.get_unchecked_mut(start..end) })
    }
    #[cfg(feature = "parallel")]
    fn frames_mut_unchecked(&mut self) -> rayon::slice::ChunksMut<'_, Complex<f32>> {
        use rayon::prelude::*;
        let chunk_size = self.bin_count;
        self.data.par_chunks_mut(chunk_size)
    }

    fn magnitude(&self) -> Spectrogram<f32> {
        Spectrogram::new(
            self.data
                .iter()
                .map(|c| spectrum_to_magnitude(c.re, c.im))
                .collect(),
            self.frame_count,
            self.bin_count,
        )
    }
    #[cfg(feature = "parallel")]
    fn magnitude_parallel(&self) -> Spectrogram<f32> {
        use rayon::prelude::*;

        Spectrogram::new(
            self.data
                .par_iter()
                .map(|c| spectrum_to_magnitude(c.re, c.im))
                .collect(),
            self.frame_count,
            self.bin_count,
        )
    }
    fn amplitude(&self, scale: f32) -> Spectrogram<f32> {
        Spectrogram::new(
            self.data
                .iter()
                .map(|c| spectrum_to_amplitude(c.re, c.im, scale))
                .collect(),
            self.frame_count,
            self.bin_count,
        )
    }
    #[cfg(feature = "parallel")]
    fn amplitude_parallel(&self, scale: f32) -> Spectrogram<f32> {
        use rayon::prelude::*;

        Spectrogram::new(
            self.data
                .par_iter()
                .map(|c| spectrum_to_amplitude(c.re, c.im, scale))
                .collect(),
            self.frame_count,
            self.bin_count,
        )
    }
    fn db(&self, reference: f32) -> Spectrogram<f32> {
        Spectrogram::new(
            self.data
                .iter()
                .map(|c| spectrum_to_db(c.re, c.im, reference))
                .collect(),
            self.frame_count,
            self.bin_count,
        )
    }
    #[cfg(feature = "parallel")]
    fn db_parallel(&self, reference: f32) -> Spectrogram<f32> {
        use rayon::prelude::*;

        Spectrogram::new(
            self.data
                .par_iter()
                .map(|c| spectrum_to_db(c.re, c.im, reference))
                .collect(),
            self.frame_count,
            self.bin_count,
        )
    }
}
impl IStftResult<Complex<f64>, f64> for StftResult<Complex<f64>> {
    fn new(frame_count: usize, bin_count: usize) -> Self {
        Self {
            data: vec![<Complex<f64> as Default>::default(); frame_count * bin_count],
            frame_count,
            bin_count,
        }
    }
    fn data(&self) -> &[Complex<f64>] { &self.data }
    fn bin_count(&self) -> usize { self.bin_count }
    fn frame_count(&self) -> usize { self.frame_count }
    fn frame_unchecked(&self, index: usize) -> &[Complex<f64>] {
        let start = index * self.bin_count;
        let end = start + self.bin_count;
        unsafe { self.data.get_unchecked(start..end) }
    }
    fn frame_mut_unchecked(&mut self, index: usize) -> &mut [Complex<f64>] {
        let start = index * self.bin_count;
        let end = start + self.bin_count;
        unsafe { self.data.get_unchecked_mut(start..end) }
    }
    fn frame(&self, index: usize) -> Result<&[Complex<f64>], StftError> {
        if index >= self.frame_count {
            return Err(StftError::FrameIndexOutOfBounds {
                index,
                frame_count: self.frame_count,
            });
        }
        let start = index * self.bin_count;
        let end = start + self.bin_count;
        Ok(&self.data[start..end])
    }
    fn frame_mut(&mut self, index: usize) -> Result<&mut [Complex<f64>], StftError> {
        if index >= self.frame_count {
            return Err(StftError::FrameIndexOutOfBounds {
                index,
                frame_count: self.frame_count,
            });
        }
        let start = index * self.bin_count;
        let end = start + self.bin_count;
        Ok(unsafe { self.data.get_unchecked_mut(start..end) })
    }
    #[cfg(feature = "parallel")]
    fn frames_mut_unchecked(&mut self) -> rayon::slice::ChunksMut<'_, Complex<f64>> {
        use rayon::prelude::*;
        let chunk_size = self.bin_count;
        self.data.par_chunks_mut(chunk_size)
    }
    fn magnitude(&self) -> Spectrogram<f64> {
        Spectrogram::new(
            self.data
                .iter()
                .map(|c| spectrum_to_magnitude(c.re, c.im))
                .collect(),
            self.frame_count,
            self.bin_count,
        )
    }
    #[cfg(feature = "parallel")]
    fn magnitude_parallel(&self) -> Spectrogram<f64> {
        use rayon::prelude::*;

        Spectrogram::new(
            self.data
                .par_iter()
                .map(|c| spectrum_to_magnitude(c.re, c.im))
                .collect(),
            self.frame_count,
            self.bin_count,
        )
    }
    fn amplitude(&self, scale: f64) -> Spectrogram<f64> {
        Spectrogram::new(
            self.data
                .iter()
                .map(|c| spectrum_to_amplitude(c.re, c.im, scale))
                .collect(),
            self.frame_count,
            self.bin_count,
        )
    }
    #[cfg(feature = "parallel")]
    fn amplitude_parallel(&self, scale: f64) -> Spectrogram<f64> {
        use rayon::prelude::*;

        Spectrogram::new(
            self.data
                .par_iter()
                .map(|c| spectrum_to_amplitude(c.re, c.im, scale))
                .collect(),
            self.frame_count,
            self.bin_count,
        )
    }
    fn db(&self, reference: f64) -> Spectrogram<f64> {
        Spectrogram::new(
            self.data
                .iter()
                .map(|c| spectrum_to_db(c.re, c.im, reference))
                .collect(),
            self.frame_count,
            self.bin_count,
        )
    }
    #[cfg(feature = "parallel")]
    fn db_parallel(&self, reference: f64) -> Spectrogram<f64> {
        use rayon::prelude::*;

        Spectrogram::new(
            self.data
                .par_iter()
                .map(|c| spectrum_to_db(c.re, c.im, reference))
                .collect(),
            self.frame_count,
            self.bin_count,
        )
    }
}
impl IStftResult<f32, f32> for StftResult<f32> {
    fn new(frame_count: usize, bin_count: usize) -> Self {
        Self {
            data: vec![<f32 as Default>::default(); frame_count * bin_count * 2],
            frame_count,
            bin_count,
        }
    }
    fn data(&self) -> &[f32] { &self.data }
    fn bin_count(&self) -> usize { self.bin_count }
    fn frame_count(&self) -> usize { self.frame_count }
    fn frame_unchecked(&self, index: usize) -> &[f32] {
        let start = index * self.bin_count * 2;
        let end = start + self.bin_count * 2;
        unsafe { self.data.get_unchecked(start..end) }
    }
    fn frame_mut_unchecked(&mut self, index: usize) -> &mut [f32] {
        let start = index * self.bin_count * 2;
        let end = start + self.bin_count * 2;
        unsafe { self.data.get_unchecked_mut(start..end) }
    }
    fn frame(&self, index: usize) -> Result<&[f32], StftError> {
        if index >= self.frame_count {
            return Err(StftError::FrameIndexOutOfBounds {
                index,
                frame_count: self.frame_count,
            });
        }
        let start = index * self.bin_count * 2;
        let end = start + self.bin_count * 2;
        Ok(&self.data[start..end])
    }
    fn frame_mut(&mut self, index: usize) -> Result<&mut [f32], StftError> {
        if index >= self.frame_count {
            return Err(StftError::FrameIndexOutOfBounds {
                index,
                frame_count: self.frame_count,
            });
        }
        let start = index * self.bin_count * 2;
        let end = start + self.bin_count * 2;
        Ok(unsafe { self.data.get_unchecked_mut(start..end) })
    }
    #[cfg(feature = "parallel")]
    fn frames_mut_unchecked(&mut self) -> rayon::slice::ChunksMut<'_, f32> {
        use rayon::prelude::*;
        let chunk_size = self.bin_count * 2;
        self.data.par_chunks_mut(chunk_size)
    }
    fn magnitude(&self) -> Spectrogram<f32> {
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
    #[cfg(feature = "parallel")]
    fn magnitude_parallel(&self) -> Spectrogram<f32> {
        use rayon::prelude::*;

        let amplitude: Vec<f32> = self
            .data
            .par_chunks_exact(self.bin_count * 2)
            .flat_map_iter(|frame| {
                frame
                    .chunks_exact(2)
                    .map(|c| spectrum_to_magnitude(c[0], c[1]))
            })
            .collect();

        Spectrogram::new(amplitude, self.frame_count, self.bin_count)
    }
    fn amplitude(&self, scale: f32) -> Spectrogram<f32> {
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
    #[cfg(feature = "parallel")]
    fn amplitude_parallel(&self, scale: f32) -> Spectrogram<f32> {
        use rayon::prelude::*;

        let amplitude: Vec<f32> = self
            .data
            .par_chunks_exact(self.bin_count * 2)
            .flat_map_iter(|frame| {
                frame
                    .chunks_exact(2)
                    .map(|c| spectrum_to_amplitude(c[0], c[1], scale))
            })
            .collect();

        Spectrogram::new(amplitude, self.frame_count, self.bin_count)
    }
    fn db(&self, reference: f32) -> Spectrogram<f32> {
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
    #[cfg(feature = "parallel")]
    fn db_parallel(&self, reference: f32) -> Spectrogram<f32> {
        use rayon::prelude::*;

        let amplitude: Vec<f32> = self
            .data
            .par_chunks_exact(self.bin_count * 2)
            .flat_map_iter(|frame| {
                frame
                    .chunks_exact(2)
                    .map(|c| spectrum_to_db(c[0], c[1], reference))
            })
            .collect();

        Spectrogram::new(amplitude, self.frame_count, self.bin_count)
    }
}
impl IStftResult<f64, f64> for StftResult<f64> {
    fn new(frame_count: usize, bin_count: usize) -> Self {
        Self {
            data: vec![<f64 as Default>::default(); frame_count * bin_count * 2],
            frame_count,
            bin_count,
        }
    }
    fn data(&self) -> &[f64] { &self.data }
    fn bin_count(&self) -> usize { self.bin_count }
    fn frame_count(&self) -> usize { self.frame_count }
    fn frame_unchecked(&self, index: usize) -> &[f64] {
        let start = index * self.bin_count * 2;
        let end = start + self.bin_count * 2;
        unsafe { self.data.get_unchecked(start..end) }
    }
    fn frame_mut_unchecked(&mut self, index: usize) -> &mut [f64] {
        let start = index * self.bin_count * 2;
        let end = start + self.bin_count * 2;
        unsafe { self.data.get_unchecked_mut(start..end) }
    }
    fn frame(&self, index: usize) -> Result<&[f64], StftError> {
        if index >= self.frame_count {
            return Err(StftError::FrameIndexOutOfBounds {
                index,
                frame_count: self.frame_count,
            });
        }
        let start = index * self.bin_count * 2;
        let end = start + self.bin_count * 2;
        Ok(&self.data[start..end])
    }
    fn frame_mut(&mut self, index: usize) -> Result<&mut [f64], StftError> {
        if index >= self.frame_count {
            return Err(StftError::FrameIndexOutOfBounds {
                index,
                frame_count: self.frame_count,
            });
        }
        let start = index * self.bin_count * 2;
        let end = start + self.bin_count * 2;
        Ok(unsafe { self.data.get_unchecked_mut(start..end) })
    }
    #[cfg(feature = "parallel")]
    fn frames_mut_unchecked(&mut self) -> rayon::slice::ChunksMut<'_, f64> {
        use rayon::prelude::*;
        let chunk_size = self.bin_count * 2;
        self.data.par_chunks_mut(chunk_size)
    }

    fn magnitude(&self) -> Spectrogram<f64> {
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
    #[cfg(feature = "parallel")]
    fn magnitude_parallel(&self) -> Spectrogram<f64> {
        use rayon::prelude::*;

        let amplitude = self
            .data
            .par_chunks_exact(self.bin_count * 2)
            .flat_map_iter(|frame| {
                frame
                    .chunks_exact(2)
                    .map(|c| spectrum_to_magnitude(c[0], c[1]))
            })
            .collect();

        Spectrogram::new(amplitude, self.frame_count, self.bin_count)
    }
    fn amplitude(&self, scale: f64) -> Spectrogram<f64> {
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
    #[cfg(feature = "parallel")]
    fn amplitude_parallel(&self, scale: f64) -> Spectrogram<f64> {
        use rayon::prelude::*;

        let amplitude = self
            .data
            .par_chunks_exact(self.bin_count * 2)
            .flat_map_iter(|frame| {
                frame
                    .chunks_exact(2)
                    .map(|c| spectrum_to_amplitude(c[0], c[1], scale))
            })
            .collect();

        Spectrogram::new(amplitude, self.frame_count, self.bin_count)
    }
    fn db(&self, reference: f64) -> Spectrogram<f64> {
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
    #[cfg(feature = "parallel")]
    fn db_parallel(&self, reference: f64) -> Spectrogram<f64> {
        use rayon::prelude::*;

        let amplitude = self
            .data
            .par_chunks_exact(self.bin_count * 2)
            .flat_map_iter(|frame| {
                frame
                    .chunks_exact(2)
                    .map(|c| spectrum_to_db(c[0], c[1], reference))
            })
            .collect();

        Spectrogram::new(amplitude, self.frame_count, self.bin_count)
    }
}
