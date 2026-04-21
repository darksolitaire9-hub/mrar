use std::io::Cursor;
use std::path::PathBuf;

// ── helpers ───────────────────────────────────────────────────────────────────

fn fixture_path() -> Option<PathBuf> {
    std::env::var("MRAR_TEST_IMAGE").ok().map(PathBuf::from)
}

/// Build a minimal in-memory PNG of given dimensions — no disk I/O.
fn make_png(w: u32, h: u32) -> Vec<u8> {
    let img = image::RgbImage::new(w, h);
    let mut buf = Vec::new();
    image::DynamicImage::ImageRgb8(img)
        .write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)
        .unwrap();
    buf
}

// ── strip_all ─────────────────────────────────────────────────────────────────

#[test]
fn strips_and_renames_jpeg() {
    let Some(path) = fixture_path() else {
        eprintln!("skipped: set MRAR_TEST_IMAGE to run this test");
        return;
    };

    let bytes = std::fs::read(&path).unwrap();
    let cleaned = mrar::pipeline::metadata::strip_all(&bytes).unwrap();

    assert!(cleaned.len() <= bytes.len());
    assert_eq!(&cleaned[..2], &[0xFF, 0xD8]); // still valid JPEG
}

// ── compress ──────────────────────────────────────────────────────────────────

#[test]
fn compress_small_image_stays_same_dimensions() {
    let buf = make_png(100, 100);
    let out = mrar::pipeline::compress::compress(&buf, Some(1920), 90, "png").unwrap();
    let decoded = image::load_from_memory(&out).unwrap();
    assert_eq!(decoded.width(), 100);
    assert_eq!(decoded.height(), 100);
}

#[test]
fn compress_large_image_fits_within_max_dim() {
    let buf = make_png(3000, 2000);
    let out = mrar::pipeline::compress::compress(&buf, Some(1920), 90, "png").unwrap();
    let decoded = image::load_from_memory(&out).unwrap();
    assert!(
        decoded.width() <= 1920,
        "width {} exceeds max_dim",
        decoded.width()
    );
    assert!(
        decoded.height() <= 1920,
        "height {} exceeds max_dim",
        decoded.height()
    );
}

#[test]
fn compress_none_max_dim_still_encodes() {
    let buf = make_png(200, 200);
    let out = mrar::pipeline::compress::compress(&buf, None, 90, "png").unwrap();
    assert!(!out.is_empty());
    let decoded = image::load_from_memory(&out).unwrap();
    assert_eq!(decoded.width(), 200);
}

#[test]
fn compress_aspect_ratio_preserved() {
    let buf = make_png(3000, 1500); // 2:1 landscape
    let out = mrar::pipeline::compress::compress(&buf, Some(1920), 90, "png").unwrap();
    let decoded = image::load_from_memory(&out).unwrap();
    assert_eq!(decoded.width(), 1920);
    assert_eq!(decoded.height(), 960);
}

// ── Config ────────────────────────────────────────────────────────────────────

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
