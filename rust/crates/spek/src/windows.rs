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

pub struct Dpss<T> {
    pub nw: T,
}
impl<T> Windows<T> for Dpss<T>
where
    T: Float + FloatConst,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T> {
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
            for i in 0..n {
                q[i][i] = T::one();
            }

            let eps = T::epsilon().sqrt();

            for _ in 0..1000 {
                // Find largest off-diagonal element.
                let mut p = 0;
                let mut r = 1;
                let mut max = T::zero();

                for i in 0..n {
                    for j in (i + 1)..n {
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
                for k in 0..n {
                    let qkp = q[k][p];
                    let qkr = q[k][r];

                    q[k][p] = c * qkp - s * qkr;
                    q[k][r] = s * qkp + c * qkr;
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
        let nw = self.nw;

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

pub struct GeneralCosine<T> {
    pub coefficients: Vec<T>,
}

impl<T> Windows<T> for GeneralCosine<T>
where
    T: Float + FloatConst,
{
    fn window(&self, size: usize, symmetric: bool) -> Vec<T> {
        general_cosine(size, symmetric, &self.coefficients)
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
        /// Modified Bessel function of the first kind, order 0, via its power
        /// series. Used by the Kaiser / Kaiser-Bessel-derived windows.
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
    pub norm: bool,
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

        // Same as SciPy _extend(M, sym).
        let n = if symmetric { size } else { size + 1 };
        let n_f = num!(n);

        let nbar = self.nbar;
        let sll = self.sll;

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
        if self.norm {
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

        let alpha = self.alpha;

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
            return Hann.window(size, symmetric);
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
                        + (T::PI() * (-num!(2.0) / alpha + T::one() + num!(2.0) * i_f / denom))
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
    #[case(PhantomData::<GeneralCosine<f64>>, GeneralCosine{ coefficients: vec![1.0f64, 1.942604, 1.340318, 0.440811, 0.043097] }, "general_cosine")]
    #[case(PhantomData::<GeneralGaussian<f64>>, GeneralGaussian{ shape: 1.0, standard_deviation: 0.5 }, "general_gaussian")]
    #[case(PhantomData::<GeneralHamming<f64>>, GeneralHamming{alpha:0.5}, "general_hamming")]
    #[case(PhantomData::<Hamming>, Hamming, "hamming")]
    #[case(PhantomData::<Hann>, Hann, "hann")]
    #[case(PhantomData::<Kaiser<f64>>, Kaiser{ beta: 0.5 }, "kaiser")]
    #[case(PhantomData::<KaiserBesselDerived<f64>>, KaiserBesselDerived{ beta: 0.5 }, "kaiser_bessel_derived")]
    #[case(PhantomData::<Lanczos>, Lanczos, "lanczos")]
    #[case(PhantomData::<Nuttall>, Nuttall, "nuttall")]
    #[case(PhantomData::<Parzen>, Parzen, "parzen")]
    #[case(PhantomData::<Taylor<f64>>, Taylor{nbar:4,sll:100.0,norm:true}, "taylor")]
    #[case(PhantomData::<Triang>, Triang, "triang")]
    #[case(PhantomData::<Tukey<f64>>, Tukey{alpha: 1.0}, "tukey")]
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
