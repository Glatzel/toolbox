mod a4;
mod amplitude;
mod db;
mod frequency;
mod mel;
mod midi;
mod octs;
mod phase;
mod power;
mod spectrogram;
mod spectrum2d;
mod tuning;

pub use a4::A4;
pub use amplitude::Amplitude;
pub use db::Db;
pub use frequency::Frequency;
pub use mel::Mel;
pub use midi::Midi;
pub use octs::Octs;
pub use phase::Phase;
pub use power::Power;
pub use spectrogram::Spectrogram;
pub use spectrum2d::Spectrum2D;
pub use tuning::Tuning;

macro_rules! simple_structure_from_t {
    ($name:ident) => {
        impl<T> From<T> for $name<T>
        where
            T: num_traits::Float,
        {
            fn from(value: T) -> Self { $name(value) }
        }
    };
}
use simple_structure_from_t;
