extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

use num_traits::{Float, FloatConst};
use thiserror::Error;

use crate::pad::PadError;
use crate::spectogram::{Spectrogram, SpectrogramError};
use crate::windows::{Window, WindowError};

#[derive(Error, Debug)]
pub enum StftError {
    #[error(transparent)]
    Pad(#[from] PadError),
    #[error(transparent)]
    Window(#[from] WindowError),
    #[error(transparent)]
    Spectrogram(#[from] SpectrogramError),

    #[error("invalid input size, expected {expected}, got {actual}")]
    InvalidFrameInputSize { expected: usize, actual: usize },
}

pub struct RealImagStft<T, const FFT_SIZE: usize, FftBackend>
where
    T: Float + FloatConst,
{
    hop_size: usize,
    win_size: usize,
    window: Vec<T>,
    fft_backend: FftBackend,
}

impl<T, const FFT_SIZE: usize, FftBackend> RealImagStft<T, FFT_SIZE, FftBackend>
where
    T: Float + FloatConst,
    FftBackend: crate::fft_backend::IFftBackend<T, FFT_SIZE>,
{
    pub fn new(
        hop_size: usize,
        win_size: usize,
        window: Window<T>,
        fft_backend: FftBackend,
    ) -> Result<Self, StftError> {
        Ok(Self {
            hop_size,
            win_size,
            window: window.window(win_size, false)?,
            fft_backend,
        })
    }
    const fn frame_count(&self, signal: &[T]) -> usize {
        ((signal.len() - self.win_size) / self.hop_size) + 1
    }
    fn frame_unchecked(&self, input: &[T]) -> Vec<T> {
        let mut frame = vec![T::zero(); FFT_SIZE];

        // apply window
        for i in 0..self.win_size {
            frame[i] = input[i] * self.window[i];
        }

        frame
    }
    fn frame(&self, input: &[T]) -> Result<Vec<T>, StftError> {
        if input.len() != self.win_size {
            return Err(StftError::InvalidFrameInputSize {
                expected: self.win_size,
                actual: input.len(),
            });
        }

        Ok(self.frame_unchecked(input))
    }
    pub fn stft_frame_unchecked(&self, signal: &[T]) -> (Vec<T>, Vec<T>) {
        let frame = self.frame_unchecked(signal);
        let mut real = vec![T::zero(); FFT_SIZE / 2 + 1];
        let mut imag = vec![T::zero(); FFT_SIZE / 2 + 1];
        self.fft_backend.fft_unchecked(&frame, &mut real, &mut imag);
        (real, imag)
    }
    pub fn stft_frame(&self, signal: &[T]) -> Result<(Vec<T>, Vec<T>), StftError> {
        let frame = self.frame(signal)?;
        let mut real = vec![T::zero(); FFT_SIZE / 2 + 1];
        let mut imag = vec![T::zero(); FFT_SIZE / 2 + 1];
        self.fft_backend.fft_unchecked(&frame, &mut real, &mut imag);
        Ok((real, imag))
    }
    pub fn stft(&self, signal: &[T]) -> Result<Spectrogram<T>, StftError> {
        let mut spectrogram = Spectrogram::new(self.frame_count(signal), FFT_SIZE / 2 + 1);
        for start in 0..signal.len() - self.frame_count(signal) {
            let frame = self.frame_unchecked(&signal[start..start + self.win_size]);
            let (real, imag) = spectrogram.frame_mut_unchecked(start);
            self.fft_backend.fft_unchecked(&frame, real, imag);
        }

        Ok(spectrogram)
    }
    pub fn istft(&self) { todo!() }
}
