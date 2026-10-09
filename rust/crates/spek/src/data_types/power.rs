use generic_num::num;
use num_traits::Float;

use super::simple_structure_from_t;
use crate::data_types::{Amplitude, Db};
pub struct Power<T>(pub T);
simple_structure_from_t!(Power);
impl<T> Power<T>
where
    T: Float,
{
    /// Squares a magnitude value: `power²`.
    ///
    /// Note that despite the name, the result is a *power*-scale quantity
    /// (`|z|²`), not an amplitude.
    pub fn to_amplitude(&self) -> Amplitude<T>
    where
        T: Float,
    {
        Amplitude(self.0.powi(2))
    }
    /// Converts a power value to decibels: `10 * log10(max(power, amin) /
    /// reference) - top_db`.
    ///
    /// # Parameters
    /// - `power`: input power.
    /// - `reference`: power that maps to 0 dB (librosa's `ref`, usually `1.0`).
    /// - `amin`: floor applied to `power` before the logarithm, so that zero
    ///   input yields a finite value instead of `-inf`.
    /// - `top_db`: constant offset **subtracted** from the result. Pass `0` for
    ///   a plain dB conversion. Unlike librosa, this is *not* a relative
    ///   dynamic-range clip.
    pub fn to_db<I, I1>(&self, reference: T, amin: I, top_db: I1) -> Db<T>
    where
        I: Into<Self>,
        I1: Into<Db<T>>,
    {
        let power = self.0.max(amin.into().0);
        Db(num!(10.0) * (power / reference).log10() - top_db.into().0)
    }
}
impl<T> Power<T>
where
    T: Float,
{
    pub fn from_amplitude(amplitude: Amplitude<T>) -> Self { amplitude.to_power() }
    pub fn from_db(db: Db<T>, reference: T) -> Self { db.to_power(reference) }
    /// Converts a complex spectrum bin to power: `re² + im²`.
    ///
    /// This is the squared magnitude `|z|²`.
    pub fn from_spectrum(real: T, imag: T) -> Self { Self(real * real + imag * imag) }
}
#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use super::*;

    const AMIN: f64 = 1e-10;

    #[test]
    fn test_to_amplitude() {
        assert_approx_eq!(f64, Power(3.0).to_amplitude().0, 9.0);
        assert_approx_eq!(f64, Power(-2.0).to_amplitude().0, 4.0);
        assert_approx_eq!(f64, Power(0.0).to_amplitude().0, 0.0);
    }
    #[test]
    fn to_db() {
        assert_approx_eq!(f64, Power(1.0).to_db(1.0, AMIN, 0.0).0, 0.0);
        assert_approx_eq!(
            f64,
            Power(10.0).to_db(1.0, AMIN, 0.0).0,
            10.0,
            epsilon = 1e-12
        );
        assert_approx_eq!(
            f64,
            Power(100.0).to_db(1.0, AMIN, 0.0).0,
            20.0,
            epsilon = 1e-12
        );
        assert_approx_eq!(
            f64,
            Power(0.1).to_db(1.0, AMIN, 0.0).0,
            -10.0,
            epsilon = 1e-12
        );
        assert_approx_eq!(f64, Power(50.0).to_db(50.0, AMIN, 0.0).0, 0.0);
        assert_approx_eq!(
            f64,
            Power(100.0).to_db(50.0, AMIN, 0.0).0,
            10.0 * 2.0_f64.log10(),
            epsilon = 1e-12
        );
        // Zero input is floored to amin: 10 * log10(1e-10) = -100 dB.
        assert_approx_eq!(
            f64,
            Power(0.0).to_db(1.0, AMIN, 0.0).0,
            -100.0,
            epsilon = 1e-9
        );
        // Negative input is floored as well.
        assert_approx_eq!(
            f64,
            Power(-5.0).to_db(1.0, AMIN, 0.0).0,
            -100.0,
            epsilon = 1e-9
        );
        assert!(Power(0.0).to_db(1.0, AMIN, 0.0).0.is_finite());
        assert_approx_eq!(
            f64,
            Power(100.0).to_db(1.0, AMIN, 10.0).0,
            10.0,
            epsilon = 1e-12
        );
    }

    #[test]
    fn test_from_spectrum() {
        assert_approx_eq!(f64, Power::from_spectrum(3.0, 4.0).0, 25.0);
        assert_approx_eq!(f64, Power::from_spectrum(-3.0, 4.0).0, 25.0);
        assert_approx_eq!(f64, Power::from_spectrum(0.0, 0.0).0, 0.0);
    }
}
