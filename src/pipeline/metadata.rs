/// Pure: strips all metadata from raw image bytes.
/// Returns clean bytes or a metastrip error.
pub fn strip_all(bytes: &[u8]) -> Result<Vec<u8>, metastrip::Error> {
    metastrip::strip_metadata(bytes)
}

/// Pure: describes what metadata was present before stripping.
pub fn bytes_saved(before: usize, after: usize) -> u64 {
    before.saturating_sub(after) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_saved_never_panics_on_growth() {
        // metastrip can add bytes in rare cases (re-encoding)
        assert_eq!(bytes_saved(100, 200), 0); // saturating sub
        assert_eq!(bytes_saved(1000, 800), 200);
    }
}
