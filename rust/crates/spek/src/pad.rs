extern crate alloc;

use alloc::vec::Vec;
use core::iter::Sum;

use num_traits::{Float, FloatConst};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PadError {
    #[error("signal is empty")]
    EmptySignal,
    #[error("signal size is too small, expected at least {min_size}, got {actual}")]
    SignalSizeTooSmall { min_size: usize, actual: usize },
}

pub trait IPad<T>
where
    T: Float + FloatConst,
{
    fn pad(&self, signal: &[T], pad_before: usize, pad_after: usize) -> Result<Vec<T>, PadError>;
}

pub enum PadMode<T> {
    Constant(T),
    Edge,
    LinearRamp,
    Maximum,
    Mean,
    Median,
    Minimum,
    Reflect,
    Symmetric,
    Wrap,
    Empty,
}

impl<T> IPad<T> for PadMode<T>
where
    T: Float + FloatConst + Sum,
{
    fn pad(&self, signal: &[T], pad_before: usize, pad_after: usize) -> Result<Vec<T>, PadError> {
        if signal.is_empty() {
            return Err(PadError::EmptySignal);
        }

        let mut padded = Vec::with_capacity(signal.len() + pad_before + pad_after);

        match self {
            Self::Constant(value) => {
                padded.extend(core::iter::repeat_n(*value, pad_before));
                padded.extend_from_slice(signal);
                padded.extend(core::iter::repeat_n(*value, pad_after));
            }

            Self::Empty => {
                padded.extend(core::iter::repeat_n(T::zero(), pad_before));
                padded.extend_from_slice(signal);
                padded.extend(core::iter::repeat_n(T::zero(), pad_after));
            }

            Self::Edge => {
                padded.extend(core::iter::repeat_n(signal[0], pad_before));

                padded.extend_from_slice(signal);

                padded.extend(core::iter::repeat_n(*signal.last().unwrap(), pad_after));
            }

            Self::Reflect => {
                for i in (1..=pad_before).rev() {
                    if i >= signal.len() {
                        return Err(PadError::SignalSizeTooSmall {
                            min_size: pad_before,
                            actual: signal.len(),
                        });
                    }
                    padded.push(signal[i]);
                }

                padded.extend_from_slice(signal);

                for i in 0..pad_after {
                    if i + 1 >= signal.len() {
                        return Err(PadError::SignalSizeTooSmall {
                            min_size: pad_after,
                            actual: signal.len(),
                        });
                    }
                    padded.push(signal[signal.len() - 2 - i]);
                }
            }

            Self::Symmetric => {
                for i in (0..pad_before).rev() {
                    let idx = i.min(signal.len() - 1);
                    padded.push(signal[idx]);
                }

                padded.extend_from_slice(signal);

                for i in 0..pad_after {
                    let idx = (signal.len() - 1).saturating_sub(i);
                    padded.push(signal[idx]);
                }
            }

            Self::Wrap => {
                // Index j (0-based, counting backwards from the start of `signal`)
                // maps to `signal[(j - pad_before) mod len]`. Using `rem_euclid`
                // on a signed offset avoids the unsigned-modulo edge case where
                // `(pad_before - i) % len == 0`, which previously produced an
                // out-of-bounds index of `len` instead of `0`.
                let len = signal.len() as isize;
                for i in 0..pad_before {
                    let offset = i as isize - pad_before as isize;
                    let idx = offset.rem_euclid(len) as usize;
                    padded.push(signal[idx]);
                }

                padded.extend_from_slice(signal);

                for i in 0..pad_after {
                    padded.push(signal[i % signal.len()]);
                }
            }

            Self::LinearRamp => {
                let first = signal[0];
                let last = *signal.last().unwrap();

                for i in 0..pad_before {
                    let alpha = T::from(i + 1).unwrap() / T::from(pad_before + 1).unwrap();

                    padded.push(first * (T::one() - alpha));
                }

                padded.extend_from_slice(signal);

                for i in 0..pad_after {
                    let alpha = T::from(i + 1).unwrap() / T::from(pad_after + 1).unwrap();

                    padded.push(last * (T::one() - alpha));
                }
            }

            Self::Maximum | Self::Minimum | Self::Mean | Self::Median => {
                let value = match self {
                    Self::Maximum => signal.iter().copied().fold(T::neg_infinity(), T::max),

                    Self::Minimum => signal.iter().copied().fold(T::infinity(), T::min),

                    Self::Mean => {
                        signal.iter().copied().sum::<T>() / T::from(signal.len()).unwrap()
                    }

                    Self::Median => {
                        // Only the middle element is needed, so a full O(n log n)
                        // sort is wasted work. `select_nth_unstable_by` partitions
                        // around the median index in O(n) average time and gives
                        // the identical result since we only ever read one slot.
                        let mut values = signal.to_vec();
                        let mid = values.len() / 2;
                        let (_, median, _) = values
                            .select_nth_unstable_by(mid, |a, b| a.partial_cmp(b).unwrap());
                        *median
                    }

                    _ => unreachable!(),
                };

                padded.extend(core::iter::repeat_n(value, pad_before));

                padded.extend_from_slice(signal);

                padded.extend(core::iter::repeat_n(value, pad_after));
            }
        }

        Ok(padded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: &[f64], b: &[f64]) {
        assert_eq!(a.len(), b.len(), "length mismatch: {:?} vs {:?}", a, b);
        for (x, y) in a.iter().zip(b.iter()) {
            assert!(
                (x - y).abs() < 1e-9,
                "values differ: {:?} vs {:?}",
                a,
                b
            );
        }
    }

    #[test]
    fn empty_signal_errors() {
        let signal: [f64; 0] = [];
        let err = PadMode::Edge.pad(&signal, 1, 1).unwrap_err();
        assert!(matches!(err, PadError::EmptySignal));
    }

    #[test]
    fn constant() {
        let signal = [1.0, 2.0, 3.0];
        let result = PadMode::Constant(9.0).pad(&signal, 2, 1).unwrap();
        assert_close(&result, &[9.0, 9.0, 1.0, 2.0, 3.0, 9.0]);
    }

    #[test]
    fn empty_mode_pads_with_zero() {
        let signal = [1.0, 2.0, 3.0];
        let result = PadMode::Empty.pad(&signal, 2, 1).unwrap();
        assert_close(&result, &[0.0, 0.0, 1.0, 2.0, 3.0, 0.0]);
    }

    #[test]
    fn edge() {
        let signal = [1.0, 2.0, 3.0];
        let result = PadMode::Edge.pad(&signal, 2, 3).unwrap();
        assert_close(&result, &[1.0, 1.0, 1.0, 2.0, 3.0, 3.0, 3.0, 3.0]);
    }

    #[test]
    fn maximum() {
        let signal = [3.0, 1.0, 4.0, 1.0, 5.0];
        let result = PadMode::Maximum.pad(&signal, 2, 2).unwrap();
        assert_close(&result, &[5.0, 5.0, 3.0, 1.0, 4.0, 1.0, 5.0, 5.0, 5.0]);
    }

    #[test]
    fn minimum() {
        let signal = [3.0, 1.0, 4.0, 1.0, 5.0];
        let result = PadMode::Minimum.pad(&signal, 1, 1).unwrap();
        assert_close(&result, &[1.0, 3.0, 1.0, 4.0, 1.0, 5.0, 1.0]);
    }

    #[test]
    fn mean() {
        let signal = [1.0, 2.0, 3.0, 4.0];
        // mean = 2.5
        let result = PadMode::Mean.pad(&signal, 1, 1).unwrap();
        assert_close(&result, &[2.5, 1.0, 2.0, 3.0, 4.0, 2.5]);
    }

    #[test]
    fn median_odd_length() {
        let signal = [5.0, 1.0, 3.0];
        // sorted: [1, 3, 5] -> median = 3
        let result = PadMode::Median.pad(&signal, 1, 0).unwrap();
        assert_close(&result, &[3.0, 5.0, 1.0, 3.0]);
    }

    #[test]
    fn median_even_length_takes_upper_middle() {
        let signal = [4.0, 1.0, 3.0, 2.0];
        // sorted: [1, 2, 3, 4] -> index len/2 == 2 -> 3
        let result = PadMode::Median.pad(&signal, 0, 1).unwrap();
        assert_close(&result, &[4.0, 1.0, 3.0, 2.0, 3.0]);
    }

    #[test]
    fn linear_ramp() {
        let signal = [4.0];
        let result = PadMode::LinearRamp.pad(&signal, 4, 0).unwrap();
        // ramps from 0 towards 4.0 over 4 steps: 4*(1-1/5), 4*(1-2/5), 4*(1-3/5), 4*(1-4/5)
        assert_close(&result, &[3.2, 2.4, 1.6, 0.8, 4.0]);
    }

    #[test]
    fn reflect() {
        let signal = [1.0, 2.0, 3.0, 4.0];
        let result = PadMode::Reflect.pad(&signal, 2, 2).unwrap();
        assert_close(&result, &[3.0, 2.0, 1.0, 2.0, 3.0, 4.0, 3.0, 2.0]);
    }

    #[test]
    fn reflect_too_large_pad_before_errors() {
        let signal = [1.0, 2.0];
        let err = PadMode::Reflect.pad(&signal, 5, 0).unwrap_err();
        assert!(matches!(err, PadError::SignalSizeTooSmall { .. }));
    }

    #[test]
    fn reflect_too_large_pad_after_errors() {
        let signal = [1.0, 2.0];
        let err = PadMode::Reflect.pad(&signal, 0, 5).unwrap_err();
        assert!(matches!(err, PadError::SignalSizeTooSmall { .. }));
    }

    #[test]
    fn symmetric() {
        let signal = [1.0, 2.0, 3.0];
        let result = PadMode::Symmetric.pad(&signal, 2, 2).unwrap();
        assert_close(&result, &[2.0, 1.0, 1.0, 2.0, 3.0, 3.0, 2.0]);
    }

    #[test]
    fn wrap_basic() {
        let signal = [1.0, 2.0, 3.0];
        let result = PadMode::Wrap.pad(&signal, 2, 2).unwrap();
        // before: last 2 elements [2, 3], after: first 2 elements [1, 2]
        assert_close(&result, &[2.0, 3.0, 1.0, 2.0, 3.0, 1.0, 2.0]);
    }

    /// Regression test for the out-of-bounds panic that occurred whenever
    /// `(pad_before - i) % signal.len() == 0` for some `i` in `0..pad_before`
    /// (e.g. pad_before=5, len=3, at i=2). Sweeping a range of sizes ensures
    /// every residue class of the modulus is exercised at least once.
    #[test]
    fn wrap_does_not_panic_across_size_combinations() {
        for len in 1..=6usize {
            let signal: Vec<f64> = (0..len).map(|v| v as f64).collect();
            for pad_before in 0..=8usize {
                for pad_after in 0..=8usize {
                    let result = PadMode::Wrap.pad(&signal, pad_before, pad_after).unwrap();
                    assert_eq!(result.len(), len + pad_before + pad_after);
                }
            }
        }
    }

    #[test]
    fn wrap_previously_panicking_case() {
        // pad_before=5, len=3: at i=2, (5-2) % 3 == 0, which used to compute
        // idx = len (3) instead of 0 and panic on out-of-bounds indexing.
        let signal = [10.0, 20.0, 30.0];
        let result = PadMode::Wrap.pad(&signal, 5, 0).unwrap();
        assert_close(&result, &[20.0, 30.0, 10.0, 20.0, 30.0, 10.0, 20.0, 30.0]);
    }

    #[test]
    fn wrap_pad_larger_than_signal() {
        let signal = [1.0, 2.0];
        let result = PadMode::Wrap.pad(&signal, 5, 3).unwrap();
        assert_close(
            &result,
            &[2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0],
        );
    }
}