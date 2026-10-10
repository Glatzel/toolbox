use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpekError {
    // pad
    #[error("signal is empty")]
    EmptySignal,
    #[error("signal size is too small, expected at least {min_size}, got {actual}")]
    SignalSizeTooSmall { min_size: usize, actual: usize },

    //window
    #[error("Tau must be positive")]
    ExponentialTau,
    #[error("Kaiser-Bessel derived asymmetric window must be symmetric")]
    KaiserBesselDerivedAsymmetric,
    #[error("Kaiser-Bessel Derived windows are only defined for even number of points")]
    KaiserBesselDerivedSize,

    //stft
    #[error("{name} size not correct, got {size} ({reason})")]
    InvalidSize {
        name: &'static str,
        size: usize,
        reason: &'static str,
    },
    #[error("size not correct, {name_a} got {size_a} and {name_b} got {size_b} ({reason})")]
    Invalid2Size {
        name_a: &'static str,
        name_b: &'static str,
        size_a: usize,
        size_b: usize,
        reason: &'static str,
    },
}
