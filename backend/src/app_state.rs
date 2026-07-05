use std::sync::Arc;

/// Shared application state injected into every Axum handler via `axum::extract::State`.
///
/// Wrap mutable / expensive resources (DB pool, config, storage client) in
/// `Arc` so that `.clone()` on `AppState` is cheap and `Send + Sync`.
#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)] // Placeholder — fields read once db/config are wired in EP-02
    inner: Arc<Inner>,
}

#[allow(dead_code)] // Placeholder — fields added in EP-02
struct Inner {
    // TODO EP-02: add `db: PgPool` (sqlx)
    // TODO EP-02: add `config: AppConfig`
}

impl AppState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Inner {}),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
