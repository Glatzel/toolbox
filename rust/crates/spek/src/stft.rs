extern crate alloc;

use alloc::vec::Vec;
use core::marker::PhantomData;

use num_traits::{Float, FloatConst};
use thiserror::Error;

use crate::fft_backend::{FftError, IFftBackend};
use crate::pad::PadError;
use crate::spectogram::{ISpectrogram, Spectrogram, SpectrogramError};
use crate::windows::{Window, WindowError};

#[derive(Error, Debug)]
pub enum StftError {
    #[error(transparent)]
    Pad(#[from] PadError),
    #[error(transparent)]
    Window(#[from] WindowError),
    #[error(transparent)]
    Spectrogram(#[from] SpectrogramError),
    #[error(transparent)]
    Fft(#[from] FftError),

    #[error("invalid input size, expected {expected}, got {actual}")]
    InvalidFrameInputSize { expected: usize, actual: usize },
}

pub struct RealImagStft<T, const FFT_SIZE: usize, FftBackend, SP>
where
    T: Float + FloatConst,
    FftBackend: IFftBackend<T, SP, FFT_SIZE>,
    Spectrogram<SP>: ISpectrogram<SP>,
{
    hop_size: usize,
    win_size: usize,
    window: Vec<T>,
    fft_backend: FftBackend,
    phantom: PhantomData<SP>,
}

impl<T, const FFT_SIZE: usize, FftBackend, SP> RealImagStft<T, FFT_SIZE, FftBackend, SP>
where
    T: Float + FloatConst,
    FftBackend: IFftBackend<T, SP, FFT_SIZE>,
    Spectrogram<SP>: ISpectrogram<SP>,
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
            phantom: PhantomData,
        })
    }
    const fn frame_count(&self, signal: &[T]) -> usize {
        ((signal.len() - self.win_size) / self.hop_size) + 1
    }
    fn frame_unchecked(&self, input: &[T]) -> Vec<T> {
        let frame: Vec<T> = input
            .iter()
            .zip(self.window.iter())
            .map(|(i, w)| *i * *w)
            .collect();
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
    pub fn stft_frame_unchecked(&self, input: &[T], scratch: &mut [SP]) -> Vec<SP> {
        let mut frame = self.frame_unchecked(input);
        let mut spectrum = self.fft_backend.new_spectrum();
        self.fft_backend
            .fft_unchecked(&mut frame, &mut spectrum, scratch);
        spectrum
    }

    pub fn stft_frame(&self, input: &[T], scratch: &mut [SP]) -> Result<Vec<SP>, StftError> {
        let mut frame = self.frame(input)?;
        let mut spectrum = self.fft_backend.new_spectrum();
        self.fft_backend.fft(&mut frame, &mut spectrum, scratch)?;
        Ok(spectrum)
    }
    pub fn stft(&self, signal: &[T]) -> Result<Spectrogram<SP>, StftError> {
        let mut spectrogram = self.fft_backend.new_spectrogram(self.frame_count(signal));
        let mut scratch = self.fft_backend.new_forward_scratch();
        for start in 0..signal.len() - self.frame_count(signal) {
            let mut frame = self.frame_unchecked(&signal[start..start + self.win_size]);
            let spectrum = spectrogram.frame_mut_unchecked(start);
            self.fft_backend
                .fft_unchecked(&mut frame, spectrum, &mut scratch);
        }

        Ok(spectrogram)
    }
    pub fn istft(&self) { todo!() }
}
