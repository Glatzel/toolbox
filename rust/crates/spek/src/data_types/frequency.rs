use generic_num::num;
use num_traits::Float;

use super::simple_structure;
use crate::data_types::{Mel, Midi, Octs, Tuning};

simple_structure!(Frequency);
impl<T> Frequency<T>
where
    T: Float,
{
    pub fn to_mel(&self, htk: bool) -> Mel<T> {
        if htk {
            Mel(num!(2595.0) * (T::one() + self.0 / num!(700.0)).log10())
        } else {
            let f_min = T::zero();
            let f_sp = num!(200.0 / 3.0);
            let mut mel = (self.0 - f_min) / f_sp;
            let min_log_hz = num!(1000.0);
            let min_log_mel = (min_log_hz - f_min) / f_sp;
            let logstep = num!(6.4.ln()) / num!(27.0);
            if self.0 >= min_log_hz {
                mel = min_log_mel + (self.0 / min_log_hz).ln() / logstep;
            }
            Mel(mel)
        }
    }
    pub fn to_midi(&self) -> Midi<T> {
        Midi(num!(12) * (self.0.log2() - num!(440.0).log2()) + num!(69))
    }
    pub fn to_octs(&self, tuning: Tuning<T>, bins_per_octave: usize) -> Octs<T> {
        let a440 = num!(440.0) * num!(2.0).powf(tuning.0 / num!(bins_per_octave));
        Octs((self.0 / (a440 / num!(16))).log2())
    }
}
impl<T> Frequency<T>
where
    T: Float,
{
    pub fn from_mel(mel: Mel<T>, htk: bool) -> Self { mel.to_frequency(htk) }
    pub fn from_midi(midi: Midi<T>) -> Self { midi.to_frequency() }
    pub fn from_octs(octs: Octs<T>, tuning: Tuning<T>, bins_per_octave: usize) -> Self {
        octs.to_frequency(tuning, bins_per_octave)
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use super::*;
    #[test]
    fn test_hz_to_mel() {
        let result: Mel<f32> = Frequency(60.0).to_mel(false);
        assert_approx_eq!(f32, result.0, 0.9);
    }
    #[test]
    fn test_to_midi() {
        let result: Midi<f32> = Frequency(60.0).to_midi();
        assert_approx_eq!(f32, result.0, 34.50637);
    }
    #[test]
    fn test_to_octs() {
        let result: Octs<f32> = Frequency(440.0).to_octs(0.0.into(), 12);
        assert_approx_eq!(f32, result.0, 4.0);
    }
}
