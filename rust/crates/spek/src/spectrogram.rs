#[derive(Debug)]
pub struct Spectrogram<T> {
    data: Vec<T>,
    frame_count: usize,
    bin_count: usize,
}
impl<T> Spectrogram<T> {
    pub const fn new(data: Vec<T>, frame_count: usize, bin_count: usize) -> Self {
        Self {
            data,
            frame_count,
            bin_count,
        }
    }
    pub fn data(&self) -> &[T] { &self.data }
    pub const fn frame_count(&self) -> usize { self.frame_count }
    pub const fn bin_count(&self) -> usize { self.bin_count }
}
