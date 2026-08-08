//! Storage key builders — the single place allowed to construct `StorageProvider`
//! keys, so the public/private prefix convention (TECHNICAL_SPEC_MVP.md §3.1) can
//! never drift out of sync between modules (media, users, owner_requests).
//!
//! Every builder takes the owning entity's id and a file extension, and generates
//! the filename itself (UUID v4) — callers never supply a filename, eliminating
//! path-traversal risk from client-controlled input at the source.

use uuid::Uuid;

use super::types::{ListingId, OwnerRequestId, UserId};

/// Builds a key under the public `listings/{listing_id}/{uuid}.{ext}` prefix.
pub fn listing_media_key(listing_id: ListingId, ext: &str) -> String {
    format!("listings/{listing_id}/{}.{ext}", Uuid::new_v4())
}

/// Builds a key under the public `avatars/{user_id}/{uuid}.{ext}` prefix.
pub fn avatar_key(user_id: UserId, ext: &str) -> String {
    format!("avatars/{user_id}/{}.{ext}", Uuid::new_v4())
}

/// Builds a key under the private `owner-requests/{request_id}/{uuid}.{ext}` prefix.
pub fn owner_request_document_key(request_id: OwnerRequestId, ext: &str) -> String {
    format!("owner-requests/{request_id}/{}.{ext}", Uuid::new_v4())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Matches `prefix/{uuid}/{uuid}.{ext}` — entity id then a v4 filename, both
    /// well-formed UUIDs, joined by the expected static segments.
    fn assert_key_shape(key: &str, prefix: &str, entity_id: &str, ext: &str) {
        let expected_start = format!("{prefix}/{entity_id}/");
        assert!(
            key.starts_with(&expected_start),
            "expected {key:?} to start with {expected_start:?}"
        );
        assert!(
            key.ends_with(&format!(".{ext}")),
            "expected {key:?} to end with .{ext}"
        );

        let filename = key
            .strip_prefix(&expected_start)
            .and_then(|rest| rest.strip_suffix(&format!(".{ext}")))
            .expect("key should have the form prefix/id/uuid.ext");
        assert!(
            Uuid::parse_str(filename).is_ok(),
            "filename component {filename:?} should be a valid UUID"
        );
    }

    #[test]
    fn listing_media_key_has_expected_shape() {
        let listing_id = ListingId::generate();
        let key = listing_media_key(listing_id, "jpg");
        assert_key_shape(&key, "listings", &listing_id.to_string(), "jpg");
    }

    #[test]
    fn avatar_key_has_expected_shape() {
        let user_id = UserId::generate();
        let key = avatar_key(user_id, "png");
        assert_key_shape(&key, "avatars", &user_id.to_string(), "png");
    }

    #[test]
    fn owner_request_document_key_has_expected_shape() {
        let request_id = OwnerRequestId::generate();
        let key = owner_request_document_key(request_id, "pdf");
        assert_key_shape(&key, "owner-requests", &request_id.to_string(), "pdf");
    }

    #[test]
    fn generated_keys_are_unique_across_calls() {
        let listing_id = ListingId::generate();
        let a = listing_media_key(listing_id, "jpg");
        let b = listing_media_key(listing_id, "jpg");
        assert_ne!(a, b, "each call must generate a fresh server-side filename");
    }

    #[test]
    fn extension_is_preserved_verbatim() {
        let user_id = UserId::generate();
        let key = avatar_key(user_id, "jpeg");
        assert!(key.ends_with(".jpeg"));
    }
}
