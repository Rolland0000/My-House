//! Image checks beyond the magic-byte validation of `file_validation`.
//!
//! Public images (listing photos, avatars) are decoded and re-encoded before
//! storage: the output carries no metadata (EXIF GPS included), nothing that
//! was appended to or hidden in the original file, and pixels only. Private
//! identity documents keep their original bytes and get a dimension check.

use std::io::Cursor;

use bytes::Bytes;
use image::codecs::jpeg::JpegEncoder;
use image::{DynamicImage, ImageDecoder, ImageFormat, ImageReader, Limits};
use tokio::sync::Semaphore;

use crate::shared::errors::AppError;
use crate::shared::file_validation::ValidatedFile;

/// Largest width or height accepted, enough for a 48 Mpx phone photo.
pub const MAX_IMAGE_DIMENSION_PX: u32 = 8000;

/// Decoder allocation ceiling: bounds the memory a crafted header can claim.
const MAX_DECODE_ALLOC_BYTES: u64 = 256 * 1024 * 1024;

const JPEG_QUALITY: u8 = 85;

/// Re-encodes running at once. Each holds a decoded image in memory and a CPU
/// core, so concurrent uploads queue instead of exhausting a small server.
const MAX_CONCURRENT_REENCODES: usize = 2;

static REENCODE_PERMITS: Semaphore = Semaphore::const_new(MAX_CONCURRENT_REENCODES);

/// A re-encoded image and the format it was written in.
pub struct ReencodedImage {
    pub bytes: Bytes,
    pub extension: &'static str,
    pub content_type: &'static str,
}

fn image_format(validated: &ValidatedFile) -> Result<ImageFormat, AppError> {
    match validated.content_type {
        "image/jpeg" => Ok(ImageFormat::Jpeg),
        "image/png" => Ok(ImageFormat::Png),
        "image/webp" => Ok(ImageFormat::WebP),
        _ => Err(AppError::InvalidFile),
    }
}

/// The format comes from the magic-byte validation, never from a guess.
fn reader(bytes: &[u8], format: ImageFormat) -> ImageReader<Cursor<&[u8]>> {
    let mut limits = Limits::default();
    limits.max_image_width = Some(MAX_IMAGE_DIMENSION_PX);
    limits.max_image_height = Some(MAX_IMAGE_DIMENSION_PX);
    limits.max_alloc = Some(MAX_DECODE_ALLOC_BYTES);

    let mut reader = ImageReader::with_format(Cursor::new(bytes), format);
    reader.limits(limits);
    reader
}

/// Reads the dimensions from the header only and refuses an empty or
/// oversized image, without decoding its pixels.
pub fn check_image_dimensions(bytes: &[u8], validated: &ValidatedFile) -> Result<(), AppError> {
    let (width, height) = reader(bytes, image_format(validated)?)
        .into_dimensions()
        .map_err(|_| AppError::InvalidFile)?;
    let within = |side: u32| (1..=MAX_IMAGE_DIMENSION_PX).contains(&side);
    if within(width) && within(height) {
        Ok(())
    } else {
        Err(AppError::InvalidFile)
    }
}

/// Decodes a public image within the limits above, applies its EXIF rotation
/// (phones store it there, not in the pixels) and writes the pixels back.
/// JPEG stays JPEG and PNG stays PNG; WebP becomes JPEG, or PNG when it has
/// transparency, because the `image` WebP encoder is lossless only and would
/// inflate photos.
pub async fn reencode_public_image(
    bytes: Bytes,
    validated: &ValidatedFile,
) -> Result<ReencodedImage, AppError> {
    let format = image_format(validated)?;
    // The semaphore is static and never closed, so acquiring cannot fail.
    let _permit = REENCODE_PERMITS
        .acquire()
        .await
        .map_err(|_| AppError::Internal)?;

    tokio::task::spawn_blocking(move || reencode(&bytes, format))
        .await
        .map_err(|error| {
            tracing::error!(error = %error, "image re-encode task failed");
            AppError::Internal
        })?
}

fn reencode(bytes: &[u8], format: ImageFormat) -> Result<ReencodedImage, AppError> {
    let invalid = |_| AppError::InvalidFile;
    let mut decoder = reader(bytes, format).into_decoder().map_err(invalid)?;
    let orientation = decoder.orientation().map_err(invalid)?;
    let mut image = DynamicImage::from_decoder(decoder).map_err(invalid)?;
    image.apply_orientation(orientation);

    let mut output = Vec::new();
    let keep_png = format == ImageFormat::Png || image.has_alpha();
    if keep_png {
        image
            .write_to(&mut Cursor::new(&mut output), ImageFormat::Png)
            .map_err(encode_failed)?;
        return Ok(ReencodedImage {
            bytes: Bytes::from(output),
            extension: "png",
            content_type: "image/png",
        });
    }

    JpegEncoder::new_with_quality(&mut output, JPEG_QUALITY)
        .encode_image(&image.to_rgb8())
        .map_err(encode_failed)?;
    Ok(ReencodedImage {
        bytes: Bytes::from(output),
        extension: "jpg",
        content_type: "image/jpeg",
    })
}

fn encode_failed(error: image::ImageError) -> AppError {
    tracing::error!(error = %error, "image re-encode failed");
    AppError::Internal
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{GenericImageView, RgbImage};

    const JPEG: ValidatedFile = ValidatedFile {
        extension: "jpg",
        content_type: "image/jpeg",
    };
    const PNG: ValidatedFile = ValidatedFile {
        extension: "png",
        content_type: "image/png",
    };

    /// A 4×2 JPEG carrying an EXIF segment with a GPS marker and `orientation`.
    fn jpeg_with_exif(orientation: u16) -> Vec<u8> {
        let mut plain = Vec::new();
        JpegEncoder::new_with_quality(&mut plain, 90)
            .encode_image(&RgbImage::from_pixel(4, 2, image::Rgb([200, 30, 30])))
            .unwrap();

        // Big-endian TIFF header, one IFD entry: Orientation (0x0112), SHORT.
        let mut tiff = b"MM\0\x2a\0\0\0\x08\0\x01\x01\x12\0\x03\0\0\0\x01".to_vec();
        tiff.extend_from_slice(&orientation.to_be_bytes());
        tiff.extend_from_slice(&[0, 0, 0, 0, 0, 0]);
        tiff.extend_from_slice(b"GPSLatitude 4.0511N");
        let mut segment = b"Exif\0\0".to_vec();
        segment.extend_from_slice(&tiff);

        let mut jpeg = plain[..2].to_vec();
        jpeg.extend_from_slice(&[0xFF, 0xE1]);
        jpeg.extend_from_slice(&((segment.len() + 2) as u16).to_be_bytes());
        jpeg.extend_from_slice(&segment);
        jpeg.extend_from_slice(&plain[2..]);
        jpeg
    }

    fn contains(haystack: &[u8], needle: &[u8]) -> bool {
        haystack
            .windows(needle.len())
            .any(|window| window == needle)
    }

    fn crc32(bytes: &[u8]) -> u32 {
        let mut crc = !0u32;
        for byte in bytes {
            crc ^= u32::from(*byte);
            for _ in 0..8 {
                crc = if crc & 1 == 1 {
                    (crc >> 1) ^ 0xEDB8_8320
                } else {
                    crc >> 1
                };
            }
        }
        !crc
    }

    /// A PNG whose header claims `side`×`side` pixels.
    fn png_claiming(side: u32) -> Vec<u8> {
        let mut ihdr = b"IHDR".to_vec();
        ihdr.extend_from_slice(&side.to_be_bytes());
        ihdr.extend_from_slice(&side.to_be_bytes());
        ihdr.extend_from_slice(&[8, 2, 0, 0, 0]);
        let mut png = b"\x89PNG\r\n\x1a\n\0\0\0\x0d".to_vec();
        png.extend_from_slice(&ihdr);
        png.extend_from_slice(&crc32(&ihdr).to_be_bytes());
        png
    }

    #[tokio::test]
    async fn reencoding_drops_exif_and_applies_the_rotation() {
        let original = jpeg_with_exif(6);
        assert!(contains(&original, b"GPSLatitude"));

        let output = reencode_public_image(Bytes::from(original), &JPEG)
            .await
            .unwrap();

        assert!(!contains(&output.bytes, b"GPSLatitude"));
        assert!(!contains(&output.bytes, b"Exif"));
        let decoded = image::load_from_memory(&output.bytes).unwrap();
        assert_eq!(
            decoded.dimensions(),
            (2, 4),
            "orientation 6 turns 4×2 into 2×4"
        );
        assert_eq!(output.content_type, "image/jpeg");
    }

    #[tokio::test]
    async fn reencoding_drops_content_appended_to_the_image() {
        let mut polyglot = jpeg_with_exif(1);
        polyglot.extend_from_slice(b"<html><script>alert(1)</script>");

        let output = reencode_public_image(Bytes::from(polyglot), &JPEG)
            .await
            .unwrap();

        assert!(!contains(&output.bytes, b"<script>"));
    }

    #[tokio::test]
    async fn an_oversized_header_is_refused_before_decoding() {
        let bomb = png_claiming(60_000);

        assert!(matches!(
            check_image_dimensions(&bomb, &PNG),
            Err(AppError::InvalidFile)
        ));
        let reencoded = reencode_public_image(Bytes::from(bomb), &PNG).await;
        assert!(matches!(reencoded, Err(AppError::InvalidFile)));
    }

    #[test]
    fn a_normal_size_header_passes_the_dimension_check() {
        assert!(check_image_dimensions(&png_claiming(3000), &PNG).is_ok());
    }
}
