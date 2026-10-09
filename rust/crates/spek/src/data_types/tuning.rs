use generic_num::num;
use num_traits::Float;

use crate::data_types::A4;

use super::simple_structure_from_t;

pub struct Tuning<T>(pub T);
simple_structure_from_t!(Tuning);

impl<T> Tuning<T>
where
    T: Float,
{
    pub fn to_a4(&self, bins_per_octave: usize) -> A4<T> {
        A4(num!(440.0) * num!(2.0).powf(self.0 / num!(bins_per_octave)))
    }
}
impl<T> Tuning<T>
where
    T: Float,
{
    pub fn from_a4(a4: A4<T>, bins_per_octave: usize) -> Self { a4.to_tuning(bins_per_octave) }
}
#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use super::*;

    #[test]
    fn test_to_a4() {
        let result: A4<f32> = Tuning(-0.318).to_a4(12);
        assert_approx_eq!(f32, result.0, 431.99167);
    }
}
