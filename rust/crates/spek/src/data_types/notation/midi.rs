use generic_num::num;
use num_traits::Float;

use super::super::simple_structure;
use crate::data_types::Hz;

simple_structure!(Midi);
impl<T> Midi<T>
where
    T: Float,
{
    pub fn to_hz(&self) -> Hz<T> {
        Hz(num!(440.0) * (num!(2.0).powf((self.0 - num!(69.0)) / num!(12.0))))
    }
}
impl<T> Midi<T>
where
    T: Float,
{
    pub fn from_hz(frequency: Hz<T>) -> Self { frequency.to_midi() }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use super::*;
    #[test]
    fn test_to_hz() {
        let result: Hz<f32> = Midi(36.0).to_hz();
        assert_approx_eq!(f32, result.0, 65.40639);
    }
}
