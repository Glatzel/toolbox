mod a4;
mod amplitude;
mod db;
mod frequency;
mod mel;
pub mod notation;
mod octs;
mod phase;
mod power;
mod signal;
mod spectrogram;
mod spectrum;

mod tuning;

pub use a4::A4;
pub use amplitude::Amplitude;
pub use db::Db;
pub use frequency::{Frequency, Hz};
pub use mel::Mel;
pub use octs::Octs;
pub use phase::Phase;
pub use power::Power;
pub use signal::{ISignal, Signal, SignalRef, SignalRefMut};
pub use spectrogram::Spectrogram;
pub use spectrum::{
    Scratch, ScratchRef, ScratchRefMut, Spectrum1D, Spectrum2D, Spectrum1DRef, Spectrum1DRefMut,
};
pub use tuning::Tuning;

macro_rules! simple_structure {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $name<T>(pub T)
        where
            T: num_traits::Float;

        impl<T> From<T> for $name<T>
        where
            T: num_traits::Float,
        {
            fn from(value: T) -> Self { $name(value) }
        }
    };
}
use simple_structure;
