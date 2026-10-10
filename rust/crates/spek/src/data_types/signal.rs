use num_traits::Float;

pub type Signal<T> = Vec<T>;
pub type SignalRef<'a, T> = &'a [T];
pub type SignalRefMut<'a, T> = &'a mut [T];

pub trait ISignal<T> {
    fn frame_count(&self, win_size: usize, hop_size: usize) -> usize
    where
        Self: AsRef<[T]>,
    {
        let len = self.as_ref().len();
        if len < win_size {
            1
        } else {
            (len - win_size) / hop_size + 1
        }
    }

    /// Allocation-free variant: writes the windowed, zero-padded, centered
    /// frame into `out` (which must have length `fft_size`).
    fn frame_into(
        &self,
        out: SignalRefMut<'_, T>,
        frame_idx: usize,
        win_size: usize,
        hop_size: usize,
        window: &[T],
    ) where
        Self: AsRef<[T]>,
        T: Float,
    {
        let fft_size = out.len();
        debug_assert!(win_size > 0, "win_size must be greater than zero");
        debug_assert!(hop_size > 0, "hop_size must be greater than zero");
        debug_assert!(fft_size >= win_size, "fft_size must be >= win_size");
        debug_assert!(
            window.len() >= win_size,
            "window length must be >= win_size"
        );
        debug_assert!(
            frame_idx < self.frame_count(win_size, hop_size),
            "frame_idx out of bounds"
        );

        let signal = self.as_ref();
        let start = frame_idx * hop_size;
        let available = signal.len().saturating_sub(start).min(win_size);
        let frame_start = (fft_size - available) / 2;
        let frame_end = frame_start + available;

        // Zero only the padding; the middle is fully overwritten below.
        out[..frame_start].fill(T::zero());
        out[frame_end..].fill(T::zero());

        // Equal-length slices let the compiler drop bounds checks and
        // vectorize.
        let src = &signal[start..start + available];
        let win = &window[..available];
        let dst = &mut out[frame_start..frame_end];

        for ((d, &s), &w) in dst.iter_mut().zip(src).zip(win) {
            *d = s * w;
        }
    }

    /// Convenience wrapper that allocates. Prefer `frame_into` in hot loops.
    fn frame(
        &self,
        frame_idx: usize,
        win_size: usize,
        hop_size: usize,
        fft_size: usize,
        window: &[T],
    ) -> Signal<T>
    where
        Self: AsRef<[T]>,
        T: Float,
    {
        // Padding is zeroed by frame_into, so uninitialized-free `vec!` is
        // only needed for correctness of the type; the cost is one memset.
        let mut out = vec![T::zero(); fft_size];
        self.frame_into(&mut out, frame_idx, win_size, hop_size, window);
        out
    }
}

impl<T> ISignal<T> for Vec<T> {}
impl<T> ISignal<T> for [T] {}
