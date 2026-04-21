use anyhow::Context;
use image::{ImageFormat, imageops::FilterType};
use std::io::Cursor;

/// Pure: compress image bytes.
/// - If image already fits within max_dim: re-encodes at `quality` only (no resize).
/// - If image exceeds max_dim: resize with Lanczos3, then re-encode.
/// - PNG/TIFF/WebP: lossless re-encode (quality param ignored for non-JPEG).
/// Returns new bytes, or original bytes if already optimal.
pub fn compress(
    buf: &[u8],
    max_dim: Option<u32>,
    quality: u8,
    ext: &str,
) -> anyhow::Result<Vec<u8>> {
    let fmt = format_for_ext(ext);
    let img = image::load_from_memory(buf).context("compress: failed to decode image")?;

    let resized = match max_dim {
        Some(dim) if img.width() > dim || img.height() > dim => {
            img.resize(dim, dim, FilterType::Lanczos3)
        }
        _ => img,
    };

    encode(resized, fmt, quality)
}

/// Pure: encode a DynamicImage into bytes for the given format.
fn encode(img: image::DynamicImage, fmt: ImageFormat, quality: u8) -> anyhow::Result<Vec<u8>> {
    let mut out: Vec<u8> = Vec::new();

    match fmt {
        ImageFormat::Jpeg => {
            image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, quality)
                .encode_image(&img)
                .context("compress: JPEG encode failed")?;
        }
        _ => {
            img.write_to(&mut Cursor::new(&mut out), fmt)
                .context("compress: encode failed")?;
        }
    }

    Ok(out)
}

/// Pure: map lowercase extension string to ImageFormat.
pub fn format_for_ext(ext: &str) -> ImageFormat {
    match ext.to_ascii_lowercase().as_str() {
        "jpg" | "jpeg" => ImageFormat::Jpeg,
        "png" => ImageFormat::Png,
        "tiff" | "tif" => ImageFormat::Tiff,
        "webp" => ImageFormat::WebP,
        _ => ImageFormat::Png,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_png(w: u32, h: u32) -> Vec<u8> {
        let img = image::RgbImage::new(w, h);
        let mut buf = Vec::new();
        image::DynamicImage::ImageRgb8(img)
            .write_to(&mut Cursor::new(&mut buf), ImageFormat::Png)
            .unwrap();
        buf
    }

    #[test]
    fn small_image_not_resized() {
        let buf = make_png(100, 100);
        let out = compress(&buf, Some(1920), 90, "png").unwrap();
        let decoded = image::load_from_memory(&out).unwrap();
        assert_eq!(decoded.width(), 100);
        assert_eq!(decoded.height(), 100);
    }

    #[test]
    fn large_image_is_shrunk_to_max_dim() {
        let buf = make_png(3000, 2000);
        let out = compress(&buf, Some(1920), 90, "png").unwrap();
        let decoded = image::load_from_memory(&out).unwrap();
        assert!(decoded.width() <= 1920 && decoded.height() <= 1920);
    }

    #[test]
    fn no_max_dim_still_encodes() {
        let buf = make_png(200, 200);
        let out = compress(&buf, None, 90, "png").unwrap();
        assert!(!out.is_empty());
    }
}
