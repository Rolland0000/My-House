mod moka;
mod provider;

pub use provider::AppCacheProvider;

use std::net::IpAddr;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;
use std::time::Duration;

use uuid::Uuid;

use moka::MokaStore;

use crate::shared::types::{PendingOtp, RefreshTokenId};

/// Staleness bound on an admin suspension taking effect (MH-32).
const IS_ACTIVE_STATUS_TTL: Duration = Duration::from_secs(8);

/// Grace window for concurrent double-refresh (MH-36) — a cache hit means
/// the reuse is within the window, by construction of the TTL.
const REFRESH_REUSE_GRACE_WINDOW: Duration = Duration::from_secs(5);

// Entry ceilings, sized per cache from the key space each one can see within
// its TTL. They bound memory when the key space is attacker-influenced; moka
// evicts least-recently-used entries past the cap.

/// Distinct users authenticating within an 8s window.
const IS_ACTIVE_STATUS_MAX_ENTRIES: u64 = 10_000;

/// Concurrent refreshes within a 5s grace window — inherently small.
const REFRESH_REPLAY_MAX_ENTRIES: u64 = 10_000;

/// Pending OTP challenges; keyed by caller-supplied email, so capped well
/// above expected signup volume rather than at it.
const OTP_MAX_ENTRIES: u64 = 50_000;

/// Same key space as [`OTP_MAX_ENTRIES`], over a shorter window.
const OTP_RATE_LIMIT_MAX_ENTRIES: u64 = 50_000;

/// Distinct client IPs seen within one rate-limit window (MH-39).
const IP_RATE_LIMIT_MAX_ENTRIES: u64 = 100_000;

/// Engine choice, made once at startup by the caller (`main.rs`).
pub fn build_cache_provider() -> Arc<dyn AppCacheProvider<Uuid, bool>> {
    Arc::new(MokaStore::new(
        IS_ACTIVE_STATUS_TTL,
        IS_ACTIVE_STATUS_MAX_ENTRIES,
    ))
}

/// Keyed by the just-revoked row's id, valued with the raw new
/// refresh-token string — only recoverable here, since only its hash is
/// persisted to `refresh_tokens`.
pub fn build_refresh_replay_cache() -> Arc<dyn AppCacheProvider<RefreshTokenId, String>> {
    Arc::new(MokaStore::new(
        REFRESH_REUSE_GRACE_WINDOW,
        REFRESH_REPLAY_MAX_ENTRIES,
    ))
}

/// OTP challenge cache, keyed by email. `ttl` is config-driven rather than a
/// fixed const like the two builders above.
pub fn build_otp_cache(ttl: Duration) -> Arc<dyn AppCacheProvider<String, PendingOtp>> {
    Arc::new(MokaStore::new(ttl, OTP_MAX_ENTRIES))
}

/// OTP request rate-limit cache, keyed by email — kept separate from
/// [`build_otp_cache`] so a rate-limit hit never clobbers a still-valid
/// pending code. Only key presence within `ttl` matters; the value carries
/// no information.
pub fn build_otp_rate_limit_cache(ttl: Duration) -> Arc<dyn AppCacheProvider<String, ()>> {
    Arc::new(MokaStore::new(ttl, OTP_RATE_LIMIT_MAX_ENTRIES))
}

/// Generic per-IP request counters (MH-39) — kept separate from
/// [`build_otp_rate_limit_cache`], which is the OTP business rule keyed by
/// email. The value is a shared atomic so concurrent requests increment the
/// same counter without a read-modify-write round-trip through the cache.
pub fn build_ip_rate_limit_cache(
    window: Duration,
) -> Arc<dyn AppCacheProvider<IpAddr, Arc<AtomicU32>>> {
    Arc::new(MokaStore::new(window, IP_RATE_LIMIT_MAX_ENTRIES))
}

/// Single point of entry for every in-memory cache the app keeps.
pub struct AppCache {
    is_active_status: Arc<dyn AppCacheProvider<Uuid, bool>>,
    refresh_replay: Arc<dyn AppCacheProvider<RefreshTokenId, String>>,
    otp: Arc<dyn AppCacheProvider<String, PendingOtp>>,
    otp_rate_limit: Arc<dyn AppCacheProvider<String, ()>>,
    ip_rate_limit: Arc<dyn AppCacheProvider<IpAddr, Arc<AtomicU32>>>,
}

impl AppCache {
    pub fn new(
        is_active_status: Arc<dyn AppCacheProvider<Uuid, bool>>,
        refresh_replay: Arc<dyn AppCacheProvider<RefreshTokenId, String>>,
        otp: Arc<dyn AppCacheProvider<String, PendingOtp>>,
        otp_rate_limit: Arc<dyn AppCacheProvider<String, ()>>,
        ip_rate_limit: Arc<dyn AppCacheProvider<IpAddr, Arc<AtomicU32>>>,
    ) -> Self {
        Self {
            is_active_status,
            refresh_replay,
            otp,
            otp_rate_limit,
            ip_rate_limit,
        }
    }

    pub fn is_active_status(&self) -> Arc<dyn AppCacheProvider<Uuid, bool>> {
        Arc::clone(&self.is_active_status)
    }

    pub fn refresh_replay(&self) -> Arc<dyn AppCacheProvider<RefreshTokenId, String>> {
        Arc::clone(&self.refresh_replay)
    }

    pub fn otp(&self) -> Arc<dyn AppCacheProvider<String, PendingOtp>> {
        Arc::clone(&self.otp)
    }

    /// Per-email "requested recently" marker — key presence is the whole
    /// signal, so the value carries nothing.
    pub fn otp_rate_limit(&self) -> Arc<dyn AppCacheProvider<String, ()>> {
        Arc::clone(&self.otp_rate_limit)
    }

    /// Per-IP request counters backing the generic rate-limit middleware.
    pub fn ip_rate_limit(&self) -> Arc<dyn AppCacheProvider<IpAddr, Arc<AtomicU32>>> {
        Arc::clone(&self.ip_rate_limit)
    }
}
