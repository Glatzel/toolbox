use generic_num::num;
use num_traits::Float;

use super::simple_structure;
use crate::data_types::Frequency;

simple_structure!(Mel);
impl<T> Mel<T>
where
    T: Float,
{
    pub fn to_frequency(&self, htk: bool) -> Frequency<T> {
        if htk {
            Frequency(num!(700.0) * (num!(10.0).powf(self.0 / num!(2595.0)) - T::one()))
        } else {
            let f_min = T::zero();
            let f_sp = num!(200.0 / 3.0);
            let mut freq = f_min + f_sp * self.0;

            let min_log_hz = num!(1000.0);
            let min_log_mel = (min_log_hz - f_min) / f_sp;
            let logstep = num!(6.4).ln() / num!(27.0);

            if self.0 >= min_log_mel {
                freq = min_log_hz * (logstep * (self.0 - min_log_mel)).exp();
            }

            Frequency(freq)
        }
    }
}

impl<T> Mel<T>
where
    T: Float,
{
    pub fn from_frequency(frequency: Frequency<T>, htk: bool) -> Self { frequency.to_mel(htk) }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use super::*;
    #[test]
    fn test_to_frequency() {
        let result: Frequency<f32> = Mel(3.0).to_frequency(false);
        assert_approx_eq!(f32, result.0, 200.0);
    }
}
