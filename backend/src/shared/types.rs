use std::collections::HashSet;
use std::fmt;
use std::sync::{Mutex, PoisonError};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ─────────────────────────────────────────────────────────────────────────────
// NewType wrappers for domain IDs
//
// These prevent mixing up bare `Uuid` values across domain boundaries
// (e.g. passing a ListingId where a UserId is expected).
//
// Each wrapper:
//   - Derives Copy, Clone, Debug, PartialEq, Eq, Hash for ergonomic use in
//     collections, assertions, and pattern matching.
//   - Derives Serialize/Deserialize so it transparently (de)serializes as a
//     plain UUID string in JSON — no extra nesting.
//   - Implements Display for logging / tracing.
//   - Provides `new()` (wrap an existing Uuid) and `generate()` (create a v4).
// ─────────────────────────────────────────────────────────────────────────────

macro_rules! define_id_type {
    (
        $(#[$meta:meta])*
        $name:ident
    ) => {
        $(#[$meta])*
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, Hash,
            Serialize, Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            /// Wraps an existing `Uuid`.
            pub fn new(id: Uuid) -> Self {
                Self(id)
            }

            /// Generates a new random v4 UUID.
            pub fn generate() -> Self {
                Self(Uuid::new_v4())
            }

            /// Returns the inner `Uuid`.
            pub fn into_inner(self) -> Uuid {
                self.0
            }

            /// Borrows the inner `Uuid`.
            pub fn as_uuid(&self) -> &Uuid {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<Uuid> for $name {
            fn from(id: Uuid) -> Self {
                Self(id)
            }
        }

        impl From<$name> for Uuid {
            fn from(wrapper: $name) -> Uuid {
                wrapper.0
            }
        }
    };
}

define_id_type!(
    /// Unique identifier for a user.
    UserId
);

define_id_type!(
    /// Unique identifier for a listing.
    ListingId
);

define_id_type!(
    /// Unique identifier for a media attachment.
    MediaId
);

define_id_type!(
    /// Unique identifier for an owner upgrade request.
    OwnerRequestId
);

define_id_type!(
    /// Unique identifier for a refresh token.
    RefreshTokenId
);

/// A pending OTP challenge, cached by email. `is_new` is resolved once at
/// creation so verification can trust the stored value instead of
/// re-querying.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingOtp {
    pub code_hash: String,
    pub is_new: bool,
    pub attempts: u32,
}

/// The two short-lived auth challenges, sharing one cache store; the key
/// namespaces ([`otp_key`] / [`registration_key`]) keep them apart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthChallenge {
    PendingOtp(PendingOtp),
    PendingRegistration { email: String },
}

/// Cache key for the OTP challenge of `email`.
pub fn otp_key(email: &str) -> String {
    format!("otp:{email}")
}

/// Cache key for the single-use registration ticket `ticket`.
pub fn registration_key(ticket: &str) -> String {
    format!("reg:{ticket}")
}

/// The distinct listings one user revealed in the current fixed window.
#[derive(Debug)]
pub struct RevealWindow {
    started_at: Instant,
    listings: Mutex<HashSet<Uuid>>,
}

impl RevealWindow {
    pub fn start() -> Self {
        Self {
            started_at: Instant::now(),
            listings: Mutex::new(HashSet::new()),
        }
    }

    /// Records `listing_id` and returns `true`, or `false` when it is a new
    /// listing and `max_listings` are already recorded.
    pub fn admit(&self, listing_id: Uuid, max_listings: usize) -> bool {
        // A panicking holder cannot leave the set half-written, so poison is ignored.
        let mut listings = self.listings.lock().unwrap_or_else(PoisonError::into_inner);
        listings.contains(&listing_id)
            || (listings.len() < max_listings && listings.insert(listing_id))
    }

    /// Whole seconds, rounded up and at least 1, until a window of `length` ends.
    pub fn seconds_left(&self, length: Duration) -> u64 {
        let remaining = length.saturating_sub(self.started_at.elapsed());
        let rounded_up = remaining.as_secs() + u64::from(remaining.subsec_nanos() > 0);
        rounded_up.max(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reveal_window_admits_distinct_listings_up_to_the_max() {
        let window = RevealWindow::start();

        assert!(window.admit(Uuid::new_v4(), 2));
        assert!(window.admit(Uuid::new_v4(), 2));
        assert!(!window.admit(Uuid::new_v4(), 2));
    }

    #[test]
    fn reveal_window_admits_a_known_listing_without_counting_it() {
        let window = RevealWindow::start();
        let known = Uuid::new_v4();

        assert!(window.admit(known, 2));
        assert!(window.admit(known, 2));
        assert!(window.admit(Uuid::new_v4(), 2));
        assert!(window.admit(known, 2), "still admitted at the max");
    }

    #[test]
    fn reveal_window_seconds_left_rounds_up_and_never_reaches_zero() {
        let window = RevealWindow {
            started_at: Instant::now() - Duration::from_millis(1_500),
            listings: Mutex::default(),
        };

        assert_eq!(window.seconds_left(Duration::from_secs(10)), 9);
        assert_eq!(window.seconds_left(Duration::from_secs(1)), 1);
    }

    #[test]
    fn test_generate_produces_unique_ids() {
        let a = UserId::generate();
        let b = UserId::generate();
        assert_ne!(a, b);
    }

    #[test]
    fn test_roundtrip_via_uuid() {
        let raw = Uuid::new_v4();
        let id = ListingId::new(raw);
        assert_eq!(id.into_inner(), raw);
    }

    #[test]
    fn test_from_uuid_conversion() {
        let raw = Uuid::new_v4();
        let id: MediaId = raw.into();
        let back: Uuid = id.into();
        assert_eq!(raw, back);
    }

    #[test]
    fn test_display_matches_uuid() {
        let raw = Uuid::new_v4();
        let id = UserId::new(raw);
        assert_eq!(id.to_string(), raw.to_string());
    }

    #[test]
    fn test_serde_transparent_roundtrip() {
        let id = OwnerRequestId::generate();
        let json = serde_json::to_string(&id).expect("serialize");
        let parsed: OwnerRequestId = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(id, parsed);
    }

    #[test]
    fn test_different_types_are_not_interchangeable() {
        // This is a compile-time guarantee — we just assert the values
        // are distinct types that don't accidentally unify.
        let user_uuid = Uuid::new_v4();
        let listing_uuid = Uuid::new_v4();
        let _user_id = UserId::new(user_uuid);
        let _listing_id = ListingId::new(listing_uuid);
        // If someone wrote `fn foo(u: UserId)` and passed `_listing_id`,
        // it would fail to compile. This test exists as documentation.
    }
}
