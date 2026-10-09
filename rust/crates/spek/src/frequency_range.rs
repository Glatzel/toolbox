use generic_num::num;
use num_traits::Float;

use crate::convert::frequency_unit::{hz_to_mel, mel_to_hz};
use crate::utils::linspace;

pub fn fft_frequencies<T>(sr: T, n_fft: usize) -> impl ExactSizeIterator<Item = T>
where
    T: Float,
{
    #[allow(
        clippy::range_plus_one,
        reason = "After fix, throw trait `std::iter::ExactSizeIterator` is not implemented for `std::ops::RangeInclusive<usize>`."
    )]
    (0..(1 + n_fft / 2)).map(move |n| num!(n) * sr / num!(n_fft))
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

pub fn mel_frequencies<T>(
    n_mels: usize,
    fmin: T,
    fmax: T,
    htk: bool,
) -> impl ExactSizeIterator<Item = T>
where
    T: Float,
{
    let min_mel = hz_to_mel(fmin, htk);
    let max_mel = hz_to_mel(fmax, htk);
    linspace(min_mel, max_mel, n_mels).map(move |mel| mel_to_hz(mel, htk))
}
pub fn tempo_frequencies<T>(n_bins: usize, hop_length: usize, sr: T) -> impl Iterator<Item = T>
where
    T: Float,
{
    std::iter::once(T::infinity())
        .chain((1..n_bins).map(move |n| num!(60.0) * sr / (num!(hop_length) * num!(n))))
}
pub fn fourier_tempo_frequencies<T>(
    sr: T,
    win_length: usize,
    hop_length: usize,
) -> impl ExactSizeIterator<Item = T>
where
    T: Float,
{
    fft_frequencies(sr * num!(60) / num!(hop_length), win_length)
}

#[cfg(test)]
mod tests {
    use num_traits::ToPrimitive;

    use super::*;
    use crate::notation::Note;
    #[test]
    fn test_fft_frequencies() {
        let result: Vec<_> = fft_frequencies::<f64>(22050.0, 16)
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
    #[test]
    fn test_mel_frequencies() {
        let result: Vec<_> = mel_frequencies::<f64>(40, 0.0, 11025.0, false)
            .take(10)
            .map(|i| format!("{i:.3}"))
            .collect();
        insta::assert_debug_snapshot!(result,@r#"
        [
            "0.000",
            "85.317",
            "170.635",
            "255.952",
            "341.269",
            "426.586",
            "511.904",
            "597.221",
            "682.538",
            "767.855",
        ]
        "#)
    }
    #[test]
    fn test_tempo_frequencies() {
        let result: Vec<_> = tempo_frequencies::<f64>(384, 512, 22050.0)
            .take(5)
            .map(|i| format!("{i:.3}"))
            .collect();
        insta::assert_debug_snapshot!(result,@r#"
        [
            "inf",
            "2583.984",
            "1291.992",
            "861.328",
            "645.996",
        ]
        "#)
    }
    #[test]
    fn test_fourier_tempo_frequencies() {
        let result: Vec<_> = fourier_tempo_frequencies::<f64>(22050.0, 384, 512)
            .take(5)
            .map(|i| format!("{i:.3}"))
            .collect();
        insta::assert_debug_snapshot!(result,@r#"
        [
            "0.000",
            "6.729",
            "13.458",
            "20.187",
            "26.917",
        ]
        "#)
    }
}
