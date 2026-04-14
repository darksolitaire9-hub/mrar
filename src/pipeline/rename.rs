use std::path::{Path, PathBuf};

/// Pure: generates a zero-padded sequential filename
pub fn make_target_name(index: u32, pad: usize, ext: &str) -> String {
    format!("{:0>width$}.{}", index, ext, width = pad)
}

/// Pure: resolves the target path for a work item
pub fn resolve_target_path(
    original: &Path,
    output_dir: &Path,
    index: u32,
    pad: usize,
    force_ext: Option<&str>,
) -> PathBuf {
    let ext = force_ext
        .or_else(|| original.extension().and_then(|e| e.to_str()))
        .unwrap_or("jpg")
        .to_lowercase();

    let filename = make_target_name(index, pad, &ext);
    output_dir.join(filename)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pads_correctly() {
        assert_eq!(make_target_name(1, 3, "jpg"), "001.jpg");
        assert_eq!(make_target_name(42, 3, "jpg"), "042.jpg");
        assert_eq!(make_target_name(1000, 3, "jpg"), "1000.jpg"); // overflow gracefully
    }

    #[test]
    fn uses_force_ext() {
        let path = PathBuf::from("photo.png");
        let out = resolve_target_path(&path, Path::new("/out"), 1, 3, Some("jpg"));
        assert_eq!(out, PathBuf::from("/out/001.jpg"));
    }

    #[test]
    fn preserves_original_ext() {
        let path = PathBuf::from("photo.webp");
        let out = resolve_target_path(&path, Path::new("/out"), 5, 3, None);
        assert_eq!(out, PathBuf::from("/out/005.webp"));
    }
}
