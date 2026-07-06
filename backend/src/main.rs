use std::net::SocketAddr;

use backend_my_house::{
    app_server::AppServer,
    app_state::AppState,
    config::AppConfig,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    // ── 1. Load .env (silently ignored in production/Docker where vars are
    //       injected directly) ─────────────────────────────────────────────
    // `dotenv_override` gives env-vars precedence over .env values, which is
    // the correct 12-factor behaviour when both sources are present.
    let _ = dotenvy::dotenv_override();

    // ── 2. Initialise structured logging (must come before config so that we
    //       can emit a structured error if config loading fails) ────────────
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // ── 3. Fail-fast config loading ─────────────────────────────────────────
    let config = AppConfig::from_env().unwrap_or_else(|err| {
        // tracing not yet fully wired to stderr at this stage; eprintln ensures
        // visibility even if the subscriber is misconfigured.
        eprintln!("FATAL — configuration error: {err}");
        std::process::exit(1);
    });

    let addr = SocketAddr::from(([0, 0, 0, 0], config.app_port));

    // ── 4. Build shared state and start the server ─────────────────────────
    let state = AppState::new(config);
    let server = AppServer::new(state, addr);

    if let Err(e) = server.run().await {
        eprintln!("Server failed: {e}");
        std::process::exit(1);
    }
}
