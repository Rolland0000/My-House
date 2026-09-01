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

/// Recovers the storage key from an avatar URL previously returned by
/// `StorageProvider::upload`, or `None` when the URL does not point at a file
/// owned by `user_id`.
///
/// Anchoring on the `avatars/{user_id}/` segment rather than stripping a
/// configured base URL keeps this working when `PUBLIC_MEDIA_BASE_URL` differs
/// between the environment that stored the URL and the one reading it back.
pub fn avatar_key_from_url(url: &str, user_id: UserId) -> Option<String> {
    let anchor = format!("avatars/{user_id}/");
    let key = &url[url.rfind(&anchor)?..];
    let filename = key.strip_prefix(&anchor)?;

    // Rejects anything but a single `uuid.ext` segment — a nested path, a
    // `..` component or a trailing query string all fail one of these.
    let (stem, extension) = filename.rsplit_once('.')?;
    if extension.is_empty() || !extension.chars().all(|c| c.is_ascii_alphanumeric()) {
        return None;
    }
    Uuid::parse_str(stem).ok()?;

    Some(key.to_string())
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
    fn avatar_key_from_url_round_trips_a_generated_key() {
        let user_id = UserId::generate();
        let key = avatar_key(user_id, "png");
        let url = format!("http://localhost/media/{key}");

        assert_eq!(avatar_key_from_url(&url, user_id), Some(key));
    }

    #[test]
    fn avatar_key_from_url_rejects_a_url_owned_by_another_user() {
        let owner = UserId::generate();
        let other = UserId::generate();
        let url = format!("http://localhost/media/{}", avatar_key(other, "jpg"));

        assert_eq!(avatar_key_from_url(&url, owner), None);
    }

    #[test]
    fn avatar_key_from_url_rejects_malformed_filenames() {
        let user_id = UserId::generate();
        let filename = Uuid::new_v4();
        for suffix in [
            "nested/photo.jpg",
            "../../etc/passwd.jpg",
            &format!("{filename}"),
            &format!("{filename}."),
            "not-a-uuid.jpg",
            &format!("{filename}.jpg?x=../evil.jpg"),
        ] {
            let url = format!("http://localhost/media/avatars/{user_id}/{suffix}");
            assert_eq!(
                avatar_key_from_url(&url, user_id),
                None,
                "expected {suffix:?} to be rejected"
            );
        }
    }

    #[test]
    fn avatar_key_from_url_rejects_a_url_without_the_avatar_prefix() {
        let user_id = UserId::generate();
        let url = format!(
            "http://localhost/media/{}",
            listing_media_key(ListingId::generate(), "jpg")
        );

        assert_eq!(avatar_key_from_url(&url, user_id), None);
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
