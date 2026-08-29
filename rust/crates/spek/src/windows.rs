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
    fn window(size: usize, symmetric: bool) -> Vec<T>;
}

pub struct Barthnn;
impl<T> Windows<T> for Barthnn
where
    T: Float + FloatConst,
{
    fn window(size: usize, symmetric: bool) -> Vec<T> {
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
    fn window(size: usize, symmetric: bool) -> Vec<T> {
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

pub struct Blackman;
impl<T> Windows<T> for Blackman
where
    T: Float,
{
    fn window(size: usize, symmetric: bool) -> Vec<T> { todo!() }
}

pub struct BlackmanHarris;
impl<T> Windows<T> for BlackmanHarris
where
    T: Float,
{
    fn window(size: usize, symmetric: bool) -> Vec<T> { todo!() }
}

pub struct Bohman;
impl<T> Windows<T> for Bohman
where
    T: Float,
{
    fn window(size: usize, symmetric: bool) -> Vec<T> { todo!() }
}

pub struct Boxcar;
impl<T> Windows<T> for Boxcar
where
    T: Float,
{
    fn window(size: usize, symmetric: bool) -> Vec<T> { todo!() }
}

pub struct Chebwin;
impl<T> Windows<T> for Chebwin
where
    T: Float,
{
    fn window(size: usize, symmetric: bool) -> Vec<T> { todo!() }
}

pub struct Cosine;
impl<T> Windows<T> for Cosine
where
    T: Float,
{
    fn window(size: usize, symmetric: bool) -> Vec<T> { todo!() }
}

pub struct Dpss;
impl<T> Windows<T> for Dpss
where
    T: Float,
{
    fn window(size: usize, symmetric: bool) -> Vec<T> { todo!() }
}

pub struct Exponential;
impl<T> Windows<T> for Exponential
where
    T: Float,
{
    fn window(size: usize, symmetric: bool) -> Vec<T> { todo!() }
}

pub struct FlatTop;
impl<T> Windows<T> for FlatTop
where
    T: Float,
{
    fn window(size: usize, symmetric: bool) -> Vec<T> { todo!() }
}

pub struct Gaussian;
impl<T> Windows<T> for Gaussian
where
    T: Float,
{
    fn window(size: usize, symmetric: bool) -> Vec<T> { todo!() }
}

pub struct GeneralCosine;
impl<T> Windows<T> for GeneralCosine
where
    T: Float,
{
    fn window(size: usize, symmetric: bool) -> Vec<T> { todo!() }
}

pub struct GeneralHamming;
impl<T> Windows<T> for GeneralHamming
where
    T: Float,
{
    fn window(size: usize, symmetric: bool) -> Vec<T> { todo!() }
}

pub struct Hamming;
impl<T> Windows<T> for Hamming
where
    T: Float,
{
    fn window(size: usize, symmetric: bool) -> Vec<T> { todo!() }
}

pub struct Hann;
impl<T> Windows<T> for Hann
where
    T: Float,
{
    fn window(size: usize, symmetric: bool) -> Vec<T> { todo!() }
}

pub struct Kaiser;
impl<T> Windows<T> for Kaiser
where
    T: Float,
{
    fn window(size: usize, symmetric: bool) -> Vec<T> { todo!() }
}

pub struct KaiserBesselDerived;
impl<T> Windows<T> for KaiserBesselDerived
where
    T: Float,
{
    fn window(size: usize, symmetric: bool) -> Vec<T> { todo!() }
}

pub struct Lanczos;
impl<T> Windows<T> for Lanczos
where
    T: Float,
{
    fn window(size: usize, symmetric: bool) -> Vec<T> { todo!() }
}

pub struct Nuttall;
impl<T> Windows<T> for Nuttall
where
    T: Float,
{
    fn window(size: usize, symmetric: bool) -> Vec<T> { todo!() }
}

pub struct Parzen;
impl<T> Windows<T> for Parzen
where
    T: Float,
{
    fn window(size: usize, symmetric: bool) -> Vec<T> { todo!() }
}

pub struct Taylor;
impl<T> Windows<T> for Taylor
where
    T: Float,
{
    fn window(size: usize, symmetric: bool) -> Vec<T> { todo!() }
}

pub struct Triang;
impl<T> Windows<T> for Triang
where
    T: Float,
{
    fn window(size: usize, symmetric: bool) -> Vec<T> { todo!() }
}

pub struct Tukey;
impl<T> Windows<T> for Tukey
where
    T: Float,
{
    fn window(size: usize, symmetric: bool) -> Vec<T> { todo!() }
}
#[cfg(test)]
mod tests {
    extern crate std;
    use core::marker::PhantomData;
    use std::format;

    use rstest::rstest;

    use super::*;
    #[rstest]
    #[case(PhantomData::<Barthnn>)]
    #[case(PhantomData::<Bartlett>)]
    #[case(PhantomData::<Blackman>)]
    #[case(PhantomData::<BlackmanHarris>)]
    #[case(PhantomData::<Bohman>)]
    #[case(PhantomData::<Boxcar>)]
    #[case(PhantomData::<Chebwin>)]
    #[case(PhantomData::<Cosine>)]
    #[case(PhantomData::<Dpss>)]
    #[case(PhantomData::<Exponential>)]
    #[case(PhantomData::<FlatTop>)]
    #[case(PhantomData::<Gaussian>)]
    #[case(PhantomData::<GeneralCosine>)]
    #[case(PhantomData::<GeneralHamming>)]
    #[case(PhantomData::<Hamming>)]
    #[case(PhantomData::<Hann>)]
    #[case(PhantomData::<Kaiser>)]
    #[case(PhantomData::<KaiserBesselDerived>)]
    #[case(PhantomData::<Lanczos>)]
    #[case(PhantomData::<Nuttall>)]
    #[case(PhantomData::<Parzen>)]
    #[case(PhantomData::<Taylor>)]
    #[case(PhantomData::<Triang>)]
    #[case(PhantomData::<Tukey>)]
    fn test<T>(#[case] _window: PhantomData<T>, #[values(true, false)] symmetric: bool)
    where
        T: Windows<f64>,
    {
        let window: Vec<f64> = T::window(10, symmetric);
        insta::assert_debug_snapshot!(
            format!(
                "{}-{}",
                std::any::type_name::<T>().split(":").last().unwrap_or(""),
                if symmetric { "symmetric" } else { "asymmetric" }
            ),
            window
        );
    }
}
