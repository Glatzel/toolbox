mod spectrum2d;
pub use spectrum2d::Spectrum2D;
pub type Spectrum1D<T> = (Vec<T>, Vec<T>);
pub type Spectrum1DRef<'a, T> = (&'a [T], &'a [T]);
pub type Spectrum1DRefMut<'a, T> = (&'a mut [T], &'a mut [T]);
pub type Scratch<T> = Spectrum1D<T>;
pub type ScratchRef<'a, T> = Spectrum1DRef<'a, T>;
pub type ScratchRefMut<'a, T> = Spectrum1DRefMut<'a, T>;
