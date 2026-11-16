use std::collections::HashSet;
use std::fmt;
use std::sync::{Arc, Mutex, PoisonError};
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
#[derive(Debug, Clone)]
pub struct PendingOtp {
    pub code_hash: String,
    pub is_new: bool,
    /// Set once at issue: a wrong attempt never extends the code's life.
    pub expires_at: Instant,
    /// The email's attempt count, shared by every code issued to it.
    pub attempt_window: Arc<OtpAttemptWindow>,
    /// `attempt_window`'s count when this code was issued.
    pub attempts_at_issue: u32,
}

/// The two short-lived auth challenges, sharing one cache store; the key
/// namespaces ([`otp_key`] / [`registration_key`]) keep them apart.
#[derive(Debug, Clone)]
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

/// OTP verification attempts on one email since its last successful sign-in,
/// counted across every code it is issued so a new code never resets them,
/// and when its last code was issued.
#[derive(Debug, Default)]
pub struct OtpAttemptWindow {
    state: Mutex<OtpAttemptState>,
}

#[derive(Debug, Default)]
struct OtpAttemptState {
    attempts: u32,
    last_code_issued_at: Option<Instant>,
}

impl OtpAttemptWindow {
    // A panicking holder cannot leave the state half-written, so poison is ignored.
    fn state(&self) -> std::sync::MutexGuard<'_, OtpAttemptState> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Counts one attempt before the code is compared and returns the total,
    /// so concurrent guesses can never exceed a code's budget.
    pub fn reserve_attempt(&self) -> u32 {
        let mut state = self.state();
        state.attempts = state.attempts.saturating_add(1);
        state.attempts
    }

    pub fn attempts(&self) -> u32 {
        self.state().attempts
    }

    pub fn record_code_issued(&self, at: Instant) {
        self.state().last_code_issued_at = Some(at);
    }

    pub fn last_code_issued_at(&self) -> Option<Instant> {
        self.state().last_code_issued_at
    }

    /// Clears the attempts and the last issue time after a successful sign-in.
    pub fn reset(&self) {
        *self.state() = OtpAttemptState::default();
    }
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
    fn otp_attempt_window_counts_reservations_and_resets() {
        let window = OtpAttemptWindow::default();

        assert_eq!(window.reserve_attempt(), 1);
        assert_eq!(window.reserve_attempt(), 2);
        assert_eq!(window.attempts(), 2);
        window.record_code_issued(Instant::now());
        window.reset();
        assert_eq!(window.attempts(), 0);
        assert_eq!(window.last_code_issued_at(), None);
    }

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
