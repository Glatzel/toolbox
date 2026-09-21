extern crate alloc;
use alloc::sync::Arc;

use generic_num::num;
use num_complex::Complex;
use num_traits::{Float, FloatConst};
use realfft::{ComplexToReal, FftNum, RealFftPlanner, RealToComplex};

use crate::Dtype;
use crate::fft_backend::IFftBackend;
use crate::stft::StftResult;

pub struct RealfftBackend<T>
where
    T: FftNum,
{
    forward_planner: Arc<dyn RealToComplex<T>>,
    inverse_planner: Arc<dyn ComplexToReal<T>>,
}

impl<T> RealfftBackend<T>
where
    T: FftNum,
{
    pub fn new(fft_size: usize) -> Self {
        let mut planner = RealFftPlanner::new();
        Self {
            forward_planner: planner.plan_fft_forward(fft_size),
            inverse_planner: planner.plan_fft_inverse(fft_size),
        }
    }
}

impl<T> IFftBackend<T> for RealfftBackend<T>
where
    T: FftNum + FloatConst + Float,
{
    fn signal_size(&self) -> usize { self.fft_size() }

    fn spectrum_size(&self) -> usize { self.fft_size() / 2 + 1 }

    fn forward_scratch_size(&self) -> usize { self.forward_planner.get_scratch_len() }

    fn inverse_scratch_size(&self) -> usize { self.inverse_planner.get_scratch_len() }

    fn fft(&self, signal: &mut [T], spectrum: &mut [Dtype<T>], scratch: &mut [Dtype<T>]) {
        self.forward_planner
            .process_with_scratch(signal, spectrum, scratch)
            .unwrap();
    }

    fn ifft(&self, spectrum: &mut [Dtype<T>], signal: &mut [T], scratch: &mut [Dtype<T>]) {
        self.inverse_planner
            .process_with_scratch(spectrum, signal, scratch)
            .unwrap();
        let n = num!(self.fft_size());
        for s in signal.iter_mut() {
            *s = *s / n;
        }
    }

    fn new_spectrum(&self) -> Vec<Dtype<T>> {
        vec![
            Complex {
                re: T::zero(),
                im: T::zero()
            };
            self.spectrum_size()
        ]
    }

    fn new_forward_scratch(&self) -> Vec<Dtype<T>> { self.forward_planner.make_scratch_vec() }

    fn new_inverse_scratch(&self) -> Vec<Dtype<T>> { self.inverse_planner.make_scratch_vec() }

    fn new_stft_result_buffer(&self, frame_count: usize) -> StftResult<T> {
        StftResult::new(frame_count, self.spectrum_size())
    }

    fn fft_size(&self) -> usize { self.forward_planner.len() }
    fn new_signal(&self) -> Vec<T> { vec![T::zero(); self.signal_size()] }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_fft_backend;
    test_fft_backend!(
        test_realfft_backend_fft4,
        f32,
        RealfftBackend<f32>,
        RealfftBackend::new(4)
    );
    test_fft_backend!(
        test_realfft_backend_fft8,
        f32,
        RealfftBackend<f32>,
        RealfftBackend::new(8)
    );
    test_fft_backend!(
        test_realfft_backend_fft7,
        f32,
        RealfftBackend<f32>,
        RealfftBackend::new(7)
    );
}
