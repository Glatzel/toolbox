use generic_num::num;
use num_traits::Float;

pub fn spectrum_to_power<T>(real: T, imag: T) -> T
where
    T: Float,
{
    real * real + imag * imag
}

pub fn spectrum_to_amplitude<T>(real: T, imag: T) -> T
where
    T: Float,
{
    real.hypot(imag)
}

pub fn spectrum_to_db<T>(real: T, imag: T, reference: T, amin: T, top_db: T) -> T
where
    T: Float,
{
    let power = spectrum_to_power(real, imag);
    power_to_db(power, reference, amin, top_db)
}

pub fn magnitude_to_amplitude<T>(magnitude: T) -> T
where
    T: Float,
{
    magnitude.powi(2)
}

pub fn amplitude_to_db<T>(amplitude: T, reference: T, amin: T, top_db: T) -> T
where
    T: Float,
{
    let amplitude = amplitude.max(amin);
    num!(20.0) * (amplitude / reference).log10() - top_db
}
pub fn db_to_amplitude<T>(db: T, reference: T) -> T
where
    T: Float,
{
    reference * num!(10.0).powf(db / num!(20))
}
pub fn power_to_db<T>(power: T, reference: T, amin: T, top_db: T) -> T
where
    T: Float,
{
    let power = power.max(amin);
    num!(10.0) * (power / reference).log10() - top_db
}
pub fn db_to_power<T>(db: T, reference: T) -> T
where
    T: Float,
{
    reference * num!(10.0).powf(db / num!(10))
}
