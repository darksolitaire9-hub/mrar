use anyhow::Context;
use fast_image_resize::images::Image;
use fast_image_resize::{PixelType, Resizer};
use image::{ColorType, DynamicImage, ImageFormat};
use std::io::Cursor;

/// Pure: compress image bytes.
/// - Decodes with `image` crate
/// - Resizes with `fast_image_resize` (SIMD AVX2/SSE4.1 auto-detected)
/// - Re-encodes to original format
/// Images already within max_dim are re-encoded only (no resize).
/// max_dim = None skips resize entirely.
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
            resize_fast(img, dim).context("compress: resize failed")?
        }
        _ => img,
    };

    encode(resized, fmt, quality)
}

/// Pure: SIMD-accelerated resize using fast_image_resize.
/// Preserves aspect ratio. Uses Lanczos3 filter.
fn resize_fast(img: DynamicImage, max_dim: u32) -> anyhow::Result<DynamicImage> {
    let (orig_w, orig_h) = (img.width(), img.height());

    let (target_w, target_h) = if orig_w >= orig_h {
        let h = (orig_h as f64 * max_dim as f64 / orig_w as f64).round() as u32;
        (max_dim, h.max(1))
    } else {
        let w = (orig_w as f64 * max_dim as f64 / orig_h as f64).round() as u32;
        (w.max(1), max_dim)
    };

    let rgba = img.to_rgba8();
    let src = Image::from_vec_u8(orig_w, orig_h, rgba.into_raw(), PixelType::U8x4)
        .context("compress: failed to create src image")?;

    let mut dst = Image::new(target_w, target_h, PixelType::U8x4);

    Resizer::new()
        .resize(&src, &mut dst, None)
        .context("compress: fast_image_resize failed")?;

    let out = image::RgbaImage::from_raw(target_w, target_h, dst.into_vec())
        .context("compress: failed to reconstruct image")?;

    Ok(DynamicImage::ImageRgba8(out))
}

/// Pure: encode DynamicImage to bytes for the given format.
fn encode(img: DynamicImage, fmt: ImageFormat, quality: u8) -> anyhow::Result<Vec<u8>> {
    let mut out: Vec<u8> = Vec::new();
    match fmt {
        ImageFormat::Jpeg => {
            // JPEG does not support alpha — convert to Rgb8 first
            let rgb = img.to_rgb8();
            image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, quality)
                .encode(
                    rgb.as_raw(),
                    rgb.width(),
                    rgb.height(),
                    ColorType::Rgb8.into(),
                )
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

    #[test]
    fn aspect_ratio_preserved() {
        let buf = make_png(3000, 1500); // 2:1 landscape
        let out = compress(&buf, Some(1920), 90, "png").unwrap();
        let decoded = image::load_from_memory(&out).unwrap();
        assert_eq!(decoded.width(), 1920);
        assert_eq!(decoded.height(), 960);
    }
}
