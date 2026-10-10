extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use core::fmt::Debug;
use std::iter::Sum;

use num_traits::{Float, FloatConst};
use parking_lot::Mutex;

#[cfg(feature = "parallel")]
use crate::data_types::Spectrum2D;
use crate::data_types::{ISignal, SignalRef};
use crate::error::SpekError;
use crate::fft::IFftBackend;
use crate::windows::Window;

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
    T: Float,
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
    T: Float + FloatConst + Debug + Sum,
    FftBackend: IFftBackend<T>,
{
    pub fn new(
        hop_size: usize,
        win_size: usize,
        window: Window<T>,
        fft_backend: FftBackend,
    ) -> Result<Self, SpekError> {
        if hop_size == 0 {
            return Err(SpekError::InvalidSize {
                name: "hop_size",
                size: 0,
                reason: "must be greater than 0, got 0",
            });
        }
        if win_size == 0 {
            return Err(SpekError::InvalidSize {
                name: "win_size",
                size: 0,
                reason: "must be greater than 0, got 0",
            });
        }
        if fft_backend.fft_size() < 2 {
            return Err(SpekError::InvalidSize {
                name: "fft_size",
                size: fft_backend.fft_size(),
                reason: "must be greater than 1",
            });
        }
        if win_size > fft_backend.fft_size() {
            return Err(SpekError::Invalid2Size {
                name_a: "win_size",
                name_b: "fft_size",
                size_a: win_size,
                size_b: fft_backend.fft_size(),
                reason: "must be less than or equal to fft_size",
            });
        }
        if hop_size > win_size {
            return Err(SpekError::Invalid2Size {
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

    pub fn stft_frame(&self, input: SignalRef<'_, T>) -> (Vec<T>, Vec<T>) {
        let frame = input.frame(
            0,
            self.win_size,
            self.hop_size,
            self.fft_backend.fft_size(),
            &self.window,
        );

        let (mut real, mut imag) = self.fft_backend.new_spectrum();
        self.fft_backend.fft(frame.as_ref(), &mut real, &mut imag);
        (real, imag)
    }

    #[cfg(feature = "parallel")]
    pub fn par_stft(&self, signal: SignalRef<'_, T>) -> Spectrum2D<T>
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

        let frame_count = signal.frame_count(self.win_size, self.hop_size);
        let mut result = self.fft_backend.new_spectrum2d(frame_count);

        result
            .frames_par_iter_mut()
            .enumerate()
            .for_each(|(frame_idx, spectrum)| {
                let frame = signal.frame(
                    frame_idx,
                    self.win_size,
                    self.hop_size,
                    self.fft_backend.fft_size(),
                    &self.window,
                );
                self.fft_backend.fft(frame.as_ref(), spectrum.0, spectrum.1);
            });

        result
    }

    pub fn stft(&self, signal: SignalRef<'_, T>) -> Spectrum2D<T> {
        let frame_count = signal.frame_count(self.win_size, self.hop_size);
        let mut spectrogram = self.fft_backend.new_spectrum2d(frame_count);
        spectrogram
            .frames_iter_mut()
            .enumerate()
            .for_each(|(frame_idx, spectrum)| {
                let frame = signal.frame(
                    frame_idx,
                    self.win_size,
                    self.hop_size,
                    self.fft_backend.fft_size(),
                    &self.window,
                );
                self.fft_backend.fft(frame.as_ref(), spectrum.0, spectrum.1);
            });

        spectrogram
    }

    // Add this field to the struct and init as `Mutex::new(None)` in the
    // constructor. norm_cache: Mutex<Option<(usize, Arc<[T]>)>>,

    /// Reciprocal overlap-add normalization for a given frame_count.
    /// Pure function of (window, hop_size, win_size, frame_count) — cached
    /// so repeated calls with the same shape skip recomputation entirely.
    fn normalization(&self, signal_len: usize, frame_count: usize) -> alloc::sync::Arc<[T]> {
        if let Some((len, buf)) = self.norm_cache.lock().as_ref()
            && *len == frame_count
        {
            return buf.clone();
        }

        let mut window_sum = vec![T::zero(); signal_len];
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
        let out_len = spectrum.signal_len(self.hop_size, self.win_size);
        let fft_size = self.fft_backend.fft_size();
        let norm = self.normalization(out_len, frame_count);

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
                    time_frame.as_mut(),
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
    pub fn par_istft(&self, spectrum: &mut Spectrum2D<T>) -> Vec<T>
    where
        T: Send + Sync,
        FftBackend: Sync,
    {
        use rayon::prelude::*;

        let frame_count = spectrum.frame_count();
        let out_len = spectrum.signal_len(self.hop_size, self.win_size);
        let fft_size = self.fft_backend.fft_size();
        let norm = self.normalization(out_len, frame_count);

        // Flat scratch buffer for all frames' windowed IFFT output.
        let mut windowed = vec![T::zero(); frame_count * fft_size];

        spectrum
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
    #[cfg(test)]
    use std::iter::Sum;

    use float_cmp::ApproxEq;
    use phastft::planner::{PlannerR2c32, PlannerR2c64};
    use rstest::rstest;

    use super::*;
    use crate::data_types::Signal;
    use crate::fft::PhastftBackend;
    use crate::windows::Window;
    #[rstest]
    #[case("f32.hop4.win7.window_hann.49" ,4, 7, Window::Hann,  PhastftBackend::<PlannerR2c32>::new(8), 49)]
    #[case("f64.hop4.win7.window_hann.50" ,4, 7, Window::Hann,  PhastftBackend::<PlannerR2c64>::new(8), 50)]
    fn test_stft<T: Float + FloatConst, FftBackend: IFftBackend<T>>(
        #[case] name: &str,
        #[case] hop_size: usize,
        #[case] win_size: usize,
        #[case] window: Window<T>,
        #[case] fft_backend: FftBackend,
        #[case] signal_len: usize,
    ) -> mischief::Result<()>
    where
        T: Debug + Float + Display + ApproxEq + Sync + Send + FloatConst + Sum,
        FftBackend: Sync,
    {
        use generic_num::num;

        let stft = Stft::new(hop_size, win_size, window.clone(), fft_backend)?;
        let signal: Signal<T> = (0..signal_len)
            .map(|i| num!(i * i))
            .collect::<Vec<_>>()
            .into();

        let spectrum = stft.stft(signal.clone().as_ref());
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
            let spectrum_par = stft.par_stft(signal.clone().as_ref());
            spectrum_par
                .iter()
                .zip(spectrum.iter())
                .for_each(|((rep, imp), (re, im))| {
                    float_cmp::assert_approx_eq!(T, *rep, *re);
                    float_cmp::assert_approx_eq!(T, *imp, *im);
                });
        }
        {
            let frame = stft.stft_frame(signal.as_slice()[..win_size].into());
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
