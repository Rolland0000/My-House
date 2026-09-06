//! Refresh-token cookie attributes, in `shared/` so every module that opens,
//! rotates, or clears the session cookie (`auth`, `users`) uses an identical
//! name/domain/path — a mismatch on removal leaves the cookie stuck in the
//! browser instead of clearing it.

use axum_extra::extract::cookie::{Cookie, SameSite};

pub const REFRESH_TOKEN_COOKIE: &str = "refresh_token";
const REFRESH_TOKEN_PATH: &str = "/api/v1/auth";

pub fn build_refresh_cookie(
    raw_token: String,
    cookie_domain: String,
    ttl_days: u64,
) -> Cookie<'static> {
    Cookie::build((REFRESH_TOKEN_COOKIE, raw_token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .domain(cookie_domain)
        .path(REFRESH_TOKEN_PATH)
        .max_age(time::Duration::seconds((ttl_days * 86_400) as i64))
        .build()
}

pub fn clear_refresh_cookie(cookie_domain: String) -> Cookie<'static> {
    Cookie::build(REFRESH_TOKEN_COOKIE)
        .domain(cookie_domain)
        .path(REFRESH_TOKEN_PATH)
        .build()
}
