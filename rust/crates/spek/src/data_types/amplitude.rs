use generic_num::num;
use num_traits::Float;

use super::simple_structure;
use crate::data_types::{Db, Power};

simple_structure!(Amplitude);

impl<T> Amplitude<T>
where
    T: Float,
{
    /// Converts an amplitude to decibels: `20 * log10(max(amplitude, amin) /
    /// reference) - top_db`.
    ///
    /// # Parameters
    /// - `reference`: amplitude that maps to 0 dB (librosa's `ref`, usually
    ///   `1.0`).
    /// - `amin`: floor applied to `amplitude` before the logarithm, so that
    ///   zero input yields a finite value instead of `-inf`.
    /// - `top_db`: constant offset **subtracted** from the result. Pass `0` for
    ///   a plain dB conversion. Unlike librosa, this is *not* a relative
    ///   dynamic-range clip, since a scalar function has no access to the
    ///   spectrogram maximum.
    pub fn to_db<I, I1>(&self, reference: T, amin: I1, top_db: I) -> Db<T>
    where
        I: Into<Db<T>>,
        I1: Into<Self>,
    {
        let amplitude = self.0.max(amin.into().0);
        Db(num!(20.0) * (amplitude / reference).log10() - top_db.into().0)
    }
    pub fn to_power(&self) -> Power<T> { Power(self.0.sqrt()) }
}
impl<T> Amplitude<T>
where
    T: Float,
{
    pub fn from_db(db: Db<T>, reference: T) -> Self { db.to_amplitude(reference) }
    pub fn from_power(power: Power<T>) -> Self { power.to_amplitude() }

    /// Converts a complex spectrum bin to amplitude (magnitude): `sqrt(re² +
    /// im²)`.
    ///
    /// Uses [`Float::hypot`], which avoids intermediate overflow/underflow.
    pub fn from_spectrum(real: T, imag: T) -> Self { Self(real.hypot(imag)) }
}
#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use super::*;

    const AMIN: f64 = 1e-10;
    #[test]
    fn test_to_db() {
        assert_approx_eq!(f64, Amplitude(1.0).to_db(1.0, AMIN, 0.0).0, 0.0);
        assert_approx_eq!(
            f64,
            Amplitude(10.0).to_db(1.0, AMIN, 0.0).0,
            20.0,
            epsilon = 1e-12
        );
        assert_approx_eq!(
            f64,
            Amplitude(0.1).to_db(1.0, AMIN, 0.0).0,
            -20.0,
            epsilon = 1e-12
        );
        // Doubling the amplitude is ~6.0206 dB.
        assert_approx_eq!(
            f64,
            Amplitude(2.0).to_db(1.0, AMIN, 0.0).0,
            20.0 * 2.0_f64.log10(),
            epsilon = 1e-12
        );
        assert_approx_eq!(
            f64,
            Amplitude(0.0).to_db(1.0, 1e-5, 0.0).0,
            -100.0,
            epsilon = 1e-9
        );
        assert!(Amplitude(0.0).to_db(1.0, AMIN, 0.0).0.is_finite());
        assert_approx_eq!(
            f64,
            Amplitude(10.0).to_db(1.0, AMIN, 5.0).0,
            15.0,
            epsilon = 1e-12
        );
    }
    #[test]
    fn test_from_spectrum() {
        assert_approx_eq!(f64, Amplitude::from_spectrum(3.0, 4.0).0, 5.0);
        assert_approx_eq!(f64, Amplitude::from_spectrum(-3.0, -4.0).0, 5.0);
        assert_approx_eq!(f64, Amplitude::from_spectrum(0.0, 0.0).0, 0.0);
    }
}
