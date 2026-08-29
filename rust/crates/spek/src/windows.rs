extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

use generic_num::num;
use num_traits::{Float, FloatConst};

/// <https://docs.scipy.org/doc/scipy/reference/signal.windows.html>
pub trait Windows<T>
where
    T: Float,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T>;
}

pub struct Barthnn;
impl<T> Windows<T> for Barthnn
where
    T: Float + FloatConst,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T> {
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

            let value = num!(0.62) - num!(0.48) * (x - num!(0.5)).abs()
                + num!(0.38) * (num!(2.0) * T::PI() * x).cos();

            window.push(value);
        }

        if !symmetric {
            window.pop();
        }

        window
    }
}
pub struct Bartlett;
impl<T> Windows<T> for Bartlett
where
    T: Float,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T> {
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

        if symmetric {
            window
        } else {
            window.pop();
            window
        }
    }
}

/// Shared helper implementing the "generalized cosine" family of windows:
/// `w[n] = sum_k (-1)^k * a_k * cos(2*pi*k*n/(N-1))`.
///
/// Hann, Hamming, Blackman, Blackman-Harris, Nuttall, Flat Top,
/// `GeneralCosine`, and `GeneralHamming` are all special cases of this,
/// differing only in the coefficient list.
fn general_cosine<T>(size: usize, symmetric: bool, coeffs: &[T]) -> Vec<T>
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

/// Modified Bessel function of the first kind, order 0, via its power series.
/// Used by the Kaiser / Kaiser-Bessel-derived windows.
fn bessel_i0<T: Float>(x: T) -> T {
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

pub struct Blackman;
impl<T> Windows<T> for Blackman
where
    T: Float + FloatConst,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T> {
        general_cosine(size, symmetric, &[num!(0.42), num!(0.5), num!(0.08)])
    }
}

pub struct BlackmanHarris;
impl<T> Windows<T> for BlackmanHarris
where
    T: Float + FloatConst,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T> {
        general_cosine(
            size,
            symmetric,
            &[num!(0.35875), num!(0.48829), num!(0.14128), num!(0.01168)],
        )
    }
}

pub struct Bohman;
impl<T> Windows<T> for Bohman
where
    T: Float + FloatConst,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T> {
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
}

pub struct Boxcar;
impl<T> Windows<T> for Boxcar
where
    T: Float,
{
    fn window(&self, size: usize, _symmetric: bool) -> Vec<T> {
        // A rectangular window is unaffected by the symmetric/periodic distinction.
        vec![T::one(); size]
    }
}

pub struct Chebwin<T>
where
    T: Float + FloatConst,
{
    pub attenuation: T,
}

impl<T> Windows<T> for Chebwin<T>
where
    T: Float + FloatConst,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T> {
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
        let at: T = self.attenuation;
        let beta = ((T::one() / order) * (num!(10.0).powf(at / num!(20.0))).acosh()).cosh();

        // Chebyshev-polynomial samples: these are the window's DFT coefficients.
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
            let half = (n + 1) / 2;
            let mut w = Vec::with_capacity(half);
            for k in 0..half {
                let mut sum = T::zero();
                for j in 0..n {
                    let angle = num!(2.0) * T::PI() * num!(j) * num!(k) / n_f;
                    sum = sum + p[j] * angle.cos();
                }
                w.push(sum);
            }
            let mut window = Vec::with_capacity(n);
            for i in (1..half).rev() {
                window.push(w[i]);
            }
            window.extend(w.iter().cloned());
            window
        } else {
            let mut pre = Vec::with_capacity(n);
            let mut pim = Vec::with_capacity(n);
            for j in 0..n {
                let phase = T::PI() * num!(j) / n_f;
                pre.push(p[j] * phase.cos());
                pim.push(p[j] * phase.sin());
            }

            let half = n / 2 + 1;
            let mut w = Vec::with_capacity(half);
            for k in 0..half {
                let mut sum = T::zero();
                for j in 0..n {
                    let angle = num!(2.0) * T::PI() * num!(j) * num!(k) / n_f;
                    // Real part of (pre + i*pim) * exp(-i*angle).
                    sum = sum + pre[j] * angle.cos() + pim[j] * angle.sin();
                }
                w.push(sum);
            }
            let mut window = Vec::with_capacity(n);
            for i in (1..half).rev() {
                window.push(w[i]);
            }
            for i in 1..half {
                window.push(w[i]);
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
}

pub struct Cosine;
impl<T> Windows<T> for Cosine
where
    T: Float + FloatConst,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T> {
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
}

pub struct Dpss<T>
where
    T: Float + FloatConst,
{
    pub nw: T,
}
impl<T> Windows<T> for Dpss<T>
where
    T: Float + FloatConst,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T> {
        if size == 0 {
            return Vec::new();
        }
        if size == 1 {
            return vec![T::one()];
        }

        let n = if symmetric { size } else { size + 1 };
        let n_f = num!(n);

        // Default time-half-bandwidth product.
        let nw = self.nw;
        let w = nw / n_f;
        let cos_2piw = (num!(2.0) * T::PI() * w).cos();
        let center = (n_f - T::one()) / num!(2.0);

        // Symmetric tridiagonal matrix (Slepian/Grunbaum construction) that
        // commutes with the time-limited sinc kernel; its top eigenvector is
        // the order-0 DPSS. Found here via power iteration.
        let mut diag = Vec::with_capacity(n);
        for i in 0..n {
            let d = center - num!(i);
            diag.push(d * d * cos_2piw);
        }
        let mut offdiag = Vec::with_capacity(n - 1);
        for i in 1..n {
            let ii = num!(i);
            offdiag.push(ii * (n_f - ii) / num!(2.0));
        }

        let mut v = vec![T::one(); n];
        for _ in 0..200 {
            let mut nv = vec![T::zero(); n];
            for i in 0..n {
                let mut val = diag[i] * v[i];
                if i > 0 {
                    val = val + offdiag[i - 1] * v[i - 1];
                }
                if i < n - 1 {
                    val = val + offdiag[i] * v[i + 1];
                }
                nv[i] = val;
            }
            let norm = nv.iter().fold(T::zero(), |acc, &x| acc + x * x).sqrt();
            if norm > T::zero() {
                for x in nv.iter_mut() {
                    *x = *x / norm;
                }
            }
            v = nv;
        }

        let max_abs = v.iter().fold(
            T::zero(),
            |acc, &x| if x.abs() > acc { x.abs() } else { acc },
        );
        let mut window: Vec<T> = if max_abs > T::zero() {
            v.iter().map(|&x| x / max_abs).collect()
        } else {
            v
        };
        // Eigenvector sign is arbitrary; make the window positive.
        if window.iter().fold(T::zero(), |acc, &x| acc + x) < T::zero() {
            for x in window.iter_mut() {
                *x = -*x;
            }
        }

        if !symmetric {
            window.pop();
        }
        window
    }
}

pub struct Exponential<T>
where
    T: Float,
{
    pub center: Option<T>,
    pub tau: Option<T>,
}
impl<T> Windows<T> for Exponential<T>
where
    T: Float,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T> {
        if size == 0 {
            return Vec::new();
        }
        if size == 1 {
            return vec![T::one()];
        }

        let n = if symmetric { size } else { size + 1 };
        let denom = num!(n - 1);
        let center = self.center.unwrap_or(denom / num!(2.0));
        // Default time constant: decay to 1/e by the edges of the window.
        let tau: T = match (self.tau, center) {
            (Some(tau), _) => tau,
            (None, center) => center,
        };

        let mut window = Vec::with_capacity(n);
        for i in 0..n {
            let d = num!(i) - center;
            window.push((-d.abs() / tau).exp());
        }

        if !symmetric {
            window.pop();
        }
        window
    }
}

pub struct FlatTop;
impl<T> Windows<T> for FlatTop
where
    T: Float + FloatConst,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T> {
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
}

pub struct Gaussian<T>
where
    T: Float,
{
    standard_deviation: T,
}
impl<T> Windows<T> for Gaussian<T>
where
    T: Float,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T> {
        if size == 0 {
            return Vec::new();
        }
        if size == 1 {
            return vec![T::one()];
        }

        let n = if symmetric { size } else { size + 1 };
        let denom = num!(n - 1);
        let std: T = self.standard_deviation;
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
}

pub struct GeneralCosine;
impl<T> Windows<T> for GeneralCosine
where
    T: Float + FloatConst,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T> {
        // No coefficient list can be passed through this trait, so this
        // defaults to the "HFT90D"-style coefficients used in SciPy's own
        // `general_cosine` example. Adjust this array for other windows in
        // this family.
        let coeffs = [
            num!(1.0),
            num!(1.942604),
            num!(1.340318),
            num!(0.440811),
            num!(0.043097),
        ];
        general_cosine(size, symmetric, &coeffs)
    }
}

pub struct GeneralGaussian<T>
where
    T: Float,
{
    pub shape: T,
    pub standard_deviation: T,
}
impl<T> Windows<T> for GeneralGaussian<T>
where
    T: Float + FloatConst,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T> {
        if size == 0 {
            return Vec::new();
        }
        if size == 1 {
            return vec![T::one()];
        }

        let n = if symmetric { size } else { size + 1 };
        let denom = num!(n - 1);
        let center = denom / num!(2.0);

        // Defaults: p = 1 reduces this to the plain Gaussian window, so pick
        // p > 1 to give it a flatter top with steeper skirts than `Gaussian`;
        // sigma chosen the same way as `Gaussian` (window spans ~+/-3 sigma).
        let p = self.shape;
        let sigma = self.standard_deviation;

        let mut window = Vec::with_capacity(n);
        for i in 0..n {
            let d = num!(i) - center;
            let ratio = (d / sigma).abs();
            let value = (-num!(0.5) * ratio.powf(num!(2.0) * p)).exp();
            window.push(value);
        }

        if !symmetric {
            window.pop();
        }
        window
    }
}
pub struct GeneralHamming<T>
where
    T: Float + FloatConst,
{
    pub alpha: T,
}
impl<T> Windows<T> for GeneralHamming<T>
where
    T: Float + FloatConst,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T> {
        let alpha = self.alpha;
        general_cosine(size, symmetric, &[alpha, T::one() - alpha])
    }
}

pub struct Hamming;
impl<T> Windows<T> for Hamming
where
    T: Float + FloatConst,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T> {
        general_cosine(size, symmetric, &[num!(0.54), num!(0.46)])
    }
}

pub struct Hann;
impl<T> Windows<T> for Hann
where
    T: Float + FloatConst,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T> {
        general_cosine(size, symmetric, &[num!(0.5), num!(0.5)])
    }
}

pub struct Kaiser<T>
where
    T: Float,
{
    beta: T,
}
impl<T> Windows<T> for Kaiser<T>
where
    T: Float,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T> {
        if size == 0 {
            return Vec::new();
        }
        if size == 1 {
            return vec![T::one()];
        }

        let beta = self.beta;
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
}

pub struct KaiserBesselDerived<T>
where
    T: Float,
{
    beta: T,
}
impl<T> Windows<T> for KaiserBesselDerived<T>
where
    T: Float,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T> {
        if size == 0 {
            return Vec::new();
        }
        if size == 1 {
            return vec![T::one()];
        }

        let n = if symmetric { size } else { size + 1 };
        // The construction requires an even length.
        let n_even = n - (n % 2);
        let half_len = n_even / 2;

        let kaiser_half: Vec<T> = Kaiser { beta: self.beta }.window(half_len + 1, true);

        let mut csum = Vec::with_capacity(kaiser_half.len());
        let mut acc = T::zero();
        for &v in &kaiser_half {
            acc = acc + v;
            csum.push(acc);
        }
        let total = csum[csum.len() - 1];

        let mut half_window = Vec::with_capacity(half_len);
        for i in 0..half_len {
            half_window.push((csum[i] / total).sqrt());
        }

        let mut window = half_window.clone();
        window.extend(half_window.iter().rev().cloned());

        // Pad by repeating the midpoint if an odd length was requested.
        while window.len() < n {
            let mid = window[window.len() / 2];
            window.insert(window.len() / 2, mid);
        }

        if !symmetric {
            window.pop();
        }
        window
    }
}

pub struct Lanczos;
impl<T> Windows<T> for Lanczos
where
    T: Float + FloatConst,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T> {
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
}

pub struct Nuttall;
impl<T> Windows<T> for Nuttall
where
    T: Float + FloatConst,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T> {
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
}

pub struct Parzen;
impl<T> Windows<T> for Parzen
where
    T: Float,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T> {
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
}

pub struct Taylor<T> {
    pub nbar: usize,
    pub sll: T,
}
impl<T> Windows<T> for Taylor<T>
where
    T: Float + FloatConst,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T> {
        if size == 0 {
            return Vec::new();
        }
        if size == 1 {
            return vec![T::one()];
        }

        let n = if symmetric { size } else { size + 1 };
        let n_f = num!(n);

        // Defaults matching SciPy: 4 nearly-constant-level sidelobes, 30 dB down.
        let nbar = self.nbar;
        let sll = self.sll;

        let b = num!(10.0).powf(sll / num!(20.0));
        let a = b.acosh() / T::PI();
        let nbar_t = num!(nbar);
        let s2 = nbar_t * nbar_t / (a * a + (nbar_t - num!(0.5)) * (nbar_t - num!(0.5)));

        let ma: Vec<usize> = (1..nbar).collect();

        let calc_fm = |m_idx: usize| -> T {
            let m_val = num!(m_idx);
            let mut numer = T::one();
            for &j in &ma {
                let j_t = num!(j);
                numer = numer
                    * (T::one()
                        - m_val * m_val / s2 / (a * a + (j_t - num!(0.5)) * (j_t - num!(0.5))));
            }
            let sign = if (m_idx + 1) % 2 == 0 {
                T::one()
            } else {
                -T::one()
            };
            numer = numer * sign;

            let mut denom = num!(2.0);
            for &j in &ma {
                if j != m_idx {
                    let j_t = num!(j);
                    denom = denom * (T::one() - m_val * m_val / (j_t * j_t));
                }
            }
            numer / denom
        };

        let fm: Vec<T> = ma.iter().map(|&m_idx| calc_fm(m_idx)).collect();

        let w_fn = |x: T| -> T {
            let mut sum = T::zero();
            for (idx, &m_idx) in ma.iter().enumerate() {
                let m_t = num!(m_idx);
                sum = sum + fm[idx] * (num!(2.0) * T::PI() * m_t * x / n_f).cos();
            }
            T::one() + num!(2.0) * sum
        };

        let center = (n_f - T::one()) / num!(2.0);
        let scale = T::one() / w_fn(center);

        let mut window = Vec::with_capacity(n);
        for i in 0..n {
            let x = num!(i) - center;
            window.push(w_fn(x) * scale);
        }

        if !symmetric {
            window.pop();
        }
        window
    }
}

pub struct Triang;
impl<T> Windows<T> for Triang
where
    T: Float,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T> {
        if size == 0 {
            return Vec::new();
        }
        if size == 1 {
            return vec![T::one()];
        }

        let n = if symmetric { size } else { size + 1 };
        let half = (n + 1) / 2;

        let mut first_half = Vec::with_capacity(half);
        let mut window = if n % 2 == 0 {
            for k in 1..=half {
                first_half.push(num!(2 * k - 1) / num!(n));
            }
            let mut w = first_half.clone();
            w.extend(first_half.iter().rev().cloned());
            w
        } else {
            for k in 1..=half {
                first_half.push(num!(2 * k) / num!(n + 1));
            }
            let mut w = first_half.clone();
            w.extend(first_half[..first_half.len() - 1].iter().rev().cloned());
            w
        };

        if !symmetric {
            window.pop();
        }
        window
    }
}

pub struct Tukey<T> {
    pub alpha: T,
}
impl<T> Windows<T> for Tukey<T>
where
    T: Float + FloatConst,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T> {
        if size == 0 {
            return Vec::new();
        }
        if size == 1 {
            return vec![T::one()];
        }

        let n = if symmetric { size } else { size + 1 };

        // Default taper fraction.
        let alpha = self.alpha;
        let denom = num!(n - 1);
        let width = ((0.5_f64 * (n as f64 - 1.0)) / 2.0).floor() as usize;

        let mut window = Vec::with_capacity(n);
        for i in 0..n {
            let value = if i <= width {
                num!(0.5)
                    * (T::one()
                        + (T::PI() * (num!(-1.0) + num!(2.0) * num!(i) / (alpha * denom))).cos())
            } else if i >= n.saturating_sub(width + 1) {
                num!(0.5)
                    * (T::one()
                        + (T::PI()
                            * (num!(-2.0) / alpha
                                + T::one()
                                + num!(2.0) * num!(i) / (alpha * denom)))
                            .cos())
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
}
#[cfg(test)]
mod tests {
    extern crate std;
    use core::marker::PhantomData;
    use std::format;

    use rstest::rstest;

    use super::*;
    #[rstest]
    #[case(PhantomData::<Barthnn>, Barthnn, "barthann")]
    #[case(PhantomData::<Bartlett>, Bartlett, "bartlett")]
    #[case(PhantomData::<Blackman>, Blackman, "blackman")]
    #[case(PhantomData::<BlackmanHarris>, BlackmanHarris, "blackmanharris")]
    #[case(PhantomData::<Bohman>, Bohman, "bohman")]
    #[case(PhantomData::<Boxcar>, Boxcar, "boxcar")]
    #[case(PhantomData::<Chebwin<f64>>, Chebwin{ attenuation: 100.0 }, "chebwin")]
    #[case(PhantomData::<Cosine>, Cosine, "cosine")]
    #[case(PhantomData::<Dpss<f64>>, Dpss{ nw: 3.0 }, "dpss")]
    #[case(PhantomData::<Exponential<f64>>, Exponential{ center: None, tau: None }, "exponential")]
    #[case(PhantomData::<FlatTop>, FlatTop, "flattop")]
    #[case(PhantomData::<Gaussian<f64>>, Gaussian{ standard_deviation:0.5}, "gaussian")]
    #[case(PhantomData::<GeneralCosine>, GeneralCosine, "general_cosine")]
    #[case(PhantomData::<GeneralGaussian<f64>>, GeneralGaussian{ shape: 1.0, standard_deviation: 0.5 }, "general_gaussian")]
    #[case(PhantomData::<GeneralHamming<f64>>, GeneralHamming{alpha:0.5}, "general_hamming")]
    #[case(PhantomData::<Hamming>, Hamming, "hamming")]
    #[case(PhantomData::<Hann>, Hann, "hann")]
    #[case(PhantomData::<Kaiser<f64>>, Kaiser{ beta: 0.5 }, "kaiser")]
    #[case(PhantomData::<KaiserBesselDerived<f64>>, KaiserBesselDerived{ beta: 0.5 }, "kaiser_bessel_derived")]
    #[case(PhantomData::<Lanczos>, Lanczos, "lanczos")]
    #[case(PhantomData::<Nuttall>, Nuttall, "nuttall")]
    #[case(PhantomData::<Parzen>, Parzen, "parzen")]
    #[case(PhantomData::<Taylor<f64>>, Taylor{nbar:4,sll:100.0}, "taylor")]
    #[case(PhantomData::<Triang>, Triang, "triang")]
    #[case(PhantomData::<Tukey<f64>>, Tukey{alpha: 0.5}, "tukey")]
    fn test<T>(
        #[case] _window: PhantomData<T>,
        #[case] window: T,
        #[case] name: &str,
        #[values(true, false)] symmetric: bool,
    ) where
        T: Windows<f64>,
    {
        let window: Vec<f64> = window.window(10, symmetric);
        insta::assert_debug_snapshot!(
            format!(
                "{}-{}",
                name,
                if symmetric { "symmetric" } else { "asymmetric" }
            ),
            window
        );
    }
}
