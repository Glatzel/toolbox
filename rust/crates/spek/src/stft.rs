extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use core::fmt::Debug;

use num_traits::{Float, FloatConst};
use thiserror::Error;

use crate::Dtype;
use crate::fft_backend::IFftBackend;
use crate::pad::PadError;
#[cfg(feature = "parallel")]
use crate::spectrum::Spectrum2D;
use crate::windows::{IWindow, WindowError};

#[derive(Error, Debug)]
pub enum StftError {
    #[error(transparent)]
    Pad(#[from] PadError),
    #[error(transparent)]
    Window(#[from] WindowError),

    #[error("{name} size not correct, got {size} ({reason})")]
    InvalidSize {
        name: &'static str,
        size: usize,
        reason: &'static str,
    },
    #[error("size not correct, {name_a} got {size_a} and {name_b} got {size_b} ({reason})")]
    Invalid2Size {
        name_a: &'static str,
        name_b: &'static str,
        size_a: usize,
        size_b: usize,
        reason: &'static str,
    },
}
/// # STFT Parameters
///
/// ```text
/// signal    #####################################################•••
///                  win_size
///          |<--------------------->|
///                      fft_size
///          |<----------------------------->|
/// frame 0   ########################00000000
///          |
///          |<-- hop_size -->|
///                           |
/// frame 1                   ########################00000000
/// ```
pub struct Stft<T, FftBackend>
where
    T: Float + FloatConst,
    FftBackend: IFftBackend<T>,
{
    hop_size: usize,
    win_size: usize,
    window: Vec<T>,
    fft_backend: FftBackend,
}

impl<T, FftBackend> Stft<T, FftBackend>
where
    T: Float + FloatConst + Debug,
    FftBackend: IFftBackend<T>,
{
    pub fn new<W: IWindow<T>>(
        hop_size: usize,
        win_size: usize,
        window: W,
        fft_backend: FftBackend,
    ) -> Result<Self, StftError> {
        if hop_size == 0 {
            return Err(StftError::InvalidSize {
                name: "hop_size",
                size: 0,
                reason: "must be greater than 0, got 0",
            });
        }
        if win_size == 0 {
            return Err(StftError::InvalidSize {
                name: "win_size",
                size: 0,
                reason: "must be greater than 0, got 0",
            });
        }
        if fft_backend.fft_size() < 2 {
            return Err(StftError::InvalidSize {
                name: "fft_size",
                size: fft_backend.fft_size(),
                reason: "must be greater than 1",
            });
        }
        if win_size > fft_backend.fft_size() {
            return Err(StftError::Invalid2Size {
                name_a: "win_size",
                name_b: "fft_size",
                size_a: win_size,
                size_b: fft_backend.fft_size(),
                reason: "must be less than or equal to fft_size",
            });
        }
        if hop_size > win_size {
            return Err(StftError::Invalid2Size {
                name_a: "hop_size",
                name_b: "win_size",
                size_a: hop_size,
                size_b: win_size,
                reason: "must be less than or equal to win_size",
            });
        }

        window.window(win_size, false)?;
        Ok(Self {
            hop_size,
            win_size,
            window: window.window(win_size, false)?,
            fft_backend,
        })
    }

    const fn frame_count(&self, signal_len: usize) -> usize {
        if signal_len < self.win_size {
            1
        } else {
            ((signal_len - self.win_size) / self.hop_size) + 1
        }
    }

    fn frame(&self, input: &[T]) -> Vec<T> {
        let mut frame: Vec<T> = input
            .iter()
            .zip(self.window.iter())
            .map(|(i, w)| *i * *w)
            .collect();
        frame.resize(self.fft_backend.fft_size(), T::zero());
        frame
    }

    pub fn stft_frame(&self, input: &[T], scratch: &mut [Dtype<T>]) -> Vec<Dtype<T>> {
        let mut frame = self.frame(input);
        let mut spectrum = self.fft_backend.new_spectrum();
        self.fft_backend.fft(&mut frame, &mut spectrum, scratch);
        spectrum
    }

    #[cfg(feature = "parallel")]
    pub fn par_stft(&self, signal: &[T]) -> Spectrum2D<T>
    where
        T: Sync + Send,
        FftBackend: Sync,
        Dtype<T>: Send,
    {
        // ASSUMPTION: `Spectrogram` exposes `frames_mut_unchecked(&mut self)
        // -> impl IndexedParallelIterator<Item = &mut Vec<SP>>` (a rayon
        // par_iter_mut over per-frame spectra) and `IFftBackend` methods are
        // `Sync`/callable from multiple threads with per-call scratch. I
        // don't have spectogram.rs / fft_backend.rs to confirm these method
        // names — adjust to match the real trait if they differ.
        use rayon::prelude::*;

        let frame_count = self.frame_count(signal.len());
        let mut result = self.fft_backend.new_spectrum2d(frame_count);

        result
            .par_iter_frame_mut()
            .enumerate()
            .for_each(|(frame_idx, spectrum)| {
                let start = frame_idx * self.hop_size;
                let mut frame = self.frame(&signal[start..start + self.win_size]);
                let mut scratch = self.fft_backend.new_forward_scratch();
                self.fft_backend.fft(&mut frame, spectrum, &mut scratch);
            });

        result
    }

    pub fn stft(&self, signal: &[T]) -> Spectrum2D<T> {
        let frame_count = self.frame_count(signal.len());
        let mut spectrogram = self.fft_backend.new_spectrum2d(frame_count);
        let mut scratch = self.fft_backend.new_forward_scratch();

        for frame_idx in 0..frame_count {
            let start = frame_idx * self.hop_size;
            let mut frame = self.frame(&signal[start..start + self.win_size]);
            let spectrum = spectrogram.frame_mut(frame_idx);
            self.fft_backend.fft(&mut frame, spectrum, &mut scratch);
        }

        spectrogram
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

    #[cfg(feature = "parallel")]
    pub fn par_istft(&self, spectrogram: &mut Spectrum2D<T>) -> Vec<T>
    where
        T: Send + Sync,
        FftBackend: Sync,
    {
        // Overlap-add has a data dependency across frames that touch the
        // same output samples (any two frames within `win_size` of each
        // other), so this parallelizes the per-frame IFFT + windowing into
        // scratch buffers and does the (cheap, O(n)) accumulation
        // sequentially, rather than writing into `output` concurrently.
        //
        // `frames_mut_unchecked` (via `par_chunks_mut`) hands each rayon
        // task its own disjoint `&mut [SP]`, split up front from the
        // underlying buffer. That's what lets the closure below be `Fn`
        // (each call gets a distinct argument, not a re-borrow of a shared
        // `&mut Spectrogram`) and is what makes this actually sound —
        // calling `spectrogram.frame_mut_unchecked(idx)` *inside* the
        // closure instead would need a fresh exclusive borrow of the same
        // `spectrogram` per call, which the compiler can't treat as `Fn`
        // and can't verify is non-aliasing across threads.
        use rayon::prelude::*;

        let frame_count = spectrogram.frame_count();
        let out_len = self.reconstructed_len(frame_count);
        let fft_size = self.fft_backend.fft_size();

        let windowed_frames: Vec<Vec<T>> = spectrogram
            .par_iter_frame_mut()
            .map(|spectrum| {
                let mut time_frame = vec![T::zero(); fft_size];
                let mut scratch = self.fft_backend.new_inverse_scratch();
                self.fft_backend
                    .ifft(spectrum, &mut time_frame, &mut scratch);
                for (i, f) in time_frame.iter_mut().enumerate().take(self.win_size) {
                    *f = (*f as T) * self.window[i];
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

    pub fn istft(&self, spectrum: &mut Spectrum2D<T>) -> Vec<T> {
        let frame_count = spectrum.frame_count();
        let out_len = self.reconstructed_len(frame_count);

        let mut output = vec![T::zero(); out_len];
        let mut window_sum = vec![T::zero(); out_len];
        let mut scratch = self.fft_backend.new_inverse_scratch();
        let fft_size = self.fft_backend.fft_size();

        for frame_idx in 0..frame_count {
            let start = frame_idx * self.hop_size;
            let spectrum = spectrum.frame_mut(frame_idx);
            let mut time_frame = vec![T::zero(); fft_size];
            self.fft_backend
                .ifft(spectrum, &mut time_frame, &mut scratch);

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
}
#[cfg(test)]
mod tests {
    use core::fmt::{Debug, Display};

    use float_cmp::ApproxEq;
    #[cfg(feature = "backend-phastft")]
    use phastft::planner::{PlannerR2c32, PlannerR2c64};
    use rstest::rstest;

    use super::*;
    use crate::conversion::spectrum_to_magnitude;
    #[cfg(feature = "backend-phastft")]
    use crate::fft_backend::phastft::PhastftBackend;
    #[cfg(feature = "backend-realfft")]
    use crate::fft_backend::realfft::RealfftBackend;
    use crate::windows::Window;
    #[rstest]
    #[cfg_attr(feature = "split",case("f32.hop4.win7.window_hann.backend_phastft.49" ,4, 7, Window::Hann,  PhastftBackend::<PlannerR2c32>::new(8), 49))]
    #[cfg_attr(feature = "split",case("f64.hop4.win7.window_hann.backend_phastft.50" ,4, 7, Window::Hann,  PhastftBackend::<PlannerR2c64>::new(8), 50))]
    #[cfg_attr(feature = "complex",case("f32.hop4.win7.window_hann.backend_realfft.49" ,4, 7, Window::Hann,  RealfftBackend::<f32>::new(8), 49))]
    fn test_stft<T: Float + FloatConst, FftBackend: IFftBackend<T>>(
        #[case] name: &str,
        #[case] hop_size: usize,
        #[case] win_size: usize,
        #[case] window: Window<T>,
        #[case] fft_backend: FftBackend,
        #[case] signal_len: usize,
    ) -> mischief::Result<()>
    where
        T: Debug + Float + Display + ApproxEq + Sync + Send,
        FftBackend: Sync,
        Dtype<T>: Send,
    {
        use generic_num::num;

        let stft = Stft::new(hop_size, win_size, window, fft_backend)?;
        let signal: Vec<T> = (0..signal_len).map(|i| num!(i * i)).collect();

        let spectrum = stft.stft(&mut signal.clone());
        insta::assert_debug_snapshot!(
            format!("{name}.spectogram"),
            spectrum
                .data()
                .iter()
                .map(|i| format!("{i:.6}"))
                .collect::<Vec<_>>()
        );

        {
            let spectrum_parallel = stft.par_stft(&mut signal.clone());
            spectrum_parallel
                .magnitude()
                .data()
                .iter()
                .zip(spectrum.magnitude().data().iter())
                .for_each(|(p, s)| {
                    float_cmp::assert_approx_eq!(T, *p, *s);
                });
        }

        {
            let magnitude = spectrum.magnitude();

            let frame = stft.stft_frame(
                &signal[0..win_size],
                &mut Vec::with_capacity(spectrum.bin_count()),
            );
            let mut frame_result = Spectrum2D::new(1, spectrum.bin_count());
            frame_result
                .frame_mut(0)
                .iter_mut()
                .enumerate()
                .for_each(|(i, v)| {
                    *v = frame[i];
                });
            let frame_magnitude = frame_result.magnitude();

            (0..spectrum.bin_count()).for_each(|i| {
                #[cfg(feature = "split")]
                let v = spectrum_to_magnitude(
                    spectrum.data()[i],
                    spectrum.data()[spectrum.bin_count() + i],
                );
                #[cfg(feature = "complex")]
                let v = spectrum_to_magnitude(spectrum.data()[i].re, spectrum.data()[i].im);
                float_cmp::assert_approx_eq!(T, v, magnitude.data()[i]);
                float_cmp::assert_approx_eq!(T, v, frame_magnitude.data()[i]);
            });
            {
                let par_magnitude = spectrum.par_magnitude();
                magnitude
                    .data()
                    .iter()
                    .zip(par_magnitude.data().iter())
                    .for_each(|(m, p)| {
                        float_cmp::assert_approx_eq!(T, *m, *p);
                    });
            }
            {
                let amplitude = spectrum.amplitude(num!(2.0));
                magnitude
                    .data()
                    .iter()
                    .zip(amplitude.data().iter())
                    .for_each(|(m, a)| {
                        float_cmp::assert_approx_eq!(T, *m * num!(2.0), *a);
                    });
            }
            {
                let par_amplitude = spectrum.par_amplitude(num!(2.0));
                magnitude
                    .data()
                    .iter()
                    .zip(par_amplitude.data().iter())
                    .for_each(|(m, a)| {
                        float_cmp::assert_approx_eq!(T, *m * num!(2.0), *a);
                    });
            }
        }
        {
            let amplitude = spectrum.amplitude(num!(1.0));
            {
                let db = spectrum.db(num!(2.0));
                amplitude
                    .data()
                    .iter()
                    .zip(db.data().iter())
                    .for_each(|(m, d)| {
                        use crate::conversion::amplitude_to_db;
                        float_cmp::assert_approx_eq!(T, amplitude_to_db(*m, num!(2.0)), *d);
                    });
            }
            {
                let par_db = spectrum.par_db(num!(2.0));
                amplitude
                    .data()
                    .iter()
                    .zip(par_db.data().iter())
                    .for_each(|(m, d)| {
                        use crate::conversion::amplitude_to_db;
                        float_cmp::assert_approx_eq!(T, amplitude_to_db(*m, num!(2.0)), *d);
                    });
            }
        }

        {
            let recovered = stft.istft(&mut spectrum.clone());
            let par_recovered = stft.par_istft(&mut spectrum.clone());
            let epsilon = num!(0.001);
            recovered.iter().zip(signal.iter()).for_each(|(r, o)| {
                assert!((*r - *o).abs() <= epsilon);
            });
            recovered
                .iter()
                .zip(par_recovered.iter())
                .for_each(|(r, p)| {
                    assert!((*r - *p).abs() <= epsilon);
                });
        }
        Ok(())
    }
}
