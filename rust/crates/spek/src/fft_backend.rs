use crate::spectogram::{ISpectrogram, Spectrogram};

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

pub trait IFftBackend<SI, SP, const N: usize>
where
    Spectrogram<SP>: ISpectrogram<SP>,
{
    fn signal_size(&self) -> usize { N }
    fn spectrum_size(&self) -> usize;
    fn forward_scratch_size(&self) -> usize;
    fn inverse_scratch_size(&self) -> usize;
    fn new_spectrum(&self) -> Vec<SP>;
    fn new_forward_scratch(&self) -> Vec<SP>;
    fn new_inverse_scratch(&self) -> Vec<SP>;
    fn new_spectrogram(&self, frame_len: usize) -> Spectrogram<SP>;
    fn fft(
        &self,
        signal: &mut [SI],
        spectrum: &mut [SP],
        scratch: &mut [SP],
    ) -> Result<(), FftError>;
    fn ifft(
        &self,
        spectrum: &mut [SP],
        signal: &mut [SI],
        scratch: &mut [SP],
    ) -> Result<(), FftError>;
    fn fft_unchecked(&self, signal: &mut [SI], spectrum: &mut [SP], scratch: &mut [SP]);
    fn ifft_unchecked(&self, spectrum: &mut [SP], signal: &mut [SI], scratch: &mut [SP]);
}

const fn check_size(name: &'static str, input: usize, expected: usize) -> Result<(), FftError> {
    if input != expected {
        return Err(FftError::SizeNotCorrect {
            name,
            expected,
            actual: input,
        });
    }
    Ok(())
}

/// `test_fft_backend!` — generates a round-trip snapshot test for any
/// `IFftBackend` implementation.
///
/// Requires `rand` and `insta` as dev-dependencies.
///
/// # Usage
/// ```ignore
/// // 3-arg form: backend is constructed via `Default::default()`
/// test_fft_backend!(test_my_backend_len_100, MyFftBackend, 100);
///
/// // 4-arg form: supply your own constructor expression instead
/// test_fft_backend!(
///     test_my_backend_custom,
///     MyFftBackend,
///     100,
///     MyFftBackend::new(some_config)
/// );
/// ```
///
/// `signal_len` need not equal `backend.signal_size()`: the signal buffer
/// is always allocated at `signal_size()`, and only the first
/// `min(signal_len, signal_size)` samples are filled with random data —
/// the rest stay at their `Default` value (i.e. zero-padded). This lets
/// you test how a backend behaves with a shorter "real" signal padded out
/// to its required transform size.
///
/// Random data is seeded deterministically so `insta` snapshots stay
/// stable across runs. Each test writes two named snapshots:
/// `<test_name>__spectrum` and `<test_name>__recovered`.
///
/// Bounds implicitly required at the call site (via monomorphization):
/// - `$backend_ty: Default` (3-arg form only)
/// - `SI: Default + core::fmt::Debug`, and `rand::distributions::Standard:
///   rand::distributions::Distribution<SI>`
/// - `SP: Default + core::fmt::Debug`
/// - `SC: Default`
///
/// No assumption is made about the numeric relationship between
/// `spectrum_size()` and the length of `new_spectrum()` — some backends
/// interleave real/imag components into one flat buffer twice that long.
#[macro_export]
macro_rules! test_fft_backend {
    ($test_name:ident, $T:ty,$backend_ty:ty) => {
        $crate::test_fft_backend!(
            $test_name,
            $T,
            $backend_ty,
            <$backend_ty as ::core::default::Default>::default()
        );
    };

    ($test_name:ident,$T:ty, $backend_ty:ty, $backend_expr:expr) => {
        #[test]
        fn $test_name() {
            use rand::rngs::StdRng;
            use rand::{RngExt, SeedableRng};

            // Fixed seed: keeps insta snapshots deterministic across runs.
            let mut rng = StdRng::seed_from_u64(0xF77_u64);

            let backend: $backend_ty = $backend_expr;

            // Allocate at `signal_size()`, but only randomize the first
            // `min(signal_len, signal_size)` samples.
            let signal_size: usize = backend.signal_size();
            let mut signal: Vec<_> = (0..signal_size).map(|_| rng.random()).collect();
            insta::assert_debug_snapshot!(
                format!("signal_{}_{}", stringify!($T), signal_size),
                signal
            );

            let mut spectrum = backend.new_spectrum();
            let mut forward_scratch = backend.new_forward_scratch();

            backend
                .fft(&mut signal, &mut spectrum, &mut forward_scratch)
                .expect(concat!(stringify!($test_name), ": fft() failed"));

            // Note: `spectrum.len()` is not asserted against
            // `backend.spectrum_size()` here — that relationship is
            // backend-specific (e.g. a real/imag-interleaved flat buffer
            // is `spectrum_size() * 2` long), not guaranteed 1:1 by the
            // trait. `new_spectrum()` is trusted as the source of truth
            // for buffer size.
            insta::assert_debug_snapshot!(concat!(stringify!($test_name), "__spectrum"), spectrum);

            let mut recovered: Vec<_> = (0..backend.signal_size())
                .map(|_| ::core::default::Default::default())
                .collect();
            let mut inverse_scratch = backend.new_inverse_scratch();

            backend
                .ifft(&mut spectrum, &mut recovered, &mut inverse_scratch)
                .expect(concat!(stringify!($test_name), ": ifft() failed"));

            insta::assert_debug_snapshot!(
                concat!(stringify!($test_name), "__recovered"),
                recovered
            );
        }
    };
}
