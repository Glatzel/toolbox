extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use core::fmt::Debug;

use num_traits::{Float, FloatConst};
use parking_lot::Mutex;
use thiserror::Error;

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
/// signal   |#####################################################•••
///          |           fft_size            |
///          |<----------------------------->|
///          |       win_size        |       |
///          |<--------------------->|       |
/// frame 0  |#######################00000000|
///          |<-- hop_size -->|           fft_size            |
///          |                |<----------------------------->|
///          |                |       win_size        |       |
///          |                |<--------------------->|       |
/// frame 1  |                |#######################00000000|
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
    norm_cache: Mutex<Option<(usize, alloc::sync::Arc<[T]>)>>,
}

impl<T, FftBackend> Stft<T, FftBackend>
where
    T: Float + FloatConst + Debug,
    FftBackend: IFftBackend<T>,
{
    pub fn new<W>(
        hop_size: usize,
        win_size: usize,
        window: W,
        fft_backend: FftBackend,
    ) -> Result<Self, StftError>
    where
        W: IWindow<T>,
    {
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
            norm_cache: Mutex::new(None),
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

    pub fn stft_frame(&self, input: &[T]) -> (Vec<T>, Vec<T>) {
        let frame = self.frame(input);
        let (mut real, mut imag) = self.fft_backend.new_spectrum();
        self.fft_backend.fft(&frame, &mut real, &mut imag);
        (real, imag)
    }

    #[cfg(feature = "parallel")]
    pub fn par_stft(&self, signal: &[T]) -> Spectrum2D<T>
    where
        T: Sync + Send,
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
        let mut result = self.fft_backend.new_spectrum2d(frame_count);

        result
            .frames_par_iter_mut()
            .enumerate()
            .for_each(|(frame_idx, spectrum)| {
                let start = frame_idx * self.hop_size;
                let frame = self.frame(&signal[start..start + self.win_size]);
                self.fft_backend.fft(&frame, spectrum.0, spectrum.1);
            });

        result
    }

    pub fn stft(&self, signal: &[T]) -> Spectrum2D<T> {
        let frame_count = self.frame_count(signal.len());
        let mut spectrogram = self.fft_backend.new_spectrum2d(frame_count);
        spectrogram
            .frames_iter_mut()
            .enumerate()
            .for_each(|(frame_idx, spectrum)| {
                let start = frame_idx * self.hop_size;
                let frame = self.frame(&signal[start..start + self.win_size]);
                self.fft_backend.fft(&frame, spectrum.0, spectrum.1);
            });

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

    // Add this field to the struct and init as `Mutex::new(None)` in the
    // constructor. norm_cache: Mutex<Option<(usize, Arc<[T]>)>>,

    /// Reciprocal overlap-add normalization for a given frame_count.
    /// Pure function of (window, hop_size, win_size, frame_count) — cached
    /// so repeated calls with the same shape skip recomputation entirely.
    fn normalization(&self, frame_count: usize) -> alloc::sync::Arc<[T]> {
        if let Some((len, buf)) = self.norm_cache.lock().as_ref()
            && *len == frame_count
        {
            return buf.clone();
        }

        let out_len = self.reconstructed_len(frame_count);
        let mut window_sum = vec![T::zero(); out_len];
        for frame_idx in 0..frame_count {
            let start = frame_idx * self.hop_size;
            for (ws, w) in window_sum[start..start + self.win_size]
                .iter_mut()
                .zip(self.window.iter())
            {
                *ws = *ws + *w * *w;
            }
        }
        // Reciprocal, computed once here rather than divided-and-branched
        // per element on every call. Where window_sum is 0, no frame ever
        // touched that sample, so output is already 0 there — multiplying
        // by 0 is a correct, branchless no-op.
        for ws in &mut window_sum {
            *ws = if *ws > T::zero() {
                T::one() / *ws
            } else {
                T::zero()
            };
        }

        let buf: alloc::sync::Arc<[T]> = alloc::sync::Arc::from(window_sum);
        *self.norm_cache.lock() = Some((frame_count, buf.clone()));
        buf
    }

    pub fn istft(&self, spectrum: &mut Spectrum2D<T>) -> Vec<T> {
        let frame_count = spectrum.frame_count();
        let out_len = self.reconstructed_len(frame_count);
        let fft_size = self.fft_backend.fft_size();
        let norm = self.normalization(frame_count);

        let mut output = vec![T::zero(); out_len];
        let (mut scratch_real, mut scratch_imag) = self.fft_backend.new_scratch();
        let mut time_frame = vec![T::zero(); fft_size]; // reused, not reallocated per frame

        spectrum
            .frames_iter_mut()
            .enumerate()
            .for_each(|(frame_idx, spectrum)| {
                let start = frame_idx * self.hop_size;
                self.fft_backend.ifft(
                    spectrum.0,
                    spectrum.1,
                    &mut time_frame,
                    &mut scratch_real,
                    &mut scratch_imag,
                );

                for ((o, w), t) in output[start..start + self.win_size]
                    .iter_mut()
                    .zip(self.window.iter())
                    .zip(time_frame.iter())
                {
                    *o = *o + *t * *w;
                }
            });

        for (o, n) in output.iter_mut().zip(norm.iter()) {
            *o = *o * *n;
        }
        output
    }

    #[cfg(feature = "parallel")]
    pub fn par_istft(&self, spectrogram: &mut Spectrum2D<T>) -> Vec<T>
    where
        T: Send + Sync,
        FftBackend: Sync,
    {
        use rayon::prelude::*;

        let frame_count = spectrogram.frame_count();
        let out_len = self.reconstructed_len(frame_count);
        let fft_size = self.fft_backend.fft_size();
        let norm = self.normalization(frame_count);

        // Flat scratch buffer for all frames' windowed IFFT output.
        let mut windowed = vec![T::zero(); frame_count * fft_size];

        spectrogram
            .frames_par_iter_mut()
            .zip(windowed.par_chunks_mut(fft_size))
            .for_each_init(
                || self.fft_backend.new_scratch(), // once per worker thread
                |(scratch_real, scratch_imag), (spectrum, time_frame)| {
                    self.fft_backend.ifft(
                        spectrum.0,
                        spectrum.1,
                        time_frame,
                        scratch_real,
                        scratch_imag,
                    );
                    for (t, w) in time_frame
                        .iter_mut()
                        .zip(self.window.iter())
                        .take(self.win_size)
                    {
                        *t = *t * *w;
                    }
                },
            );

        // Data-dependent overlap-add stays sequential (unavoidable — adjacent
        // frames write the same output samples), but it's now allocation-free.
        let mut output = vec![T::zero(); out_len];
        for (frame_idx, time_frame) in windowed.chunks(fft_size).enumerate() {
            let start = frame_idx * self.hop_size;
            for (o, t) in output[start..start + self.win_size]
                .iter_mut()
                .zip(time_frame[..self.win_size].iter())
            {
                *o = *o + *t;
            }
        }

        for (o, n) in output.iter_mut().zip(norm.iter()) {
            *o = *o * *n;
        }
        output
    }
}
#[cfg(test)]
mod tests {
    use core::fmt::{Debug, Display};

    use float_cmp::ApproxEq;
    use phastft::planner::{PlannerR2c32, PlannerR2c64};
    use rstest::rstest;

    use super::*;
    use crate::fft_backend::phastft::PhastftBackend;
    use crate::windows::Window;
    #[rstest]
    #[case("f32.hop4.win7.window_hann.backend_phastft.49" ,4, 7, Window::Hann,  PhastftBackend::<PlannerR2c32>::new(8), 49)]
    #[case("f64.hop4.win7.window_hann.backend_phastft.50" ,4, 7, Window::Hann,  PhastftBackend::<PlannerR2c64>::new(8), 50)]

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
    {
        use generic_num::num;

        let stft = Stft::new(hop_size, win_size, window, fft_backend)?;
        let signal: Vec<T> = (0..signal_len).map(|i| num!(i * i)).collect();

        let spectrum = stft.stft(&mut signal.clone());
        insta::assert_debug_snapshot!(
            format!("{name}.spectogram"),
            spectrum
                .real()
                .iter()
                .zip(spectrum.imag())
                .map(|(r, i)| (format!("{r:.6}"), format!("{i:.6}")))
                .collect::<Vec<_>>()
        );

        {
            let frame = stft.stft_frame(&signal[0..win_size]);
            frame
                .0
                .iter()
                .zip(frame.1.iter())
                .enumerate()
                .for_each(|(i, (re, im))| {
                    float_cmp::assert_approx_eq!(T, *re, spectrum.real()[i]);
                    float_cmp::assert_approx_eq!(T, *im, spectrum.imag()[i]);
                });
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
