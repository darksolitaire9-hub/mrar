// tests/pipeline.rs

use std::io::Cursor;
use std::path::PathBuf;

/// Build a minimal in-memory PNG of given dimensions — no disk I/O.
fn make_png(w: u32, h: u32) -> Vec<u8> {
    let img = image::RgbImage::new(w, h);
    let mut buf = Vec::new();
    image::DynamicImage::ImageRgb8(img)
        .write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)
        .unwrap();
    buf
}

// ── strip_all_security ───────────────────────────────────────────────────────

#[test]
fn strip_all_handles_garbage_bytes() {
    // Completely random bytes, not a valid image.
    let data = vec![0u8; 1024];
    let result = mrar::pipeline::metadata::strip_all(&data);
    assert!(
        result.is_err(),
        "strip_all should return an error for invalid/garbage image data"
    );
}

#[test]
fn strip_all_handles_truncated_jpeg() {
    // Requires MRAR_TEST_IMAGE to point to a real JPEG.
    let Some(path) = std::env::var("MRAR_TEST_IMAGE").ok().map(PathBuf::from) else {
        eprintln!("skipped: set MRAR_TEST_IMAGE to run this test");
        return;
    };

    let mut bytes = std::fs::read(&path).unwrap();
    bytes.truncate(bytes.len() / 4); // cut most of it off

    let result = mrar::pipeline::metadata::strip_all(&bytes);
    assert!(
        result.is_err(),
        "strip_all should not accept truncated JPEG as valid"
    );
}

// ── compress_security ────────────────────────────────────────────────────────

#[test]
fn compress_rejects_invalid_image_data() {
    // Not a valid PNG or JPEG by any stretch.
    let data = vec![1, 2, 3, 4, 5];
    let result = mrar::pipeline::compress::compress(&data, Some(1920), 90, "png");
    assert!(
        result.is_err(),
        "compress should return an error for invalid image data"
    );
}

#[test]
fn compress_does_not_panic_on_large_dimensions() {
    // Very large in-memory PNG; validate it still returns an error or result, but doesn't panic.
    let buf = make_png(8000, 8000);
    let result = mrar::pipeline::compress::compress(&buf, Some(4096), 90, "png");
    assert!(
        result.is_ok() || result.is_err(),
        "compress should not panic on large dimensions"
    );
}

// ── Config safety ────────────────────────────────────────────────────────────

#[test]
fn config_defaults_shrink_to_none() {
    use mrar::cli::{CliArgs, Config};
    let args = CliArgs {
        input: Some(PathBuf::from(".")),
        output: None,
        start: 1,
        pad: 3,
        ext: None,
        shrink: None,
        quality: 90,
        dry_run: false,
    };
    let config = Config::resolve(args);
    assert!(config.shrink.is_none());
}

#[test]
fn config_clamps_quality() {
    use mrar::cli::{CliArgs, Config};
    let args = CliArgs {
        input: Some(PathBuf::from(".")),
        output: None,
        start: 1,
        pad: 3,
        ext: None,
        shrink: Some(1920),
        quality: 0, // invalid — should clamp to 1
        dry_run: false,
    };
    let config = Config::resolve(args);
    assert_eq!(config.quality, 1);
}

#[test]
fn config_output_defaults_to_input_slash_output() {
    use mrar::cli::{CliArgs, Config};
    let args = CliArgs {
        input: Some(PathBuf::from("/tmp/photos")),
        output: None,
        start: 1,
        pad: 3,
        ext: None,
        shrink: None,
        quality: 90,
        dry_run: false,
    };
    let config = Config::resolve(args);
    assert_eq!(config.output_dir, PathBuf::from("/tmp/photos/output"));
}
