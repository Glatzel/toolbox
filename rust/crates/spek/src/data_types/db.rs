use generic_num::num;
use num_traits::Float;

use super::simple_structure_from_t;
use crate::data_types::{Amplitude, Power};

pub struct Db<T>(pub T);
simple_structure_from_t!(Db);
impl<T> Db<T>
where
    T: Float,
{
    /// Converts decibels back to amplitude: `reference * 10^(db / 20)`.
    ///
    /// Inverse of [`amplitude_to_db`] (with `top_db = 0` and no `amin`
    /// clamping).
    pub fn to_amplitude(&self, reference: T) -> Amplitude<T> {
        Amplitude(reference * num!(10.0).powf(self.0 / num!(20)))
    }
    /// Converts decibels back to power: `reference * 10^(db / 10)`.
    ///
    /// Inverse of [`power_to_db`] (with `top_db = 0` and no `amin` clamping).
    pub fn to_power(&self, reference: T) -> Power<T>
    where
        T: Float,
    {
        Power(reference * num!(10.0).powf(self.0 / num!(10)))
    }
}

impl<T> Db<T>
where
    T: Float,
{
    pub fn from_power<I, I1>(
        power: Power<T>,
        reference: T,
        amin: I1,
        top_db: I,
    ) -> Self where I: Into<Self>, I1: Into<Power<T>> {
        power.to_db(reference, amin, top_db)
    }
    /// Converts a complex spectrum bin directly to decibels (power scale).
    ///
    /// Equivalent to [`spectrum_to_power`] followed by [`power_to_db`].
    ///
    /// See [`power_to_db`] for the meaning of `reference`, `amin` and `top_db`.
    pub fn from_spectrum<I, I1>(real: T, imag: T, reference: T, amin: I1, top_db: I) -> Self
    where
        I: Into<Self>,
        I1: Into<Power<T>>,
    {
        Power::from_spectrum(real, imag).to_db(reference, amin, top_db)
    }
}
#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use super::*;

    const AMIN: f64 = 1e-10;
    #[test]
    fn test_to_amplitude() {
        assert_approx_eq!(f64, Db(0.0).to_amplitude(1.0).0, 1.0);
        assert_approx_eq!(f64, Db(20.0).to_amplitude(1.0).0, 10.0, epsilon = 1e-12);
        assert_approx_eq!(f64, Db(-20.0).to_amplitude(1.0).0, 0.1, epsilon = 1e-12);
        assert_approx_eq!(f64, Db(20.0).to_amplitude(2.0).0, 20.0, epsilon = 1e-10);
    }
    #[test]
    fn test_to_power() {
        assert_approx_eq!(f64, Db(0.0).to_power(1.0).0, 1.0);
        assert_approx_eq!(f64, Db(10.0).to_power(1.0).0, 10.0, epsilon = 1e-12);
        assert_approx_eq!(f64, Db(-10.0).to_power(1.0).0, 0.1, epsilon = 1e-12);
        assert_approx_eq!(f64, Db(20.0).to_power(0.5).0, 50.0, epsilon = 1e-10);
    }
    #[test]
    fn test_from_spectrum() {
        // |3+4i|² = 25 -> 10 * log10(25)
        let expected = 10.0 * 25.0_f64.log10();
        assert_approx_eq!(
            f64,
            Db::from_spectrum(3.0, 4.0, 1.0, AMIN, 0.0).0,
            expected,
            epsilon = 1e-12
        );
        assert_approx_eq!(
            f64,
            Db::from_spectrum(3.0, 4.0, 1.0, AMIN, 0.0).0,
            Power::from_spectrum(3.0, 4.0).to_db(1.0, AMIN, 0.0).0,
            epsilon = 1e-12
        );
        // 10 * log10(25) == 20 * log10(5): power path agrees with amplitude
        // path.
        assert_approx_eq!(
            f64,
            Db::from_spectrum(3.0, 4.0, 1.0, AMIN, 0.0).0,
            Power::from_spectrum(3.0, 4.0).to_db(1.0, AMIN, 0.0).0,
            epsilon = 1e-12
        );
    }
}
