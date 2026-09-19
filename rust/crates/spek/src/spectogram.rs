extern crate alloc;
use alloc::vec::Vec;

use num_traits::{Float, FloatConst};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum SpectrogramError {
    #[error("frame index out of bounds, index: {index}, frames: {frames}")]
    FrameIndexOutOfBounds { index: usize, frames: usize },
}

pub struct Spectrogram<T>
where
    T: Float + FloatConst,
{
    pub real: Vec<T>,
    pub imag: Vec<T>,
    pub frames: usize,
    pub bins: usize,
}

impl<T> Spectrogram<T>
where
    T: Float + FloatConst,
{
    pub fn new(frames: usize, bins: usize) -> Self {
        Self {
            real: Vec::with_capacity(frames * bins),
            imag: Vec::with_capacity(frames * bins),
            frames,
            bins,
        }
    }
    pub fn frame_unchecked(&self, index: usize) -> (&[T], &[T]) {
        let start = index * self.bins;
        let end = start + self.bins;

        unsafe {
            (
                &self.real.get_unchecked(start..end),
                &self.imag.get_unchecked(start..end),
            )
        }
    }
    pub fn frame_mut_unchecked(&mut self, index: usize) -> (&mut [T], &mut [T]) {
        let start = index * self.bins;
        let end = start + self.bins;

        unsafe {
            (
                self.real.get_unchecked_mut(start..end),
                self.imag.get_unchecked_mut(start..end),
            )
        }
    }
    pub fn frame(&self, index: usize) -> Result<(&[T], &[T]), SpectrogramError> {
        if index >= self.frames {
            return Err(SpectrogramError::FrameIndexOutOfBounds {
                index,
                frames: self.frames,
            });
        }

        let start = index * self.bins;
        let end = start + self.bins;

        Ok((&self.real[start..end], &self.imag[start..end]))
    }

    pub fn frame_mut(&mut self, index: usize) -> Result<(&mut [T], &mut [T]), SpectrogramError> {
        if index >= self.frames {
            return Err(SpectrogramError::FrameIndexOutOfBounds {
                index,
                frames: self.frames,
            });
        }

        let start = index * self.bins;
        let end = start + self.bins;

        Ok((&mut self.real[start..end], &mut self.imag[start..end]))
    }
}
