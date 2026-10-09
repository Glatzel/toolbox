use generic_num::num;
use num_traits::Float;

use crate::data_types::{Db, Frequency, Power};

/// Frequency weighting curve to apply.
/// # References
///
/// - [librosa.frequency_weighting](https://librosa.org/doc/latest/api/generated/librosa.frequency_weighting.html)
/// - [librosa.A_weighting](https://librosa.org/doc/latest/api/generated/librosa.A_weighting.html)
/// - [librosa.B_weighting](https://librosa.org/doc/latest/api/generated/librosa.B_weighting.html)
/// - [librosa.C_weighting](https://librosa.org/doc/latest/api/generated/librosa.C_weighting.html)
/// - [librosa.D_weighting](https://librosa.org/doc/latest/api/generated/librosa.D_weighting.html)
/// - [librosa.Z_weighting](https://librosa.org/doc/latest/api/generated/librosa.Z_weighting.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeightingKind {
    /// A-weighting, the most common curve for general loudness.
    A,

    ///B-weighting, a legacy curve for medium-level sounds.
    B,

    ///C-weighting, nearly flat; used for high-level sounds and peaks.
    C,

    /// D-weighting, designed for aircraft noise measurement.
    D,

    ///Zero weighting, i.e. 0 dB at every frequency (subject to `min_db`).
    Z,
}

/// Apply a perceptual frequency weighting to a single power value, in dB.
///
/// The weighting offset of `frequency` is added to the dB-scaled `power`:
/// `frequency_weighting(frequency, kind, min_db) + power_to_db(power,
/// reference, amin, top_db)`.
///
/// This is the scalar counterpart of `librosa.perceptual_weighting`, which
/// operates on a whole power spectrogram and a vector of frequencies.
///
/// # Parameters
///
/// - `power`: Power value, e.g. a magnitude-squared STFT bin.
/// - `frequency`: Frequency of the bin in Hz.
/// - `reference`: Reference power for the dB conversion (`ref` in librosa).
/// - `amin`: Minimum power threshold, used to avoid `log(0)`.
/// - `top_db`: Dynamic range threshold passed to [`power_to_db`].
/// - `kind`: Weighting curve, see [`WeightingKind`].
/// - `min_db`: Optional lower bound for the weighting curve in dB.
///
/// # Returns
///
/// The weighted power in dB.
///
/// # Notes
///
/// - librosa always uses its default `min_db = -80.0` for the weighting curve
///   here (extra keyword arguments are forwarded to `power_to_db`). Pass
///   `Some(-80.0)` to match it.
/// - librosa applies `top_db` relative to the maximum of the whole input array;
///   a scalar has no such maximum.
///
/// # References
///
/// - [librosa.perceptual_weighting](https://librosa.org/doc/latest/api/generated/librosa.perceptual_weighting.html)
pub fn perceptual_weighting<T>(
    power: impl Into<Power<T>>,
    frequency: impl Into<Frequency<T>>,
    reference: T,
    amin: impl Into<Power<T>>,
    top_db: impl Into<Db<T>>,
    kind: WeightingKind,
    min_db: impl Into<Db<T>>,
) -> T
where
    T: Float,
{
    frequency_weighting(frequency.into().0, kind, min_db.into().0)
        + power
            .into()
            .to_db(reference, amin.into().0, top_db.into().0)
            .0
}

/// Compute the weighting of a frequency, in dB, for the given curve.
///
/// Dispatches to [`a_weighting`], [`b_weighting`], [`c_weighting`],
/// [`d_weighting`], or a flat zero curve for [`WeightingKind::Z`].
///
/// # Parameters
///
/// - `frequency`: Frequency in Hz.
/// - `kind`: Weighting curve, see [`WeightingKind`].
/// - `min_db`: Optional lower bound for the result in dB. librosa defaults to
///   `-80.0`; `None` disables clipping.
///
/// # Returns
///
/// The weighting in dB. For [`WeightingKind::Z`] this is `0.0`, or
/// `max(min_db, 0.0)` when `min_db` is given.
///
/// # References
///
/// - [librosa.frequency_weighting](https://librosa.org/doc/latest/api/generated/librosa.frequency_weighting.html)
/// - [librosa.Z_weighting](https://librosa.org/doc/latest/api/generated/librosa.Z_weighting.html)
pub fn frequency_weighting<T>(frequency: T, kind: WeightingKind, min_db: impl Into<Db<T>>) -> T
where
    T: Float,
{
    match kind {
        WeightingKind::A => a_weighting(frequency, min_db),
        WeightingKind::B => b_weighting(frequency, min_db),
        WeightingKind::C => c_weighting(frequency, min_db),
        WeightingKind::D => d_weighting(frequency, min_db),
        WeightingKind::Z => min_db.into().0.max(num!(0.0)),
    }
}

/// Compute several weighting curves over the same set of frequencies.
///
/// This is the lazy counterpart of `librosa.multi_frequency_weighting`, which
/// returns a stacked array of shape `(len(kinds), len(frequencies))`.
///
/// # Parameters
///
/// - `frequencies`: Frequencies in Hz.
/// - `kinds`: Weighting curves to compute (librosa defaults to `"ZAC"`).
/// - `min_db`: Optional lower bound applied to every curve in dB.
///
/// # Returns
///
/// An iterator with one inner iterator per entry in `kinds`, in the same
/// order. Each inner iterator yields the weighting in dB of every frequency in
/// `frequencies`.
///
/// # References
///
/// - [librosa.multi_frequency_weighting](https://librosa.org/doc/latest/api/generated/librosa.multi_frequency_weighting.html)
pub fn multi_frequency_weighting<'a, T, I>(
    frequencies: &'a [T],
    kinds: I,
    min_db: impl Into<Db<T>>,
) -> impl Iterator<Item = impl Iterator<Item = T> + 'a> + 'a
where
    T: Float,
    I: IntoIterator<Item = WeightingKind>,
    I::IntoIter: 'a,
{
    let min_db = min_db.into().0;
    kinds.into_iter().map(move |kind| {
        frequencies
            .iter()
            .copied()
            .map(move |frequency| frequency_weighting(frequency, kind, min_db))
    })
}

/// Compute the A-weighting of a frequency, in dB.
///
/// Uses the constants `12194.217`, `20.598997`, `107.65265` and `737.86223`
/// and an offset of `2.0` dB, identical to librosa.
///
/// # Parameters
///
/// - `frequency`: Frequency in Hz.
/// - `min_db`: Optional lower bound for the result in dB. librosa defaults to
///   `-80.0`; `None` disables clipping.
///
/// # Returns
///
/// The A-weighting in dB. At `frequency == 0` the unclipped result is `-inf`.
///
/// # References
///
/// - [librosa.A_weighting](https://librosa.org/doc/latest/api/generated/librosa.A_weighting.html)
pub fn a_weighting<T>(frequency: T, min_db: impl Into<Db<T>>) -> T
where
    T: Float,
{
    let f_sq = frequency * frequency;

    let c1 = num!(12194.217_f64);
    let c2 = num!(20.598997_f64);
    let c3 = num!(107.65265_f64);
    let c4 = num!(737.86223_f64);

    let weights = num!(2.0)
        + num!(20.0)
            * ((c1 * c1).log10() + num!(2.0) * f_sq.log10()
                - (f_sq + c1 * c1).log10()
                - (f_sq + c2 * c2).log10()
                - num!(0.5) * (f_sq + c3 * c3).log10()
                - num!(0.5) * (f_sq + c4 * c4).log10());

    min_db.into().0.max(weights)
}

/// Compute the B-weighting of a frequency, in dB.
///
/// Uses the constants `12194.217`, `20.598997` and `158.48932` and an offset
/// of `0.17` dB, identical to librosa.
///
/// # Parameters
///
/// - `frequency`: Frequency in Hz.
/// - `min_db`: Optional lower bound for the result in dB. librosa defaults to
///   `-80.0`; `None` disables clipping.
///
/// # Returns
///
/// The B-weighting in dB. At `frequency == 0` the unclipped result is `-inf`.
///
/// # References
///
/// - [librosa.B_weighting](https://librosa.org/doc/latest/api/generated/librosa.B_weighting.html)
pub fn b_weighting<T>(frequency: T, min_db: impl Into<Db<T>>) -> T
where
    T: Float,
{
    let f_sq = frequency * frequency;

    let c1 = num!(12194.217_f64);
    let c2 = num!(20.598997_f64);
    let c3 = num!(158.48932_f64);

    let weights = num!(0.17)
        + num!(20.0)
            * ((c1 * c1).log10() + num!(1.5) * f_sq.log10()
                - (f_sq + c1 * c1).log10()
                - (f_sq + c2 * c2).log10()
                - num!(0.5) * (f_sq + c3 * c3).log10());

    min_db.into().0.max(weights)
}

/// Compute the C-weighting of a frequency, in dB.
///
/// Uses the constants `12194.217` and `20.598997` and an offset of `0.062` dB,
/// identical to librosa.
///
/// # Parameters
///
/// - `frequency`: Frequency in Hz.
/// - `min_db`: Optional lower bound for the result in dB. librosa defaults to
///   `-80.0`; `None` disables clipping.
///
/// # Returns
///
/// The C-weighting in dB. At `frequency == 0` the unclipped result is `-inf`.
///
/// # References
///
/// - [librosa.C_weighting](https://librosa.org/doc/latest/api/generated/librosa.C_weighting.html)
pub fn c_weighting<T>(frequency: T, min_db: impl Into<Db<T>>) -> T
where
    T: Float,
{
    let f_sq = frequency * frequency;

    let c1 = num!(12194.217_f64);
    let c2 = num!(20.598997_f64);

    let weights = num!(0.062)
        + num!(20.0)
            * ((c1 * c1).log10() + f_sq.log10()
                - (f_sq + c1 * c1).log10()
                - (f_sq + c2 * c2).log10());

    min_db.into().0.max(weights)
}

/// Compute the D-weighting of a frequency, in dB.
///
/// Uses the constants `8.3046305e-3`, `1018.7`, `1039.6`, `3136.5`, `3424.0`,
/// `282.7` and `1160.0`, identical to librosa.
///
/// # Parameters
///
/// - `frequency`: Frequency in Hz.
/// - `min_db`: Optional lower bound for the result in dB. librosa defaults to
///   `-80.0`; `None` disables clipping.
///
/// # Returns
///
/// The D-weighting in dB. At `frequency == 0` the unclipped result is `-inf`.
///
/// # References
///
/// - [librosa.D_weighting](https://librosa.org/doc/latest/api/generated/librosa.D_weighting.html)
pub fn d_weighting<T>(frequency: T, min_db: impl Into<Db<T>>) -> T
where
    T: Float,
{
    let f_sq = frequency * frequency;

    let c1 = num!(8.3046305e-3_f64);
    let c2 = num!(1018.7_f64);
    let c3 = num!(1039.6_f64);
    let c4 = num!(3136.5_f64);
    let c5 = num!(3424.0_f64);
    let c6 = num!(282.7_f64);
    let c7 = num!(1160.0_f64);

    let c1_sq = c1 * c1;
    let c2_sq = c2 * c2;
    let c3_sq = c3 * c3;
    let c4_sq = c4 * c4;
    let c5_sq = c5 * c5;
    let c6_sq = c6 * c6;
    let c7_sq = c7 * c7;

    let weights = num!(20.0)
        * (num!(0.5) * f_sq.log10() - c1_sq.log10()
            + num!(0.5)
                * (((c2_sq - f_sq) * (c2_sq - f_sq) + c3_sq * f_sq).log10()
                    - ((c4_sq - f_sq) * (c4_sq - f_sq) + c5_sq * f_sq).log10()
                    - (c6_sq + f_sq).log10()
                    - (c7_sq + f_sq).log10()));

    min_db.into().0.max(weights)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Octave-band centre frequencies in Hz.
    const FREQUENCIES: [f64; 5] = [31.5, 125.0, 500.0, 2000.0, 8000.0];
    const MIN_DB: f64 = -80.0;

    fn format_all(values: impl Iterator<Item = f64>) -> Vec<String> {
        values.map(|v| format!("{v:.3}")).collect()
    }

    #[test]
    fn test_a_weighting() {
        let result = format_all(FREQUENCIES.iter().map(|&f| a_weighting(f, MIN_DB)));
        insta::assert_debug_snapshot!(result,@r#"
        [
            "-39.525",
            "-16.188",
            "-3.247",
            "1.202",
            "-1.147",
        ]
        "#)
    }

    #[test]
    fn test_b_weighting() {
        let result = format_all(FREQUENCIES.iter().map(|&f| b_weighting(f, MIN_DB)));
        insta::assert_debug_snapshot!(result,@r#"
        [
            "-17.124",
            "-4.226",
            "-0.275",
            "-0.089",
            "-2.941",
        ]
        "#)
    }

    #[test]
    fn test_c_weighting() {
        let result = format_all(FREQUENCIES.iter().map(|&f| c_weighting(f, MIN_DB)));
        insta::assert_debug_snapshot!(result,@r#"
        [
            "-3.030",
            "-0.172",
            "0.033",
            "-0.169",
            "-3.047",
        ]
        "#)
    }

    #[test]
    fn test_d_weighting() {
        let result = format_all(FREQUENCIES.iter().map(|&f| d_weighting(f, MIN_DB)));
        insta::assert_debug_snapshot!(result,@r#"
        [
            "-16.719",
            "-5.568",
            "-0.280",
            "7.950",
            "5.463",
        ]
        "#)
    }

    #[test]
    fn test_z_weighting() {
        let flat = format_all(
            FREQUENCIES
                .iter()
                .map(|&f| frequency_weighting(f, WeightingKind::Z, MIN_DB)),
        );
        let floored = format_all(
            FREQUENCIES
                .iter()
                .map(|&f| frequency_weighting(f, WeightingKind::Z, 5.0)),
        );
        insta::assert_debug_snapshot!((flat, floored),@r#"
        (
            [
                "0.000",
                "0.000",
                "0.000",
                "0.000",
                "0.000",
            ],
            [
                "5.000",
                "5.000",
                "5.000",
                "5.000",
                "5.000",
            ],
        )
        "#)
    }

    #[test]
    fn test_weighting_min_db() {
        // With a floor, DC is clamped to it.
        let result = format_all([0.0, 31.5].iter().map(|&f| a_weighting(f, MIN_DB)));
        insta::assert_debug_snapshot!(result,@r#"
        [
            "-80.000",
            "-39.525",
        ]
        "#)
    }

    #[test]
    fn test_frequency_weighting_dispatch() {
        for &f in &FREQUENCIES {
            assert_eq!(
                frequency_weighting(f, WeightingKind::A, MIN_DB),
                a_weighting(f, MIN_DB)
            );
            assert_eq!(
                frequency_weighting(f, WeightingKind::B, MIN_DB),
                b_weighting(f, MIN_DB)
            );
            assert_eq!(
                frequency_weighting(f, WeightingKind::C, MIN_DB),
                c_weighting(f, MIN_DB)
            );
            assert_eq!(
                frequency_weighting(f, WeightingKind::D, MIN_DB),
                d_weighting(f, MIN_DB)
            );
        }
    }

    #[test]
    fn test_multi_frequency_weighting() {
        let result: Vec<Vec<String>> = multi_frequency_weighting(
            &FREQUENCIES,
            [WeightingKind::Z, WeightingKind::A, WeightingKind::C],
            MIN_DB,
        )
        .map(format_all)
        .collect();
        insta::assert_debug_snapshot!(result,@r#"
        [
            [
                "0.000",
                "0.000",
                "0.000",
                "0.000",
                "0.000",
            ],
            [
                "-39.525",
                "-16.188",
                "-3.247",
                "1.202",
                "-1.147",
            ],
            [
                "-3.030",
                "-0.172",
                "0.033",
                "-0.169",
                "-3.047",
            ],
        ]
        "#)
    }

    #[test]
    fn test_perceptual_weighting() {
        let (power, frequency, reference, amin, top_db) = (0.5_f64, 1000.0, 1.0, 1e-10, 80.0);
        let result = perceptual_weighting(
            power,
            frequency,
            reference,
            amin,
            top_db,
            WeightingKind::A,
            MIN_DB,
        );
        let expected =
            a_weighting(frequency, MIN_DB) + Power(power).to_db(reference, amin, top_db).0;
        assert_eq!(result, expected);
    }
}
