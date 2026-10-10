use generic_num::num;
use num_traits::Float;

use crate::data_types::{Hz, Mel, Tuning};
use crate::utils::linspace;
impl<T> Hz<T>
where
    T: Float,
{
    /// Compute the center frequencies of the non-negative FFT bins.
    ///
    /// This is an alternative interface to `numpy.fft.rfftfreq`, returning the
    /// frequencies corresponding to the non-negative bins of a real-valued FFT.
    ///
    /// # Parameters
    ///
    /// - `sr`: Audio sampling rate in Hz. Must be greater than zero.
    /// - `n_fft`: FFT frame length. Must be greater than zero.
    ///
    /// # Returns
    ///
    /// An iterator of length `1 + n_fft / 2`, containing frequencies in Hz:
    /// `0, sr / n_fft, 2 * sr / n_fft, ..., sr / 2` (for even `n_fft`).
    ///
    /// # References
    ///
    /// - [librosa.fft_frequencies](https://librosa.org/doc/latest/generated/librosa.fft_frequencies.html)
    pub fn fft_frequencies(sr: T, n_fft: usize) -> impl ExactSizeIterator<Item = Hz<T>> {
        #[allow(
            clippy::range_plus_one,
            reason = "RangeInclusive does not implement ExactSizeIterator."
        )]
        (0..(1 + n_fft / 2)).map(move |n| Hz(num!(n) * sr / num!(n_fft)))
    }

    /// Compute the center frequencies of Constant-Q Transform (CQT) bins.
    ///
    /// The frequencies are geometrically spaced, with `bins_per_octave` bins
    /// per octave. The tuning parameter shifts all frequencies by a fractional
    /// number of bins.
    ///
    /// # Parameters
    ///
    /// - `n_bins`: Number of CQT frequency bins.
    /// - `fmin`: Minimum frequency in Hz, corresponding to the first bin.
    /// - `bins_per_octave`: Number of bins per octave. Must be greater than
    ///   zero.
    /// - `tuning`: Tuning deviation in fractional bins. A value of zero applies
    ///   no tuning correction.
    ///
    /// # Returns
    ///
    /// An iterator of length `n_bins` containing the center frequencies in Hz.
    /// The frequency of bin `n` is
    /// `fmin * 2^(n / bins_per_octave) * 2^(tuning / bins_per_octave)`.
    ///
    /// # References
    ///
    /// - [librosa.cqt_frequencies](https://librosa.org/doc/latest/generated/librosa.cqt_frequencies.html)
    pub fn cqt_frequencies<I, I1>(
        n_bins: usize,
        fmin: I,
        bins_per_octave: usize,
        tuning: I1,
    ) -> impl ExactSizeIterator<Item = T>
    where
        I: Into<Hz<T>>,
        I1: Into<Tuning<T>>,
    {
        let correction = num!(2.0).powf(tuning.into().0 / num!(bins_per_octave));
        let fmin = fmin.into().0;
        (0..n_bins)
            .map(move |n| num!(2.0).powf(num!(n) / num!(bins_per_octave)) * fmin * correction)
    }

    /// Compute frequencies uniformly spaced on the Mel scale.
    ///
    /// The function converts the lower and upper frequency limits to the Mel
    /// scale, generates uniformly spaced Mel values between those limits, and
    /// converts the values back to Hz.
    ///
    /// # Parameters
    ///
    /// - `n_mels`: Number of Mel-spaced frequency points.
    /// - `fmin`: Lower frequency limit in Hz.
    /// - `fmax`: Upper frequency limit in Hz.
    /// - `htk`: Whether to use the HTK Mel scale instead of the default librosa
    ///   Mel scale.
    ///
    /// # Returns
    ///
    /// An iterator of length `n_mels` containing frequencies in Hz, uniformly
    /// spaced in the Mel domain. The first and last frequencies correspond to
    /// `fmin` and `fmax`, respectively, when `n_mels` is greater than zero.
    ///
    /// # References
    ///
    /// - [librosa.mel_frequencies](https://librosa.org/doc/latest/generated/librosa.mel_frequencies.html)
    pub fn mel_frequencies(
        n_mels: usize,
        fmin: Hz<T>,
        fmax: Hz<T>,
        htk: bool,
    ) -> impl ExactSizeIterator<Item = Hz<T>> {
        let min_mel = fmin.to_mel(htk);
        let max_mel = fmax.to_mel(htk);
        linspace(min_mel.0, max_mel.0, n_mels).map(move |mel| Mel(mel).to_hz(htk))
    }

    /// Compute tempo frequencies corresponding to the lag bins of a tempogram.
    ///
    /// The first bin represents zero lag and is assigned positive infinity.
    /// The remaining bins correspond to positive lag values and are converted
    /// to beats per minute (BPM).
    ///
    /// # Parameters
    ///
    /// - `n_bins`: Number of lag bins. Expected to be greater than zero.
    /// - `hop_length`: Number of audio samples between successive frames. Must
    ///   be greater than zero.
    /// - `sr`: Audio sampling rate in Hz. Must be greater than zero.
    ///
    /// # Returns
    ///
    /// An iterator yielding `n_bins` tempo frequencies in BPM. The first value
    /// is positive infinity; each subsequent value at index `n` is
    /// `60 * sr / (hop_length * n)`.
    ///
    /// # References
    ///
    /// - [librosa.tempo_frequencies](https://librosa.org/doc/latest/generated/librosa.tempo_frequencies.html)
    pub fn tempo_frequencies(
        n_bins: usize,
        hop_length: usize,
        sr: T,
    ) -> impl Iterator<Item = Hz<T>> {
        std::iter::once(Hz(T::infinity()))
            .chain((1..n_bins).map(move |n| Hz(num!(60.0) * sr / (num!(hop_length) * num!(n)))))
    }

    /// Compute tempo frequencies corresponding to the bins of a Fourier
    /// tempogram.
    ///
    /// This function uses the FFT frequency grid, scaling the effective
    /// sampling rate by `60 / hop_length` to express the resulting
    /// frequencies in BPM.
    ///
    /// # Parameters
    ///
    /// - `sr`: Audio sampling rate in Hz. Must be greater than zero.
    /// - `win_length`: Length of the Fourier tempogram window in frames. Must
    ///   be greater than zero.
    /// - `hop_length`: Number of audio samples between successive frames. Must
    ///   be greater than zero.
    ///
    /// # Returns
    ///
    /// An iterator of length `1 + win_length / 2` containing the tempo
    /// frequencies in BPM, including zero BPM for the DC bin.
    ///
    /// # References
    ///
    /// - [librosa.fourier_tempo_frequencies](https://librosa.org/doc/latest/generated/librosa.fourier_tempo_frequencies.html)
    pub fn fourier_tempo_frequencies(
        sr: T,
        win_length: usize,
        hop_length: usize,
    ) -> impl ExactSizeIterator<Item = Hz<T>> {
        Hz::fft_frequencies(sr * num!(60) / num!(hop_length), win_length)
    }
}
#[cfg(test)]
mod tests {
    use num_traits::ToPrimitive;

    use super::*;
    use crate::data_types::notation::Note;
    #[test]
    fn test_fft_frequencies() {
        let result: Vec<_> = Hz::fft_frequencies(22050.0, 16)
            .map(|i| i.0.to_u16().unwrap())
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
            pitch: crate::data_types::notation::Pitch::C,
            accs: Vec::new(),
            octave: Some(2),
            cents: None,
        }
        .to_hz();
        let result: Vec<_> = Hz::cqt_frequencies::<_, _>(24, fmin, 12, 0.0)
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
        let result: Vec<_> = Hz::mel_frequencies(40, 0.0.into(), 11025.0.into(), false)
            .take(10)
            .map(|i| format!("{:.3}", i.0))
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
        let result: Vec<_> = Hz::tempo_frequencies(384, 512, 22050.0)
            .take(5)
            .map(|i| format!("{:.3}", i.0))
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
        let result: Vec<_> = Hz::fourier_tempo_frequencies(22050.0, 384, 512)
            .take(5)
            .map(|i| format!("{:.3}", i.0))
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
