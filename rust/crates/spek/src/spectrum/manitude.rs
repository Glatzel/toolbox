use generic_num::num;
use num_traits::Float;
pub fn phase<T>(real: T, imag: T, magnitude: T) -> (T, T)
where
    T: Float,
{
    let phase_real = real / magnitude;
    let phase_imag = imag / magnitude;
    (phase_real, phase_imag)
}
/// Converts a complex spectrum bin to power: `re² + im²`.
///
/// This is the squared magnitude `|z|²`.
pub fn spectrum_to_power<T>(real: T, imag: T) -> T
where
    T: Float,
{
    real * real + imag * imag
}

/// Converts a complex spectrum bin to amplitude (magnitude): `sqrt(re² + im²)`.
///
/// Uses [`Float::hypot`], which avoids intermediate overflow/underflow.
pub fn spectrum_to_amplitude<T>(real: T, imag: T) -> T
where
    T: Float,
{
    real.hypot(imag)
}

/// Converts a complex spectrum bin directly to decibels (power scale).
///
/// Equivalent to [`spectrum_to_power`] followed by [`power_to_db`].
///
/// See [`power_to_db`] for the meaning of `reference`, `amin` and `top_db`.
pub fn spectrum_to_db<T>(real: T, imag: T, reference: T, amin: T, top_db: T) -> T
where
    T: Float,
{
    let power = spectrum_to_power(real, imag);
    power_to_db(power, reference, amin, top_db)
}

/// Squares a magnitude value: `magnitude²`.
///
/// Note that despite the name, the result is a *power*-scale quantity
/// (`|z|²`), not an amplitude.
pub fn magnitude_to_amplitude<T>(magnitude: T) -> T
where
    T: Float,
{
    magnitude.powi(2)
}

/// Converts an amplitude to decibels: `20 * log10(max(amplitude, amin) /
/// reference) - top_db`.
///
/// # Parameters
/// - `amplitude`: input amplitude (magnitude).
/// - `reference`: amplitude that maps to 0 dB (librosa's `ref`, usually `1.0`).
/// - `amin`: floor applied to `amplitude` before the logarithm, so that zero
///   input yields a finite value instead of `-inf`.
/// - `top_db`: constant offset **subtracted** from the result. Pass `0` for a
///   plain dB conversion. Unlike librosa, this is *not* a relative
///   dynamic-range clip, since a scalar function has no access to the
///   spectrogram maximum.
pub fn amplitude_to_db<T>(amplitude: T, reference: T, amin: T, top_db: T) -> T
where
    T: Float,
{
    let amplitude = amplitude.max(amin);
    num!(20.0) * (amplitude / reference).log10() - top_db
}

/// Converts decibels back to amplitude: `reference * 10^(db / 20)`.
///
/// Inverse of [`amplitude_to_db`] (with `top_db = 0` and no `amin` clamping).
pub fn db_to_amplitude<T>(db: T, reference: T) -> T
where
    T: Float,
{
    reference * num!(10.0).powf(db / num!(20))
}

/// Converts a power value to decibels: `10 * log10(max(power, amin) /
/// reference) - top_db`.
///
/// # Parameters
/// - `power`: input power.
/// - `reference`: power that maps to 0 dB (librosa's `ref`, usually `1.0`).
/// - `amin`: floor applied to `power` before the logarithm, so that zero input
///   yields a finite value instead of `-inf`.
/// - `top_db`: constant offset **subtracted** from the result. Pass `0` for a
///   plain dB conversion. Unlike librosa, this is *not* a relative
///   dynamic-range clip.
pub fn power_to_db<T>(power: T, reference: T, amin: T, top_db: T) -> T
where
    T: Float,
{
    let power = power.max(amin);
    num!(10.0) * (power / reference).log10() - top_db
}

/// Converts decibels back to power: `reference * 10^(db / 10)`.
///
/// Inverse of [`power_to_db`] (with `top_db = 0` and no `amin` clamping).
pub fn db_to_power<T>(db: T, reference: T) -> T
where
    T: Float,
{
    reference * num!(10.0).powf(db / num!(10))
}
pub fn pcen() { todo!() }
pub fn mu_compress() { todo!() }
pub fn mu_expand() { todo!() }
#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use super::*;

    const AMIN: f64 = 1e-10;

    #[test]
    fn spectrum_to_power_is_squared_magnitude() {
        assert_approx_eq!(f64, spectrum_to_power(3.0, 4.0), 25.0);
        assert_approx_eq!(f64, spectrum_to_power(-3.0, 4.0), 25.0);
        assert_approx_eq!(f64, spectrum_to_power(0.0, 0.0), 0.0);
    }

    #[test]
    fn spectrum_to_amplitude_is_magnitude() {
        assert_approx_eq!(f64, spectrum_to_amplitude(3.0, 4.0), 5.0);
        assert_approx_eq!(f64, spectrum_to_amplitude(-3.0, -4.0), 5.0);
        assert_approx_eq!(f64, spectrum_to_amplitude(0.0, 0.0), 0.0);
    }

    #[test]
    fn spectrum_amplitude_and_power_are_consistent() {
        let (re, im) = (1.5_f64, -2.5_f64);
        let amp = spectrum_to_amplitude(re, im);
        assert_approx_eq!(f64, amp * amp, spectrum_to_power(re, im), epsilon = 1e-12);
    }

    #[test]
    fn magnitude_to_amplitude_squares() {
        assert_approx_eq!(f64, magnitude_to_amplitude(3.0), 9.0);
        assert_approx_eq!(f64, magnitude_to_amplitude(-2.0), 4.0);
        assert_approx_eq!(f64, magnitude_to_amplitude(0.0), 0.0);
    }

    #[test]
    fn power_to_db_known_values() {
        assert_approx_eq!(f64, power_to_db(1.0, 1.0, AMIN, 0.0), 0.0);
        assert_approx_eq!(
            f64,
            power_to_db(10.0, 1.0, AMIN, 0.0),
            10.0,
            epsilon = 1e-12
        );
        assert_approx_eq!(
            f64,
            power_to_db(100.0, 1.0, AMIN, 0.0),
            20.0,
            epsilon = 1e-12
        );
        assert_approx_eq!(
            f64,
            power_to_db(0.1, 1.0, AMIN, 0.0),
            -10.0,
            epsilon = 1e-12
        );
    }

    #[test]
    fn power_to_db_respects_reference() {
        // 50 relative to 50 is 0 dB; 100 relative to 50 is ~3.0103 dB.
        assert_approx_eq!(f64, power_to_db(50.0, 50.0, AMIN, 0.0), 0.0);
        assert_approx_eq!(
            f64,
            power_to_db(100.0, 50.0, AMIN, 0.0),
            10.0 * 2.0_f64.log10(),
            epsilon = 1e-12
        );
    }

    #[test]
    fn power_to_db_clamps_to_amin() {
        // Zero input is floored to amin: 10 * log10(1e-10) = -100 dB.
        assert_approx_eq!(
            f64,
            power_to_db(0.0, 1.0, AMIN, 0.0),
            -100.0,
            epsilon = 1e-9
        );
        // Negative input is floored as well.
        assert_approx_eq!(
            f64,
            power_to_db(-5.0, 1.0, AMIN, 0.0),
            -100.0,
            epsilon = 1e-9
        );
        assert!(power_to_db(0.0, 1.0, AMIN, 0.0).is_finite());
    }

    #[test]
    fn power_to_db_subtracts_top_db() {
        assert_approx_eq!(
            f64,
            power_to_db(100.0, 1.0, AMIN, 10.0),
            10.0,
            epsilon = 1e-12
        );
    }

    #[test]
    fn amplitude_to_db_known_values() {
        assert_approx_eq!(f64, amplitude_to_db(1.0, 1.0, AMIN, 0.0), 0.0);
        assert_approx_eq!(
            f64,
            amplitude_to_db(10.0, 1.0, AMIN, 0.0),
            20.0,
            epsilon = 1e-12
        );
        assert_approx_eq!(
            f64,
            amplitude_to_db(0.1, 1.0, AMIN, 0.0),
            -20.0,
            epsilon = 1e-12
        );
        // Doubling the amplitude is ~6.0206 dB.
        assert_approx_eq!(
            f64,
            amplitude_to_db(2.0, 1.0, AMIN, 0.0),
            20.0 * 2.0_f64.log10(),
            epsilon = 1e-12
        );
    }

    #[test]
    fn amplitude_to_db_clamps_to_amin() {
        // 20 * log10(1e-5) = -100 dB.
        assert_approx_eq!(
            f64,
            amplitude_to_db(0.0, 1.0, 1e-5, 0.0),
            -100.0,
            epsilon = 1e-9
        );
        assert!(amplitude_to_db(0.0, 1.0, AMIN, 0.0).is_finite());
    }

    #[test]
    fn amplitude_to_db_subtracts_top_db() {
        assert_approx_eq!(
            f64,
            amplitude_to_db(10.0, 1.0, AMIN, 5.0),
            15.0,
            epsilon = 1e-12
        );
    }

    #[test]
    fn spectrum_to_db_matches_power_path() {
        // |3+4i|² = 25 -> 10 * log10(25)
        let expected = 10.0 * 25.0_f64.log10();
        assert_approx_eq!(
            f64,
            spectrum_to_db(3.0, 4.0, 1.0, AMIN, 0.0),
            expected,
            epsilon = 1e-12
        );
        assert_approx_eq!(
            f64,
            spectrum_to_db(3.0, 4.0, 1.0, AMIN, 0.0),
            power_to_db(spectrum_to_power(3.0, 4.0), 1.0, AMIN, 0.0),
            epsilon = 1e-12
        );
        // 10 * log10(25) == 20 * log10(5): power path agrees with amplitude
        // path.
        assert_approx_eq!(
            f64,
            spectrum_to_db(3.0, 4.0, 1.0, AMIN, 0.0),
            amplitude_to_db(spectrum_to_amplitude(3.0, 4.0), 1.0, AMIN, 0.0),
            epsilon = 1e-12
        );
    }

    #[test]
    fn db_to_power_known_values() {
        assert_approx_eq!(f64, db_to_power(0.0, 1.0), 1.0);
        assert_approx_eq!(f64, db_to_power(10.0, 1.0), 10.0, epsilon = 1e-12);
        assert_approx_eq!(f64, db_to_power(-10.0, 1.0), 0.1, epsilon = 1e-12);
        assert_approx_eq!(f64, db_to_power(20.0, 0.5), 50.0, epsilon = 1e-10);
    }

    #[test]
    fn db_to_amplitude_known_values() {
        assert_approx_eq!(f64, db_to_amplitude(0.0, 1.0), 1.0);
        assert_approx_eq!(f64, db_to_amplitude(20.0, 1.0), 10.0, epsilon = 1e-12);
        assert_approx_eq!(f64, db_to_amplitude(-20.0, 1.0), 0.1, epsilon = 1e-12);
        assert_approx_eq!(f64, db_to_amplitude(20.0, 2.0), 20.0, epsilon = 1e-10);
    }

    #[test]
    fn power_round_trip() {
        for &p in &[1e-6, 0.01, 0.5, 1.0, 3.7, 250.0, 1e6] {
            let db = power_to_db(p, 1.0, AMIN, 0.0);
            assert_approx_eq!(f64, db_to_power(db, 1.0), p, epsilon = 1e-9 * p.max(1.0));
        }
    }

    #[test]
    fn amplitude_round_trip() {
        for &a in &[1e-6, 0.01, 0.5, 1.0, 3.7, 250.0, 1e6] {
            let db = amplitude_to_db(a, 2.0, AMIN, 0.0);
            assert_approx_eq!(
                f64,
                db_to_amplitude(db, 2.0),
                a,
                epsilon = 1e-9 * a.max(1.0)
            );
        }
    }

    #[test]
    fn works_with_f32() {
        assert_approx_eq!(f32, spectrum_to_power(3.0_f32, 4.0), 25.0);
        assert_approx_eq!(f32, spectrum_to_amplitude(3.0_f32, 4.0), 5.0);
        assert_approx_eq!(
            f32,
            power_to_db(100.0_f32, 1.0, 1e-10, 0.0),
            20.0,
            epsilon = 1e-4
        );
        assert_approx_eq!(
            f32,
            amplitude_to_db(10.0_f32, 1.0, 1e-10, 0.0),
            20.0,
            epsilon = 1e-4
        );
        assert_approx_eq!(f32, db_to_power(10.0_f32, 1.0), 10.0, epsilon = 1e-4);
        assert_approx_eq!(f32, db_to_amplitude(20.0_f32, 1.0), 10.0, epsilon = 1e-4);
    }
}
