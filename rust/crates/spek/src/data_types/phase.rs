use num_traits::Float;


use crate::data_types::Power;
pub struct Phase<T> {
    pub re: T,
    pub im: T,
}
impl<T> Phase<T> {
    pub fn from_spectrum(real: T, imag: T, power: Power<T>) -> (T, T)
    where
        T: Float,
    {
        let phase_real = real / power.0;
        let phase_imag = imag / power.0;
        (phase_real, phase_imag)
    }
}
