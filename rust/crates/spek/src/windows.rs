//! Window functions for spectral analysis and filtering.
//!
//! This module provides the window family exposed by SciPy's
//! `scipy.signal.windows` namespace, with a Rust API based on [`Window`] and
//! individual window constructors.
//!
//! Windowing reduces the discontinuities introduced when a finite segment of a
//! signal is treated as periodic. In spectral analysis this controls spectral
//! leakage; different windows trade main-lobe width, sidelobe attenuation, and
//! amplitude accuracy differently.
//!
//! The [`Window::window`] `symmetric` argument follows SciPy's `sym`
//! convention: `true` produces a symmetric window, typically useful for filter
//! design, while `false` produces a periodic/DFT-even window, useful when the
//! window is multiplied with a signal before an FFT.
//!
//! Reference: <https://docs.scipy.org/doc/scipy/reference/signal.windows.html>
//! and the corresponding individual SciPy window documentation.

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

use generic_num::num;
use num_traits::{Float, FloatConst};
use thiserror::Error;

/// Errors that can occur while constructing a window.
#[derive(Error, Debug)]
pub enum WindowError {
    /// The exponential window requires a strictly positive `tau`.
    #[error("Tau must be positive")]
    ExponentialTau,
    /// Kaiser-Bessel derived windows are only supported in symmetric mode.
    #[error("Kaiser-Bessel derived asymmetric window must be symmetric")]
    KaiserBesselDerivedAsymmetric,
    /// Kaiser-Bessel derived windows require an even number of samples.
    #[error("Kaiser-Bessel Derived windows are only defined for even number of points")]
    KaiserBesselDerivedSize,
}

/// A parameterized window function.
///
/// The variants correspond to the window functions listed by
/// [`scipy.signal.windows`](https://docs.scipy.org/doc/scipy/reference/signal.windows.html).
/// Use [`Window::window`] to generate `size` samples.
///
/// # Choosing a window
///
/// There is no universally best window. A boxcar preserves the narrowest
/// main lobe but has high sidelobes; tapered windows generally reduce leakage
/// at the cost of a wider main lobe. Windows such as `FlatTop` prioritize
/// amplitude accuracy, while `Dpss` is designed for concentration of energy
/// within a chosen bandwidth.
pub enum Window<T>
where
    T: Float + FloatConst,
{
    /// Modified Bartlett-Hann window.
    Barthnn,

    /// Bartlett (triangular) window.
    Bartlett,

    /// Blackman window.
    Blackman,

    /// Minimum 4-term Blackman-Harris window.
    BlackmanHarris,

    /// Bohman window.
    Bohman,

    /// Boxcar, or rectangular, window.
    Boxcar,

    /// Dolph-Chebyshev window with the requested sidelobe attenuation in dB.
    Chebwin { attenuation: T },

    /// Window with a simple cosine shape.
    Cosine,

    /// Discrete Prolate Spheroidal Sequence (DPSS) window with time-bandwidth
    /// product `nw`.
    Dpss { nw: T },

    /// Exponential (Poisson) window, optionally specifying its center and decay
    /// `tau`.
    Exponential { center: Option<T>, tau: Option<T> },

    /// Flat-top window, designed for accurate amplitude measurements.
    FlatTop,

    /// Gaussian window with the given standard deviation.
    Gaussian { standard_deviation: T },

    /// Generic weighted sum of cosine terms.
    GeneralCosine { coeffs: Vec<T> },

    /// Generalized Gaussian window with shape and standard-deviation
    /// parameters.
    GeneralGaussian { shape: T, standard_deviation: T },

    /// Generalized Hamming window parameterized by `alpha`.
    GeneralHamming { alpha: T },

    /// Hamming window.
    Hamming,

    /// Hann window.
    Hann,

    /// Kaiser window with shape parameter `beta`.
    Kaiser { beta: T },

    /// Kaiser-Bessel derived window with shape parameter `beta`.
    KaiserBesselDerived { beta: T },

    /// Lanczos (sinc) window.
    Lanczos,

    /// Minimum 4-term Blackman-Harris window according to Nuttall.
    Nuttall,

    /// Parzen window.
    Parzen,

    /// Taylor window with `nbar` near-invariant sidelobes, sidelobe level `sll`
    /// in dB, and optional normalization.
    Taylor { nbar: usize, sll: T, norm: bool },

    /// Triangular window.
    Triang,

    /// Tukey (tapered cosine) window with taper fraction `alpha`.
    Tukey { alpha: T },
}
impl<T> Window<T>
where
    T: Float + FloatConst,
{
    /// Generate the requested window.
    ///
    /// `size` is the number of returned samples. When `symmetric` is `true`,
    /// the generated window is symmetric. When it is `false`, the internally
    /// extended symmetric window is truncated by one sample to produce the
    /// periodic/DFT-even form used for spectral analysis.
    ///
    /// This convention mirrors SciPy's `sym` parameter.
    ///
    /// # Errors
    ///
    /// Returns an error when the selected window has parameter or size
    /// restrictions, such as a non-positive exponential `tau` or an
    /// asymmetric/odd-sized Kaiser-Bessel derived window.
    pub fn window(&self, size: usize, symmetric: bool) -> Result<Vec<T>, WindowError> {
        let result = match self {
            Self::Barthnn => barthnn(size, symmetric),
            Self::Bartlett => bartlett(size, symmetric),
            Self::Blackman => blackman(size, symmetric),
            Self::BlackmanHarris => blackman_harris(size, symmetric),
            Self::Bohman => bohman(size, symmetric),
            Self::Boxcar => boxcar(size, symmetric),
            Self::Chebwin { attenuation } => chebwin(size, symmetric, *attenuation),
            Self::Cosine => cosine(size, symmetric),
            Self::Dpss { nw } => dpss(size, symmetric, *nw),
            Self::Exponential { center, tau } => exponential(size, symmetric, *center, *tau)?,
            Self::FlatTop => flat_top(size, symmetric),
            Self::Gaussian { standard_deviation } => gaussian(size, symmetric, *standard_deviation),
            Self::GeneralCosine { coeffs } => general_cosine(size, symmetric, coeffs),
            Self::GeneralGaussian {
                shape,
                standard_deviation,
            } => general_gaussian(size, symmetric, *shape, *standard_deviation),
            Self::GeneralHamming { alpha } => general_hamming(size, symmetric, *alpha),
            Self::Hamming => hamming(size, symmetric),
            Self::Hann => hann(size, symmetric),
            Self::Kaiser { beta } => kaiser(size, symmetric, *beta),
            Self::KaiserBesselDerived { beta } => kaiser_bessel_derived(size, symmetric, *beta)?,
            Self::Lanczos => lanczos(size, symmetric),
            Self::Nuttall => nuttall(size, symmetric),
            Self::Parzen => parzen(size, symmetric),
            Self::Taylor { nbar, sll, norm } => taylor(size, symmetric, *nbar, *sll, *norm),
            Self::Triang => triang(size, symmetric),
            Self::Tukey { alpha } => tukey(size, symmetric, *alpha),
        };
        Ok(result)
    }
}

/// Return a modified Bartlett-Hann window.
///
/// The window combines a linear term with a cosine term and is also known as
/// the modified Bartlett-Hann window in SciPy.
///
/// `size` is the number of samples and `symmetric` selects symmetric versus
/// periodic/DFT-even construction.
pub fn barthnn<T>(size: usize, symmetric: bool) -> Vec<T>
where
    T: Float + FloatConst,
{
    if size == 0 {
        return Vec::new();
    }
    if size == 1 {
        return vec![T::one()];
    }
    let n = if symmetric { size } else { size + 1 };
    let denom = num!(n - 1);
    let mut window = Vec::with_capacity(size);
    for i in 0..n {
        let x = num!(i) / denom;
        let value = num!(0.62)
            - num!(0.48) * (x - num!(0.5)).abs()
            - num!(0.38) * (num!(2.0) * T::PI() * x).cos();
        window.push(value);
    }
    if !symmetric {
        window.pop();
    }
    window
}

/// Return a Bartlett window.
///
/// The Bartlett window is triangular and tapers to zero at both ends of the
/// symmetric form.
pub fn bartlett<T>(size: usize, symmetric: bool) -> Vec<T>
where
    T: Float,
{
    if size == 0 {
        return Vec::new();
    }

    if size == 1 {
        return vec![T::one()];
    }

    let n = if symmetric { size } else { size + 1 };
    let denominator = num!(n - 1);

    let mut window = Vec::with_capacity(n);

    for i in 0..n {
        let i = T::from(i).unwrap();

        let value = T::one() - ((num!(2.0) * i - denominator).abs() / denominator);

        window.push(value);
    }

    if !symmetric {
        window.pop();
    }
    window
}

/// Return a Blackman window.
///
/// This is a 3-term cosine window with coefficients `0.42`, `0.50`, and
/// `0.08`.
pub fn blackman<T>(size: usize, symmetric: bool) -> Vec<T>
where
    T: Float + FloatConst,
{
    general_cosine(size, symmetric, &[num!(0.42), num!(0.5), num!(0.08)])
}

/// Return a minimum 4-term Blackman-Harris window.
///
/// The coefficients are the standard minimum 4-term Blackman-Harris values
/// used by SciPy.
pub fn blackman_harris<T>(size: usize, symmetric: bool) -> Vec<T>
where
    T: Float + FloatConst,
{
    general_cosine(
        size,
        symmetric,
        &[num!(0.35875), num!(0.48829), num!(0.14128), num!(0.01168)],
    )
}

/// Return a Bohman window.
///
/// The Bohman window is a smoothly tapered window with zero-valued endpoints
/// in its symmetric form.
pub fn bohman<T>(size: usize, symmetric: bool) -> Vec<T>
where
    T: Float + FloatConst,
{
    if size == 0 {
        return Vec::new();
    }
    if size == 1 {
        return vec![T::one()];
    }

    let n = if symmetric { size } else { size + 1 };
    let denom = num!(n - 1);

    let mut window = Vec::with_capacity(n);
    for i in 0..n {
        let x = num!(-1.0) + num!(2.0) * num!(i) / denom;
        let fac = x.abs();
        let value = (T::one() - fac) * (T::PI() * fac).cos() + (T::PI() * fac).sin() / T::PI();
        window.push(value);
    }

    if !symmetric {
        window.pop();
    }
    window
}

/// Return a boxcar (rectangular) window.
///
/// Every sample is one. The symmetric/periodic distinction has no effect on
/// the result.
pub fn boxcar<T>(size: usize, _symmetric: bool) -> Vec<T>
where
    T: Float + FloatConst,
{
    // A rectangular window is unaffected by the symmetric/periodic
    // distinction.
    vec![T::one(); size]
}

/// Return a Dolph-Chebyshev window.
///
/// `attenuation` specifies the desired sidelobe attenuation in decibels.
/// Dolph-Chebyshev windows are constructed to provide equal-height sidelobes
/// in the frequency domain for the requested attenuation.
pub fn chebwin<T>(size: usize, symmetric: bool, attenuation: T) -> Vec<T>
where
    T: Float + FloatConst,
{
    if size == 0 {
        return Vec::new();
    }
    if size == 1 {
        return vec![T::one()];
    }

    let n = if symmetric { size } else { size + 1 };
    let n_f = num!(n);
    let order = n_f - T::one();

    // Default sidelobe attenuation, in dB.
    let at = attenuation;
    let beta = ((T::one() / order) * (num!(10.0).powf(at / num!(20.0))).acosh()).cosh();

    // Chebyshev-polynomial samples: these are the window's DFT
    // coefficients.
    let mut p = Vec::with_capacity(n);
    for k in 0..n {
        let x = beta * (T::PI() * num!(k) / n_f).cos();
        let value = if x > T::one() {
            (order * x.acosh()).cosh()
        } else if x < -T::one() {
            let sign = if n % 2 == 0 { -T::one() } else { T::one() };
            sign * (order * (-x).acosh()).cosh()
        } else {
            (order * x.acos()).cos()
        };
        p.push(value);
    }

    // Synthesize the window via a direct (naive) inverse DFT rather than
    // an FFT, since only a real-valued summation is needed here.
    let w_full = if n % 2 == 1 {
        let half = n.div_ceil(2);
        let mut w = Vec::with_capacity(half);
        for k in 0..half {
            let mut sum = T::zero();
            for (j, item) in p.iter().enumerate().take(n) {
                let angle = num!(2.0) * T::PI() * num!(j) * num!(k) / n_f;
                sum = sum + *item * angle.cos();
            }
            w.push(sum);
        }
        let mut window = Vec::with_capacity(n);
        for i in (1..half).rev() {
            window.push(w[i]);
        }
        window.extend(w.iter().copied());
        window
    } else {
        let mut pre = Vec::with_capacity(n);
        let mut pim = Vec::with_capacity(n);
        for (j, item) in p.iter().enumerate().take(n) {
            let phase = T::PI() * num!(j) / n_f;
            pre.push(*item * phase.cos());
            pim.push(*item * phase.sin());
        }

        let half = n / 2 + 1;
        let mut w = Vec::with_capacity(half);
        for k in 0..half {
            let mut sum = T::zero();
            for j in 0..n {
                let angle = num!(2.0) * T::PI() * num!(j) * num!(k) / n_f; // Real part of (pre + i*pim) * exp(-i*angle).
                sum = sum + pre[j] * angle.cos() + pim[j] * angle.sin();
            }
            w.push(sum);
        }
        let mut window = Vec::with_capacity(n);
        for i in (1..half).rev() {
            window.push(w[i]);
        }
        for i in w.iter().take(half).skip(1) {
            window.push(*i);
        }
        window
    };

    let max_val = w_full.iter().fold(
        T::zero(),
        |acc, &x| if x.abs() > acc { x.abs() } else { acc },
    );

    let mut window: Vec<T> = if max_val > T::zero() {
        w_full.iter().map(|&x| x / max_val).collect()
    } else {
        w_full
    };

    if !symmetric {
        window.pop();
    }
    window
}

/// Return a window with a simple cosine shape.
///
/// The window uses a half-cycle sine/cosine construction and does not reach
/// zero at the center.
pub fn cosine<T>(size: usize, symmetric: bool) -> Vec<T>
where
    T: Float + FloatConst,
{
    if size == 0 {
        return Vec::new();
    }
    if size == 1 {
        return vec![T::one()];
    }

    let n = if symmetric { size } else { size + 1 };
    let n_f = num!(n);

    let mut window = Vec::with_capacity(n);
    for i in 0..n {
        let value = (T::PI() / n_f * (num!(i) + num!(0.5))).sin();
        window.push(value);
    }

    if !symmetric {
        window.pop();
    }
    window
}

/// Compute the first DPSS (Slepian) window.
///
/// `nw` is the time-bandwidth product (`NW` in SciPy). DPSS windows maximize
/// spectral energy concentration within a specified bandwidth. This function
/// returns the principal (`Kmax = 1`) sequence normalized to a peak value of
/// one.
pub fn dpss<T>(size: usize, symmetric: bool, nw: T) -> Vec<T>
where
    T: Float + FloatConst,
{
    fn largest_eigenvector<T>(diag: &[T], offdiag: &[T]) -> Vec<T>
    where
        T: Float,
    {
        let n = diag.len();

        let mut a = vec![vec![T::zero(); n]; n];
        for i in 0..n {
            a[i][i] = diag[i];

            if i + 1 < n {
                a[i][i + 1] = offdiag[i];
                a[i + 1][i] = offdiag[i];
            }
        }

        // Eigenvector matrix.
        let mut q = vec![vec![T::zero(); n]; n];
        for (i, item) in q.iter_mut().enumerate().take(n) {
            item[i] = T::one();
        }

        let eps = T::epsilon().sqrt();

        for _ in 0..1000 {
            // Find largest off-diagonal element.
            let mut p = 0;
            let mut r = 1;
            let mut max = T::zero();

            for i in 0..n {
                for (j, _item) in a.iter().enumerate().take(n).skip(i + 1) {
                    let x = a[i][j].abs();
                    if x > max {
                        max = x;
                        p = i;
                        r = j;
                    }
                }
            }

            if max <= eps {
                break;
            }

            let app = a[p][p];
            let arr = a[r][r];
            let apr = a[p][r];

            let tau = (arr - app) / (num!(2.0) * apr);

            let t = if tau >= T::zero() {
                T::one() / (tau + (T::one() + tau * tau).sqrt())
            } else {
                -T::one() / (-tau + (T::one() + tau * tau).sqrt())
            };

            let c = T::one() / (T::one() + t * t).sqrt();
            let s = t * c;

            // Rotate A.
            #[allow(
                clippy::needless_range_loop,
                reason = "indexed access is required to update the p/r columns and preserve matrix symmetry"
            )]
            for k in 0..n {
                if k != p && k != r {
                    let akp = a[k][p];
                    let akr = a[k][r];

                    a[k][p] = c * akp - s * akr;
                    a[p][k] = a[k][p];

                    a[k][r] = s * akp + c * akr;
                    a[r][k] = a[k][r];
                }
            }

            a[p][p] = c * c * app - num!(2.0) * s * c * apr + s * s * arr;
            a[r][r] = s * s * app + num!(2.0) * s * c * apr + c * c * arr;

            a[p][r] = T::zero();
            a[r][p] = T::zero();

            // Rotate eigenvectors.
            for k in q.iter_mut().take(n) {
                let qkp = k[p];
                let qkr = k[r];

                k[p] = c * qkp - s * qkr;
                k[r] = s * qkp + c * qkr;
            }
        }

        // Find largest algebraic eigenvalue.
        let mut max_index = 0;
        for i in 1..n {
            if a[i][i] > a[max_index][max_index] {
                max_index = i;
            }
        }

        let mut v = vec![T::zero(); n];
        for i in 0..n {
            v[i] = q[i][max_index];
        }

        v
    }

    if size == 0 {
        return Vec::new();
    }

    if size == 1 {
        return vec![T::one()];
    }

    // SciPy _extend()
    let n = if symmetric { size } else { size + 1 };

    let n_f = num!(n);

    // W = NW / M
    let w = nw / n_f;

    let two_pi_w = num!(2.0) * T::PI() * w;
    let cos_2piw = two_pi_w.cos();

    // Tridiagonal DPSS matrix
    let center = (n_f - T::one()) / num!(2.0);

    let mut diag = Vec::with_capacity(n);

    for i in 0..n {
        let x = center - num!(i);
        diag.push(x * x * cos_2piw);
    }

    let mut offdiag = Vec::with_capacity(n - 1);

    for i in 1..n {
        let i_f = num!(i);
        offdiag.push(i_f * (n_f - i_f) / num!(2.0));
    }

    // IMPORTANT:
    // solve the largest eigenvalue/eigenvector of this
    // symmetric tridiagonal matrix.
    let mut window = largest_eigenvector(&diag, &offdiag);

    // SciPy:
    //
    // fix_even = windows[::2, ...].sum(axis=1) < 0
    //
    // For Kmax=1 this is simply:
    // if sum(window) < 0 => negate.
    let sum = window.iter().fold(T::zero(), |acc, &x| acc + x);

    if sum < T::zero() {
        for x in &mut window {
            *x = -*x;
        }
    }

    // norm != 2
    //
    // SciPy:
    //     windows /= windows.max()
    let max = window.iter().fold(T::neg_infinity(), |acc, &x| acc.max(x));

    for x in &mut window {
        *x = *x / max;
    }

    // "approximate" correction.
    //
    // SciPy applies this when the INTERNAL M is even,
    // before truncation.
    if n % 2 == 0 {
        let n2 = n_f * n_f;
        let correction = n2 / (n2 + nw);

        for x in &mut window {
            *x = *x * correction;
        }
    }

    // SciPy _truncate()
    if !symmetric {
        window.pop();
    }

    window
}

/// Return an exponential (Poisson) window.
///
/// `center` controls the location of the peak. If omitted, the center follows
/// SciPy's symmetric/periodic convention. `tau` controls exponential decay and
/// must be strictly positive.
pub fn exponential<T>(
    size: usize,
    symmetric: bool,
    center: Option<T>,
    tau: Option<T>,
) -> Result<Vec<T>, WindowError>
where
    T: Float + FloatConst,
{
    if size == 0 {
        return Ok(Vec::new());
    }

    if size == 1 {
        return Ok(vec![T::one()]);
    }

    // SciPy:
    // center = (M if not sym and M > 1 else M - 1) / 2
    //
    // Note that this is the ORIGINAL M, not the internally
    // extended M = M + 1 used by many other windows.
    let size_f = num!(size);

    let center = center.unwrap_or_else(|| {
        if symmetric {
            (size_f - T::one()) / num!(2.0)
        } else {
            size_f / num!(2.0)
        }
    });

    // SciPy requires tau > 0.
    let tau = match tau.unwrap_or_else(|| T::one()) {
        tau if tau > T::zero() => tau,
        _ => return Err(WindowError::ExponentialTau),
    };

    let mut window = Vec::with_capacity(size);

    for i in 0..size {
        let d = num!(i) - center;
        window.push((-d.abs() / tau).exp());
    }

    Ok(window)
}

/// Return a flat-top window.
///
/// The flat-top design has a broad, nearly constant peak and is intended to
/// reduce amplitude-measurement error in spectral analysis.
pub fn flat_top<T>(size: usize, symmetric: bool) -> Vec<T>
where
    T: Float + FloatConst,
{
    general_cosine(
        size,
        symmetric,
        &[
            num!(0.21557895),
            num!(0.41663158),
            num!(0.277263158),
            num!(0.083578947),
            num!(0.006947368),
        ],
    )
}

/// Return a Gaussian window.
///
/// `standard_deviation` is the Gaussian standard deviation in samples.
pub fn gaussian<T>(size: usize, symmetric: bool, standard_deviation: T) -> Vec<T>
where
    T: Float,
{
    if size == 0 {
        return Vec::new();
    }
    if size == 1 {
        return vec![T::one()];
    }

    let n = if symmetric { size } else { size + 1 };
    let denom = num!(n - 1);
    let std = standard_deviation;
    let center = denom / num!(2.0);

    let mut window = Vec::with_capacity(n);
    for i in 0..n {
        let d = num!(i) - center;
        let value = (-num!(0.5) * (d / std) * (d / std)).exp();
        window.push(value);
    }

    if !symmetric {
        window.pop();
    }
    window
}

/// Return a generic weighted sum of cosine terms.
///
/// `coeffs` contains the cosine coefficients. The implementation uses the
/// alternating-sign form
/// `a[0] - a[1] cos(...) + a[2] cos(...) - ...`, matching SciPy's
/// `general_cosine` convention.
pub fn general_cosine<T>(size: usize, symmetric: bool, coeffs: &[T]) -> Vec<T>
where
    T: Float + FloatConst,
{
    if size == 0 {
        return Vec::new();
    }
    if size == 1 {
        return vec![T::one()];
    }

    let n = if symmetric { size } else { size + 1 };
    let denom = num!(n - 1);

    let mut window = Vec::with_capacity(n);
    for i in 0..n {
        let mut value = T::zero();
        let mut sign = T::one();
        for (k, &a) in coeffs.iter().enumerate() {
            let angle = num!(2.0) * T::PI() * num!(k) * num!(i) / denom;
            value = value + sign * a * angle.cos();
            sign = -sign;
        }
        window.push(value);
    }

    if !symmetric {
        window.pop();
    }
    window
}

/// Return a generalized Gaussian window.
///
/// `shape` controls the exponent of the generalized Gaussian and
/// `standard_deviation` controls its width. The exponent is `2 * shape`.
pub fn general_gaussian<T>(size: usize, symmetric: bool, shape: T, standard_deviation: T) -> Vec<T>
where
    T: Float + FloatConst,
{
    if size == 0 {
        return Vec::new();
    }
    if size == 1 {
        return vec![T::one()];
    }

    let n = if symmetric { size } else { size + 1 };
    let denom = num!(n - 1);
    let center = denom / num!(2.0);

    let sigma = standard_deviation;

    let mut window = Vec::with_capacity(n);
    for i in 0..n {
        let d = num!(i) - center;
        let ratio = (d / sigma).abs();
        let value = (-num!(0.5) * ratio.powf(num!(2.0) * shape)).exp();
        window.push(value);
    }

    if !symmetric {
        window.pop();
    }
    window
}

/// Return a generalized Hamming window.
///
/// `alpha` is the leading cosine-series coefficient. `alpha = 0.54` gives
/// the standard Hamming window.
pub fn general_hamming<T>(size: usize, symmetric: bool, alpha: T) -> Vec<T>
where
    T: Float + FloatConst,
{
    general_cosine(size, symmetric, &[alpha, T::one() - alpha])
}

/// Return a Hamming window.
///
/// This is the standard two-term cosine window with coefficients `0.54` and
/// `0.46`.
pub fn hamming<T>(size: usize, symmetric: bool) -> Vec<T>
where
    T: Float + FloatConst,
{
    general_cosine(size, symmetric, &[num!(0.54), num!(0.46)])
}

/// Return a Hann window.
///
/// This is the standard raised-cosine Hann window with equal coefficients
/// `0.5` and `0.5`.
pub fn hann<T>(size: usize, symmetric: bool) -> Vec<T>
where
    T: Float + FloatConst,
{
    general_cosine(size, symmetric, &[num!(0.5), num!(0.5)])
}

/// Return a Kaiser window.
///
/// `beta` controls the trade-off between main-lobe width and sidelobe
/// attenuation. Larger values produce stronger tapering and lower sidelobes.
pub fn kaiser<T>(size: usize, symmetric: bool, beta: T) -> Vec<T>
where
    T: Float + FloatConst,
{
    /// Modified Bessel function of the first kind, order 0, via itspower
    /// series. Used by the Kaiser / Kaiser-Bessel-derived windows.
    fn bessel_i0<T>(x: T) -> T
    where
        T: Float,
    {
        let mut sum = T::one();
        let mut term = T::one();
        let x2 = (x * x) / num!(4.0);
        let mut k = T::one();

        for _ in 0..200 {
            term = term * x2 / (k * k);
            sum = sum + term;
            if term < T::epsilon() * sum {
                break;
            }
            k = k + T::one();
        }

        sum
    }
    if size == 0 {
        return Vec::new();
    }
    if size == 1 {
        return vec![T::one()];
    }

    let n = if symmetric { size } else { size + 1 };
    let denom = num!(n - 1);
    let i0_beta = bessel_i0(beta);

    let mut window = Vec::with_capacity(n);
    for i in 0..n {
        let x = num!(2.0) * num!(i) / denom - T::one();
        let inner = (T::one() - x * x).max(T::zero());
        let arg = beta * inner.sqrt();
        window.push(bessel_i0(arg) / i0_beta);
    }

    if !symmetric {
        window.pop();
    }
    window
}

/// Return a Kaiser-Bessel derived window.
///
/// The construction is based on a cumulative sum of a Kaiser window followed
/// by square-root normalization. As in SciPy, this window is only defined here
/// for an even `size` and `symmetric = true`.
pub fn kaiser_bessel_derived<T>(
    size: usize,
    symmetric: bool,
    beta: T,
) -> Result<Vec<T>, WindowError>
where
    T: Float + FloatConst,
{
    if size == 0 {
        return Ok(Vec::new());
    }

    if !symmetric {
        return Err(WindowError::KaiserBesselDerivedAsymmetric);
    }
    if !size.is_multiple_of(2) {
        return Err(WindowError::KaiserBesselDerivedSize);
    }

    // SciPy:
    // kaiser(M // 2 + 1, beta)
    let half = size / 2;

    let kaiser_window = kaiser(half + 1, true, beta);

    // SciPy:
    // csum = cumulative_sum(kaiser_window)
    let mut csum = Vec::with_capacity(half + 1);
    let mut acc = T::zero();

    for &x in &kaiser_window {
        acc = acc + x;
        csum.push(acc);
    }

    // SciPy:
    // half_window = sqrt(csum[:-1] / csum[-1])
    let total = csum[half];

    let mut half_window = Vec::with_capacity(half);

    for i in csum.iter().take(half) {
        half_window.push((*i / total).sqrt());
    }

    // SciPy:
    // concat((half_window, flip(half_window)))
    let mut window = Vec::with_capacity(size);

    window.extend_from_slice(&half_window);
    window.extend(half_window.iter().rev().copied());

    Ok(window)
}

/// Return a Lanczos (sinc) window.
///
/// The window is the normalized sinc function sampled over `[-1, 1]`, with
/// the value at zero defined by continuity as one.
pub fn lanczos<T>(size: usize, symmetric: bool) -> Vec<T>
where
    T: Float + FloatConst,
{
    if size == 0 {
        return Vec::new();
    }
    if size == 1 {
        return vec![T::one()];
    }

    let n = if symmetric { size } else { size + 1 };
    let denom = num!(n - 1);

    let mut window = Vec::with_capacity(n);
    for i in 0..n {
        let x = num!(2.0) * num!(i) / denom - T::one();
        let value = if x == T::zero() {
            T::one()
        } else {
            let px = T::PI() * x;
            px.sin() / px
        };
        window.push(value);
    }

    if !symmetric {
        window.pop();
    }
    window
}

/// Return a minimum 4-term Blackman-Harris window according to Nuttall.
///
/// This is a 4-term cosine-series window with Nuttall's coefficients.
pub fn nuttall<T>(size: usize, symmetric: bool) -> Vec<T>
where
    T: Float + FloatConst,
{
    general_cosine(
        size,
        symmetric,
        &[
            num!(0.3635819),
            num!(0.4891775),
            num!(0.1365995),
            num!(0.0106411),
        ],
    )
}

/// Return a Parzen window.
///
/// The Parzen window is a piecewise cubic window with a smooth taper toward
/// both ends.
pub fn parzen<T>(size: usize, symmetric: bool) -> Vec<T>
where
    T: Float + FloatConst,
{
    if size == 0 {
        return Vec::new();
    }
    if size == 1 {
        return vec![T::one()];
    }

    let n = if symmetric { size } else { size + 1 };
    let n_f = num!(n);
    let half_n = n_f / num!(2.0);
    let quarter = (n_f - T::one()) / num!(4.0);
    let center = (n_f - T::one()) / num!(2.0);

    let mut window = Vec::with_capacity(n);
    for i in 0..n {
        let d = num!(i) - center;
        let r = d.abs() / half_n;
        let value = if d.abs() <= quarter {
            T::one() - num!(6.0) * r * r + num!(6.0) * r * r * r
        } else {
            num!(2.0) * (T::one() - r).powi(3)
        };
        window.push(value);
    }

    if !symmetric {
        window.pop();
    }
    window
}

/// Return a Taylor window.
///
/// `nbar` controls the number of nearly constant sidelobes, `sll` specifies
/// the desired sidelobe level in decibels, and `norm` controls normalization.
/// With `norm = true`, the continuous Taylor formula is normalized at its
/// midpoint, matching SciPy rather than simply dividing by the largest
/// discrete sample.
pub fn taylor<T>(size: usize, symmetric: bool, nbar: usize, sll: T, norm: bool) -> Vec<T>
where
    T: Float + FloatConst,
{
    if size == 0 {
        return Vec::new();
    }

    if size == 1 {
        return vec![T::one()];
    }

    // Same as SciPy _extend(M, sym).
    let n = if symmetric { size } else { size + 1 };
    let n_f = num!(n);

    // SciPy:
    // B = 10**(sll / 20)
    // A = acosh(B) / pi
    // s2 = nbar**2 / (A**2 + (nbar - 0.5)**2)
    let b = num!(10.0).powf(sll / num!(20.0));
    let a = b.acosh() / T::PI();
    let nbar_t = num!(nbar);

    let half = num!(0.5);
    let s2 = nbar_t * nbar_t / (a * a + (nbar_t - half) * (nbar_t - half));

    // ma = [1, 2, ..., nbar - 1]
    let ma: Vec<usize> = (1..nbar).collect();

    // SciPy Fm calculation.
    let mut fm = Vec::with_capacity(ma.len());

    for (mi, &m) in ma.iter().enumerate() {
        let m_t = num!(m);
        let m2 = m_t * m_t;

        let mut numer = T::one();

        for &j in &ma {
            let j_t = num!(j);
            let j_minus_half = j_t - half;

            numer = numer * (T::one() - m2 / s2 / (a * a + j_minus_half * j_minus_half));
        }

        // SciPy:
        //
        // signs[::2] = 1
        // signs[1::2] = -1
        //
        // Therefore mi = 0, 2, 4 ... => +1
        //             mi = 1, 3, 5 ... => -1
        if mi % 2 == 0 {
        } else {
            numer = -numer;
        }

        let mut denom = num!(2.0);

        // prod(1 - m2 / m2[:mi])
        for &j in &ma[..mi] {
            let j_t = num!(j);
            denom = denom * (T::one() - m2 / (j_t * j_t));
        }

        // prod(1 - m2 / m2[mi+1:])
        for &j in &ma[mi + 1..] {
            let j_t = num!(j);
            denom = denom * (T::one() - m2 / (j_t * j_t));
        }

        fm.push(numer / denom);
    }

    // SciPy:
    //
    // W(n) = 1 + 2 * sum(
    //     Fm * cos(2*pi*ma*(n-M/2+0.5)/M)
    // )
    let w_fn = |i: T| -> T {
        let x = i - n_f / num!(2.0) + num!(0.5);

        let mut sum = T::zero();

        for (idx, &m) in ma.iter().enumerate() {
            let m_t = num!(m);

            sum = sum + fm[idx] * (num!(2.0) * T::PI() * m_t * x / n_f).cos();
        }

        T::one() + num!(2.0) * sum
    };

    let mut window = Vec::with_capacity(n);

    for i in 0..n {
        window.push(w_fn(num!(i)));
    }

    // SciPy:
    //
    // if norm:
    //     scale = 1.0 / W((M - 1) / 2)
    //     w *= scale
    //
    // Notice that this is NOT simply:
    //
    //     w /= w.max()
    //
    // for even M. W((M - 1)/2) evaluates the continuous formula
    // at the midpoint between the two central samples.
    if norm {
        let center = (n_f - T::one()) / num!(2.0);
        let scale = T::one() / w_fn(center);

        for x in &mut window {
            *x = *x * scale;
        }
    }

    // Same as SciPy _truncate().
    if !symmetric {
        window.pop();
    }

    window
}

/// Return a triangular window.
///
/// The triangular window has a linear rise and fall. Its exact central value
/// depends on whether the requested length is odd or even.
pub fn triang<T>(size: usize, symmetric: bool) -> Vec<T>
where
    T: Float + FloatConst,
{
    if size == 0 {
        return Vec::new();
    }
    if size == 1 {
        return vec![T::one()];
    }

    let n = if symmetric { size } else { size + 1 };
    let half = n.div_ceil(2);

    let mut first_half = Vec::with_capacity(half);
    let mut window = if n % 2 == 0 {
        for k in 1..=half {
            first_half.push(num!(2 * k - 1) / num!(n));
        }
        let mut w = first_half.clone();
        w.extend(first_half.iter().rev().copied());
        w
    } else {
        for k in 1..=half {
            first_half.push(num!(2 * k) / num!(n + 1));
        }
        let mut w = first_half.clone();
        w.extend(first_half[..first_half.len() - 1].iter().rev().copied());
        w
    };

    if !symmetric {
        window.pop();
    }
    window
}

/// Return a Tukey (tapered cosine) window.
///
/// `alpha` controls the fraction of the window occupied by cosine tapers:
/// `alpha <= 0` gives a boxcar window, while `alpha >= 1` gives a Hann window.
pub fn tukey<T>(size: usize, symmetric: bool, alpha: T) -> Vec<T>
where
    T: Float + FloatConst,
{
    if size == 0 {
        return Vec::new();
    }

    if size == 1 {
        return vec![T::one()];
    }

    // SciPy:
    // if alpha <= 0:
    //     return ones
    if alpha <= T::zero() {
        return vec![T::one(); size];
    }

    // SciPy:
    // elif alpha >= 1:
    //     return hann(M, sym=sym)
    if alpha >= T::one() {
        return hann(size, symmetric);
    }

    // SciPy _extend(M, sym)
    let n = if symmetric { size } else { size + 1 };
    let n_f = num!(n);

    // width = floor(alpha * (M - 1) / 2)
    let width = (alpha * (n_f - T::one()) / num!(2.0))
        .floor()
        .to_usize()
        .unwrap();

    let denom = alpha * (n_f - T::one());

    let mut window = Vec::with_capacity(n);

    for i in 0..n {
        let value = if i <= width {
            let i_f = num!(i);

            num!(0.5) * (T::one() + (T::PI() * (-T::one() + num!(2.0) * i_f / denom)).cos())
        } else if i >= n - width - 1 {
            let i_f = num!(i);

            num!(0.5)
                * (T::one()
                    + (T::PI() * (-num!(2.0) / alpha + T::one() + num!(2.0) * i_f / denom)).cos())
        } else {
            T::one()
        };

        window.push(value);
    }

    if !symmetric {
        window.pop();
    }

    window
}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::format;

    use rstest::rstest;

    use super::*;
    #[rstest]
    #[case(Window::Barthnn, "barthann")]
    #[case(Window::Bartlett, "bartlett")]
    #[case(Window::Blackman, "blackman")]
    #[case(Window::BlackmanHarris, "blackmanharris")]
    #[case(Window::Bohman, "bohman")]
    #[case(Window::Boxcar, "boxcar")]
    #[case(Window:: Chebwin{ attenuation: 100.0 },"chebwin")]
    #[case(Window::Cosine, "cosine")]
    #[case(Window::Dpss{ nw: 3.0 }, "dpss")]
    #[case(Window::Exponential{ center: None, tau: None }, "exponential")]
    #[case(Window::FlatTop, "flattop")]
    #[case(Window::Gaussian{ standard_deviation: 0.5 }, "gaussian")]
    #[case(Window::GeneralCosine{ coeffs: vec![1.0f64, 1.942604, 1.340318, 0.440811, 0.043097] }, "general_cosine")]
    #[case(Window::GeneralGaussian{ shape: 0.6, standard_deviation: 0.5 }, "general_gaussian")]
    #[case(Window::GeneralHamming{ alpha: 0.5 }, "general_hamming")]
    #[case(Window::Hamming, "hamming")]
    #[case(Window::Hann, "hann")]
    #[case(Window::Kaiser{ beta: 0.5 }, "kaiser")]
    #[case(Window::KaiserBesselDerived{ beta: 0.5 }, "kaiser_bessel_derived")]
    #[case(Window::Lanczos, "lanczos")]
    #[case(Window::Nuttall, "nuttall")]
    #[case(Window::Parzen, "parzen")]
    #[case(Window::Taylor{ nbar: 4, sll: 100.0, norm: true }, "taylor")]
    #[case(Window::Triang, "triang")]
    #[case(Window::Tukey{ alpha: 0.6 }, "tukey")]
    fn test(
        #[case] window: Window<f64>,
        #[case] name: &str,
        #[values(true, false)] symmetric: bool,
    ) -> mischief::Result<()> {
        if name == "kaiser_bessel_derived" && !symmetric {
            return Ok(());
        }
        let window = window
            .window(10, symmetric)?
            .iter()
            .map(|x| format!("{:.14}", x))
            .collect::<Vec<_>>();
        insta::assert_debug_snapshot!(
            format!(
                "{}-{}",
                name,
                if symmetric { "symmetric" } else { "asymmetric" }
            ),
            window
        );
        Ok(())
    }
}
