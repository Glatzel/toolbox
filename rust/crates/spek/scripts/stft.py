import numpy as np
from scipy.fft import rfft
from scipy.signal.windows import hann

signal = np.arange(
    49,
    dtype=np.float32,
)

hop_size = 4
win_size = 7
fft_size = 8

window = hann(win_size, False)

print("window:")
print(window)

spectrogram = []

for start in range(0, len(signal) - win_size + 1, hop_size):
    # Same as Rust:
    # signal[start:start + win_size] * window
    frame = signal[start : start + win_size] * window

    # Zero-pad win_size -> fft_size
    frame = np.pad(
        frame,
        (0, fft_size - win_size),
        mode="constant",
    )

    print(f"\nframe {start // hop_size}:")
    print(frame)

    # Same one-sided real FFT as Rust
    spectrum = rfft(frame)

    print("spectrum:")
    print(spectrum)

    spectrogram.append(spectrum)

spectrogram = np.asarray(spectrogram)

print("\nspectrogram:")
print(spectrogram)

print("\nshape:")
print(spectrogram.shape)
