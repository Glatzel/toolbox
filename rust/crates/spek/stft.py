import numpy as np
from scipy.signal.windows import hann
from scipy.fft import rfft


signal = np.array(
    [
        0.2709815,
        0.14039207,
        0.9799018,
        0.5933856,
        0.61059517,
        0.94498706,
        0.9437634,
        0.017621636,
        0.1433295,
        0.14989191,
        0.3500976,
        0.3432361,
        0.4915536,
        0.3338886,
        0.4633286,
        0.2339502,
        0.5905582,
        0.58521235,
        0.92837656,
        0.4617837,
        0.04721266,
        0.40581423,
        0.54787743,
        0.73895377,
        0.24903673,
        0.680407,
        0.38292885,
        0.1413948,
        0.12494874,
        0.26719624,
        0.04359156,
        0.90439653,
        0.34061247,
        0.7874479,
        0.7853363,
        0.31355858,
        0.06391233,
        0.725452,
        0.060834944,
        0.9306435,
        0.40191567,
        0.6267639,
        0.136805,
        0.74838084,
        0.66039455,
        0.14560574,
        0.07766461,
        0.2333228,
        0.049257338,
    ],
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
    frame = signal[start:start + win_size] * window

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