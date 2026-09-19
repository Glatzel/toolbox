extern crate alloc;
use alloc::vec::Vec;

use num_complex::Complex;
use num_traits::{Float, FloatConst};

use crate::SpekError;
use crate::pad::IPad;
use crate::windows::Window;

pub struct Stft<T, PAD>
where
    T: Float + FloatConst,
    PAD: IPad<T>,
{
    fft_size: usize,
    hop_size: usize,
    win_size: usize,
    window: Vec<T>,
    center: bool,
    pad_mode: PAD,
}
impl<T, PAD> Stft<T, PAD>
where
    T: Float + FloatConst,
    PAD: IPad<T>,
{
    pub fn new(
        fft_size: usize,
        hop_size: usize,
        win_size: usize,
        window: Window<T>,
        center: bool,
        pad_mode: PAD,
    ) -> Result<Self, SpekError> {
        Ok(Self {
            fft_size,
            hop_size,
            win_size,
            window: window.window(win_size, false)?,
            center,
            pad_mode,
        })
    }
}
