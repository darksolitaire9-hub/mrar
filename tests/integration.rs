// tests/integration.rs
use std::path::PathBuf;

fn fixture_path() -> Option<PathBuf> {
    // reads MRAR_TEST_IMAGE from environment
    std::env::var("MRAR_TEST_IMAGE").ok().map(PathBuf::from)
}

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
