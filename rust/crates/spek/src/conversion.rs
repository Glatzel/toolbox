use num_traits::Float;

pub fn spectrum_to_magnitude<T>(real: T, imag: T) -> T
where
    T: Float,
{
    real.hypot(imag)
}

pub fn spectrum_to_amplitude<T>(real: T, imag: T, scale: T) -> T
where
    T: Float,
{
    scale * spectrum_to_magnitude(real, imag)
}

pub fn spectrum_to_db<T>(real: T, imag: T, reference: T) -> T
where
    T: Float,
{
    let magnitude = real.hypot(imag);

    if magnitude.is_zero() {
        T::neg_infinity()
    } else {
        T::from(20.0).unwrap() * (magnitude / reference).log10()
    }
}
pub fn magnitude_to_amplitude<T>(magnitude: T, scale: T) -> T
where
    T: Float,
{
    magnitude * scale
}

pub fn amplitude_to_db<T>(amplitude: T, reference: T) -> T
where
    T: Float,
{
    if amplitude.is_zero() {
        T::neg_infinity()
    } else {
        T::from(20.0).unwrap() * (amplitude / reference).log10()
    }
}
