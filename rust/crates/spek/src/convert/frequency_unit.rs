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
fn _hz_to_mel() { todo!() }
fn _hz_to_octs() { todo!() }
fn _mel_to_hz() { todo!() }
fn _octs_to_hz() { todo!() }
fn _a4_to_tuning() { todo!() }
fn _tuning_to_a4() { todo!() }

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use crate::convert::frequency_unit::midi_to_hz;

    #[test]
    fn test_midi_to_hz() {
        let result: f32 = midi_to_hz(36.0);
        assert_approx_eq!(f32, result, 65.40639);
    }
}
