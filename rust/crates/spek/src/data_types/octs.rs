use generic_num::num;
use num_traits::Float;

use super::simple_structure;
use crate::data_types::{Hz, Tuning};

simple_structure!(Octs);

impl<T> Octs<T>
where
    T: Float,
{
    pub fn to_hz(&self, tuning: Tuning<T>, bins_per_octave: usize) -> Hz<T> {
        let a440 = num!(440.0) * num!(2.0).powf(tuning.0 / num!(bins_per_octave));
        Hz((a440 / num!(16)) * (num!(2.0).powf(self.0)))
    }
}

impl<T> Octs<T>
where
    T: Float,
{
    pub fn from_hz(frequency: Hz<T>, tuning: Tuning<T>, bins_per_octave: usize) -> Self {
        frequency.to_octs(tuning, bins_per_octave)
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use super::*;
    #[test]
    fn test_frequency() {
        let result: Hz<f32> = Octs(1.0).to_hz(0.0.into(), 12);
        assert_approx_eq!(f32, result.0, 55.0);
    }
}
