extern crate alloc;

use alloc::vec;
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

pub struct Stft<T, const FFT_SIZE: usize, FftBackend, SP>
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

impl<T, const FFT_SIZE: usize, FftBackend, SP> Stft<T, FFT_SIZE, FftBackend, SP>
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

    /// Number of complete frames that fit in `signal`.
    ///
    /// Returns 0 rather than underflowing/panicking when `signal` is
    /// shorter than the window; callers that need a hard error for that
    /// case should check `signal.len() < self.win_size` first (as `stft`
    /// and `istft` do).
    const fn frame_count(&self, signal_len: usize) -> usize {
        if signal_len < self.win_size {
            0
        } else {
            ((signal_len - self.win_size) / self.hop_size) + 1
        }
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

    /// Core STFT loop, shared by the checked/unchecked/parallel variants.
    /// `signal.len() >= self.win_size` must already hold.
    fn stft_frames_unchecked(&self, signal: &[T]) -> Spectrogram<SP> {
        let frame_count = self.frame_count(signal.len());
        let mut spectrogram = self.fft_backend.new_spectrogram(frame_count);
        let mut scratch = self.fft_backend.new_forward_scratch();

        for frame_idx in 0..frame_count {
            let start = frame_idx * self.hop_size;
            let mut frame = self.frame_unchecked(&signal[start..start + self.win_size]);
            let spectrum = spectrogram.frame_mut_unchecked(frame_idx);
            self.fft_backend
                .fft_unchecked(&mut frame, spectrum, &mut scratch);
        }

        spectrogram
    }

    #[cfg(feature = "parallel")]
    pub fn stft_parallel_unchecked(&self, signal: &[T]) -> Spectrogram<SP>
    where
        T: Sync,
        SP: Send + Sync,
        FftBackend: Sync,
    {
        // ASSUMPTION: `Spectrogram` exposes `frames_mut_unchecked(&mut self)
        // -> impl IndexedParallelIterator<Item = &mut Vec<SP>>` (a rayon
        // par_iter_mut over per-frame spectra) and `IFftBackend` methods are
        // `Sync`/callable from multiple threads with per-call scratch. I
        // don't have spectogram.rs / fft_backend.rs to confirm these method
        // names — adjust to match the real trait if they differ.
        use rayon::prelude::*;

        let frame_count = self.frame_count(signal.len());
        let mut spectrogram = self.fft_backend.new_spectrogram(frame_count);

        spectrogram
            .frames_mut_unchecked()
            .enumerate()
            .for_each(|(frame_idx, spectrum)| {
                let start = frame_idx * self.hop_size;
                let mut frame = self.frame_unchecked(&signal[start..start + self.win_size]);
                let mut scratch = self.fft_backend.new_forward_scratch();
                self.fft_backend
                    .fft_unchecked(&mut frame, spectrum, &mut scratch);
            });

        spectrogram
    }

    #[cfg(feature = "parallel")]
    pub fn stft_parallel(&self, signal: &[T]) -> Result<Spectrogram<SP>, StftError>
    where
        T: Sync,
        SP: Send + Sync,
        FftBackend: Sync,
    {
        if signal.len() < self.win_size {
            return Err(StftError::InvalidFrameInputSize {
                expected: self.win_size,
                actual: signal.len(),
            });
        }
        Ok(self.stft_parallel_unchecked(signal))
    }

    pub fn stft_unchecked(&self, signal: &[T]) -> Spectrogram<SP> {
        self.stft_frames_unchecked(signal)
    }

    pub fn stft(&self, signal: &[T]) -> Result<Spectrogram<SP>, StftError> {
        if signal.len() < self.win_size {
            return Err(StftError::InvalidFrameInputSize {
                expected: self.win_size,
                actual: signal.len(),
            });
        }
        Ok(self.stft_frames_unchecked(signal))
    }

    /// Length of the reconstructed signal for a spectrogram with
    /// `frame_count` frames, given this STFT's window/hop size.
    const fn reconstructed_len(&self, frame_count: usize) -> usize {
        if frame_count == 0 {
            0
        } else {
            (frame_count - 1) * self.hop_size + self.win_size
        }
    }

    /// Windowed overlap-add inverse STFT.
    ///
    /// ASSUMPTION: `IFftBackend` provides `new_inverse_scratch(&self) ->
    /// <scratch type>` and `ifft_unchecked(&self, spectrum: &mut Vec<SP>,
    /// frame_out: &mut Vec<T>, scratch: &mut [SP])`, mirroring the
    /// forward `fft`/`fft_unchecked` shape. `Spectrogram` is assumed to
    /// expose `frame_count(&self) -> usize` and `frame_unchecked(&self,
    /// idx: usize) -> &Vec<SP>`. I don't have fft_backend.rs / spectogram.rs
    /// to confirm these names — please correct if they differ.
    fn istft_frames_unchecked(&self, spectrogram: &mut Spectrogram<SP>) -> Vec<T> {
        let frame_count = spectrogram.frame_count();
        let out_len = self.reconstructed_len(frame_count);

        let mut output = vec![T::zero(); out_len];
        let mut window_sum = vec![T::zero(); out_len];
        let mut scratch = self.fft_backend.new_inverse_scratch();

        for frame_idx in 0..frame_count {
            let start = frame_idx * self.hop_size;
            let mut spectrum = spectrogram.frame_mut_unchecked(frame_idx);
            let mut time_frame = vec![T::zero(); self.win_size];
            self.fft_backend
                .ifft_unchecked(&mut spectrum, &mut time_frame, &mut scratch);

            for i in 0..self.win_size {
                // Re-apply the analysis window on the way out (standard
                // weighted overlap-add) and accumulate the window-squared
                // sum so overlapping regions can be normalized afterwards.
                let w = self.window[i];
                output[start + i] = output[start + i] + time_frame[i] * w;
                window_sum[start + i] = window_sum[start + i] + w * w;
            }
        }

        for i in 0..out_len {
            if window_sum[i] > T::zero() {
                output[i] = output[i] / window_sum[i];
            }
        }

        output
    }

    #[cfg(feature = "parallel")]
    pub fn istft_parallel_unchecked(&self, spectrogram: &mut Spectrogram<SP>) -> Vec<T>
    where
        T: Send + Sync,
        SP: Sync,
        FftBackend: Sync,
    {
        // Overlap-add has a data dependency across frames that touch the
        // same output samples (any two frames within `win_size` of each
        // other), so this parallelizes the per-frame IFFT + windowing into
        // scratch buffers and does the (cheap, O(n)) accumulation
        // sequentially, rather than writing into `output` concurrently.
        use rayon::prelude::*;

        let frame_count = spectrogram.frame_count();
        let out_len = self.reconstructed_len(frame_count);

        let windowed_frames: Vec<Vec<T>> = (0..frame_count)
            .into_par_iter()
            .map(|frame_idx| {
                let mut time_frame = vec![T::zero(); self.win_size];
                let mut scratch = self.fft_backend.new_inverse_scratch();
                self.fft_backend.ifft_unchecked(
                    spectrogram.frame_mut_unchecked(frame_idx),
                    &mut time_frame,
                    &mut scratch,
                );
                for i in 0..self.win_size {
                    time_frame[i] = time_frame[i] * self.window[i];
                }
                time_frame
            })
            .collect();

        let mut output = vec![T::zero(); out_len];
        let mut window_sum = vec![T::zero(); out_len];
        for (frame_idx, time_frame) in windowed_frames.into_iter().enumerate() {
            let start = frame_idx * self.hop_size;
            for i in 0..self.win_size {
                output[start + i] = output[start + i] + time_frame[i];
                window_sum[start + i] = window_sum[start + i] + self.window[i] * self.window[i];
            }
        }
        for i in 0..out_len {
            if window_sum[i] > T::zero() {
                output[i] = output[i] / window_sum[i];
            }
        }

        output
    }

    #[cfg(feature = "parallel")]
    pub fn istft_parallel(&self, spectrogram: &mut Spectrogram<SP>) -> Result<Vec<T>, StftError>
    where
        T: Send + Sync,
        SP: Sync,
        FftBackend: Sync,
    {
        Ok(self.istft_parallel_unchecked(spectrogram))
    }

    pub fn istft_unchecked(&self, spectrogram: &mut Spectrogram<SP>) -> Vec<T> {
        self.istft_frames_unchecked(spectrogram)
    }

    pub fn istft(&self, spectrogram: &mut Spectrogram<SP>) -> Result<Vec<T>, StftError> {
        Ok(self.istft_frames_unchecked(spectrogram))
    }
}
