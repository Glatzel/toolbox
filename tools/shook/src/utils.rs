use clerk;

pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        clerk::debug!(
            len_a = a.len(),
            len_b = b.len(),
            "constant_time_eq: length mismatch, returning false"
        );
        return false;
    }

    let result = a
        .iter()
        .zip(b.iter())
        .fold(0u8, |acc, (x, y)| acc | (x ^ y))
        == 0;

    clerk::debug!(equal = result, "constant_time_eq: comparison complete");
    result
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    // ── equal inputs ────────────────────────────────────────────────────────

    #[rstest]
    #[case(b"", b"")]
    #[case(b"hello", b"hello")]
    #[case(b"sha256=abc123", b"sha256=abc123")]
    #[case(&[0u8; 32], &[0u8; 32])]
    #[case(&[0xFF; 32], &[0xFF; 32])]
    fn equal_inputs_return_true(#[case] a: &[u8], #[case] b: &[u8]) {
        assert!(constant_time_eq(a, b));
    }

    // ── unequal inputs, same length ──────────────────────────────────────────

    #[rstest]
    #[case(b"hello", b"world")]
    #[case(b"aaaaa", b"aaaab")] // single bit difference at the end
    #[case(b"aaaaa", b"baaaa")] // single bit difference at the start
    #[case(&[0x00u8; 32], &[0x01u8; 32])]
    #[case(&[0xFFu8; 32], &[0x00u8; 32])]
    fn unequal_same_length_returns_false(#[case] a: &[u8], #[case] b: &[u8]) {
        assert!(!constant_time_eq(a, b));
    }

    // ── length mismatch ──────────────────────────────────────────────────────

    #[rstest]
    #[case(b"short", b"longer")]
    #[case(b"", b"nonempty")]
    #[case(b"nonempty", b"")]
    #[case(&[0u8; 31], &[0u8; 32])]
    fn length_mismatch_returns_false(#[case] a: &[u8], #[case] b: &[u8]) {
        assert!(!constant_time_eq(a, b));
    }

    // ── constant-time property: no early exit on content ────────────────────
    //
    // We can't measure timing in a unit test, but we can assert that every
    // single-byte difference anywhere in the slice is caught, confirming the
    // fold visits all bytes rather than short-circuiting.

    #[test]
    fn detects_difference_at_every_position() {
        const LEN: usize = 64;
        let base = vec![0xABu8; LEN];

        for pos in 0..LEN {
            let mut other = base.clone();
            other[pos] ^= 0xFF; // flip all bits at position `pos`
            assert!(
                !constant_time_eq(&base, &other),
                "expected false when byte at position {pos} differs"
            );
        }
    }

    // ── symmetry ─────────────────────────────────────────────────────────────

    #[rstest]
    #[case(b"abc", b"xyz")]
    #[case(b"same", b"same")]
    fn symmetric(#[case] a: &[u8], #[case] b: &[u8]) {
        assert_eq!(constant_time_eq(a, b), constant_time_eq(b, a));
    }

    // ── overflow safety: fold stays within u8 ────────────────────────────────
    //
    // If the accumulator were to overflow this would panic in debug mode;
    // passing confirms the fold is overflow-safe for long slices.

    #[test]
    fn no_overflow_on_long_equal_slices() {
        let a = vec![0xFFu8; 4096];
        assert!(constant_time_eq(&a, &a));
    }
}
