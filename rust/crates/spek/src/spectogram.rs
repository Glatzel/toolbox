extern crate alloc;
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
    fn frame_unchecked(&self, index: usize) -> &[T];
    fn frame_mut_unchecked(&mut self, index: usize) -> &mut [T];
    fn frame(&self, index: usize) -> Result<&[T], SpectrogramError>;
    fn frame_mut(&mut self, index: usize) -> Result<&mut [T], SpectrogramError>;
}
pub struct Spectrogram<T> {
    pub data: Vec<T>,
    pub frames: usize,
    pub bins: usize,
}

macro_rules! impl_spectrogram {
    ($ty:ty, $stride:expr) => {
        impl ISpectrogram<$ty> for Spectrogram<$ty> {
            fn new(frames: usize, bins: usize) -> Self {
                Self {
                    data: Vec::with_capacity(frames * bins * $stride),
                    frames,
                    bins,
                }
            }

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
        }
    };
}

impl_spectrogram!(Complex<f32>, 1);
impl_spectrogram!(Complex<f64>, 1);
impl_spectrogram!(f32, 2);
impl_spectrogram!(f64, 2);
