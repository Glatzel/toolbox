use generic_num::num;
use num_traits::Float;

pub fn fft_frequencies<T>(sr: usize, n_fft: usize) -> impl ExactSizeIterator<Item = T>
where
    T: Float,
{
    #[allow(
        clippy::range_plus_one,
        reason = "After fix, throw trait `std::iter::ExactSizeIterator` is not implemented for `std::ops::RangeInclusive<usize>`."
    )]
    (0..(1 + n_fft / 2)).map(move |n| num!(sr * n / n_fft))
}

pub fn cqt_frequencies<T>(
    n_bins: usize,
    fmin: T,
    bins_per_octave: usize,
    tuning: T,
) -> impl ExactSizeIterator<Item = T>
where
    T: Float,
{
    let correction = num!(2.0).powf(tuning / num!(bins_per_octave));
    (0..n_bins).map(move |n| num!(2.0).powf(num!(n) / num!(bins_per_octave)) * fmin * correction)
}

fn _mel_frequencies() { todo!() }
fn _tempo_frequencies() { todo!() }
fn _fourier_tempo_frequencies() { todo!() }

#[cfg(test)]
mod tests {
    use num_traits::ToPrimitive;

    use super::*;
    use crate::notation::Note;
    #[test]
    fn test_fft_frequencies() {
        let result: Vec<_> = fft_frequencies::<f64>(22050, 16)
            .map(|i| i.to_u16().unwrap())
            .collect();
        insta::assert_debug_snapshot!(result,@"
        [
            0,
            1378,
            2756,
            4134,
            5512,
            6890,
            8268,
            9646,
            11025,
        ]
        ")
    }
    #[test]
    fn test_cqt_frequencies() {
        let fmin = Note {
            pitch: crate::notation::Pitch::C,
            accs: Vec::new(),
            octave: Some(2),
            cents: None,
        }
        .to_hz();
        let result: Vec<_> = cqt_frequencies::<f64>(24, fmin, 12, 0.0)
            .map(|i| format!("{i:.3}"))
            .collect();
        insta::assert_debug_snapshot!(result,@r#"
        [
            "65.406",
            "69.296",
            "73.416",
            "77.782",
            "82.407",
            "87.307",
            "92.499",
            "97.999",
            "103.826",
            "110.000",
            "116.541",
            "123.471",
            "130.813",
            "138.591",
            "146.832",
            "155.563",
            "164.814",
            "174.614",
            "184.997",
            "195.998",
            "207.652",
            "220.000",
            "233.082",
            "246.942",
        ]
        "#)
    }
}
