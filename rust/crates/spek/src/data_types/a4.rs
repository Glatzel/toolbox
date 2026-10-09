use generic_num::num;
use num_traits::Float;

use super::simple_structure;
use crate::data_types::Tuning;

simple_structure!(A4);

impl<T> A4<T>
where
    T: Float,
{
    pub fn to_tuning(&self, bins_per_octave: usize) -> Tuning<T> {
        Tuning(num!(bins_per_octave) * (self.0.log2() - num!(440.0).log2()))
    }
}
impl<T> A4<T>
where
    T: Float,
{
    pub fn from_tuning(tuning: Tuning<T>, bins_per_octave: usize) -> Self {
        tuning.to_a4(bins_per_octave)
    }
}
#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use super::*;
    #[test]
    fn test_octs_to_tuning() {
        let result: Tuning<f32> = A4(432.0).to_tuning(12);
        assert_approx_eq!(f32, result.0, -0.3176651);
    }
}
