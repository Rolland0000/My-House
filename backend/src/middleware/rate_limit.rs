//! Generic per-IP rate-limit middleware (MH-39) — defense-in-depth request
//! cap, layered on top of (not replacing) MH-33's OTP-specific email limiter.
//!
//! Counting is fixed-window: a client may send up to 2x `max_requests` across
//! a window boundary (the tail of one window plus the head of the next).
//! Accepted at MVP scale — this is an abuse guard, not a traffic shaper.

use std::net::{IpAddr, SocketAddr};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

use axum::extract::{ConnectInfo, Request, State};
use axum::http::{header::RETRY_AFTER, HeaderMap, HeaderValue};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use ipnet::IpNet;

use crate::infra::cache::AppCacheProvider;
use crate::shared::errors::AppError;

const X_FORWARDED_FOR: &str = "x-forwarded-for";

/// Shared state for [`rate_limit`], built once at startup from `AppConfig`.
///
/// The counter store is injected as an [`AppCacheProvider`] so the middleware
/// stays independent of the cache engine (see `infra/cache`).
#[derive(Clone)]
pub struct RateLimitState {
    max_requests: u32,
    window_seconds: u64,
    trusted_proxies: Arc<Vec<IpNet>>,
    counters: Arc<dyn AppCacheProvider<IpAddr, Arc<AtomicU32>>>,
}

impl RateLimitState {
    pub fn new(
        max_requests: u32,
        window: Duration,
        trusted_proxies: Vec<IpNet>,
        counters: Arc<dyn AppCacheProvider<IpAddr, Arc<AtomicU32>>>,
    ) -> Self {
        Self {
            max_requests,
            window_seconds: window.as_secs(),
            trusted_proxies: Arc::new(trusted_proxies),
            counters,
        }
    }

    fn is_trusted(&self, ip: IpAddr) -> bool {
        self.trusted_proxies.iter().any(|net| net.contains(&ip))
    }

    /// Increments the fixed-window counter for `key` and reports whether this
    /// request is still within `max_requests`.
    ///
    /// Two requests racing on a key's first hit may each seed a counter, one
    /// of which is discarded — an undercount bounded by that initial burst,
    /// after which every request shares the stored atomic.
    async fn allow(&self, key: IpAddr) -> bool {
        let counter = match self.counters.get(&key).await {
            Some(existing) => existing,
            None => {
                let seeded = Arc::new(AtomicU32::new(0));
                self.counters.insert(key, Arc::clone(&seeded)).await;
                seeded
            }
        };
        let count = counter.fetch_add(1, Ordering::Relaxed) + 1;
        count <= self.max_requests
    }

    /// Resolves the key to rate-limit on, per the trusted-proxy-list method.
    ///
    /// `X-Forwarded-For` is read only when the immediate peer is itself a
    /// trusted proxy; a direct connection therefore can never spoof its key.
    /// The header is then walked right-to-left, skipping trusted hops, so the
    /// first untrusted entry — the real client — is what gets counted.
    fn resolve_client_ip(&self, headers: &HeaderMap, peer: SocketAddr) -> IpAddr {
        let peer_ip = peer.ip();
        if !self.is_trusted(peer_ip) {
            return peer_ip;
        }

        headers
            .get(X_FORWARDED_FOR)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| {
                value
                    .rsplit(',')
                    .filter_map(|entry| entry.trim().parse::<IpAddr>().ok())
                    .find(|ip| !self.is_trusted(*ip))
            })
            .unwrap_or(peer_ip)
    }
}

/// Axum middleware: rejects with `AppError::RateLimited` (429) once a
/// client IP exceeds `max_requests` within the configured fixed window.
pub async fn rate_limit(
    State(state): State<RateLimitState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> Response {
    let ip = state.resolve_client_ip(request.headers(), peer);

    if state.allow(ip).await {
        return next.run(request).await;
    }

    tracing::warn!(client_ip = %ip, "rate limit exceeded");

    let mut response = AppError::RateLimited.into_response();
    if let Ok(value) = HeaderValue::from_str(&state.window_seconds.to_string()) {
        response.headers_mut().insert(RETRY_AFTER, value);
    }
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::extract::connect_info::MockConnectInfo;
    use axum::http::{Request as HttpRequest, StatusCode};
    use axum::routing::get;
    use axum::Router;
    use tower::ServiceExt;

    use crate::infra::cache::build_ip_rate_limit_cache;

    const NGINX: &str = "172.18.0.2";

    fn state(max_requests: u32, trusted: &[&str]) -> RateLimitState {
        state_with_window(max_requests, trusted, Duration::from_secs(60))
    }

    fn state_with_window(max_requests: u32, trusted: &[&str], window: Duration) -> RateLimitState {
        RateLimitState::new(
            max_requests,
            window,
            trusted.iter().map(|n| n.parse().unwrap()).collect(),
            build_ip_rate_limit_cache(window),
        )
    }

    fn headers_with_xff(value: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(X_FORWARDED_FOR, value.parse().unwrap());
        headers
    }

    fn peer(ip: &str) -> SocketAddr {
        SocketAddr::new(ip.parse().unwrap(), 12345)
    }

    #[test]
    fn ignores_x_forwarded_for_from_an_untrusted_peer() {
        let state = state(10, &["172.18.0.0/16"]);
        let headers = headers_with_xff("203.0.113.7");
        assert_eq!(
            state.resolve_client_ip(&headers, peer("198.51.100.1")),
            "198.51.100.1".parse::<IpAddr>().unwrap()
        );
    }

    #[test]
    fn reads_x_forwarded_for_from_a_trusted_peer() {
        let state = state(10, &["172.18.0.0/16"]);
        let headers = headers_with_xff("203.0.113.7");
        assert_eq!(
            state.resolve_client_ip(&headers, peer(NGINX)),
            "203.0.113.7".parse::<IpAddr>().unwrap()
        );
    }

    #[test]
    fn skips_trusted_hops_and_returns_the_first_untrusted_entry() {
        let state = state(10, &["172.18.0.0/16"]);
        let headers = headers_with_xff("203.0.113.7, 172.18.0.9");
        assert_eq!(
            state.resolve_client_ip(&headers, peer(NGINX)),
            "203.0.113.7".parse::<IpAddr>().unwrap()
        );
    }

    #[test]
    fn falls_back_to_peer_ip_when_trusted_header_is_missing() {
        let state = state(10, &["172.18.0.0/16"]);
        assert_eq!(
            state.resolve_client_ip(&HeaderMap::new(), peer(NGINX)),
            NGINX.parse::<IpAddr>().unwrap()
        );
    }

    #[tokio::test]
    async fn allows_requests_under_the_limit_and_blocks_the_one_that_crosses_it() {
        let state = state(2, &[]);
        let ip: IpAddr = "127.0.0.1".parse().unwrap();

        assert!(state.allow(ip).await);
        assert!(state.allow(ip).await);
        assert!(!state.allow(ip).await);
    }

    #[tokio::test]
    async fn distinct_clients_get_distinct_counters() {
        let state = state(1, &[]);
        assert!(state.allow("127.0.0.1".parse().unwrap()).await);
        assert!(state.allow("127.0.0.2".parse().unwrap()).await);
    }

    #[tokio::test]
    async fn resets_after_the_window_elapses() {
        let state = state_with_window(1, &[], Duration::from_millis(50));
        let ip: IpAddr = "127.0.0.1".parse().unwrap();

        assert!(state.allow(ip).await);
        assert!(!state.allow(ip).await);

        tokio::time::sleep(Duration::from_millis(150)).await;

        assert!(state.allow(ip).await);
    }

    /// Covers the middleware entry point itself: the branch wiring `allow`
    /// to either `next` or the 429 response.
    #[tokio::test]
    async fn middleware_passes_then_rejects_with_429_and_retry_after() {
        let app = Router::new()
            .route("/", get(|| async { "ok" }))
            .layer(axum::middleware::from_fn_with_state(
                state(1, &[]),
                rate_limit,
            ))
            .layer(MockConnectInfo(peer("127.0.0.1")));

        let first = app
            .clone()
            .oneshot(HttpRequest::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(first.status(), StatusCode::OK);

        let second = app
            .oneshot(HttpRequest::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(second.status(), StatusCode::TOO_MANY_REQUESTS);
        assert_eq!(second.headers()[RETRY_AFTER], "60");
    }
}
