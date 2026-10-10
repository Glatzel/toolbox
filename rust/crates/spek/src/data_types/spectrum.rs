pub type Spectrum<T> = (Vec<T>, Vec<T>);
pub type SpectrumRef<'a, T> = (&'a [T], &'a [T]);
pub type SpectrumRefMut<'a, T> = (&'a mut [T], &'a mut [T]);
pub type Scratch<T> = Spectrum<T>;
pub type ScratchRef<'a, T> = SpectrumRef<'a, T>;
pub type ScratchRefMut<'a, T> = SpectrumRefMut<'a, T>;
