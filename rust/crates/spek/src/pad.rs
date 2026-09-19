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
            PadMode::Constant(_) | PadMode::Empty => {
                let value = match self {
                    PadMode::Constant(v) => *v,
                    _ => T::zero(),
                };

                padded.extend(core::iter::repeat(value).take(pad_before));
                padded.extend_from_slice(signal);
                padded.extend(core::iter::repeat(value).take(pad_after));
            }

            PadMode::Edge => {
                padded.extend(core::iter::repeat(signal[0]).take(pad_before));

                padded.extend_from_slice(signal);

                padded.extend(core::iter::repeat(*signal.last().unwrap()).take(pad_after));
            }

            PadMode::Reflect => {
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

            PadMode::Symmetric => {
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

            PadMode::Wrap => {
                for i in 0..pad_before {
                    let idx = signal.len() - (pad_before - i) % signal.len();
                    padded.push(signal[idx]);
                }

                padded.extend_from_slice(signal);

                for i in 0..pad_after {
                    padded.push(signal[i % signal.len()]);
                }
            }

            PadMode::LinearRamp => {
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

            PadMode::Maximum | PadMode::Minimum | PadMode::Mean | PadMode::Median => {
                let value = match self {
                    PadMode::Maximum => signal.iter().copied().fold(T::neg_infinity(), T::max),

                    PadMode::Minimum => signal.iter().copied().fold(T::infinity(), T::min),

                    PadMode::Mean => {
                        signal.iter().copied().sum::<T>() / T::from(signal.len()).unwrap()
                    }

                    PadMode::Median => {
                        let mut values = signal.to_vec();
                        values.sort_by(|a, b| a.partial_cmp(b).unwrap());

                        values[values.len() / 2]
                    }

                    _ => unreachable!(),
                };

                padded.extend(core::iter::repeat(value).take(pad_before));

                padded.extend_from_slice(signal);

                padded.extend(core::iter::repeat(value).take(pad_after));
            }
        }

        Ok(padded)
    }
}
