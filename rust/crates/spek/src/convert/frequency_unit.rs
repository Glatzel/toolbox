use std::iter::Sum;

use generic_num::num;
use num_traits::Float;

use crate::notation::Note;

fn _hz_to_note() { todo!() }
fn _hz_to_midi() { todo!() }
fn _hz_to_svara_h() { todo!() }
fn _hz_to_svara_c() { todo!() }
fn _hz_to_fjs() { todo!() }
pub fn midi_to_hz<T>(midi: T) -> T
where
    T: Float + Sum,
{
    num!(440.0) * (num!(2.0).powf((midi - num!(69.0)) / num!(12.0)))
}
fn _midi_to_note() { todo!() }
fn _midi_to_svara_h() { todo!() }
fn _midi_to_svara_c() { todo!() }
fn _note_to_hz<T>(note: &Note) -> T
where
    T: Float + Sum,
{
    note.to_hz()
}
pub fn note_to_midi<T>(note: &Note) -> T
where
    T: Float + Sum,
{
    note.to_midi()
}
fn _note_to_svara_h() { todo!() }
fn _note_to_svara_c() { todo!() }
pub fn hz_to_mel<T>(frequency: T, htk: bool) -> T
where
    T: Float,
{
    if htk {
        num!(2595.0) * (T::one() + frequency / num!(700.0)).log10()
    } else {
        let f_min = T::zero();
        let f_sp = num!(200.0 / 3.0);
        let mut mel = (frequency - f_min) / f_sp;
        let min_log_hz = num!(1000.0);
        let min_log_mel = (min_log_hz - f_min) / f_sp;
        let logstep = num!(6.4.ln()) / num!(27.0);
        if frequency >= min_log_hz {
            mel = min_log_mel + (frequency / min_log_hz).ln() / logstep;
        }
        mel
    }
}
fn _hz_to_octs() { todo!() }
pub fn mel_to_hz<T>(mel: T, htk: bool) -> T
where
    T: Float,
{
    if htk {
        num!(700.0) * (num!(10.0).powf(mel / num!(2595.0)) - T::one())
    } else {
        let f_min = T::zero();
        let f_sp = num!(200.0 / 3.0);
        let mut freq = f_min + f_sp * mel;

        let min_log_hz = num!(1000.0);
        let min_log_mel = (min_log_hz - f_min) / f_sp;
        let logstep = num!(6.4).ln() / num!(27.0);

        if mel >= min_log_mel {
            freq = min_log_hz * (logstep * (mel - min_log_mel)).exp();
        }

        freq
    }
}
fn _octs_to_hz() { todo!() }
fn _a4_to_tuning() { todo!() }
fn _tuning_to_a4() { todo!() }

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use super::*;

    #[test]
    fn test_midi_to_hz() {
        let result: f32 = midi_to_hz(36.0);
        assert_approx_eq!(f32, result, 65.40639);
    }
    #[test]
    fn test_hz_to_mel() {
        let result: f32 = hz_to_mel(60.0, false);
        assert_approx_eq!(f32, result, 0.9);
    }
    #[test]
    fn test_mel_to_hz() {
        let result: f32 = mel_to_hz(3.0, false);
        assert_approx_eq!(f32, result, 200.0);
    }
}
