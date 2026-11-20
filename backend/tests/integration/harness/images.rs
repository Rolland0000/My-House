#![cfg(feature = "integration")]

use std::io::Cursor;

use image::codecs::jpeg::JpegEncoder;
use image::{ImageFormat, Rgb, RgbImage};

const JPEG_QUALITY: u8 = 90;
const FILL: Rgb<u8> = Rgb([200, 30, 30]);

fn solid(width: u32, height: u32) -> RgbImage {
    RgbImage::from_pixel(width, height, FILL)
}

pub fn jpeg(width: u32, height: u32) -> Vec<u8> {
    let mut bytes = Vec::new();
    JpegEncoder::new_with_quality(&mut bytes, JPEG_QUALITY)
        .encode_image(&solid(width, height))
        .expect("encode the JPEG");
    bytes
}

/// A JPEG carrying an APP1 EXIF segment with `orientation` and a GPS marker
/// string, so tests can check the upload strips both.
pub fn jpeg_with_exif(width: u32, height: u32, orientation: u16) -> Vec<u8> {
    let plain = jpeg(width, height);

    // Big-endian TIFF header, one IFD entry: Orientation (0x0112), SHORT.
    let mut tiff = b"MM\0\x2a\0\0\0\x08\0\x01\x01\x12\0\x03\0\0\0\x01".to_vec();
    tiff.extend_from_slice(&orientation.to_be_bytes());
    tiff.extend_from_slice(&[0, 0, 0, 0, 0, 0]);
    tiff.extend_from_slice(b"GPSLatitude 4.0511N");
    let mut segment = b"Exif\0\0".to_vec();
    segment.extend_from_slice(&tiff);
    let segment_length = u16::try_from(segment.len() + 2).expect("EXIF segment fits in 64 KiB");

    // The segment goes right after the SOI marker (the first two bytes).
    let mut with_exif = plain[..2].to_vec();
    with_exif.extend_from_slice(&[0xFF, 0xE1]);
    with_exif.extend_from_slice(&segment_length.to_be_bytes());
    with_exif.extend_from_slice(&segment);
    with_exif.extend_from_slice(&plain[2..]);
    with_exif
}

pub fn png(width: u32, height: u32) -> Vec<u8> {
    let mut bytes = Vec::new();
    solid(width, height)
        .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
        .expect("encode the PNG");
    bytes
}
