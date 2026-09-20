#[cfg(feature = "backend-phastft")]
pub mod phastft;
#[cfg(feature = "backend-realfft")]
pub mod realfft;
#[cfg(feature = "backend-rustfft")]
pub mod rustfft;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum FftError {
    #[error("size not equal, real: {real}, imag: {imag}")]
    SizeNotEqual { real: usize, imag: usize },
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

pub trait IFftBackend<SI, SP, SC, const N_FFT: usize> {
    fn signal_size(&self) -> usize;
    fn spectrum_size(&self) -> usize;
    fn forward_scratch_size(&self) -> usize;
    fn inverse_scratch_size(&self) -> usize;
    fn new_spectrum(&self) -> Vec<SP>;
    fn new_forward_scratch(&self) -> Vec<SC>;
    fn new_inverse_scratch(&self) -> Vec<SC>;
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
const fn check_size_at_least(
    name: &'static str,
    input: usize,
    expected: usize,
) -> Result<(), FftError> {
    if input < expected {
        return Err(FftError::SizeTooSmall {
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
#[cfg(test)]
#[cfg_attr(test, macro_export)]
macro_rules! test_fft_backend {
    ($test_name:ident, $backend_ty:ty, $signal_len:expr) => {
        $crate::test_fft_backend!(
            $test_name,
            $backend_ty,
            $signal_len,
            <$backend_ty as ::core::default::Default>::default()
        );
    };

    ($test_name:ident, $backend_ty:ty, $signal_len:expr, $backend_expr:expr) => {
        #[test]
        fn $test_name() {
            use rand::{rngs::StdRng, Rng, SeedableRng};

            // Fixed seed: keeps insta snapshots deterministic across runs.
            let mut rng = StdRng::seed_from_u64(0xF77_u64);

            let backend: $backend_ty = $backend_expr;

            let signal_size = backend.signal_size();
            let spectrum_size = backend.spectrum_size();

            // Allocate at `signal_size()`, but only randomize the first
            // `min(signal_len, signal_size)` samples.
            let fill_len = core::cmp::min($signal_len, signal_size);
            let mut signal: Vec<_> = (0..signal_size)
                .map(|_| ::core::default::Default::default())
                .collect();
            for sample in signal.iter_mut().take(fill_len) {
                *sample = rng.gen();
            }

            let mut spectrum = backend.new_spectrum();
            let mut forward_scratch = backend.new_forward_scratch();

            backend
                .fft(&mut signal, &mut spectrum, &mut forward_scratch)
                .expect(concat!(stringify!($test_name), ": fft() failed"));

            assert_eq!(
                spectrum.len(),
                spectrum_size,
                "{}: spectrum length mismatch",
                stringify!($test_name)
            );
            insta::assert_debug_snapshot!(
                concat!(stringify!($test_name), "__spectrum"),
                spectrum
            );

            let mut recovered: Vec<_> = (0..signal_size)
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
