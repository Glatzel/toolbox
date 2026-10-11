import numpy as np
from scipy.fft import irfft, rfft


def generate(signal: np.ndarray) -> None:
    fft_size = len(signal)
    spectrum = rfft(signal, n=fft_size)  # complex, length fft_size // 2 + 1
    recovered = irfft(spectrum, n=fft_size)  # real, length fft_size

    expected_spectrum_len = fft_size // 2 + 1
    assert spectrum.shape[0] == expected_spectrum_len, (
        f"spectrum length {spectrum.shape[0]} != N/2+1 ({expected_spectrum_len})"
    )

    print(f"fft_size={fft_size}")
    print("signal:        ", np.array2string(signal, precision=6))
    print("spectrum (re): ", np.array2string(spectrum.real, precision=6))
    print("spectrum (im): ", np.array2string(spectrum.imag, precision=6))
    print("recovered:     ", np.array2string(recovered, precision=6))

    max_err = float(np.max(np.abs(recovered - signal)))
    print(f"max |recovered - signal| = {max_err:.3e}")


if __name__ == "__main__":
    signal = np.arange(
        4,
        dtype=np.float32,
    )
    generate(signal)
