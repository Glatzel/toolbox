pub trait SplitAtUnsafe {
    /// Splits `self` into two halves at byte offset `idx`, without checking
    /// that `idx` is a valid boundary for the underlying data.
    ///
    /// # Safety
    ///
    /// `idx` must be a valid UTF-8 char boundary within `self`
    /// (0..=self.len()). Passing an index that falls inside a multi-byte
    /// character, or that is out of bounds, is undefined behavior.
    unsafe fn split_at_unsafe(&self, idx: usize) -> (&Self, &Self);
}

impl SplitAtUnsafe for str {
    #[inline(always)]
    unsafe fn split_at_unsafe(&self, idx: usize) -> (&str, &str) {
        // SAFETY: caller guarantees `idx` is a valid char boundary in `self`.
        unsafe { (self.get_unchecked(..idx), self.get_unchecked(idx..)) }
    }
}
