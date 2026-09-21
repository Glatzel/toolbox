use crate::stft::{IStftResult, StftResult};

#[cfg(feature = "backend-phastft")]
pub mod phastft;
#[cfg(feature = "backend-realfft")]
pub mod realfft;
#[cfg(feature = "backend-rustfft")]
pub mod rustfft;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum FftError {
    #[error("{name} size not correct, expected {expected}, got {actual}")]
    SizeNotCorrect {
        name: &'static str,
        expected: usize,
        actual: usize,
    },
    #[error("{name} size not equal, {a} != {b}")]
    SizeNotEqual {
        name: &'static str,
        a: usize,
        b: usize,
    },
    #[error("{name} size not correct, expected at least {expected}, got {actual}")]
    SizeTooSmall {
        name: &'static str,
        expected: usize,
        actual: usize,
    },
    #[error("size not power of two, got {size}")]
    SizeNotPowerOfTwo { size: usize },
    #[error("fft_direction error. expected {expected:?}, got {actual:?}")]
    FftDirection {
        expected: FftDirection,
        actual: FftDirection,
    },
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FftDirection {
    Forward,
    Inverse,
}

pub trait IFftBackend<SI, SP>
where
    StftResult<SP>: IStftResult<SP, SI>,
{
    fn fft_size(&self) -> usize;
    fn signal_size(&self) -> usize;
    fn spectrum_size(&self) -> usize;
    fn forward_scratch_size(&self) -> usize;
    fn inverse_scratch_size(&self) -> usize;
    fn new_signal(&self) -> Vec<SI>;
    fn new_spectrum(&self) -> Vec<SP>;
    fn new_forward_scratch(&self) -> Vec<SP>;
    fn new_inverse_scratch(&self) -> Vec<SP>;
    fn new_stft_result_buffer(&self, frame_count: usize) -> StftResult<SP>;
    fn fft(&self, signal: &mut [SI], spectrum: &mut [SP], scratch: &mut [SP]);
    fn ifft(&self, spectrum: &mut [SP], signal: &mut [SI], scratch: &mut [SP]);
}

#[cfg(test)]
#[macro_export]
macro_rules! test_fft_backend {
    // --- Default backend, no attributes (unchanged) ---
    ($test_name:ident, $T:ty, $backend_ty:ty) => {
        $crate::test_fft_backend!(
            $test_name,
            $T,
            $backend_ty,
            <$backend_ty as ::core::default::Default>::default()
        );
    };

    // --- Default backend, with attributes e.g. #[should_panic] ---
    ($(#[$attr:meta])+ $test_name:ident, $T:ty, $backend_ty:ty) => {
        $crate::test_fft_backend!(
            $(#[$attr])+
            $test_name,
            $T,
            $backend_ty,
            <$backend_ty as ::core::default::Default>::default()
        );
    };

    // --- Explicit backend expr, no attributes (unchanged) ---
    ($test_name:ident, $T:ty, $backend_ty:ty, $backend_expr:expr) => {
        #[test]
        fn $test_name() -> mischief::Result<()> {
            $crate::__test_fft_backend_body!($test_name, $T, $backend_ty, $backend_expr)
        }
    };

    // --- Explicit backend expr, with attributes ---
    ($(#[$attr:meta])+ $test_name:ident, $T:ty, $backend_ty:ty, $backend_expr:expr) => {
        #[test]
        $(#[$attr])+
        fn $test_name() {
            (|| -> mischief::Result<()> {
                $crate::__test_fft_backend_body!($test_name, $T, $backend_ty, $backend_expr)
            })()
            .unwrap();
        }
    };
}

// Shared body, factored out so it isn't duplicated across the two
// implementation arms above. Not part of the public API.
#[cfg(test)]
#[doc(hidden)]
#[macro_export]
macro_rules! __test_fft_backend_body {
    ($test_name:ident, $T:ty, $backend_ty:ty, $backend_expr:expr) => {{
        let backend: $backend_ty = $backend_expr;

        // Allocate at `signal_size()`, but only randomize the first
        // `min(signal_len, signal_size)` samples.
        let signal_size: usize = backend.signal_size();
        let mut signal: Vec<_> = (0..signal_size).map(|i| (i * i) as $T).collect();

        let mut spectrum = backend.new_spectrum();
        let mut forward_scratch = backend.new_forward_scratch();

        backend.fft(&mut signal, &mut spectrum, &mut forward_scratch);

        // Note: `spectrum.len()` is not asserted against
        // `backend.spectrum_size()` here — that relationship is
        // backend-specific (e.g. a real/imag-interleaved flat buffer
        // is `spectrum_size() * 2` long), not guaranteed 1:1 by the
        // trait. `new_spectrum()` is trusted as the source of truth
        // for buffer size.
        insta::assert_debug_snapshot!(
            concat!(stringify!($test_name), "__spectrum"),
            spectrum
                .iter()
                .map(|i| format!("{i:.5}"))
                .collect::<Vec<_>>()
        );

        let mut recovered: Vec<_> = (0..backend.signal_size())
            .map(|_| ::core::default::Default::default())
            .collect();
        let mut inverse_scratch = backend.new_inverse_scratch();

        backend.ifft(&mut spectrum, &mut recovered, &mut inverse_scratch);

        signal.iter().zip(recovered.iter()).for_each(|(s, r)| {
            float_cmp::assert_approx_eq!($T, *r, *s, epsilon = 0.0001);
        });
        Ok(())
    }};
}
