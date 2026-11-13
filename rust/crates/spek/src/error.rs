use mischief::IDiagnosis;

#[derive(Debug, IDiagnosis)]
#[diagnosis(error, display)]
pub enum SpekError {
    // pad
    #[diagnosis(description = "signal is empty")]
    EmptySignal,
    #[diagnosis(
        description = "signal size is too small, expected at least {min_size}, got {actual}"
    )]
    SignalSizeTooSmall { min_size: usize, actual: usize },

    //window
    #[diagnosis(description = "Tau must be positive")]
    ExponentialTau,
    #[diagnosis(
        description = "Kaiser-Bessel Derived windows are only defined for even number of points"
    )]
    KaiserBesselDerivedSize,

    //stft
    #[diagnosis(description = "{name} size not correct, got {size} ({reason})")]
    InvalidSize {
        name: &'static str,
        size: usize,
        reason: &'static str,
    },
    #[diagnosis(
        description = "size not correct, {name_a} got {size_a} and {name_b} got {size_b} ({reason})"
    )]
    Invalid2Size {
        name_a: &'static str,
        name_b: &'static str,
        size_a: usize,
        size_b: usize,
        reason: &'static str,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display_is_unchanged() {
        assert_eq!(
            SpekError::SignalSizeTooSmall {
                min_size: 16,
                actual: 8,
            }
            .to_string()
            .as_str(),
            "signal size is too small, expected at least 16, got 8"
        );
        assert_eq!(
            SpekError::InvalidSize {
                name: "window",
                size: 3,
                reason: "must be even",
            }
            .to_string()
            .as_str(),
            "window size not correct, got 3 (must be even)"
        );
        assert_eq!(
            SpekError::Invalid2Size {
                name_a: "window",
                name_b: "hop",
                size_a: 3,
                size_b: 4,
                reason: "invalid ratio",
            }
            .to_string()
            .as_str(),
            "size not correct, window got 3 and hop got 4 (invalid ratio)"
        );
    }
}
