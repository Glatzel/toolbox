import numpy as np
from scipy.fft import irfft, rfft


def generate(signal: np.ndarray) -> None:
    n_fft = len(signal)
    spectrum = rfft(signal, n=n_fft)  # complex, length n_fft // 2 + 1
    recovered = irfft(spectrum, n=n_fft)  # real, length n_fft

    expected_spectrum_len = n_fft // 2 + 1
    assert spectrum.shape[0] == expected_spectrum_len, (
        f"spectrum length {spectrum.shape[0]} != N/2+1 ({expected_spectrum_len})"
    )

    print(f"n_fft={n_fft}")
    print("signal:        ", np.array2string(signal, precision=6))
    print("spectrum (re): ", np.array2string(spectrum.real, precision=6))
    print("spectrum (im): ", np.array2string(spectrum.imag, precision=6))
    print("recovered:     ", np.array2string(recovered, precision=6))

    max_err = float(np.max(np.abs(recovered - signal)))
    print(f"max |recovered - signal| = {max_err:.3e}")


if __name__ == "__main__":
    signal = np.array(
        [
            0.2709815,
            0.14039207,
            0.9799018,
            0.5933856,
        ],
        dtype=np.float32,
    )
    generate(signal)
