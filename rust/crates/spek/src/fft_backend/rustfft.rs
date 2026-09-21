extern crate alloc;
use alloc::sync::Arc;

use generic_num::num;
use num_complex::Complex;
use num_traits::Float;
use rustfft::{Fft, FftNum};

use crate::Dtype;
use crate::fft_backend::{FftError, IFftBackend};
use crate::stft::StftResult;

pub struct RustfftBackend<T>
where
    T: FftNum,
{
    forward_planner: Arc<dyn Fft<T>>,
    inverse_planner: Arc<dyn Fft<T>>,
    phantom: core::marker::PhantomData<T>,
}
impl<T> RustfftBackend<T>
where
    T: FftNum,
{
    pub fn new(
        forward_planner: Arc<dyn Fft<T>>,
        inverse_planner: Arc<dyn Fft<T>>,
    ) -> Result<Self, FftError> {
        if forward_planner.len() != inverse_planner.len() {
            return Err(FftError::SizeNotEqual {
                name: "planner",
                a: forward_planner.len(),
                b: inverse_planner.len(),
            });
        }

        Ok(Self {
            forward_planner,
            inverse_planner,
            phantom: core::marker::PhantomData,
        })
    }
}
impl<T> IFftBackend<T> for RustfftBackend<T>
where
    T: FftNum + Float,
{
    fn fft(&self, signal: &mut [T], spectrum: &mut [Dtype<T>], scratch: &mut [Dtype<T>]) {
        for (dst, &src) in spectrum.iter_mut().zip(signal.iter()) {
            *dst = Complex::new(src, T::zero());
        }
        self.forward_planner.process_with_scratch(spectrum, scratch);
    }

    fn ifft(&self, spectrum: &mut [Dtype<T>], signal: &mut [T], scratch: &mut [Dtype<T>]) {
        self.inverse_planner.process_with_scratch(spectrum, scratch);
        let n = num!(self.fft_size());

        for (dst, src) in signal.iter_mut().zip(spectrum.iter()) {
            *dst = src.re / n;
        }
    }

    fn spectrum_size(&self) -> usize { self.fft_size() }

    fn forward_scratch_size(&self) -> usize { self.forward_planner.get_inplace_scratch_len() }

    fn inverse_scratch_size(&self) -> usize { self.inverse_planner.get_inplace_scratch_len() }

    fn new_spectrum(&self) -> Vec<Dtype<T>> {
        vec![
            Complex {
                re: T::zero(),
                im: T::zero()
            };
            self.spectrum_size()
        ]
    }

    fn new_forward_scratch(&self) -> Vec<Dtype<T>> {
        vec![
            Complex {
                re: T::zero(),
                im: T::zero()
            };
            self.forward_scratch_size()
        ]
    }

    fn new_inverse_scratch(&self) -> Vec<Dtype<T>> {
        vec![
            Complex {
                re: T::zero(),
                im: T::zero()
            };
            self.inverse_scratch_size()
        ]
    }
    fn new_stft_result_buffer(&self, frame_count: usize) -> StftResult<T> {
        StftResult::new(frame_count, self.spectrum_size())
    }

    fn fft_size(&self) -> usize { self.forward_planner.len() }

    fn signal_size(&self) -> usize { self.fft_size() }

    fn new_signal(&self) -> Vec<T> { vec![T::zero(); self.signal_size()] }
}
#[cfg(test)]
mod tests {

    use super::*;
    use crate::test_fft_backend;

    test_fft_backend!(
        test_rustfft_backend_fft4,
        f32,
        RustfftBackend<f32>,
        RustfftBackend::new(
            rustfft::FftPlanner::new().plan_fft_forward(4),
            rustfft::FftPlanner::new().plan_fft_inverse(4)
        )
        .unwrap()
    );
    test_fft_backend!(
        test_rustfft_backend_fft8,
        f32,
        RustfftBackend<f32>,
        RustfftBackend::new(
            rustfft::FftPlanner::new().plan_fft_forward(8),
            rustfft::FftPlanner::new().plan_fft_inverse(8)
        )
        .unwrap()
    );
    test_fft_backend!(
        test_rustfft_backend_fft7,
        f32,
        RustfftBackend<f32>,
        RustfftBackend::new(
            rustfft::FftPlanner::new().plan_fft_forward(7),
            rustfft::FftPlanner::new().plan_fft_inverse(7)
        )
        .unwrap()
    );
}
