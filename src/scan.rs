use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const SUPPORTED: &[&str] = &["jpg", "jpeg", "png", "tiff", "tif", "webp"];

/// Pure: decides if a path has a supported image extension
pub fn is_supported_image(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
        .as_deref()
        .map(|ext| SUPPORTED.contains(&ext))
        .unwrap_or(false)
}

/// Impure: walks input_dir and returns sorted list of image paths
pub fn discover_images(input_dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut paths: Vec<PathBuf> = WalkDir::new(input_dir)
        .min_depth(1)
        .max_depth(1) // flat scan — subdirs are separate categories
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .filter(|p| is_supported_image(p))
        .collect();

    paths.sort(); // deterministic order every run
    Ok(paths)
}
