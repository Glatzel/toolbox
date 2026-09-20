extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

use num_complex::Complex;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum SpectrogramError {
    #[error("frame index out of bounds, index: {index}, frames: {frames}")]
    FrameIndexOutOfBounds { index: usize, frames: usize },
}
pub trait ISpectrogram<T> {
    fn new(frames: usize, bins: usize) -> Self;
    fn frame_count(&self) -> usize;
    fn frame_unchecked(&self, index: usize) -> &[T];
    fn frame_mut_unchecked(&mut self, index: usize) -> &mut [T];
    fn frame(&self, index: usize) -> Result<&[T], SpectrogramError>;
    fn frame_mut(&mut self, index: usize) -> Result<&mut [T], SpectrogramError>;
    /// Mutable parallel iterator over each frame's bins, in frame order.
    /// Used by the `parallel` STFT path so each frame can be FFT'd on its
    /// own thread without indexing back into `data` per-call.
    #[cfg(feature = "parallel")]
    fn frames_mut_unchecked(&mut self) -> rayon::slice::ChunksMut<'_, T>
    where
        T: Send;
}
#[derive(Debug)]
pub struct Spectrogram<T> {
    pub data: Vec<T>,
    pub frames: usize,
    pub bins: usize,
}
unsafe impl<T> Send for Spectrogram<T> {}
unsafe impl<T> Sync for Spectrogram<T> {}

macro_rules! impl_spectrogram {
    ($ty:ty, $stride:expr) => {
        impl ISpectrogram<$ty> for Spectrogram<$ty> {
            fn new(frames: usize, bins: usize) -> Self {
                Self {
                    // `with_capacity` alone leaves `len() == 0`, so every
                    // `get_unchecked(..)` below would index past the end of
                    // the vec into uninitialized memory (UB), and the
                    // checked accessors would panic. Actually initialize
                    // the storage so every frame/bin slot is valid.
                    data: vec![<$ty as Default>::default(); frames * bins * $stride],
                    frames,
                    bins,
                }
            }
            fn frame_count(&self) -> usize { self.frames }

            fn frame_unchecked(&self, index: usize) -> &[$ty] {
                let start = index * self.bins * $stride;
                let end = start + self.bins * $stride;

                unsafe { self.data.get_unchecked(start..end) }
            }

            fn frame_mut_unchecked(&mut self, index: usize) -> &mut [$ty] {
                let start = index * self.bins * $stride;
                let end = start + self.bins * $stride;

                unsafe { self.data.get_unchecked_mut(start..end) }
            }

            fn frame(&self, index: usize) -> Result<&[$ty], SpectrogramError> {
                if index >= self.frames {
                    return Err(SpectrogramError::FrameIndexOutOfBounds {
                        index,
                        frames: self.frames,
                    });
                }

                let start = index * self.bins * $stride;
                let end = start + self.bins * $stride;

                Ok(&self.data[start..end])
            }

            fn frame_mut(&mut self, index: usize) -> Result<&mut [$ty], SpectrogramError> {
                if index >= self.frames {
                    return Err(SpectrogramError::FrameIndexOutOfBounds {
                        index,
                        frames: self.frames,
                    });
                }

                let start = index * self.bins * $stride;
                let end = start + self.bins * $stride;

                Ok(unsafe { self.data.get_unchecked_mut(start..end) })
            }

            #[cfg(feature = "parallel")]
            fn frames_mut_unchecked(&mut self) -> rayon::slice::ChunksMut<'_, $ty> {
                use rayon::prelude::*;
                let chunk_size = self.bins * $stride;
                self.data.par_chunks_mut(chunk_size)
            }
        }
    };
}

impl_spectrogram!(Complex<f32>, 1);
impl_spectrogram!(Complex<f64>, 1);
impl_spectrogram!(f32, 2);
impl_spectrogram!(f64, 2);
