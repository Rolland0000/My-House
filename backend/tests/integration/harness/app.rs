#![cfg(feature = "integration")]

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use axum::body::{to_bytes, Body};
use axum::extract::connect_info::MockConnectInfo;
use axum::http::{header, HeaderMap, Method, Request, StatusCode};
use axum::Router;
use backend_my_house::app_state::AppState;
use backend_my_house::config::{
    AdminConfig, AppConfig, AppEnv, AuthConfig, RateLimitConfig, SmtpConfig, SmtpSecurity,
    StorageConfig, StorageProvider as StorageEngine,
};
use backend_my_house::infra::cache::{build_cache_provider, AppCache};
use backend_my_house::infra::mailer::Mailer;
use backend_my_house::infra::storage::{LocalFsStorage, StorageProvider};
use backend_my_house::route::build_router;
use backend_my_house::shared::session_cookie::REFRESH_TOKEN_COOKIE;
use bytes::Bytes;
use serde_json::Value;
use sqlx::PgPool;
use tempfile::TempDir;
use tower::ServiceExt;

use super::cache::TestCache;

pub const TEST_JWT_SECRET: &str = "integration-test-jwt-secret-0123456789abcdef0123456789";
pub const TEST_ACCESS_TTL_SECONDS: u64 = 900;
/// Every request comes from this address (TEST-NET-3), as the rate limiter sees it.
pub const TEST_PEER: SocketAddr =
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(203, 0, 113, 10), 40_000));
pub const TEST_PUBLIC_MEDIA_BASE_URL: &str = "http://localhost/media";
pub const TEST_ADMIN_NOTIFICATION_EMAIL: &str = "admin-notify@test.myhouse.app";

const TEST_OTP_TTL_SECONDS: u64 = 600;
const TEST_OTP_MAX_ATTEMPTS: u32 = 3;
const TEST_OTP_RATE_LIMIT_SECONDS: u64 = 60;
const TEST_REFRESH_TTL_DAYS: u64 = 30;
/// High enough that no flow test trips the generic per-IP limit by accident.
const TEST_RATE_LIMIT_MAX_REQUESTS: u32 = 100_000;
const TEST_RATE_LIMIT_WINDOW_SECONDS: u64 = 60;
const TEST_CONTACT_REVEAL_MAX_PER_WINDOW: u32 = 30;
const TEST_CONTACT_REVEAL_WINDOW_SECONDS: u64 = 28_800;
const MULTIPART_BOUNDARY: &str = "myhouse-integration-boundary";

/// Knobs a test may turn before the app is built.
#[derive(Debug, Clone, Default)]
pub struct TestAppOptions {
    /// Expiry applied to every test cache; `None` keeps entries until invalidated.
    pub cache_ttl: Option<Duration>,
    /// Uses the production `is_active` cache (8 s TTL) instead of a test cache.
    pub real_is_active_cache: bool,
}

/// The real router on a per-test database, storage directory and caches.
pub struct TestApp {
    router: Router,
    pub pool: PgPool,
    pub state: AppState,
    storage_dir: TempDir,
}

impl TestApp {
    pub fn new(pool: PgPool) -> Self {
        Self::with_options(pool, TestAppOptions::default())
    }

    pub fn with_options(pool: PgPool, options: TestAppOptions) -> Self {
        let storage_dir = TempDir::new().expect("create the temporary storage root");
        let config = test_config(&storage_dir);
        let mailer = Mailer::new(&config.smtp).expect("build the test mailer");
        let storage: Arc<dyn StorageProvider> = Arc::new(LocalFsStorage::new(
            storage_dir.path(),
            TEST_PUBLIC_MEDIA_BASE_URL,
        ));
        let state = AppState::new(
            config,
            pool.clone(),
            Arc::new(mailer),
            storage,
            test_caches(&options),
        );
        let router = build_router(state.clone()).layer(MockConnectInfo(TEST_PEER));

        Self {
            router,
            pool,
            state,
            storage_dir,
        }
    }

    pub fn config(&self) -> &AppConfig {
        self.state.config()
    }

    /// Root of the `LocalFsStorage` backing this app; storage keys resolve under it.
    pub fn storage_root(&self) -> &Path {
        self.storage_dir.path()
    }

    pub async fn send(&self, request: TestRequest) -> TestResponse {
        let response = self
            .router
            .clone()
            .oneshot(request.into_http())
            .await
            .expect("the router is infallible");
        let status = response.status();
        let headers = response.headers().clone();
        let raw = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("read the response body");
        let body = serde_json::from_slice(&raw).unwrap_or(Value::Null);
        TestResponse {
            status,
            headers,
            body,
            raw_body: raw,
        }
    }
}

fn test_config(storage_dir: &TempDir) -> AppConfig {
    AppConfig {
        app_port: 0,
        app_env: AppEnv::Development,
        database_url: String::new(),
        allowed_origins: vec!["http://localhost:5173".to_string()],
        auth: AuthConfig {
            jwt_secret: TEST_JWT_SECRET.to_string(),
            jwt_access_ttl_seconds: TEST_ACCESS_TTL_SECONDS,
            jwt_refresh_ttl_days: TEST_REFRESH_TTL_DAYS,
            otp_ttl_seconds: TEST_OTP_TTL_SECONDS,
            otp_max_attempts: TEST_OTP_MAX_ATTEMPTS,
            otp_rate_limit_seconds: TEST_OTP_RATE_LIMIT_SECONDS,
            cookie_domain: "localhost".to_string(),
        },
        storage: StorageConfig {
            provider: StorageEngine::Local,
            local_path: storage_dir.path().display().to_string(),
            public_media_base_url: TEST_PUBLIC_MEDIA_BASE_URL.to_string(),
        },
        smtp: SmtpConfig {
            host: Ipv4Addr::LOCALHOST.to_string(),
            port: closed_local_port(),
            security: SmtpSecurity::None,
            credentials: None,
            from: "noreply@test.myhouse.app".to_string(),
        },
        admin: AdminConfig {
            notification_email: TEST_ADMIN_NOTIFICATION_EMAIL.to_string(),
            bootstrap_on_startup: false,
            bootstrap_email: None,
            bootstrap_password: None,
        },
        rate_limit: RateLimitConfig {
            max_requests: TEST_RATE_LIMIT_MAX_REQUESTS,
            window_seconds: TEST_RATE_LIMIT_WINDOW_SECONDS,
            trusted_proxies: Vec::new(),
            contact_reveal_max_per_window: TEST_CONTACT_REVEAL_MAX_PER_WINDOW,
            contact_reveal_window_seconds: TEST_CONTACT_REVEAL_WINDOW_SECONDS,
        },
    }
}

/// A port nothing listens on, so every email send fails fast in its background task.
fn closed_local_port() -> u16 {
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).expect("bind an ephemeral port");
    listener.local_addr().expect("read the bound port").port()
}

fn test_caches(options: &TestAppOptions) -> AppCache {
    let ttl = options.cache_ttl;
    let is_active_status = if options.real_is_active_cache {
        build_cache_provider()
    } else {
        TestCache::new(ttl)
    };
    AppCache::new(
        is_active_status,
        TestCache::new(ttl),
        TestCache::new(ttl),
        TestCache::new(ttl),
        TestCache::new(ttl),
        TestCache::new(ttl),
        TestCache::new(ttl),
    )
}

/// One part of a `multipart/form-data` body.
pub struct MultipartPart {
    name: String,
    file: Option<(String, String)>,
    data: Vec<u8>,
}

impl MultipartPart {
    pub fn text(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            file: None,
            data: value.as_bytes().to_vec(),
        }
    }

    pub fn file(name: &str, filename: &str, content_type: &str, data: Vec<u8>) -> Self {
        Self {
            name: name.to_string(),
            file: Some((filename.to_string(), content_type.to_string())),
            data,
        }
    }
}

/// A request to the app, built fluently: `TestRequest::post(uri).bearer(&token).json(body)`;
/// other methods go through `TestRequest::new(Method::PUT, uri)`.
pub struct TestRequest {
    method: Method,
    uri: String,
    bearer: Option<String>,
    refresh_cookie: Option<String>,
    content_type: Option<String>,
    body: Vec<u8>,
}

impl TestRequest {
    pub fn new(method: Method, uri: &str) -> Self {
        Self {
            method,
            uri: uri.to_string(),
            bearer: None,
            refresh_cookie: None,
            content_type: None,
            body: Vec::new(),
        }
    }

    pub fn get(uri: &str) -> Self {
        Self::new(Method::GET, uri)
    }

    pub fn post(uri: &str) -> Self {
        Self::new(Method::POST, uri)
    }

    pub fn bearer(mut self, access_token: &str) -> Self {
        self.bearer = Some(access_token.to_string());
        self
    }

    pub fn refresh_cookie(mut self, raw_refresh_token: &str) -> Self {
        self.refresh_cookie = Some(raw_refresh_token.to_string());
        self
    }

    pub fn json(mut self, body: Value) -> Self {
        self.content_type = Some("application/json".to_string());
        self.body = serde_json::to_vec(&body).expect("serialize the JSON body");
        self
    }

    pub fn multipart(mut self, parts: Vec<MultipartPart>) -> Self {
        let mut body = Vec::new();
        for part in parts {
            body.extend_from_slice(format!("--{MULTIPART_BOUNDARY}\r\n").as_bytes());
            let disposition = match &part.file {
                Some((filename, content_type)) => format!(
                    "Content-Disposition: form-data; name=\"{}\"; filename=\"{filename}\"\r\nContent-Type: {content_type}\r\n\r\n",
                    part.name
                ),
                None => format!(
                    "Content-Disposition: form-data; name=\"{}\"\r\n\r\n",
                    part.name
                ),
            };
            body.extend_from_slice(disposition.as_bytes());
            body.extend_from_slice(&part.data);
            body.extend_from_slice(b"\r\n");
        }
        body.extend_from_slice(format!("--{MULTIPART_BOUNDARY}--\r\n").as_bytes());
        self.content_type = Some(format!(
            "multipart/form-data; boundary={MULTIPART_BOUNDARY}"
        ));
        self.body = body;
        self
    }

    fn into_http(self) -> Request<Body> {
        let mut builder = Request::builder().method(self.method).uri(self.uri);
        if let Some(access_token) = self.bearer {
            builder = builder.header(header::AUTHORIZATION, format!("Bearer {access_token}"));
        }
        if let Some(raw_refresh_token) = self.refresh_cookie {
            builder = builder.header(
                header::COOKIE,
                format!("{REFRESH_TOKEN_COOKIE}={raw_refresh_token}"),
            );
        }
        if let Some(content_type) = self.content_type {
            builder = builder.header(header::CONTENT_TYPE, content_type);
        }
        builder
            .body(Body::from(self.body))
            .expect("build the test request")
    }
}

pub struct TestResponse {
    pub status: StatusCode,
    pub headers: HeaderMap,
    /// Parsed JSON body, or `Value::Null` when the body is empty or not JSON.
    pub body: Value,
    /// The body bytes as received, for non-JSON responses such as file downloads.
    pub raw_body: Bytes,
}

impl TestResponse {
    /// Value of the cookie `name` set by this response, if any.
    pub fn set_cookie(&self, name: &str) -> Option<String> {
        let prefix = format!("{name}=");
        self.headers
            .get_all(header::SET_COOKIE)
            .iter()
            .filter_map(|value| value.to_str().ok())
            .find_map(|cookie| {
                cookie
                    .strip_prefix(&prefix)
                    .map(|rest| rest.split(';').next().unwrap_or_default().to_string())
            })
    }
}
