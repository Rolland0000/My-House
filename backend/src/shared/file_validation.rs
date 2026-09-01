//! Magic-byte file validation, shared by every upload path (avatars, listing
//! photos, owner-request documents).
//!
//! The declared `Content-Type` and the client filename are never inputs here:
//! the format is read from the bytes themselves, and the extension handed back
//! to the storage key builders comes from that detection alone.

use crate::shared::errors::AppError;

/// Upper bound for image uploads (TECHNICAL_SPEC_MVP.md §4.2).
pub const MAX_IMAGE_SIZE_BYTES: usize = 5 * 1024 * 1024;

/// Accepted image formats, as `(detected mime, canonical extension)`.
const ALLOWED_IMAGE_TYPES: [(&str, &str); 3] = [
    ("image/jpeg", "jpg"),
    ("image/png", "png"),
    ("image/webp", "webp"),
];

/// A payload whose format was identified from its own magic bytes.
pub struct ValidatedFile {
    pub extension: &'static str,
    pub content_type: &'static str,
}

/// Accepts `bytes` only if it is a non-empty, within-budget JPEG, PNG or WebP.
pub fn validate_image(bytes: &[u8], max_size_bytes: usize) -> Result<ValidatedFile, AppError> {
    if bytes.is_empty() || bytes.len() > max_size_bytes {
        return Err(AppError::InvalidFile);
    }

    let detected = infer::get(bytes).ok_or(AppError::InvalidFile)?;

    ALLOWED_IMAGE_TYPES
        .iter()
        .find(|(mime, _)| *mime == detected.mime_type())
        .map(|&(content_type, extension)| ValidatedFile {
            extension,
            content_type,
        })
        .ok_or(AppError::InvalidFile)
}

#[cfg(test)]
mod tests {
    use super::*;

    const JPEG: &[u8] = &[0xFF, 0xD8, 0xFF, 0xE0];
    const PNG: &[u8] = &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
    const WEBP: &[u8] = b"RIFF\0\0\0\0WEBP";
    const PDF: &[u8] = b"%PDF-1.7\n1 0 obj\n";
    const SHELL_SCRIPT: &[u8] = b"#!/bin/sh\nrm -rf /\n";

    #[test]
    fn accepts_supported_image_formats() {
        for (bytes, expected_extension, expected_content_type) in [
            (JPEG, "jpg", "image/jpeg"),
            (PNG, "png", "image/png"),
            (WEBP, "webp", "image/webp"),
        ] {
            let validated = validate_image(bytes, MAX_IMAGE_SIZE_BYTES)
                .unwrap_or_else(|_| panic!("{expected_extension} header should be accepted"));
            assert_eq!(validated.extension, expected_extension);
            assert_eq!(validated.content_type, expected_content_type);
        }
    }

    /// The caller's filename never reaches this function, so a non-image is
    /// rejected no matter what extension it was uploaded under.
    #[test]
    fn rejects_non_image_payloads() {
        for bytes in [PDF, SHELL_SCRIPT] {
            assert!(matches!(
                validate_image(bytes, MAX_IMAGE_SIZE_BYTES),
                Err(AppError::InvalidFile)
            ));
        }
    }

    #[test]
    fn rejects_empty_payload() {
        assert!(matches!(
            validate_image(&[], MAX_IMAGE_SIZE_BYTES),
            Err(AppError::InvalidFile)
        ));
    }

    #[test]
    fn rejects_payload_over_max_size() {
        let mut oversized = PNG.to_vec();
        oversized.resize(MAX_IMAGE_SIZE_BYTES + 1, 0);

        assert!(matches!(
            validate_image(&oversized, MAX_IMAGE_SIZE_BYTES),
            Err(AppError::InvalidFile)
        ));
    }
}
