from pathlib import Path

import numpy as np
import scipy

print(scipy)

snap_path = Path(__file__).parent.absolute() / "src" / "snapshots"
print(snap_path)

for f in snap_path.glob("*.snap"):
    method, s = f.stem.split("__")[-1].split("-")
    sym = s == "symmetric"

    fn = eval(f"scipy.signal.windows.{method}")
    window = None
    expected = np.array(eval("".join(f.read_text().splitlines()[4:])), dtype=np.float64)
    match method:
        case "barthann":
            window = fn(10, sym=sym)
        case "bartlett":
            window = fn(10, sym=sym)
        case "blackman":
            window = fn(10, sym=sym)
        case "blackmanharris":
            window = fn(10, sym=sym)
        case "bohman":
            window = fn(10, sym=sym)
        case "boxcar":
            window = fn(10, sym=sym)
        case "chebwin":
            window = fn(10, at=100, sym=sym)
        case "cosine":
            window = fn(10, sym=sym)
        case "dpss":
            window = fn(10, 3.0, sym=sym)
        case "exponential":
            window = fn(10, sym=sym)
        case "flattop":
            window = fn(10, sym=sym)
        case "gaussian":
            window = fn(10, 0.5, sym=sym)
        case "general_cosine":
            window = fn(10, [1.0, 1.942604, 1.340318, 0.440811, 0.043097], sym=sym)
        case "general_gaussian":
            window = fn(10, 0.6, 0.5, sym=sym)
        case "general_hamming":
            window = fn(10, 0.5, sym=sym)
        case "hamming":
            window = fn(10, sym=sym)
        case "hann":
            window = fn(10, sym=sym)
        case "kaiser":
            window = fn(10, 0.5, sym=sym)
        case "kaiser_bessel_derived":
            window = fn(10, 0.5, sym=True)
        case "lanczos":
            window = fn(10, sym=sym)
        case "nuttall":
            window = fn(10, sym=sym)
        case "parzen":
            window = fn(10, sym=sym)
        case "taylor":
            window = fn(10, nbar=4, sll=100, norm=True, sym=sym)
        case "triang":
            window = fn(10, sym=sym)
        case "tukey":
            window = fn(10, alpha=0.6, sym=sym)

    print(f"{method}-{s}")
    np.testing.assert_allclose(
        window,
        expected,
        rtol=1e-10,
        atol=1e-8,
    )
