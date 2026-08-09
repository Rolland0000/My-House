use std::net::SocketAddr;
use std::sync::Arc;

use backend_my_house::{
    app_server::AppServer,
    app_state::AppState,
    config::{AppConfig, AppEnv},
    infra::db,
    infra::mailer::Mailer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

#[tokio::main]
async fn main() {
    // ── 1. Resolve deployment phase from the REAL environment ─────────────────
    //
    // APP_ENV must be set by the OS / Docker / CI before the process
    // starts.  It is the ONLY variable that is never loaded from a .env file.
    //
    //   APP_ENV=development      → developer machine  (loads .env)
    //   APP_ENV=staging  → staging server      (no .env)
    //   APP_ENV=prod     → production          (no .env)
    //
    // Absence defaults to `dev` so `cargo run` works without any extra setup.
    let app_env = AppEnv::from_real_env().unwrap_or_else(|err| {
        eprintln!("FATAL — APP_ENV: {err}");
        std::process::exit(1);
    });

    // ── 2. Load .env ONLY in dev ──────────────────────────────────────────────
    //
    // In staging/prod, variables are injected by the orchestrator; loading a
    // .env file would silently override them — a security and ops hazard.
    //
    // `dotenv` (non-overriding) lets real env vars take precedence over .env
    // values, which is the correct 12-factor behaviour when both sources are
    // present — it only fills in variables that aren't already set.
    if app_env.is_dev() {
        match dotenvy::dotenv() {
            Ok(path) => eprintln!("[dev] Loaded .env from {}", path.display()),
            Err(e) => eprintln!("[dev] No .env file found ({e}), using OS environment"),
        }
    }

    // ── 3. Initialise structured logging ─────────────────────────────────────
    //
    // Logging is initialised after dotenvy so that RUST_LOG from .env is
    // already in the environment when EnvFilter reads it.
    //
    // Format is JSON in staging/production (machine-parseable for whatever
    // log capture the hosting platform provides — no shipping/aggregation is
    // wired at MVP, cf. Epic "(rattrapage) Logging & Request Tracing" scope
    // boundary) and pretty/human-readable in development. Verbosity is
    // controlled exclusively via RUST_LOG — no dedicated log-level config var.
    let fmt_layer = if app_env.is_dev() {
        tracing_subscriber::fmt::layer().pretty().boxed()
    } else {
        tracing_subscriber::fmt::layer().json().boxed()
    };

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(fmt_layer)
        .init();

    // ── 4. Fail-fast config loading ───────────────────────────────────────────
    //
    // app_env is passed in so AppConfig stores it without a second env::var.
    let config = AppConfig::from_env(app_env).unwrap_or_else(|err| {
        eprintln!("FATAL — configuration error: {err}");
        std::process::exit(1);
    });

    let addr = SocketAddr::from(([0, 0, 0, 0], config.app_port));

    // ── 5. Connect to PostgreSQL and run pending migrations ──────────────────
    let db_pool = db::connect_db(&config.database_url)
        .await
        .unwrap_or_else(|err| {
            eprintln!("FATAL — database connection failed: {err}");
            std::process::exit(1);
        });
    tracing::info!("Connected to PostgreSQL — migrations applied");

    // ── 6. Build the SMTP mailer ───────────────────────────────────────────────
    let mailer = Mailer::new(&config).unwrap_or_else(|err| {
        eprintln!("FATAL — mailer configuration error: {err}");
        std::process::exit(1);
    });

    // ── 7. Build shared state and start the server ────────────────────────────
    let state = AppState::new(config, db_pool, Arc::new(mailer));
    let server = AppServer::new(state, addr);

    if let Err(e) = server.run().await {
        eprintln!("Server failed: {e}");
        std::process::exit(1);
    }
}
