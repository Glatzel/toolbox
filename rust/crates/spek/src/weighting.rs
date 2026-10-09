use generic_num::num;
use num_traits::Float;

use crate::spectrum::power_to_db;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeightingKind {
    A,
    B,
    C,
    D,
    Z,
}

pub fn perceptual_weighting<T>(
    power: T,
    frequency: T,
    reference: T,
    amin: T,
    top_db: T,
    kind: WeightingKind,
    min_db: Option<T>,
) -> T
where
    T: Float,
{
    frequency_weighting(frequency, kind, min_db) + power_to_db(power, reference, amin, top_db)
}

pub fn frequency_weighting<T>(frequency: T, kind: WeightingKind, min_db: Option<T>) -> T
where
    T: Float,
{
    match kind {
        WeightingKind::A => a_weighting(frequency, min_db),
        WeightingKind::B => b_weighting(frequency, min_db),
        WeightingKind::C => c_weighting(frequency, min_db),
        WeightingKind::D => d_weighting(frequency, min_db),
        WeightingKind::Z => min_db.map_or(num!(0.0), |min_db| min_db.max(num!(0.0))),
    }
}

pub fn multi_frequency_weighting<'a, T, I>(
    frequencies: &'a [T],
    kinds: I,
    min_db: Option<T>,
) -> impl Iterator<Item = impl Iterator<Item = T> + 'a> + 'a
where
    T: Float,
    I: IntoIterator<Item = WeightingKind>,
    I::IntoIter: 'a,
{
    kinds.into_iter().map(move |kind| {
        frequencies
            .iter()
            .copied()
            .map(move |frequency| frequency_weighting(frequency, kind, min_db))
    })
}

pub fn a_weighting<T>(frequency: T, min_db: Option<T>) -> T
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

    match min_db {
        Some(min_db) => min_db.max(weights),
        None => weights,
    }
}

pub fn b_weighting<T>(frequency: T, min_db: Option<T>) -> T
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

    match min_db {
        Some(min_db) => min_db.max(weights),
        None => weights,
    }
}

pub fn c_weighting<T>(frequency: T, min_db: Option<T>) -> T
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

    match min_db {
        Some(min_db) => min_db.max(weights),
        None => weights,
    }
}

pub fn d_weighting<T>(frequency: T, min_db: Option<T>) -> T
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

    match min_db {
        Some(min_db) => min_db.max(weights),
        None => weights,
    }
}
